# Batch-25 independent verifier report — KF-342 + KF-345

**Verdicts: KF-342 PRESENT-AND-CORRECT · KF-345 PRESENT-AND-CORRECT.** Neither returns to the build path.

**Verified commits:** `6ada8a4` (KF-342), `a104394` (KF-345) in worktree `.dev/worktrees/batch-25`
(base `6bb6cd1` + report cherry-picks, detached HEAD). Test instance 127.0.0.1:3244,
throwaway redb (removed), server killed by PID after verification. Production untouched,
nothing pushed, tracker not modified.

## KF-342 — Monte Carlo forecasting: PRESENT-AND-CORRECT

- Both Reports-menu paths (`#topbar-reports-btn` → `#topbar-reports-menu
  [data-report="montecarlo"]`, 2 instances incl. board menu) navigate to
  `/b/:id/monte-carlo` with **zero new toasts**; old "not available yet" toast
  string for montecarlo is gone from `static/app.js`.
- Target-column dropdown lists all board columns, defaults to the Done column
  (resolved server-side via `is_done`, not a name bolt).
- After "Run forecast": headline "Forecast to complete 10 tasks starting 12. Oct";
  status "Forecast based on 3 completed tasks in the selected period (1000 simulations)."
  — the "3 completed tasks" exactly matches my independent count of "moved"-to-Done
  arrivals in the export-data dataset.
- Four probability lines in KanbanFlow's order/colors: 95% green `#43a047`,
  85% gold `#c49000`, 75% red `#c0392b`, 50% purple `#8e44ad`; dates strictly
  ordered 50% ≤ 75% ≤ 85% ≤ 95% and mathematically sane (3 arrivals/90 days →
  50% date ~11 months out for 10 tasks).
- Histogram renders (505–514 bars across runs) with 4 dashed percentile lines
  labeled 50%/75%/85%/95%; throughput-basis line chart renders on load and
  refreshes on period/target-column change; month x-labels decluttered, no overlap.
- "Show more options" reveals the Simulations input (initially `hidden`,
  confirmed `display:none`); setting 100 sims → status reports "(100 simulations)".
- Target column with zero arrivals (To-do) → empty state "No completed tasks in
  the selected period — the forecast needs historical throughput." Empty board →
  same empty state. Singular "1 task" headline for task count 1.
- 404 on unknown board. 1100px viewport: 0px horizontal overflow. Dark
  color-scheme: zero JS errors (page renders light — pre-existing KF-349, not a
  regression).

### Independent algorithm verification (shipped code, node harness)
Extracted `dailyThroughput`, `runSimulation`, `percentile` verbatim from
`static/app.js` and tested against mathematical expectations — **13/13 PASS**:
- 1 arrival/day × 90 days, 10 tasks → every one of 1000 sims completes in exactly
  10 days; all percentiles collapse to start+10d (deterministic).
- Zero throughput → `null` (empty-state path).
- Single arrival day, 1 task → median ≈ theoretical 62d, 95th pct ≈ theoretical
  269d (geometric with p=1/90).
- Percentile index math correct (p50/p95/p25 on a 4-element array).
- `dailyThroughput` counts only in-window arrivals into the target column's
  current name; "Moved from Back to the future to Done" splits on the last " to ".
- A from-scratch independent reimplementation (seeded LCG, own code) agrees with
  the shipped functions' median within 15% on shared random data.
- Note: the simulation uses unseeded `Math.random()` — repeated runs give
  slightly different dates (inherent to Monte Carlo, not a defect).

## KF-345 — Time estimate: PRESENT-AND-CORRECT

- Menu routing works for both menu instances, zero toasts; old toast string gone.
- Rendered table matches my independent Python recomputation from export-data
  **exactly**, cell for cell: Yellow 4h 30m / 1h 30m / 3h 00m; Green 2h 30m /
  0h 00m / 1h 00m; Red 0h 00m × 3 (unestimated task contributes 0, not counted);
  Blue 0h 00m / 8h 00m / 0h 00m. Status "Total estimated workload: 20h 30m
  across 7 tasks with estimates." Row order follows first appearance on the
  board; headers are the board's column names; cells use KanbanFlow's "23h 00m"
  format.
- 404 on unknown board. Empty board: "No time estimates on this board." /
  "No tasks on this board."
- Dark color-scheme: zero JS errors (light rendering = KF-349, pre-existing).
- "No color" row: not exercisable live — the API ignores `color_id: null` on
  PATCH and defaults new tasks to Yellow, so a colorless task cannot currently
  exist. The defensive branch (`t.color_id || ''` → 'No color') mirrors
  TaskCountPage's established pattern; no defect.

## Break-attempts survived

- Empty boards (both pages), 404s, dark color-scheme, 1100px viewport (0px
  overflow), zero-throughput target column, single-day throughput spike,
  100-simulation runs, period switching (90→30d), unestimated tasks, special
  characters n/a (no text input surfaces beyond numeric).
- Console 401s on `/api/timer/settings` + `/api/timer/status` also occur on the
  older Export report page — pre-existing global timer poller behavior, not
  batch-25. CDN htmx/sortable `ERR_TUNNEL_CONNECTION_FAILED` is the known
  sandbox-proxy limitation.
- Mid-verification environment restart wiped /tmp (test DB, jars, screenshots);
  re-seeded from scratch on a fresh throwaway DB and re-confirmed all live
  findings — results reproduced identically.

## L0 re-run (worktree)

`cargo fmt --check` PASS · `cargo build` PASS · `cargo clippy --all-targets`
0 warnings · `cargo test` **86 passed / 0 failed** · `node --check static/app.js` PASS.

## Release note

Cherry-pick `6ada8a4` and `a104394` onto **origin/main** (batch-19–24 pattern —
local master `6bb6cd1` is behind origin/main's app tree); never merge. Note the
builder's remark: a 12-line histogram label-declutter fix for KF-342 was amended
into the KF-345 commit — evaluate/release the final tree, not per-commit purity.

Screenshots were reviewed live (light + dark for both pages); /tmp was wiped
mid-run so the PNGs are ephemeral and not archived — findings above are the record.

DONE: KF-342 PRESENT-AND-CORRECT, KF-345 PRESENT-AND-CORRECT
