//! HTTP routes: pages, the task JSON/form API, time tracking, and auth.
//!
//! API handlers accept both `application/json` (used by `app.js` fetch calls)
//! and form-encoded bodies (used by htmx), so either client works.

use std::collections::HashMap;

use askama::Template;
use axum::{
    body::Bytes,
    extract::{Extension, Path, Query, State},
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    middleware,
    response::{IntoResponse, Redirect, Response},
    routing::{get, patch, post},
    Form, Json, Router,
};
use chrono::{DateTime, Duration, Local, Utc};
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::auth::{self, AuthUser};
use crate::models::*;
use crate::AppState;

/// Build the full router. The auth middleware layer is added *before*
/// `/login` and `/static` are registered, so those two stay public.
pub fn router(state: AppState) -> Router {
    let router = Router::new()
        .route("/", get(root))
        .route("/logout", post(logout))
        .route("/b/:board_id", get(board_page))
        .route("/api/tasks", post(create_task))
        .route(
            "/api/tasks/:id",
            patch(update_task).delete(delete_task),
        )
        .route("/api/tasks/:id/card", get(task_card))
        .route("/api/tasks/:id/modal", get(task_modal))
        .route("/api/tasks/:id/move", post(move_task))
        .route("/api/tasks/:id/time", post(log_time))
        .route("/api/columns", post(create_column))
        .route(
            "/api/columns/:id",
            patch(update_column).delete(delete_column),
        )
        .route("/api/columns/:id/move", post(move_column))
        .route("/api/swimlanes", post(create_swimlane))
        .route(
            "/api/swimlanes/:id",
            patch(update_swimlane).delete(delete_swimlane),
        )
        .route("/api/swimlanes/:id/move", post(move_swimlane))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::auth_middleware,
        ))
        .route("/login", get(login_page).post(login_submit));
    // Static assets (`static/app.js`, `static/style.css`): served from disk
    // in debug builds so edits show up without a rebuild; embedded in the
    // binary in release builds so `cargo install` produces a fully
    // self-contained binary with no `./static` directory needed next to it.
    #[cfg(debug_assertions)]
    let router = router.nest_service("/static", ServeDir::new("static"));
    #[cfg(not(debug_assertions))]
    let router = router.route("/static/*path", get(serve_embedded_static));
    router.with_state(state)
}

/// Static assets embedded in release builds so the installed binary is
/// self-contained (see `router`).
#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "static/"]
struct EmbeddedStatic;

