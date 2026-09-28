//! Domain types shared by the DB layer, routes, and templates.

use serde::{Deserialize, Serialize};

/// Pomodoro size of a task. `Many` (stored as 4) means ">3 pomodori".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    One = 1,
    Two = 2,
    Three = 3,
    Many = 4,
}

impl Size {
    /// Convert a stored integer to a [`Size`], defaulting to [`Size::One`].
    pub fn from_i64(value: i64) -> Self {
        match value {
            2 => Size::Two,
            3 => Size::Three,
            4 => Size::Many,
            _ => Size::One,
        }
    }

    /// Card background color for this size (KanbanFlow-style pastels).
    pub fn color_hex(self) -> &'static str {
        match self {
            Size::One => "#FFF9C4",   // yellow
            Size::Two => "#C8E6C9",   // green
            Size::Three => "#BBDEFB", // blue
            Size::Many => "#FFCDD2",  // red
        }
    }

    /// Human label, e.g. "2 Pomodori".
    pub fn label(self) -> &'static str {
        match self {
            Size::One => "1 Pomodoro",
            Size::Two => "2 Pomodori",
            Size::Three => "3 Pomodori",
            Size::Many => ">3 Pomodori",
        }
    }
}

// ---- Row types: one per table, serialized as JSON values in redb ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardRow {
    pub id: String,
    pub name: String,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnRow {
    pub id: String,
    pub board_id: String,
    pub name: String,
    pub position: i64,
    pub wip_limit: Option<i64>,
    /// When true, moving a task into this column stamps `completed_at`;
    /// moving out clears it. (Replaces the old name-based "Done" check.)
    pub is_done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwimlaneRow {
    pub id: String,
    pub board_id: String,
    pub name: String,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRow {
    pub id: String,
    pub column_id: String,
    pub swimlane_id: Option<String>,
    pub name: String,
    pub description: String,
    pub size: i64,
    pub position: f64,
    pub created_at: String,
    pub completed_at: Option<String>,
    /// Aggregated by the DB layer on read; 0 for freshly created tasks.
    pub total_minutes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryRow {
    pub id: String,
    pub task_id: String,
    pub minutes: i64,
    pub note: String,
    pub started_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRow {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRow {
    pub user_id: String,
    pub created_at: String,
}
