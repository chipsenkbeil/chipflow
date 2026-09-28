CREATE TABLE boards (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    position INTEGER NOT NULL
);

CREATE TABLE columns (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL REFERENCES boards(id),
    name TEXT NOT NULL,
    position INTEGER NOT NULL,
    wip_limit INTEGER NULL,
    is_done INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE swimlanes (
    id TEXT PRIMARY KEY,
    board_id TEXT NOT NULL REFERENCES boards(id),
    name TEXT NOT NULL,
    position INTEGER NOT NULL
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    column_id TEXT NOT NULL REFERENCES columns(id),
    swimlane_id TEXT REFERENCES swimlanes(id),
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    size INTEGER NOT NULL DEFAULT 1 CHECK(size BETWEEN 1 AND 4),
    position REAL NOT NULL,
    created_at TEXT NOT NULL,
    completed_at TEXT NULL
);

CREATE TABLE time_entries (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    minutes INTEGER NOT NULL,
    note TEXT NOT NULL DEFAULT '',
    started_at TEXT NOT NULL
);

CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE sessions (
    token TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    created_at TEXT NOT NULL
);

CREATE INDEX idx_tasks_column_swimlane ON tasks(column_id, swimlane_id);
CREATE INDEX idx_time_entries_task ON time_entries(task_id);