#[cfg(not(debug_assertions))]
async fn serve_embedded_static(Path(path): Path<String>) -> impl IntoResponse {
    match EmbeddedStatic::get(&path) {
        Some(file) => {
            let content_type = if path.ends_with(".js") {
                "text/javascript"
            } else if path.ends_with(".css") {
                "text/css"
            } else {
                "application/octet-stream"
            };
            ([(CONTENT_TYPE, content_type)], file.data).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// ---- Errors ----

/// Simple error type that renders as an HTTP response.
pub struct AppError {
    status: StatusCode,
    message: String,
}

impl AppError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::internal(error.to_string())
    }
}

/// Parse a request body as JSON or as a form, based on Content-Type.
async fn parse_body<T: DeserializeOwned>(
    headers: &HeaderMap,
    body: Bytes,
) -> Result<T, AppError> {
    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    if content_type.contains("application/json") {
        serde_json::from_slice(&body)
            .map_err(|error| AppError::bad_request(format!("invalid JSON: {error}")))
    } else {
        serde_urlencoded::from_bytes(&body)
            .map_err(|error| AppError::bad_request(format!("invalid form body: {error}")))
    }
}

// ---- View models for templates ----

/// Task shaped for rendering: colors/labels resolved, dates formatted.
#[derive(Debug, Clone)]
struct TaskView {
    id: String,
    name: String,
    description: String,
    size: i64,
    color_hex: &'static str,
    size_label: &'static str,
    total_minutes: i64,
    completed_at: Option<String>,
    /// "Sep 28" style rendering of `completed_at`, for cards.
    completed_display: Option<String>,
}

impl TaskView {
    fn from_row(row: &TaskRow) -> Self {
        let size = Size::from_i64(row.size);
        Self {
            id: row.id.clone(),
            name: row.name.clone(),
            description: row.description.clone(),
            size: size as i64,
            color_hex: size.color_hex(),
            size_label: size.label(),
            total_minutes: row.total_minutes,
            completed_at: row.completed_at.clone(),
            completed_display: row.completed_at.as_deref().map(format_day),
        }
    }
}

/// "Sep 28" rendering of a stored RFC3339 timestamp.
fn format_day(rfc3339: &str) -> String {
    DateTime::parse_from_rfc3339(rfc3339)
        .map(|dt| dt.with_timezone(&Local).format("%b %d").to_string())
        .unwrap_or_else(|_| rfc3339.to_string())
}

/// Grouping label for completed tasks: Today / Yesterday / "Sep 27, 2026".
fn done_group_label(rfc3339: &str) -> String {
    let today = Local::now().date_naive();
    match DateTime::parse_from_rfc3339(rfc3339)
        .map(|dt| dt.with_timezone(&Local).date_naive())
    {
        Ok(date) if date == today => "Today".to_string(),
        Ok(date) if date == today - Duration::days(1) => "Yesterday".to_string(),
        Ok(date) => date.format("%b %d, %Y").to_string(),
        Err(_) => rfc3339.to_string(),
    }
}

/// One column x swimlane cell on the board.
#[derive(Debug, Clone)]
struct CellView {
    column_id: String,
    swimlane_id: String,
    is_done: bool,
    /// Tasks for normal columns.
    tasks: Vec<TaskView>,
    /// Tasks for the Done column, grouped under date headers.
    done_groups: Vec<DoneGroup>,
}

#[derive(Debug, Clone)]
struct DoneGroup {
    label: String,
    tasks: Vec<TaskView>,
}

/// One swimlane band: its header plus one cell per column.
#[derive(Debug, Clone)]
struct BandView {
    id: String,
    name: String,
    task_count: usize,
    cells: Vec<CellView>,
}

/// Column header data: id, name, task count, optional WIP limit, done flag.
#[derive(Debug, Clone)]
struct ColumnHead {
    id: String,
    name: String,
    wip_limit: Option<i64>,
    count: usize,
    at_limit: bool,
    is_done: bool,
}

#[derive(Template)]
#[template(path = "board.html")]
struct BoardTemplate {
    board_id: String,
    board_name: String,
    username: String,
    columns: Vec<ColumnHead>,
    bands: Vec<BandView>,
}

#[derive(Template)]
#[template(path = "task_card.html")]
struct TaskCardTemplate {
    task: TaskView,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: bool,
}

#[derive(Debug, Clone)]
struct TimeEntryView {
    minutes: i64,
    note: String,
    started_display: String,
}

#[derive(Template)]
#[template(path = "time_entries.html")]
struct TimeEntriesTemplate {
    entries: Vec<TimeEntryView>,
}

#[derive(Template)]
#[template(path = "modal.html")]
struct ModalTemplate {
    task: TaskView,
    entries: Vec<TimeEntryView>,
    pomodoro_minutes: u32,
    is_done: bool,
}

// ---- Small DB helpers ----

/// Fetch one task with its total logged minutes, shaped for templates.
async fn fetch_task_view(pool: &SqlitePool, id: &str) -> Result<Option<TaskView>, AppError> {
    let row: Option<TaskRow> = sqlx::query_as(
        "SELECT t.id, t.column_id, t.swimlane_id, t.name, t.description, t.size,
                t.position, t.created_at, t.completed_at,
                COALESCE(te.total, 0) AS total_minutes
         FROM tasks t
         LEFT JOIN (SELECT task_id, SUM(minutes) AS total FROM time_entries GROUP BY task_id) te
                ON te.task_id = t.id
         WHERE t.id = ?1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.as_ref().map(TaskView::from_row))
}

/// Time entries for a task, newest first, with display-ready timestamps.
async fn fetch_entries(pool: &SqlitePool, task_id: &str) -> Result<Vec<TimeEntryView>, AppError> {
    let rows: Vec<TimeEntryRow> = sqlx::query_as(
        "SELECT id, task_id, minutes, note, started_at
         FROM time_entries WHERE task_id = ?1 ORDER BY started_at DESC",
    )
    .bind(task_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| TimeEntryView {
            minutes: row.minutes,
            note: row.note.clone(),
            started_display: DateTime::parse_from_rfc3339(&row.started_at)
                .map(|dt| {
                    dt.with_timezone(&Local)
                        .format("%b %d, %Y %H:%M")
                        .to_string()
                })
                .unwrap_or(row.started_at),
        })
        .collect())
}

