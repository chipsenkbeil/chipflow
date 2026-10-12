# Builder evidence — batch-21: KF-343 Task count + KF-344 Throughput reports

## What was built

KF-343 and KF-344 were unimplemented Reports-menu destinations: the menu
items rendered (`data-report="count"` / `data-report="throughput"`), but
`BoardReports.reportAction`'s `default:` branch showed a "not available yet"
toast. Both are now functional full-page reports, following the KF-346
Export pattern (full-page template + route + client-side rendering from the
shared `GET /api/boards/:id/export-data` dataset — no new endpoints).

KanbanFlow reference (kanbanflow.com/analytics-reporting + screenshots):
- **Throughput**: "shows how many tasks are moving through a column" —
  a line chart, green line with dots, y-axis "Task count", x-axis weekly
  dates (screenshot: Throughput.png, 7 weekly points).
- **Task count** ("Time estimate & Task count"): "analyzing the number of
  tasks in different columns" — a table with one row per color and one
  column per board column (screenshot: TimeEstimateReport.png shows
  Color | Backlog | This week | In progress).

### KF-343 — Task count (`e786ce5`)
- `GET /b/:board_id/task-count` → `TaskCountTemplate` (`templates/task_count.html`).
- `TaskCountPage` (static/app.js): fetches the shared dataset, builds a
  color × column count table. Rows: one per color that has tasks (in first-
  appearance order) plus a "No color" row when needed; colors with no tasks
  are omitted. Columns: the board's runtime columns in order (never bolted
  to seeded names, per repo AGENTS.md).
- `reportAction` gains `case 'count':` → `/b/:id/task-count` (covers both
  the board-menu Reports submenu and the topbar reports menu).
- Styles: `.report-table`, `.report-color-dot`, `.sr-only`.

### KF-344 — Throughput (`41d4734`)
- `GET /b/:board_id/throughput` → `ThroughputTemplate` (`templates/throughput.html`).
- `ThroughputPage` (static/app.js): buckets `completed_at` into the last 12
  week buckets (Monday-start, local time; unparseable dates ignored) and
  renders an inline-SVG line chart — green line + dots, y-axis "Task count",
  x labels "1. Jan" style, tooltips per point. No chart library (vanilla JS,
  matching the app's no-dependency frontend).
- `reportAction` gains `case 'throughput':` → `/b/:id/throughput`.
- Styles: `.line-chart`, `.chart-grid`, `.chart-tick`, `.chart-axis-label`,
  `.chart-line`, `.chart-dot`.

### Shared
- `agents.md` / `agents/skill.md`: document the two new report pages as
  rendering client-side from the shared export-data dataset (repo AGENTS.md
  requires this when API-adjacent behavior changes; no new endpoints).

## Commits (worktree `.dev/worktrees/batch-21`, base `6bb6cd1` + `6900ca1` KF-346)
- `e786ce5` KF-343: implement Task count report (tasks per column by color)
- `41d4734` KF-344: implement Throughput report (tasks completed per week chart)
  (amended once: widened chart right padding so the last x label isn't clipped)

## L0 (exact, per defect)
- `cargo fmt --check` — PASS (both)
- `cargo build` — PASS (both; Askama compiles the new templates)
- `cargo clippy --all-targets` — 0 warnings (both)
- `cargo test` — 86 passed, 0 failed (both)
- `node --check static/app.js` — PASS (both)

## Local confirmation (test instance 127.0.0.1:3234, throwaway DB, killed afterward)
- Seeded board "Reports Test": 3 columns (To-do, In progress, Done), 6 tasks
  across 4 colors, 2 tasks completed via the move endpoint (completed_at stamped).
- Task count page: table renders Yellow/Green/Blue/Red × 3 columns with
  correct counts (1/1/0, 1/1/0, 0/1/1, 0/0/1); status "6 tasks." Screenshot reviewed.
- Throughput page: SVG chart renders 12 weekly buckets, y-axis "Task count",
  green line flat at 0 with a spike to 2 in the current week; status "2 tasks
  completed in the last 12 weeks." Screenshot reviewed; matches KanbanFlow's
  chart style.
- Menu navigation: topbar Reports menu → Throughput item navigates to
  `/b/:id/throughput` (no toast).
- 404 on unknown board for both pages; `export-data` still serves the dataset.
- Empty board: task-count shows "No tasks on this board." / "0 tasks.";
  throughput renders a flat zero chart / "0 tasks completed in the last 12 weeks."
- Dark mode: no JS errors on either page (the pre-existing `.timer-page`
  dark-mode shell gap is tracked as KF-349, not a regression).

Test server killed by port PID, port confirmed free, throwaway DB removed.
Production untouched, nothing pushed, tracker not modified.

## Notes for the verifier / release path
- Same stale-ordering pattern as prior batches: both sections show OPEN first
  with older lines beneath. Confirm against the **deployed** tree per process.
- Release note: worktree base is `6bb6cd1` (+ KF-346 cherry-pick `6900ca1`);
  local master is behind `origin/main`'s app tree — the release worker must
  cherry-pick `e786ce5` and `41d4734` onto origin/main (batch-19/20 pattern),
  never merge.
- Design decision for follow-on report builders: the export-data dataset
  remains the shared source; Burndown, Cumulative flow, Cycle & lead time,
  Due date performance, and Time estimate can all render from it client-side.
