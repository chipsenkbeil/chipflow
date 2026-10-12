# Verifier evidence — batch-21: KF-343 Task count + KF-344 Throughput

Independent verifier (not the builder). Verified the built commits in worktree
`.dev/worktrees/batch-21` (detached HEAD `41d4734`; base `6bb6cd1` + `6900ca1` KF-346).

## L0 re-run (exact)
- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — 86 passed, 0 failed
- `node --check static/app.js` — PASS

## KF-343 — Task count: PRESENT-AND-CORRECT
- Both Reports menus (topbar + board-menu submenu) carry `data-report="count"`; clicking navigates to `/b/:id/task-count` with **zero toasts** (the old "Task count report is not available" string is gone from `static/app.js`).
- Seeded board (3 columns; 7 tasks across yellow/green/blue/red): rendered table matched independently computed expectations exactly — Yellow 1/1/1, Green 1/0/1, Blue 0/1/0, Red 0/0/1; status "7 tasks."
- 404 on unknown board. Empty board: "No tasks on this board." / "0 tasks."
- Close (×) returns to the board. Column names, color labels/values passed through `escapeHtml`; `board_id` in the template is the DB-stored UUID (handler 404s unknown IDs first), Askama escapes `{{ }}` by default.
- The "No color" row branch (`label: 'No color'`) is defensive — the API always applies the board default color, so a null `color_id` cannot currently occur; code-reviewed as correct.

## KF-344 — Throughput: PRESENT-AND-CORRECT
- `data-report="throughput"` click navigates to `/b/:id/throughput` with zero toasts; old toast string gone.
- 3 tasks completed via the move endpoint (completed_at stamped on crossing into the `is_done` Done column): chart shows 12 dots, y-axis "Task count", green line flat at 0 with a spike to **3** in the current week; status "3 tasks completed in the last 12 weeks."
- Tooltips: "N tasks, week of D. Mon" for all 12 points; bucket labels verified Monday-start (`date -d` confirms 2026-07-27 and 2026-10-12 are Mondays).
- Week-bucket edge cases unit-tested in node against the shipped `weekStart`/`bucketByWeek` logic: Monday 00:00:00 lands in the new week's bucket, Sunday 23:59:59 in the prior week's, >12-week-old and unparseable dates ignored. (Bucketing is browser-local Monday-start, documented in the code comment.)
- Last x-label not clipped: right edge 1049.1px < svg edge 1064px (the builder's padding amend holds).
- 404 on unknown board. Empty board: flat zero chart, 12 all-zero tooltips, "0 tasks completed in the last 12 weeks."
- Close (×) returns to the board. Y-axis scales as `ceil(max/5)*5`.

## Break-attempts survived
- Dark mode (`color_scheme="dark"`): no JS errors on either page (the `.timer-page` shell's lack of dark variants is the pre-existing KF-349 gap, not a regression).
- 1100px narrow viewport: no horizontal overflow on the throughput page.
- Empty boards, 404s, special characters in task names (`"quoted, comma"` seeded — counts unaffected, no errors).

## KanbanFlow fidelity
Spot-checked against the live sources: kanbanflow.com/analytics-reporting describes Throughput as "shows how many tasks are moving through a column... Measure the performance of your system" (line chart, green line + dots — matches); Task count is documented as breaking down tasks across columns (the color × column table matches the builder's "analyzing the number of tasks in different columns" reading).

## Residual notes (not failures)
- Dark-mode shell gap is KF-349 (pre-existing, already filed).
- Release: cherry-pick `e786ce5` and `41d4734` onto origin/main (batch-19/20 pattern — local master is behind origin/main's app tree); never merge.

## Verdicts
- KF-343: PRESENT-AND-CORRECT — none return to the build path.
- KF-344: PRESENT-AND-CORRECT — none return to the build path.

Test instance (127.0.0.1:3235) killed by PID, port confirmed free, throwaway DB removed. Production untouched, nothing pushed, tracker not modified.