// ---- Page handlers ----

/// Redirect to the first board.
async fn root(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
) -> Result<Response, AppError> {
    let id: Option<String> = sqlx::query_scalar("SELECT id FROM boards ORDER BY position LIMIT 1")
        .fetch_optional(&state.db.pool)
        .await?;
    match id {
        Some(id) => Ok(Redirect::to(&format!("/b/{id}")).into_response()),
        None => Err(AppError::internal("no boards found")),
    }
}

/// Full board page: columns with counts/WIP usage, swimlane bands, task cells.
async fn board_page(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(board_id): Path<String>,
) -> Result<BoardTemplate, AppError> {
    let pool = &state.db.pool;

    let board: Option<BoardRow> =
        sqlx::query_as("SELECT id, name, position FROM boards WHERE id = ?1")
            .bind(&board_id)
            .fetch_optional(pool)
            .await?;
    let board = board.ok_or_else(|| AppError::not_found("board not found"))?;

    let columns: Vec<ColumnRow> = sqlx::query_as(
        "SELECT id, board_id, name, position, wip_limit, is_done
         FROM columns WHERE board_id = ?1 ORDER BY position",
    )
    .bind(&board_id)
    .fetch_all(pool)
    .await?;

    let swimlanes: Vec<SwimlaneRow> = sqlx::query_as(
        "SELECT id, board_id, name, position
         FROM swimlanes WHERE board_id = ?1 ORDER BY position",
    )
    .bind(&board_id)
    .fetch_all(pool)
    .await?;

    // One query for every task on the board, ordered for rendering.
    let tasks: Vec<TaskRow> = sqlx::query_as(
        "SELECT t.id, t.column_id, t.swimlane_id, t.name, t.description, t.size,
                t.position, t.created_at, t.completed_at,
                COALESCE(te.total, 0) AS total_minutes
         FROM tasks t
         JOIN columns c ON c.id = t.column_id
         LEFT JOIN swimlanes s ON s.id = t.swimlane_id
         LEFT JOIN (SELECT task_id, SUM(minutes) AS total FROM time_entries GROUP BY task_id) te
                ON te.task_id = t.id
         WHERE c.board_id = ?1
         ORDER BY c.position, COALESCE(s.position, 9999), t.position, t.id",
    )
    .bind(&board_id)
    .fetch_all(pool)
    .await?;

    let column_heads: Vec<ColumnHead> = columns
        .iter()
        .map(|col| {
            let count = tasks.iter().filter(|t| t.column_id == col.id).count();
            let at_limit = col
                .wip_limit
                .map(|limit| count as i64 >= limit)
                .unwrap_or(false);
            ColumnHead {
                id: col.id.clone(),
                name: col.name.clone(),
                wip_limit: col.wip_limit,
                count,
                at_limit,
                is_done: col.is_done,
            }
        })
        .collect();

    let mut bands = Vec::new();
    for lane in &swimlanes {
        let mut cells = Vec::new();
        for col in &columns {
            let is_done = col.is_done;
            let cell_tasks: Vec<TaskView> = tasks
                .iter()
                .filter(|t| {
                    t.column_id == col.id && t.swimlane_id.as_deref() == Some(lane.id.as_str())
                })
                .map(TaskView::from_row)
                .collect();

            // Done-column tasks group under Today / Yesterday / date headers.
            let mut done_groups: Vec<DoneGroup> = Vec::new();
            if is_done {
                for task in &cell_tasks {
                    let label = task
                        .completed_at
                        .as_deref()
                        .map(done_group_label)
                        .unwrap_or_else(|| "Completed".to_string());
                    match done_groups.last_mut() {
                        Some(group) if group.label == label => group.tasks.push(task.clone()),
                        _ => done_groups.push(DoneGroup {
                            label,
                            tasks: vec![task.clone()],
                        }),
                    }
                }
            }

            cells.push(CellView {
                column_id: col.id.clone(),
                swimlane_id: lane.id.clone(),
                is_done,
                tasks: if is_done {
                    Vec::new()
                } else {
                    cell_tasks
                },
                done_groups,
            });
        }
        let lane_task_count = tasks
            .iter()
            .filter(|t| t.swimlane_id.as_deref() == Some(lane.id.as_str()))
            .count();
        bands.push(BandView {
            id: lane.id.clone(),
            name: lane.name.clone(),
            task_count: lane_task_count,
            cells,
        });
    }

    Ok(BoardTemplate {
        board_id: board.id.clone(),
        board_name: board.name,
        username: user.username,
        columns: column_heads,
        bands,
    })
}

