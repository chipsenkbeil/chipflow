# Batch-22 independent verifier report — KF-337 Burndown + KF-339 Cumulative flow

**Date:** 2026-10-12 (UTC). **Verifier:** independent (not the builder).
**Built commits:** `c9887f3` (KF-337), `ab4823e` (KF-339) in worktree
`.dev/worktrees/batch-22` (base `6bb6cd1` + report cherry-picks, detached HEAD).

## Verdicts

- **KF-337 — Burndown: PRESENT-AND-CORRECT**
- **KF-339 — Cumulative flow: PRESENT-AND-CORRECT**

Neither returns to the build path. One residual note on KF-339 (renamed-column
edge) is documented below — recommended as a follow-up defect, not a failure.

## Method

- Built the worktree; L0 re-run green: `cargo fmt --check` PASS,
  `cargo build` PASS, `cargo clippy --all-targets` 0 warnings,
  `cargo test` **86 passed / 0 failed**, `node --check static/app.js` PASS.
- Test instance on 127.0.0.1:3237 with throwaway DB `/tmp/v22t/test.db`
  (`--database-path`; the `DATABASE_PATH` env var — not `CHIPFLOW_DB`).
  Admin via `/setup` (fields `username`/`password`/`confirm`), login fields
  `user`/`pass`. Killed by PID afterward; port confirmed free; DB removed.
- Seeded: board "Verify Board", columns To-do / In progress / Done
  (`POST /api/columns` takes only `board_id`+`name`; `is_done` is set via
  `PATCH /api/columns/:id` with `{"is_done":true}` — the Done stamp on
  `completed_at` only fires when the target column has `is_done=true`).
  5 tasks: Task 1 (To-do→In progress→Done, completed), Task 2 (To-do→Done,
  completed), Task 3 (To-do→In progress), Task 4/5 (To-do).
- Playwright Chromium renders at 1600×900 with computed measurements;
  screenshots visually reviewed (`/tmp/v22t/burndown.png`, `cfd.png`,
  `cfd-dark.png` — ephemeral).

## KF-337 evidence

- Both Reports-menu paths (topbar `#topbar-reports-btn` menu and board-menu →
  Reports submenu, `data-report="burndown"`) navigate to `/b/:id/burndown`
  with **zero toasts**. Old "Burndown report is not available" string is gone;
  the 2 remaining "not available" toasts in `app.js` are for other
  unimplemented reports, not burndown.
- 404 on unknown board for the page.
- 30 dots rendered; `<title>` tooltips: 29× "0 tasks remaining, …" and
  "3 tasks remaining, 12. Oct". Status line "3 tasks remaining."
- **Independent math check:** recomputed remaining-per-day from the live
  `/api/boards/:id/export-data` dataset in Python
  (`created ≤ day-end AND (completed_at null OR > day-end)`): 29 days at 0,
  today at 3 — **exact match** with the rendered series.
- Chart: y-axis "Tasks remaining" 0–5, x labels every 5 days ("13. Sep"
  style), green line + dots, per-dot tooltips.

## KF-339 evidence

- Both menu paths (`data-report="flow"`) navigate to `/b/:id/cumulative-flow`
  with zero toasts. 404 on unknown board.
- 3 bands (SVG paths) with titles "Done: up to 2 tasks",
  "In progress: up to 1 tasks", "To-do: up to 2 tasks"; legend
  "To-do / In progress / Done" with color swatches; status
  "5 tasks across 3 columns over the last 30 days."
- **Independent math check:** reconstructed each task's column timeline from
  history events in Python and aggregated per-column counts at each of the
  last 30 day-ends: all 29 prior days 0; today To-do=2, In progress=1,
  Done=2, total 5. **Decoded the rendered SVG geometry** (viewBox 720×320,
  baseline y=286, 54px/task): bottom band 286→178 = 2 tasks (Done),
  middle 178→124 = 1 task (In progress), top 124→16 = 2 tasks (To-do).
  Bands sum to the true total each day; bottom band = the board's last
  column (Done). **Exact match.**
- Chart: Tableau-10 palette bands, legend with swatches, per-band tooltips,
  "1. Jan"-style x labels. Matches KanbanFlow's described CFD
  ("tasks in different columns over time", bottleneck analysis).

## Break-attempts survived

- **Empty board:** both pages render "No tasks on this board."; burndown 30
  zero dots; CFD shows current column in legend; no JS errors.
- **Dark mode:** both charts render, no JS errors (the `.timer-page` shell's
  lack of dark variants is the pre-existing KF-349, not a regression).
- **1100px viewport:** 0px horizontal overflow on both pages.
- **Special characters:** task names with quotes/commas — no errors.
- **Renamed column** (see residual note below).
- **404s:** unknown board → 404 on both pages.

## Residual note (KF-339, not a failure)

Renaming a column ("In progress" → "Doing") leaves a **phantom band**:
the timeline is reconstructed purely from history detail *names*
(`timelineFor` in `static/app.js`), so a task whose last history event says
"Moved from To-do to In progress" is attributed to the stale "In progress"
band on the current day, while the real "Doing" band shows 0. Observed:
legend "To-do / Doing / Done / In progress", status
"5 tasks across 4 columns", titles "In progress: up to 1" + "Doing: up to 0".
The chart still sums to the true total (5) — no data is lost — and deleted
columns are handled sensibly the same way. Fix direction for a follow-up:
for the task's latest position, prefer `colNameById[task.column_id]` (the
authoritative current column) over the history-derived name. Recommended as
a new LOW follow-up defect; KanbanFlow's exact rename behavior is
unestablished, and the core contract (functional CFD destination with
correct per-day, per-column math) is met.

## L0 (worktree, exact)

- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — 86 passed, 0 failed
- `node --check static/app.js` — PASS

## Cleanup

Test server killed by PID, port 3237 confirmed free, throwaway DB removed.
Production untouched, nothing pushed, tracker not modified.

## Release note

Cherry-pick `c9887f3` and `ab4823e` onto **origin/main** (batch-19/20/21
pattern — local master `6bb6cd1` is behind origin/main's app tree); never
merge.
