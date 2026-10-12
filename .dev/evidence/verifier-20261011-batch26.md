# Batch-26 independent verifier report — KF-349 + KF-350

**Verdicts: KF-349 PRESENT-AND-CORRECT · KF-350 PRESENT-AND-CORRECT.** Neither returns to the build path.

**Verified commits:** `d194eb0` (KF-349), `6664c46` (KF-350) in worktree `.dev/worktrees/batch-26`
(base `6bb6cd1` + report cherry-picks, detached HEAD). Test instance 127.0.0.1:3246,
throwaway redb (removed), server killed by PID 17032 after verification. Production
untouched, nothing pushed, tracker not modified.

## KF-349 — Report dark-mode variants: PRESENT-AND-CORRECT

- Built the worktree; ran a live test instance; Playwright-drove 7 report pages
  (Export, Cumulative flow, Calendar, Monte Carlo, Due date, Board history, Time estimate).
- **Dark mode, measured contrast** (WCAG AA needs ≥ 4.5 for normal text):
  - All 7 pages — h1: **16.22**, `.report-desc`: **7.03** (pre-fix h1 was `#111827` on the
    dark body, ≈ 1.1 — effectively invisible).
  - Due-date status colors: On time 10.25 / Late 10.69 / Overdue 6.45 / Upcoming 7.02;
    table th 11.86 / td 12.12.
  - Monte Carlo settings panel: dark surface `#1f2937`, light input text `#e5e7eb`.
  - Board history feed: text 14.42 / timestamp 7.03. Time-estimate table th: 11.86.
  - Screenshots visually reviewed: cumulative-flow-dark, calendar-dark, due-date-dark
    (plus monte-carlo, board-history, time-estimate) — headings, descriptions, legends,
    charts, tables, filters all readable; no invisible text.
- **Light mode unchanged**: light-mode computed values (h1 15.54 / desc 6.62) match the
  pre-fix light values exactly; every override in the commit is `body.dark`-scoped
  (verified by reading the full diff — no unscoped selector).
- Break-attempts survived: empty boards in dark mode ("No tasks with due dates on this
  board.", "No changes on this board.", empty CFD), 404 on unknown board, 1100px
  viewport in dark mode (0px horizontal overflow), zero JS errors.
- Environment note: jsdelivr CDN (htmx/sortable) is blocked by the sandbox proxy, so
  navigations used `domcontentloaded` instead of `networkidle` — pre-existing limitation,
  not an app bug.

## KF-350 — CFD phantom band: PRESENT-AND-CORRECT

- Reproduced the exact defect scenario live: seeded board "B26 Verify", moved a task
  To-do → In progress (history froze "Moved from To-do to In progress"), then renamed
  the column "In progress" → "Working". The task's `column_id` points at the renamed
  column while history names the old one — the phantom case.
- Extracted the SHIPPED `timelineFor`/`parseMoveDetail` verbatim from `static/app.js`
  into node and ran both variants against the live data:
  - OLD (fix removed): tail col = `"In progress"` — phantom reproduced.
  - NEW (shipped): tail col = `"Working"` — phantom gone.
  - Fallback (column_id unresolvable, e.g. deleted column): keeps history-derived
    name `"In progress"` — correct.
  - Current-day bands summed: `{To-do: 1, Working: 1}` = 2 = true total — sums preserved.
- Live CFD page: legend reads "To-do | Working | Done | In progress"; the rendered
  stack shows both tasks in the Working band, no visible phantom band. The "In progress"
  legend entry is the accepted zero-count residual per the fix direction (old names remain
  as zero-height bands so totals sum — `columnOrder` behavior unchanged).
- Earlier timeline entries keep event-time names (correct for historical days); only the
  tail entry is rewritten.

## L0 re-run (worktree)

`cargo fmt --check` PASS · `cargo build` PASS · `cargo clippy --all-targets` 0 warnings ·
`cargo test` **87 passed / 0 failed** · `node --check static/app.js` PASS.

## Release note

Cherry-pick `d194eb0` + `6664c46` onto current `origin/main` (batch-19–25 pattern); never
merge local master `6bb6cd1`. No Rust surface changed (CSS/JS-only); `agents.md` /
`agents/skill.md` need no updates.

DONE: KF-349 PRESENT-AND-CORRECT, KF-350 PRESENT-AND-CORRECT