// ---- Auth pages ----

async fn login_page(Query(query): Query<HashMap<String, String>>) -> impl IntoResponse {
    LoginTemplate {
        error: query.contains_key("error"),
    }
}

#[derive(Deserialize)]
struct LoginForm {
    user: String,
    pass: String,
}

async fn login_submit(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<Response, AppError> {
    let row: Option<UserRow> =
        sqlx::query_as("SELECT id, username, password_hash FROM users WHERE username = ?1")
            .bind(form.user.trim())
            .fetch_optional(&state.db.pool)
            .await?;

    let authenticated = row
        .as_ref()
        .map(|user| auth::verify_password(&user.password_hash, &form.pass))
        .unwrap_or(false);
    if !authenticated {
        return Ok(Redirect::to("/login?error=1").into_response());
    }

    let token = auth::create_session(&state.db.pool, &row.expect("checked above").id)
        .await
        .map_err(AppError::from)?;
    Ok(auth::redirect_with_cookie("/", &auth::session_cookie(&token)))
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    if let Some(token) = auth::session_token_from_headers(&headers) {
        let _ = sqlx::query("DELETE FROM sessions WHERE token = ?1")
            .bind(&token)
            .execute(&state.db.pool)
            .await;
    }
    auth::redirect_with_cookie("/login", &auth::clear_session_cookie())
}

// ---- Task API ----

#[derive(Deserialize)]
struct CreateTaskInput {
    column_id: String,
    swimlane_id: Option<String>,
    name: String,
    size: Option<i64>,
}

/// Create a task; returns the rendered card fragment (for htmx appends).
async fn create_task(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<TaskCardTemplate, AppError> {
    let input: CreateTaskInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::bad_request("task name is required"));
    }

    let column: Option<ColumnRow> =
        sqlx::query_as("SELECT id, board_id, name, position, wip_limit, is_done FROM columns WHERE id = ?1")
            .bind(&input.column_id)
            .fetch_optional(pool)
            .await?;
    let column = column.ok_or_else(|| AppError::bad_request("unknown column"))?;

    // Default to the board's first swimlane when none is given.
    let swimlane_id = match input.swimlane_id.filter(|s| !s.is_empty()) {
        Some(id) => Some(id),
        None => {
            sqlx::query_scalar(
                "SELECT id FROM swimlanes WHERE board_id = ?1 ORDER BY position LIMIT 1",
            )
            .bind(&column.board_id)
            .fetch_optional(pool)
            .await?
        }
    };

    let size = Size::from_i64(input.size.unwrap_or(1));

    // Append at the end of the cell.
    let max_position: Option<f64> = sqlx::query_scalar(
        "SELECT MAX(position) FROM tasks
         WHERE column_id = ?1 AND IFNULL(swimlane_id, '') = IFNULL(?2, '')",
    )
    .bind(&column.id)
    .bind(&swimlane_id)
    .fetch_one(pool)
    .await?;
    let position = max_position.map(|max| max + 1.0).unwrap_or(0.0);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO tasks (id, column_id, swimlane_id, name, description, size, position, created_at)
         VALUES (?1, ?2, ?3, ?4, '', ?5, ?6, ?7)",
    )
    .bind(&id)
    .bind(&column.id)
    .bind(&swimlane_id)
    .bind(&name)
    .bind(size as i64)
    .bind(position)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(TaskCardTemplate {
        task: TaskView {
            id,
            name,
            description: String::new(),
            size: size as i64,
            color_hex: size.color_hex(),
            size_label: size.label(),
            total_minutes: 0,
            completed_at: None,
            completed_display: None,
        },
    })
}

