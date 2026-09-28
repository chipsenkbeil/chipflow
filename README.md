# pomodoro-kanban

A self-hosted kanban board inspired by KanbanFlow, with pomodoro-sized task
cards, WIP limits, swimlanes, a pomodoro timer that logs time entries, and a
single-admin login.

## Self-host quickstart

```sh
# Set the admin credentials (required on first run), then build and run:
ADMIN_USER=admin ADMIN_PASS='choose-a-strong-password' docker compose up --build -d
```

Open http://localhost:3000 and log in with those credentials.

Configuration (environment variables):

- `ADMIN_USER` / `ADMIN_PASS` — admin credentials, read from env on first run
  only (the argon2 hash is stored in the database afterwards).
- `POMODORO_MINUTES` — pomodoro length in minutes (default: `25`).
- `PORT` — port the server listens on (default: `3000`).
- `DATABASE_URL` — sqlite URL (default: `sqlite:data/app.db?mode=rwc`;
  docker compose sets `sqlite:/data/app.db?mode=rwc`).

The SQLite database lives in the named volume `kanban-data` (mounted at
`/data` in the container), so it survives rebuilds and restarts.

To shut down:

```sh
docker compose down
```

## Development

```sh
DATABASE_URL=sqlite:./kanban.db ADMIN_USER=admin ADMIN_PASS=secret cargo run
```

Requires Rust 1.70+ (edition 2021).

## Using it

- Drag cards between columns and swimlanes; dropping into **Done** stamps the
  completion date, dragging out clears it. The **In progress** header turns
  amber at its WIP limit of 3.
- Click a card to open its detail modal: edit name/description, pick a
  pomodoro size (card color follows), log time manually, or run the pomodoro
  timer — when it finishes, the minutes are logged on the task automatically.
- Card colors encode size: 1 pomodoro = yellow, 2 = green, 3 = blue,
  >3 = red (see the legend at the bottom of the board).

