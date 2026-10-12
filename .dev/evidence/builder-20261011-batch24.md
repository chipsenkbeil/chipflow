# Builder evidence — batch-24: KF-336 (Board history) + KF-338 (Calendar)

Session: 2026-10-11 ~22:10–23:30 CDT. Builder: Dale subagent (ef063a85).
Worktree: `.dev/worktrees/batch-24` on `6bb6cd1` + cherry-picks of the four
shipped report releases (KF-346 `d10b16c`, KF-343/344 `109aade`, KF-337/339
`4a900de`, KF-340/341 `520ac4b`) — all applied cleanly. No merge into
origin/main; the release worker cherry-picks afterward.

## KF-336 — Board history report (`f2b0935`)

**KanbanFlow reference** (kanbanflow.com/analytics-reporting, BoardHistory.png,
viewed 2026-10-11): "See a detailed log of changes that are made on the board.
Or for a specific task. Filter the history log by user or type of change.
Compare a field's value side-by-side before and after the change."
Screenshot layout: avatar + actor name left, relative timestamp right; entry
headlines ("Task X was deleted from column Backlog", "Emma added to task Y
with text" + comment body, "Task Z has changed properties: Column." +
expanded Old value|New value table (Column | Backlog | In progress), "Task W
was created in column This week").

**What was built** (files: `src/routes.rs`, `templates/board_history.html`,
`static/app.js`, `static/style.css`, `agents.md`, `agents/skill.md`):
- `GET /b/:board_id/board-history` (Askama template, `.timer-page` shell).
- `BoardHistoryPage` JS module rendering client-side from
  `/api/boards/:id/export-data`. Collects every task's `history` events,
  skipping `comment_added` (duplicates the comment bodies, which get their own
  entries with exact timestamps + text). Sorted newest-first.
- Per-kind headlines matching the reference tone: "Task X was created in
  column Y", "Task X was renamed." + `Renamed to "…"` detail, "Task X was
  moved." / "has changed properties: Column." (moved events render "was
  moved." when the detail parses as a column move, else "has changed
  properties: Column."), "Task X: logged 25m" (time), "Emma added to task X
  with text" + comment body (comments), generic "has changed properties:
  <label>." for `*_updated` kinds (due/description/color/estimate/labels/
  members) and "has changed <field>." for `due_done_updated` /
  `grouping_date_updated`.
- Old|New comparison table (expandable `<details>`) for `moved` events,
  parsed from the "Moved from X to Y" detail string — matches the reference's
  side-by-side compare.
- Actor: `.user-avatar` circle with initials + username (actor is a username;
  no separate user lookup needed). Relative timestamps: "Today HH:MM",
  "Yesterday HH:MM", "12. Oct HH:MM".
- User filter (distinct history actors) and type filter (distinct kinds)
  — the "filter by user or type of change" from the reference.
- `reportAction` case `history` → `/b/:id/board-history` (both menus).

**Known gaps**: task-deletion events cannot be shown (ChipFlow does not log
deletions; a deleted task's history is gone — KanbanFlow's "Task X was deleted
from column Backlog" has no data source). `comment_added` is consumed by the
comment entries rather than shown separately.

**L0**: `cargo fmt --check` ✓, `cargo build` ✓, `cargo clippy --all-targets`
0 warnings ✓, `cargo test` 86 passed / 0 failed ✓, `node --check` ✓.

## KF-338 — Calendar report (`5b830f2`)

**KanbanFlow reference** (same page, Calendar.png, viewed 2026-10-11): "Get a
better overview of when your tasks are due with the calendar view. Filter on
user, color, labels or swimlane. You can also show the dates in your external
calendar by subscribing to a feed." Screenshot: month grid, Mon–Sun headers;
tasks as color-coded chips on their due dates, each chip "task name" + subline
"On today → In progress" (relative due → column name).

**What was built** (same six files + `templates/calendar.html`):
- `GET /b/:board_id/calendar`. `CalendarPage` JS module, Monday-start month
  grid (weeks from the Monday of the 1st's week to the Sunday of the last
  day's week), adjacent-month days grayed, today highlighted.
- Task chips on their due-date days, background = task color value (luminance
  check for readable text), each chip: name + "{relative} → {column}" subline
  ("On today", "On tomorrow", "Yesterday", "On 12. Oct").
- Filters: user (member_ids), color (color_id), label (labels), swimlane
  (swimlane_id) — the four reference filters. Month nav: ‹ / Today / ›.
- `reportAction` case `calendar` → `/b/:id/calendar`.

**Known gaps**: the external iCal feed subscription is out of scope for this
defect (needs unauthenticated feed URLs — a security-sensitive design choice;
possible follow-up). `due_repeat` recurrences are not expanded (single chip
per task on its `due_at`).

**L0**: `cargo fmt --check` ✓, `cargo build` ✓, `cargo clippy --all-targets`
0 warnings ✓, `cargo test` 86 passed / 0 failed ✓, `node --check` ✓.

## Local confirmation (test instance 127.0.0.1:3241, throwaway redb)

Seeded board ("History Test Board"): 4 tasks; created, moved (To-do →
In progress), renamed, comment ("Draft is ready for review."), 25m time log,
due/label updates; 3 tasks with due dates (today, tomorrow, +3 days).
- Board history: rendered 12 changes newest-first; moved entry expanded to
  Column | To-do | In progress; comment body shown; type filter "moved" → "1
  change."; 404 on unknown board; menu item present in both menus.
- Calendar: October 2026 grid; 3 chips with correct sublines ("On today →
  To-do", "On tomorrow → In progress", "On 15. Oct → To-do"); today
  highlighted; ‹/Today/› nav works (Nov → Oct); 404 on unknown board; no JS
  errors in dark mode at 1100px.
- Server killed by PID; port 3241 free. No production touched.

Screenshots: /tmp/b24live/history.png, /tmp/b24live/calendar.png (ephemeral).

## Commits

- `f2b0935` KF-336: implement Board history report (chronological change log)
  — 6 files, +396/−4, new `templates/board_history.html`.
- `5b830f2` KF-338: implement Calendar report (tasks by due date on month
  grid) — 6 files, +412/−4, new `templates/calendar.html`.