#[derive(Deserialize)]
struct UpdateTaskInput {
    name: Option<String>,
    description: Option<String>,
    size: Option<i64>,
}

/// Patch name/description/size; returns the refreshed card fragment.
async fn update_task(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<TaskCardTemplate, AppError> {
    let input: UpdateTaskInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM tasks WHERE id = ?1")
        .bind(&id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(AppError::not_found("task not found"));
    }

    if let Some(name) = input.name {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(AppError::bad_request("task name is required"));
        }
        sqlx::query("UPDATE tasks SET name = ?1 WHERE id = ?2")
            .bind(&name)
            .bind(&id)
            .execute(pool)
            .await?;
    }
    if let Some(description) = input.description {
        sqlx::query("UPDATE tasks SET description = ?1 WHERE id = ?2")
            .bind(&description)
            .bind(&id)
            .execute(pool)
            .await?;
    }
    if let Some(size) = input.size {
        let size = Size::from_i64(size);
        sqlx::query("UPDATE tasks SET size = ?1 WHERE id = ?2")
            .bind(size as i64)
            .bind(&id)
            .execute(pool)
            .await?;
    }

    let task = fetch_task_view(pool, &id)
        .await?
        .ok_or_else(|| AppError::not_found("task not found"))?;
    Ok(TaskCardTemplate { task })
}

#[derive(Deserialize)]
struct MoveTaskInput {
    column_id: String,
    swimlane_id: Option<String>,
    position: f64,
}

/// Move a task to a new column/swimlane/position. Crossing into the Done
/// column stamps `completed_at`; crossing out clears it.
async fn move_task(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let input: MoveTaskInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let task: Option<TaskRow> = sqlx::query_as("SELECT * FROM tasks WHERE id = ?1")
        .bind(&id)
        .fetch_optional(pool)
        .await?;
    let task = task.ok_or_else(|| AppError::not_found("task not found"))?;
    let old_column_id = task.column_id.clone();
    let old_swimlane_id = task.swimlane_id.clone();

    let target: Option<ColumnRow> =
        sqlx::query_as("SELECT id, board_id, name, position, wip_limit, is_done FROM columns WHERE id = ?1")
            .bind(&input.column_id)
            .fetch_optional(pool)
            .await?;
    let target = target.ok_or_else(|| AppError::bad_request("unknown column"))?;

    let was_done: bool = sqlx::query_scalar("SELECT is_done FROM columns WHERE id = ?1")
        .bind(&old_column_id)
        .fetch_optional(pool)
        .await?
        .unwrap_or(false);
    let now_done = target.is_done;

    // Keep the current swimlane when the client doesn't name one.
    let swimlane_id = input
        .swimlane_id
        .filter(|s| !s.is_empty())
        .or_else(|| old_swimlane_id.clone());

    let completed_at = if now_done && !was_done {
        Some(Utc::now().to_rfc3339())
    } else if !now_done && was_done {
        None
    } else {
        task.completed_at.clone()
    };

    sqlx::query(
        "UPDATE tasks SET column_id = ?1, swimlane_id = ?2, position = ?3, completed_at = ?4
         WHERE id = ?5",
    )
    .bind(&input.column_id)
    .bind(&swimlane_id)
    .bind(input.position)
    .bind(&completed_at)
    .bind(&id)
    .execute(pool)
    .await?;

    // Keep positions dense in both affected cells.
    renumber_cell(pool, &input.column_id, swimlane_id.as_deref()).await?;
    if input.column_id != old_column_id || swimlane_id.as_deref() != old_swimlane_id.as_deref() {
        renumber_cell(pool, &old_column_id, old_swimlane_id.as_deref()).await?;
    }

    Ok(StatusCode::OK)
}

