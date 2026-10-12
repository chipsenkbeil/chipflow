# Builder evidence — batch-26 (KF-349, KF-350)

Worktree: `.dev/worktrees/batch-26` (base `6bb6cd1` + cherry-picks of the 7
report release commits `d1d7d0f d10b16c 109aade 4a900de 520ac4b 93954ab a8bd27a`,
all applied cleanly). Never pushed; production untouched.

## KF-349 — dark-mode variants for report pages — commit `d194eb0`

**Change** (`static/style.css`, +83): appended a `body.dark`-scoped block covering
the full report surface — `.timer-page` shell (h1/h2, tabs, log entries,
`.report-desc`, `.btn`, statistics components), `.report-table`,
chart SVG classes (`.line-chart`, `.chart-grid`, `.chart-tick`,
`.chart-axis-label`, `.chart-dot`), CFD/chart legends, report toggle buttons,
Monte Carlo settings/inputs, due-date summary + status colors, history feed +
compare table, calendar grid/nav/buttons. Palette matches the app's dark theme
(`--bg:#0f172a`, `--ink:#e5e7eb`, `--muted:#9ca3af`); surfaces use `#1f2937`,
borders `#374151`/`#4b5563`. Light mode untouched (overrides are all
`body.dark`-scoped, higher specificity than the light rules).

**Local confirmation** (test instance 127.0.0.1:3245, throwaway redb, killed,
port free): Playwright screenshots of Export, Cumulative flow, and Calendar
pages in light then dark mode. Dark-mode computed-style probe: h1
`rgb(243,244,246)` on body `rgb(15,23,42)` — high contrast (pre-fix h1 was
`#111827` on the dark body). Screenshots visually reviewed: headings,
descriptions, buttons, chart panels, legends, calendar grid all readable;
light mode pixel-identical to before. Screenshots were ephemeral (`/tmp`,
discarded); the verifier re-runs them.

## KF-350 — CFD latest position prefers current column name — commit `6664c46`

**Change** (`static/app.js`, +8, in `CumulativeFlowPage.timelineFor`): after the
timeline is built and sorted, the final entry (the task's latest position) now
uses `colNameById[task.column_id]` — the column's CURRENT name — instead of
the name frozen in the history detail string. Falls back to the
history-derived name when the column id is unresolvable. Earlier timeline
entries keep event-time names (correct for historical days); `columnOrder`
unchanged (old names remain as zero-height bands so totals still sum).

**Local confirmation**: seeded board "B26 Test" (columns To-do / In progress /
Done), moved a task To-do → In progress ("Moved from To-do to Doing" recorded
after an intermediate rename), then renamed the column to "Working" — the
exact phantom scenario: history names the old column while `column_id` points
at the renamed one. Extracted the SHIPPED `timelineFor`/`seriesByDay`/
`columnOrder`/`parseMoveDetail` verbatim from `static/app.js` into node and ran
both variants against the live export-data:
- OLD: band "Doing" = 1, band "Working" = 0 (phantom band, renamed column
  empty), total 4.
- NEW: band "Working" = 1, band "Doing" = 0, total 4 (sums preserved).
Playwright screenshot of the CFD page confirms the current-day stack shows the
task in the "Working" band. Test server killed by PID, port free, DB removed.

## L0 (exact, per defect, worktree tree)

- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — **87 passed, 0 failed**
  (9+37+3+12+8+6+4+2+2+1+3)
- `node --check static/app.js` — PASS

No Rust surface changed (CSS/JS-only fixes); `agents.md`/`agents/skill.md`
need no updates (no API behavior change). No JS test infra exists in the repo
(batches 20–25 convention); KF-350 logic was verified via the node extraction
above instead.

## For the verifier / release

- Commits: `d194eb0` (KF-349), `6664c46` (KF-350).
- Release worker: cherry-pick both onto current `origin/main`; never merge
  local master `6bb6cd1`.
- Known residual: the CFD legend still lists the old column name as a
  zero-count entry (zero-height band, invisible in the chart) — accepted per
  the defect's fix direction, which scopes to latest-position attribution.
