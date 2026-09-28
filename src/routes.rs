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
use chrono::{DateTime, Duration, Local};
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tower_http::services::ServeDir;

use crate::auth::{self, AuthUser};
use crate::db::Db;
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
        .route("/api/tasks/:id", patch(update_task).delete(delete_task))
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

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self::internal(error.to_string())
    }
}

/// Parse a request body as JSON or as a form, based on Content-Type.
async fn parse_body<T: DeserializeOwned>(headers: &HeaderMap, body: Bytes) -> Result<T, AppError> {
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
    column_id: String,
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
            column_id: row.column_id.clone(),
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
    match DateTime::parse_from_rfc3339(rfc3339).map(|dt| dt.with_timezone(&Local).date_naive()) {
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
fn fetch_task_view(db: &Db, id: &str) -> Result<Option<TaskView>, AppError> {
    let row = db.get_task_with_minutes(id).map_err(AppError::from)?;
    Ok(row.as_ref().map(TaskView::from_row))
}

/// Time entries for a task, newest first, with display-ready timestamps.
fn fetch_entries(db: &Db, task_id: &str) -> Result<Vec<TimeEntryView>, AppError> {
    let rows = db.list_entries(task_id).map_err(AppError::from)?;
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
    let id: Option<String> = state.db.first_board_id().map_err(AppError::from)?;
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
    let db = &state.db;

    let board = db
        .get_board(&board_id)
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("board not found"))?;

    let columns = db.list_columns(&board_id).map_err(AppError::from)?;

    let swimlanes = db.list_swimlanes(&board_id).map_err(AppError::from)?;

    // One query for every task on the board, ordered for rendering.
    let tasks = db.board_tasks(&board_id).map_err(AppError::from)?;

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
                tasks: if is_done { Vec::new() } else { cell_tasks },
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
    let row = state
        .db
        .user_by_username(form.user.trim())
        .map_err(AppError::from)?;

    let authenticated = row
        .as_ref()
        .map(|user| auth::verify_password(&user.password_hash, &form.pass))
        .unwrap_or(false);
    if !authenticated {
        return Ok(Redirect::to("/login?error=1").into_response());
    }

    let token =
        auth::create_session(&state.db, &row.expect("checked above").id).map_err(AppError::from)?;
    Ok(auth::redirect_with_cookie(
        "/",
        &auth::session_cookie(&token),
    ))
}

async fn logout(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    if let Some(token) = auth::session_token_from_headers(&headers) {
        let _ = state.db.delete_session(&token);
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
    let db = &state.db;

    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::bad_request("task name is required"));
    }

    let column = db
        .get_column(&input.column_id)
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::bad_request("unknown column"))?;

    // Default to the board's first swimlane when none is given.
    let swimlane_id = match input.swimlane_id.filter(|s| !s.is_empty()) {
        Some(id) => Some(id),
        None => db
            .list_swimlanes(&column.board_id)
            .map_err(AppError::from)?
            .into_iter()
            .next()
            .map(|lane| lane.id),
    };

    let size = Size::from_i64(input.size.unwrap_or(1));
    let id = db
        .create_task(&column.id, swimlane_id.as_deref(), &name, size as i64)
        .map_err(AppError::from)?;

    Ok(TaskCardTemplate {
        task: TaskView {
            id,
            column_id: column.id.clone(),
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
    let db = &state.db;

    if db.get_task(&id).map_err(AppError::from)?.is_none() {
        return Err(AppError::not_found("task not found"));
    }

    if let Some(name) = input.name.as_deref() {
        if name.trim().is_empty() {
            return Err(AppError::bad_request("task name is required"));
        }
    }
    let size = input.size.map(Size::from_i64).map(|size| size as i64);
    db.update_task(
        &id,
        input.name.as_deref().map(str::trim),
        input.description.as_deref(),
        size,
    )
    .map_err(AppError::from)?;

    let task = fetch_task_view(db, &id)?.ok_or_else(|| AppError::not_found("task not found"))?;
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
    let db = &state.db;

    if db.get_task(&id).map_err(AppError::from)?.is_none() {
        return Err(AppError::not_found("task not found"));
    }
    if db
        .get_column(&input.column_id)
        .map_err(AppError::from)?
        .is_none()
    {
        return Err(AppError::bad_request("unknown column"));
    }

    // Keep the current swimlane when the client doesn't name one.
    let swimlane_id = input.swimlane_id.filter(|s| !s.is_empty());

    // Crossing into the Done column stamps `completed_at`; crossing out
    // clears it. Both affected cells are renumbered densely inside.
    db.move_task(
        &id,
        &input.column_id,
        swimlane_id.as_deref(),
        input.position,
    )
    .map_err(AppError::from)?;

    Ok(StatusCode::OK)
}

/// Delete a task and its time entries.
async fn delete_task(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let deleted = state.db.delete_task(&id).map_err(AppError::from)?;
    if !deleted {
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
    let task =
        fetch_task_view(&state.db, &id)?.ok_or_else(|| AppError::not_found("task not found"))?;
    Ok(TaskCardTemplate { task })
}

/// Render the task-detail modal: editable fields, timer, time entries.
async fn task_modal(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<ModalTemplate, AppError> {
    let db = &state.db;
    let task = fetch_task_view(db, &id)?.ok_or_else(|| AppError::not_found("task not found"))?;
    let is_done = db
        .get_column(&task.column_id)
        .map_err(AppError::from)?
        .map(|column| column.is_done)
        .unwrap_or(false);
    let entries = fetch_entries(db, &id)?;
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
    let db = &state.db;

    if !(1..=24 * 60).contains(&input.minutes) {
        return Err(AppError::bad_request("minutes must be between 1 and 1440"));
    }
    if db.get_task(&id).map_err(AppError::from)?.is_none() {
        return Err(AppError::not_found("task not found"));
    }

    db.create_entry(&id, input.minutes, &input.note.unwrap_or_default())
        .map_err(AppError::from)?;

    Ok(TimeEntriesTemplate {
        entries: fetch_entries(db, &id)?,
    })
}

// ---- Column & swimlane management ----

/// Resolve the board for a create call: an explicit id when given,
/// otherwise the first board (mirrors the `/` redirect).
fn resolve_board(db: &Db, board_id: Option<String>) -> Result<String, AppError> {
    if let Some(id) = board_id.filter(|s| !s.is_empty()) {
        return match db.board_exists(&id).map_err(AppError::from)? {
            true => Ok(id),
            false => Err(AppError::bad_request("unknown board")),
        };
    }
    db.first_board_id()
        .map_err(AppError::from)?
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
    let db = &state.db;

    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::bad_request("column name is required"));
    }
    if let Some(limit) = input.wip_limit {
        if limit < 1 {
            return Err(AppError::bad_request("WIP limit must be at least 1"));
        }
    }

    let board_id = resolve_board(db, input.board_id)?;
    let id = db
        .create_column(&board_id, &name, input.wip_limit)
        .map_err(AppError::from)?;
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
    let db = &state.db;

    if db.get_column(&id).map_err(AppError::from)?.is_none() {
        return Err(AppError::not_found("column not found"));
    }

    if let Some(name) = input.name.as_deref() {
        let name = name.trim();
        if name.is_empty() {
            return Err(AppError::bad_request("column name is required"));
        }
        db.rename_column(&id, name).map_err(AppError::from)?;
    }
    if let Some(wip_limit) = input.wip_limit {
        if let Some(limit) = wip_limit {
            if limit < 1 {
                return Err(AppError::bad_request("WIP limit must be at least 1"));
            }
        }
        db.set_column_wip(&id, wip_limit).map_err(AppError::from)?;
    }
    if let Some(is_done) = input.is_done {
        db.set_column_done(&id, is_done).map_err(AppError::from)?;
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
    let db = &state.db;

    if !db
        .move_column(&id, input.position)
        .map_err(AppError::from)?
    {
        return Err(AppError::not_found("column not found"));
    }
    Ok(StatusCode::OK)
}

/// Delete a column. Refuses with 400 while it still holds tasks.
async fn delete_column(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let db = &state.db;

    if db.get_column(&id).map_err(AppError::from)?.is_none() {
        return Err(AppError::not_found("column not found"));
    }
    let task_count = db.count_tasks_in_column(&id).map_err(AppError::from)?;
    if task_count > 0 {
        return Err(AppError::bad_request(format!(
            "cannot delete column with {task_count} task(s); move or delete them first"
        )));
    }
    db.delete_column(&id).map_err(AppError::from)?;
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
    let db = &state.db;

    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::bad_request("swimlane name is required"));
    }

    let board_id = resolve_board(db, input.board_id)?;
    let id = db
        .create_swimlane(&board_id, &name)
        .map_err(AppError::from)?;
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
    let db = &state.db;

    let name = input
        .name
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| AppError::bad_request("swimlane name is required"))?;
    if !db.rename_swimlane(&id, &name).map_err(AppError::from)? {
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
    let db = &state.db;

    if !db
        .move_swimlane(&id, input.position)
        .map_err(AppError::from)?
    {
        return Err(AppError::not_found("swimlane not found"));
    }
    Ok(StatusCode::OK)
}

/// Delete a swimlane. Refuses with 400 while it still holds tasks.
async fn delete_swimlane(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let db = &state.db;

    if db.get_swimlane(&id).map_err(AppError::from)?.is_none() {
        return Err(AppError::not_found("swimlane not found"));
    }
    let task_count = db.count_tasks_in_swimlane(&id).map_err(AppError::from)?;
    if task_count > 0 {
        return Err(AppError::bad_request(format!(
            "cannot delete swimlane with {task_count} task(s); move or delete them first"
        )));
    }
    db.delete_swimlane(&id).map_err(AppError::from)?;
    Ok(StatusCode::OK)
}
