# Builder evidence — batch-25: KF-342 Monte Carlo forecasting + KF-345 Time estimate

## What was built

KF-342 and KF-345 were the last two unimplemented Reports-menu destinations:
the menu items rendered (`data-report="montecarlo"` / `data-report="estimate"`),
but `BoardReports.reportAction`'s `default:` branch showed a "not available yet"
toast. Both are now functional full-page reports, following the established
report pattern (KF-346/343/344/337/339/340/341/336/338): full-page Askama
template + axum route + client-side rendering from the shared
`GET /api/boards/:id/export-data` dataset — no new endpoints.

KanbanFlow reference (kanbanflow.com/analytics-reporting + screenshots,
viewed 2026-10-11):
- **Monte Carlo forecasting**: "Spending a lot of time on estimates? Use
  forecasts instead to predict when your tasks will be completed. The forecast
  is based on running 1000s of simulations from your historical throughput."
  The product screenshot (MonteCarloForecasting.png) shows: a left Settings
  panel (Task count, Start date, Period = "Last 90 days", Target column =
  "Done", "Show more options" checkbox, green "Run forecast" button); a
  Forecast panel ("Forecast to complete 10 tasks starting August 30",
  probability lines "January 9, 2024 (95% probability)" green / "December 19
  (85% probability)" gold / "December 2 (75% probability)" red / "November 17
  (50% probability)" purple); a "Monte Carlo chart" histogram of completion
  dates with dashed vertical percentile lines labeled 50%/75%/85%/95%; and a
  "Throughput basis" daily-throughput line chart.
- **Time estimate**: "Get an overview of the estimated workload on your
  board. Identify if someone on your team has an unrealistic amount of work
  assigned to them." The product screenshot (TimeEstimateReport.png) shows a
  color × column table (rows = colors with dots, e.g. IT/Marketing/Sales…;
  columns = Backlog/This week/In progress) with cells "23h 00m" — summed
  time estimates.

### KF-342 — Monte Carlo forecasting (`6ada8a4`)

- `GET /b/:board_id/monte-carlo` → `MonteCarloTemplate`
  (`templates/monte_carlo.html`). The handler additionally resolves the Done
  column id from runtime configuration (`list_columns` → `is_done`; never
  bolted to a column name) and passes it to the template so the target-column
  dropdown defaults to Done (falls back to the last column).
- `MonteCarloPage` (static/app.js):
  - Settings: task count (default 10), start date (default today), period
    (30/60/90 days, default 90), target column (all board columns, default
    Done), "Show more options" (reveals simulations count, default 1000),
    green "Run forecast" button.
  - Throughput basis: per-day arrival counts into the target column over the
    period, from `history` "moved" events parsed as "Moved from X to Y"
    (same deterministic parse as CumulativeFlowPage), rendered as a line
    chart on page load and refreshed when period/target change.
  - On run: N simulations (default 1000); each simulated day samples one
    historical day's throughput uniformly with replacement until the task
    count is reached (5-simulated-year safety cap). Sorted completion dates
    → 50/75/85/95 percentile dates; headline "Forecast to complete N tasks
    starting <date>"; colored probability lines (green/gold/red/purple,
    KanbanFlow's colors); histogram (one bar/day) with dashed percentile
    lines; month x-labels decluttered to ~8 evenly spaced labels.
  - Empty state: "No completed tasks in the selected period — the forecast
    needs historical throughput."
- `reportAction` gains `case 'montecarlo':` → `/b/:id/monte-carlo` (covers
  both Reports menus).
- New styles: `.mc-layout`, `.mc-settings`, `.mc-field`, `.mc-check`,
  `.mc-run` (green), `.mc-headline`, `.mc-forecast-lines`, `.mc-bar`,
  `.mc-pct`.
- Docs updated in `agents.md`/`agents/skill.md`.

Design notes: throughput is arrivals into the *target* column (not just
`completed_at`), so the dropdown is functional for any column. Arrivals
match the column's current name — events logged under a pre-rename name are
not counted (same class of limitation as KF-350, documented). History is
capped by the DB layer, so very old completions may be absent from the
basis.

### KF-345 — Time estimate (`a104394`)

- `GET /b/:board_id/time-estimate` → `TimeEstimateTemplate`
  (`templates/time_estimate.html`).
- `TimeEstimatePage` (static/app.js): mirrors TaskCountPage's color × column
  grouping (incl. "No color" row, first-appearance row order) but sums
  `estimate_hours` per cell instead of counting tasks; cells formatted
  "23h 00m" (KanbanFlow's cell format); tasks without an estimate contribute
  0 and are not counted as estimated.
- Status: "Total estimated workload: 16h 00m across 4 tasks with estimates."
  / "No time estimates on this board." / "No tasks on this board."
- `reportAction` gains `case 'estimate':` → `/b/:id/time-estimate`.
- No new CSS (reuses `.report-table`, `.report-color-dot`).
- Docs updated in `agents.md`/`agents/skill.md`.

## Commits (worktree `.dev/worktrees/batch-25`, base `5b830f2`)

- `6ada8a4` KF-342: implement Monte Carlo forecasting report
  (throughput-sampled simulations)
- `a104394` KF-345: implement Time estimate report (estimates by color and
  column)
- Note: a 12-line cosmetic fix (declutter overlapping histogram month
  labels) for the KF-342 chart was amended into the KF-345 commit rather
  than KF-342. Both commits cherry-pick cleanly and the final tree is
  identical; the verifier should evaluate the final tree.

## L0 (exact, per defect)

- `cargo fmt --check` — PASS (both)
- `cargo build` — PASS (both; Askama compiles the new templates)
- `cargo clippy --all-targets` — 0 warnings (both)
- `cargo test` — 86 passed, 0 failed (both)
- `node --check static/app.js` — PASS (both)

No new Rust tests (client-side only; the Rust surface is two trivial
template handlers + a Done-column lookup, matching the batches 20–24
precedent).

## Local confirmation (test instance 127.0.0.1:3243, throwaway redb — killed, port free)

Seeded "MC Test Board": 3 columns (To-do, In progress, Done w/ is_done), 5
tasks — 4 with `estimate_hours` (4h Yellow/To-do, 2.5h Green/To-do,
1.5h Yellow/In progress, 8h Blue/In progress), 3 moved to Done via the move
endpoint (history "moved" events stamped).

- Monte Carlo page: 200, init wired, 404 on unknown board. Playwright run:
  headline "Forecast to complete 10 tasks starting 12. Oct"; probability
  lines in correct order (50% earliest → 95% latest); status "Forecast based
  on 3 completed tasks in the selected period (1000 simulations)."; zero JS
  errors. Histogram + percentile dashes + throughput-basis line chart all
  render. Independent node recomputation of the simulation against the live
  export-data agreed (3 arrivals detected; percentile ordering sane).
  Screenshot reviewed — matches KanbanFlow's layout (settings left,
  forecast right, histogram + basis charts).
- Time estimate page: 200, 404 on unknown board. Table renders Green/Blue/
  Yellow rows × To-do/In progress/Done columns with correct sums
  (2h 30m, 8h 00m, 5h 30m); status "Total estimated workload: 16h 00m
  across 4 tasks with estimates."; zero JS errors. Screenshot reviewed —
  matches KanbanFlow's color × column matrix.
- Menu navigation: both `data-report` items route via `reportAction` (cases
  added); the old "not available yet" toasts are gone for both keys.
- Test server killed by PID, port confirmed free, throwaway DB removed.

## Notes for the verifier / release path

- Same stale-ordering pattern as prior batches: both sections show OPEN
  first with older lines beneath. Confirm against the **deployed** tree per
  process.
- Release note: worktree base is `5b830f2` (batch-24 tip, includes all
  shipped reports); local master is behind `origin/main`'s app tree — the
  release worker must cherry-pick `6ada8a4` and `a104394` onto origin/main
  (batch-19–24 pattern), never merge.
- "Show more options" currently reveals only the simulations count; if
  KanbanFlow's panel has more options, that's a follow-up, not a blocker.

Production untouched, nothing pushed, tracker not modified.
