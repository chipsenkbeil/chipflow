//! SQLite connection, migrations, and first-run seeding.

use std::env;
use std::path::Path;
use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use uuid::Uuid;

use crate::auth::hash_password;

/// Thin wrapper around the connection pool.
#[derive(Clone)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    /// Connect using `DATABASE_URL` (default `sqlite:./kanban.db`),
    /// creating the file (and any missing parent directories) when needed,
    /// then run migrations and seed the admin user + starter board on first run.
    pub async fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./kanban.db".to_string());

        // SQLite won't create missing parent directories itself.
        if let Some(path) = url.strip_prefix("sqlite:") {
            let path = path.split('?').next().unwrap_or(path);
            if let Some(parent) = Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)?;
                }
            }
        }

        let options = SqliteConnectOptions::from_str(&url)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Self::seed(&pool).await?;
        Ok(Self { pool })
    }

    /// First-run seeding: admin user from `ADMIN_USER`/`ADMIN_PASS`, plus the
    /// starter "General" board. Each part runs only when its table is empty,
    /// so restarting never duplicates anything.
    async fn seed(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;
        if users == 0 {
            if let (Ok(username), Ok(password)) =
                (env::var("ADMIN_USER"), env::var("ADMIN_PASS"))
            {
                if !username.trim().is_empty() && !password.is_empty() {
                    let hash = hash_password(&password)?;
                    sqlx::query(
                        "INSERT INTO users (id, username, password_hash) VALUES (?1, ?2, ?3)",
                    )
                    .bind(Uuid::new_v4().to_string())
                    .bind(username.trim())
                    .bind(hash)
                    .execute(pool)
                    .await?;
                }
            }
        }

        let boards: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM boards")
            .fetch_one(pool)
            .await?;
        if boards == 0 {
            let board_id = Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO boards (id, name, position) VALUES (?1, 'General', 0)")
                .bind(&board_id)
                .execute(pool)
                .await?;

            let columns: [(&str, Option<i64>, bool); 4] = [
                ("Work To-do", None, false),
                ("Do today", None, false),
                ("In progress", Some(3), false),
                ("Done", None, true),
            ];
            for (i, (name, wip_limit, is_done)) in columns.into_iter().enumerate() {
                sqlx::query(
                    "INSERT INTO columns (id, board_id, name, position, wip_limit, is_done)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                )
                .bind(Uuid::new_v4().to_string())
                .bind(&board_id)
                .bind(name)
                .bind(i as i64)
                .bind(wip_limit)
                .bind(is_done)
                .execute(pool)
                .await?;
            }

            for (i, name) in ["PERSONAL TO-DO", "BACKLOG"].into_iter().enumerate() {
                sqlx::query(
                    "INSERT INTO swimlanes (id, board_id, name, position) VALUES (?1, ?2, ?3, ?4)",
                )
                .bind(Uuid::new_v4().to_string())
                .bind(&board_id)
                .bind(name)
                .bind(i as i64)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }
}
