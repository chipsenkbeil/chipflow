# Builder evidence — batch-22: KF-337 Burndown + KF-339 Cumulative flow reports

## What was built

KF-337 and KF-339 were unimplemented Reports-menu destinations: the menu
items rendered (`data-report="burndown"` / `data-report="flow"`), but
`BoardReports.reportAction`'s `default:` branch showed a "not available yet"
toast. Both are now functional full-page reports, following the KF-346/343/344
pattern (full-page template + route + client-side rendering from the shared
`GET /api/boards/:id/export-data` dataset — no new endpoints).

KanbanFlow reference (kanbanflow.com/analytics-reporting):
- **Burndown**: "The Burndown chart shows how many tasks remain to be done.
  Track progress over time and predict when all work will be done."
- **Cumulative flow**: "The Cumulative Flow diagram shows how many tasks
  there are in different columns over time. Use the diagram to analyze
  your workflow and identify bottlenecks."

### KF-337 — Burndown (`c9887f3`)
- `GET /b/:board_id/burndown` → `BurndownTemplate` (`templates/burndown.html`).
- `BurndownPage` (static/app.js): computes remaining tasks per day over the
  last 30 days — created on/before day-end AND not completed (completed_at
  null or after day-end). Renders an inline-SVG line chart reusing the
  `.line-chart`/`.chart-grid`/`.chart-tick`/`.chart-axis-label`/`.chart-line`/
  `.chart-dot` classes: y-axis "Tasks remaining", x labels "1. Jan" style
  every 5 days, per-dot tooltips. No chart library.
- `reportAction` gains `case 'burndown':` → `/b/:id/burndown` (covers both
  the board-menu Reports submenu and the topbar reports menu).
- Status line: "N tasks remaining." / "No tasks on this board."
- Docs updated in `agents.md`/`agents/skill.md`.

### KF-339 — Cumulative flow (`ab4823e`)
- `GET /b/:board_id/cumulative-flow` → `CumulativeFlowTemplate`
  (`templates/cumulative_flow.html`).
- `CumulativeFlowPage` (static/app.js): reconstructs each task's column
  timeline from its history trail — "created" events ("Task created in
  {name}") give the initial column, "moved" events ("Moved from {from} to
  {to}", split on the last " to ") give transitions. Tasks with no usable
  events fall back to their current column for the whole window. Aggregates
  per-column counts at each of the last 30 day-ends and renders a stacked
  area chart (inline SVG, no library): the board's last column is the bottom
  band (canonical CFD reading — completed work grows along the bottom),
  one band per column in board order, Tableau-10 categorical palette,
  per-band tooltips, legend with color swatches. Column order = the board's
  runtime columns first, then any extra names seen in history
  (renamed/deleted columns) in first-appearance order, so the chart always
  sums to the true total.
- `reportAction` gains `case 'flow':` → `/b/:id/cumulative-flow`.
- New styles: `.cfd-legend`, `.cfd-legend-item`, `.cfd-swatch`.
- Status line: "N tasks across M columns over the last 30 days."
- Docs updated in `agents.md`/`agents/skill.md`.

### Shared
- Worktree base `6bb6cd1` + cherry-picks of `dc136fd` (KF-346), `1ad0757`
  (KF-343), `863b7fa` (KF-344) so the new reports build directly on the
  established pattern.

## Commits (worktree `.dev/worktrees/batch-22`)
- `c9887f3` KF-337: implement Burndown report (tasks remaining per day chart)
- `ab4823e` KF-339: implement Cumulative flow report (tasks per column over time)

## L0 (exact, per defect)
- `cargo fmt --check` — PASS (both)
- `cargo build` — PASS (both; Askama compiles the new templates)
- `cargo clippy --all-targets` — 0 warnings (both)
- `cargo test` — 86 passed, 0 failed (both)
- `node --check static/app.js` — PASS (both)

## Local confirmation (test instance 127.0.0.1:3236, throwaway DB, killed afterward)
- Seeded board "Burndown Test": 3 columns (To-do, In progress, Done), 5 tasks;
  2 moved to Done (completed_at stamped, "moved" history logged), 1 moved to
  In progress.
- Burndown page: SVG line chart renders 30 dots, y-axis "Tasks remaining",
  flat 0 for 29 days with a spike to 3 on the current day; status "3 tasks
  remaining." Screenshot reviewed. Client-side math independently re-checked
  in node against the live export-data: identical.
- Cumulative flow page: stacked area chart renders 3 bands (To-do/In
  progress/Done), legend with 3 swatches, y-axis "Tasks"; today stacks to
  To-do=2, In progress=1, Done=2; status "5 tasks across 3 columns over the
  last 30 days." Screenshot reviewed. Band math independently re-checked in
  node: exact match.
- Menu routing: topbar Reports menu → Burndown navigates to `/b/:id/burndown`,
  → Cumulative flow navigates to `/b/:id/cumulative-flow`; zero toasts.
- 404 on unknown board for both pages (server-side handler).
- Dark mode + 1100px viewport: no JS errors on either page (the pre-existing
  `.timer-page` dark-mode shell gap is tracked as KF-349, not a regression).

Test server killed by port PID, port confirmed free, throwaway DB removed.
Production untouched, nothing pushed, tracker not modified.

## Notes for the verifier / release path
- Both sections still show OPEN first (stale-ordering pattern) — verify
  against the deployed tree per process.
- Release note: worktree commits `c9887f3` and `ab4823e` sit on
  `6bb6cd1` + the three report cherry-picks; local master is behind
  origin/main's app tree — the release worker must cherry-pick both onto
  origin/main (batch-19/20/21 pattern), never merge.
- Design decision for follow-on report builders: Cycle & lead time (KF-340),
  Due date performance (KF-341), Time estimate (KF-345), and Calendar
  (KF-338) can all render client-side from the same export-data dataset;
  Monte Carlo (KF-342) and Board history (KF-336) are the remaining
  heavier lifts.