/// Rewrite positions in one column x swimlane cell as dense 0..n ordering.
async fn renumber_cell(
    pool: &SqlitePool,
    column_id: &str,
    swimlane_id: Option<&str>,
) -> Result<(), AppError> {
    let ids: Vec<String> = sqlx::query_scalar(
        "SELECT id FROM tasks
         WHERE column_id = ?1 AND IFNULL(swimlane_id, '') = IFNULL(?2, '')
         ORDER BY position, id",
    )
    .bind(column_id)
    .bind(swimlane_id)
    .fetch_all(pool)
    .await?;
    for (index, task_id) in ids.iter().enumerate() {
        sqlx::query("UPDATE tasks SET position = ?1 WHERE id = ?2")
            .bind(index as f64)
            .bind(task_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Delete a task and its time entries.
async fn delete_task(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pool = &state.db.pool;
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM time_entries WHERE task_id = ?1")
        .bind(&id)
        .execute(&mut *tx)
        .await?;
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?1")
        .bind(&id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    if result.rows_affected() == 0 {
        return Err(AppError::not_found("task not found"));
    }
    Ok(StatusCode::OK)
}

/// Render one task's card fragment (used by htmx refreshes).
async fn task_card(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<TaskCardTemplate, AppError> {
    let task = fetch_task_view(&state.db.pool, &id)
        .await?
        .ok_or_else(|| AppError::not_found("task not found"))?;
    Ok(TaskCardTemplate { task })
}

/// Render the task-detail modal: editable fields, timer, time entries.
async fn task_modal(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<ModalTemplate, AppError> {
    let pool = &state.db.pool;
    let task = fetch_task_view(pool, &id)
        .await?
        .ok_or_else(|| AppError::not_found("task not found"))?;
    let is_done: bool =
        sqlx::query_scalar("SELECT c.is_done FROM columns c JOIN tasks t ON t.column_id = c.id WHERE t.id = ?1")
            .bind(&id)
            .fetch_optional(pool)
            .await?
            .unwrap_or(false);
    let entries = fetch_entries(pool, &id).await?;
    Ok(ModalTemplate {
        task,
        entries,
        pomodoro_minutes: state.pomodoro_minutes,
        is_done,
    })
}

#[derive(Deserialize)]
struct LogTimeInput {
    minutes: i64,
    note: Option<String>,
}

/// Log minutes on a task; returns the refreshed time-entries fragment.
async fn log_time(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<TimeEntriesTemplate, AppError> {
    let input: LogTimeInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    if !(1..=24 * 60).contains(&input.minutes) {
        return Err(AppError::bad_request("minutes must be between 1 and 1440"));
    }
    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM tasks WHERE id = ?1")
        .bind(&id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(AppError::not_found("task not found"));
    }

    sqlx::query(
        "INSERT INTO time_entries (id, task_id, minutes, note, started_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&id)
    .bind(input.minutes)
    .bind(input.note.unwrap_or_default())
    .bind(Utc::now().to_rfc3339())
    .execute(pool)
    .await?;

    Ok(TimeEntriesTemplate {
        entries: fetch_entries(pool, &id).await?,
    })
}

// ---- Column & swimlane management ----

/// Resolve the board for a create call: an explicit id when given,
/// otherwise the first board (mirrors the `/` redirect).
async fn resolve_board(pool: &SqlitePool, board_id: Option<String>) -> Result<String, AppError> {
    if let Some(id) = board_id.filter(|s| !s.is_empty()) {
        let exists: Option<String> = sqlx::query_scalar("SELECT id FROM boards WHERE id = ?1")
            .bind(&id)
            .fetch_optional(pool)
            .await?;
        return exists.ok_or_else(|| AppError::bad_request("unknown board"));
    }
    sqlx::query_scalar("SELECT id FROM boards ORDER BY position LIMIT 1")
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::internal("no boards found"))
}

#[derive(Deserialize)]
struct CreateColumnInput {
    board_id: Option<String>,
    name: String,
    wip_limit: Option<i64>,
}

/// Create a column at the end of the board's column order; returns its id.
async fn create_column(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<serde_json::Value>, AppError> {
    let input: CreateColumnInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::bad_request("column name is required"));
    }
    if let Some(limit) = input.wip_limit {
        if limit < 1 {
            return Err(AppError::bad_request("WIP limit must be at least 1"));
        }
    }

    let board_id = resolve_board(pool, input.board_id).await?;
    let max_position: Option<i64> =
        sqlx::query_scalar("SELECT MAX(position) FROM columns WHERE board_id = ?1")
            .bind(&board_id)
            .fetch_one(pool)
            .await?;

    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO columns (id, board_id, name, position, wip_limit, is_done)
         VALUES (?1, ?2, ?3, ?4, ?5, 0)",
    )
    .bind(&id)
    .bind(&board_id)
    .bind(&name)
    .bind(max_position.map(|max| max + 1).unwrap_or(0))
    .bind(input.wip_limit)
    .execute(pool)
    .await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)]
struct UpdateColumnInput {
    name: Option<String>,
    /// `Some(None)` (JSON null) clears the limit; absent leaves it alone.
    wip_limit: Option<Option<i64>>,
    is_done: Option<bool>,
}

/// Rename a column, set/clear its WIP limit, and/or flip its done flag.
async fn update_column(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let input: UpdateColumnInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM columns WHERE id = ?1")
        .bind(&id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(AppError::not_found("column not found"));
    }

    if let Some(name) = input.name {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(AppError::bad_request("column name is required"));
        }
        sqlx::query("UPDATE columns SET name = ?1 WHERE id = ?2")
            .bind(&name)
            .bind(&id)
            .execute(pool)
            .await?;
    }
    if let Some(wip_limit) = input.wip_limit {
        if let Some(limit) = wip_limit {
            if limit < 1 {
                return Err(AppError::bad_request("WIP limit must be at least 1"));
            }
        }
        sqlx::query("UPDATE columns SET wip_limit = ?1 WHERE id = ?2")
            .bind(wip_limit)
            .bind(&id)
            .execute(pool)
            .await?;
    }
    if let Some(is_done) = input.is_done {
        sqlx::query("UPDATE columns SET is_done = ?1 WHERE id = ?2")
            .bind(is_done)
            .bind(&id)
            .execute(pool)
            .await?;
    }
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
struct MoveColumnInput {
    position: i64,
}

/// Reorder a column within its board; positions are renumbered densely.
async fn move_column(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let input: MoveColumnInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let column: Option<ColumnRow> = sqlx::query_as(
        "SELECT id, board_id, name, position, wip_limit, is_done FROM columns WHERE id = ?1",
    )
    .bind(&id)
    .fetch_optional(pool)
    .await?;
    let column = column.ok_or_else(|| AppError::not_found("column not found"))?;

    let mut ids: Vec<String> = sqlx::query_scalar(
        "SELECT id FROM columns WHERE board_id = ?1 ORDER BY position, id",
    )
    .bind(&column.board_id)
    .fetch_all(pool)
    .await?;
    ids.retain(|column_id| column_id != &id);
    let at = input.position.clamp(0, ids.len() as i64) as usize;
    ids.insert(at, id.clone());
    for (index, column_id) in ids.iter().enumerate() {
        sqlx::query("UPDATE columns SET position = ?1 WHERE id = ?2")
            .bind(index as i64)
            .bind(column_id)
            .execute(pool)
            .await?;
    }
    Ok(StatusCode::OK)
}

/// Delete a column. Refuses with 400 while it still holds tasks.
async fn delete_column(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pool = &state.db.pool;

    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM columns WHERE id = ?1")
        .bind(&id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(AppError::not_found("column not found"));
    }
    let task_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks WHERE column_id = ?1")
        .bind(&id)
        .fetch_one(pool)
        .await?;
    if task_count > 0 {
        return Err(AppError::bad_request(format!(
            "cannot delete column with {task_count} task(s); move or delete them first"
        )));
    }
    sqlx::query("DELETE FROM columns WHERE id = ?1")
        .bind(&id)
        .execute(pool)
        .await?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
struct CreateSwimlaneInput {
    board_id: Option<String>,
    name: String,
}

/// Create a swimlane at the end of the board's swimlane order; returns its id.
async fn create_swimlane(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<serde_json::Value>, AppError> {
    let input: CreateSwimlaneInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::bad_request("swimlane name is required"));
    }

    let board_id = resolve_board(pool, input.board_id).await?;
    let max_position: Option<i64> =
        sqlx::query_scalar("SELECT MAX(position) FROM swimlanes WHERE board_id = ?1")
            .bind(&board_id)
            .fetch_one(pool)
            .await?;

    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO swimlanes (id, board_id, name, position) VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(&id)
    .bind(&board_id)
    .bind(&name)
    .bind(max_position.map(|max| max + 1).unwrap_or(0))
    .execute(pool)
    .await?;
    Ok(Json(serde_json::json!({ "id": id })))
}

#[derive(Deserialize)]
struct UpdateSwimlaneInput {
    name: Option<String>,
}

/// Rename a swimlane.
async fn update_swimlane(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let input: UpdateSwimlaneInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let name = input
        .name
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| AppError::bad_request("swimlane name is required"))?;
    let result = sqlx::query("UPDATE swimlanes SET name = ?1 WHERE id = ?2")
        .bind(&name)
        .bind(&id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::not_found("swimlane not found"));
    }
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
struct MoveSwimlaneInput {
    position: i64,
}

/// Reorder a swimlane within its board; positions are renumbered densely.
async fn move_swimlane(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let input: MoveSwimlaneInput = parse_body(&headers, body).await?;
    let pool = &state.db.pool;

    let lane: Option<SwimlaneRow> =
        sqlx::query_as("SELECT id, board_id, name, position FROM swimlanes WHERE id = ?1")
            .bind(&id)
            .fetch_optional(pool)
            .await?;
    let lane = lane.ok_or_else(|| AppError::not_found("swimlane not found"))?;

    let mut ids: Vec<String> = sqlx::query_scalar(
        "SELECT id FROM swimlanes WHERE board_id = ?1 ORDER BY position, id",
    )
    .bind(&lane.board_id)
    .fetch_all(pool)
    .await?;
    ids.retain(|lane_id| lane_id != &id);
    let at = input.position.clamp(0, ids.len() as i64) as usize;
    ids.insert(at, id.clone());
    for (index, lane_id) in ids.iter().enumerate() {
        sqlx::query("UPDATE swimlanes SET position = ?1 WHERE id = ?2")
            .bind(index as i64)
            .bind(lane_id)
            .execute(pool)
            .await?;
    }
    Ok(StatusCode::OK)
}

/// Delete a swimlane. Refuses with 400 while it still holds tasks.
async fn delete_swimlane(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let pool = &state.db.pool;

    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM swimlanes WHERE id = ?1")
        .bind(&id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(AppError::not_found("swimlane not found"));
    }
    let task_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks WHERE swimlane_id = ?1")
        .bind(&id)
        .fetch_one(pool)
        .await?;
    if task_count > 0 {
        return Err(AppError::bad_request(format!(
            "cannot delete swimlane with {task_count} task(s); move or delete them first"
        )));
    }
    sqlx::query("DELETE FROM swimlanes WHERE id = ?1")
        .bind(&id)
        .execute(pool)
        .await?;
    Ok(StatusCode::OK)
}
