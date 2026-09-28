//! pomodoro-kanban: a small self-hosted kanban board with pomodoro time tracking.
//!
//! Configuration (environment variables):
//! - `DATABASE_PATH`: path to the redb database file, default `./data/kanban.redb`
//! - `ADMIN_USER` / `ADMIN_PASS`: single admin credentials, used on first run
//!   to create the admin user in the database
//! - `PORT`: HTTP port, default 3000
//! - `POMODORO_MINUTES`: default pomodoro length for the timer UI (25)

mod auth;
mod db;
mod models;
mod routes;

use std::env;

use crate::db::Db;

/// Shared application state: the DB pool plus server config.
#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub pomodoro_minutes: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Db::connect()?;

    let pomodoro_minutes: u32 = env::var("POMODORO_MINUTES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(25);
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3000);

    let state = AppState {
        db,
        pomodoro_minutes,
    };
    let app = routes::router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!(
        "pomodoro-kanban listening on http://0.0.0.0:{port} (pomodoro: {pomodoro_minutes} min)"
    );
    axum::serve(listener, app).await?;
    Ok(())
}
