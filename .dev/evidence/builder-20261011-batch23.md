# Builder evidence — batch-23: KF-340 Cycle & lead time + KF-341 Due date performance

## What was built

KF-340 and KF-341 were unimplemented Reports-menu destinations: the menu
items rendered (`data-report="cycle"` / `data-report="due"`), but
`BoardReports.reportAction`'s `default:` branch showed a "not available yet"
toast. Both are now functional full-page reports, following the established
report pattern (KF-346/343/344/337/339): full-page Askama template + axum
route + client-side rendering from the shared `GET /api/boards/:id/export-data`
dataset — no new endpoints.

KanbanFlow reference:
- **Cycle & lead time** (kanbanflow.com/analytics-reporting): "The
  Cycle/Lead time chart shows how long it has taken for tasks to move between
  different columns in your workflow. Use it to see if you become faster or
  slower at delivering value to your customers." The product screenshot
  (CycleAndLeadTime.png, 510×260) shows: y-axis "Average time (days)",
  x-axis dates ("7. Mar" style), a green solid line with dots, a dashed gray
  "Trend (moving average)" line, and a legend ("Cycle/Lead time" + "Trend
  (moving average)").
- **Due date performance**: measures how well tasks meet their deadlines —
  on-time vs late completion, overdue, and upcoming due dates.

Definitions used (documented on the page and in agents docs):
- **Lead time** = task creation → completion.
- **Cycle time** = first column move (first `moved` history event) →
  completion; falls back to creation when the task was never moved.

### KF-340 — Cycle & lead time (`ab08624`)
- `GET /b/:board_id/cycle-lead` → `CycleLeadTemplate`
  (`templates/cycle_lead.html`).
- `CycleLeadPage` (static/app.js): computes per-completed-task cycle/lead
  days from the dataset (`created_at`, `completed_at`, `history[].kind ===
  "moved"`), buckets by completion week (12 Monday-start weeks), renders an
  inline-SVG line chart reusing the existing chart classes — green line +
  dots, y-axis "Average time (days)", x labels "1. Jan" style, per-dot
  tooltips with task counts — plus a dashed-gray 3-week centered
  moving-average trend line (`.chart-trend`), matching KanbanFlow's chart.
- Metric toggle (Cycle time / Lead time) above the chart; legend under the
  chart mirrors the KanbanFlow screenshot.
- `reportAction` gains `case 'cycle':` → `/b/:id/cycle-lead` (covers both
  Reports menus).
- New styles: `.chart-trend`, `.chart-legend*`, `.report-toggle*`.
- Status line: "N tasks completed in the last 12 weeks." / "No completed
  tasks in the last 12 weeks."
- Docs updated in `agents.md`/`agents/skill.md`.

### KF-341 — Due date performance (`0627780`)
- `GET /b/:board_id/due-date` → `DueDateTemplate`
  (`templates/due_date.html`).
- `DueDatePage` (static/app.js): classifies every task with a parseable
  `due_at` — On time (`completed_at <= due_at`, or the due date was marked
  done via `due_done`), Late (`completed_at > due_at`), Overdue (past due,
  not completed), Upcoming (future due, not completed) — and renders
  summary counts (`.report-summary`) plus a sortable-by-due-date table
  (Task | Due date | Completed | Status) reusing `.report-table`.
- `reportAction` gains `case 'due':` → `/b/:id/due-date`.
- New styles: `.report-summary*`, `.report-status-ontime/late/overdue/upcoming`.
- Status line: "X of Y tasks with due dates completed on time." / "No tasks
  with due dates on this board."
- Docs updated in `agents.md`/`agents/skill.md`.

## Commits (worktree `.dev/worktrees/batch-23`, base `6bb6cd1` + `dc136fd`/`1ad0757`/`863b7fa`)
- `ab08624` KF-340: implement Cycle & lead time report (avg time per week chart)
- `0627780` KF-341: implement Due date performance report (on-time/late/overdue table)

## L0 (exact, per defect)
- `cargo fmt --check` — PASS (both)
- `cargo build` — PASS (both; Askama compiles the new templates)
- `cargo clippy --all-targets` — 0 warnings (both)
- `cargo test` — 86 passed, 0 failed (both)
- `node --check static/app.js` — PASS (both)

## Local confirmation (test instance 127.0.0.1:3238, throwaway DB, killed afterward)
- Seeded board: 3 columns (To-do, In progress, Done w/ is_done), 9 tasks —
  2 completed via move endpoint (moved To-do → In progress → Done, with due
  dates: one past = late, one future = on time), 1 overdue, 1 upcoming, rest
  without due dates.
- Cycle & lead page: chart renders 12 dots + dashed trend line; legend
  "Cycle/Lead time" + "Trend (moving average)"; status "2 tasks completed in
  the last 12 weeks." Toggle to Lead time re-renders (12 dots). Screenshot
  reviewed — matches KanbanFlow's chart layout.
- Due date page: summary 1 on time / 1 late / 1 overdue / 1 upcoming;
  table rows sorted by due date with correct statuses; status "1 of 4 tasks
  with due dates completed on time."
- Menu navigation: both `data-report` items present in both menus; clicking
  navigates with zero toasts. 404 on unknown board for both pages.
- Dark mode: no JS errors on either page; 1100px viewport: no horizontal
  overflow (the pre-existing `.timer-page` dark-mode shell gap is tracked as
  KF-349, not a regression).
- Test server killed by port PID, port confirmed free, throwaway DB removed.

## Notes for the verifier / release path
- Same stale-ordering pattern as prior batches: both sections show OPEN first
  with older lines beneath. Confirm against the **deployed** tree per process.
- Release note: worktree base is `6bb6cd1` (+ KF-346/343/344 cherry-picks);
  local master is behind `origin/main`'s app tree — the release worker must
  cherry-pick `ab08624` and `0627780` onto origin/main (batch-19/20/21/22
  pattern), never merge.
- Design decision for follow-on report builders: the export-data dataset
  (incl. per-task `history` with machine-readable `kind` + RFC3339
  `created_at`) remains the shared source; Monte Carlo (KF-342), Board
  history (KF-336), Calendar (KF-338), and Time estimate (KF-345) can all
  render client-side from it — no new endpoints needed so far.

Production untouched, nothing pushed, tracker not modified.
