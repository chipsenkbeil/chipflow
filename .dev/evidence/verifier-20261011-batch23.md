# Batch-23 independent verifier report — KF-340 + KF-341

**Verdict: both PRESENT-AND-CORRECT. Nothing returns to the build path.**

Verified commits: `ab08624` (KF-340) and `0627780` (KF-341) in worktree
`.dev/worktrees/batch-23` (base `6bb6cd1` + report cherry-picks, detached
HEAD). Built the tree, ran a live test instance (127.0.0.1:3239, throwaway
DB, since killed). Production untouched, nothing pushed, tracker not
modified.

## Method

Seeded board (Pomodoro template: Work To-do / Do today / In progress /
Done): 6 tasks with due dates (1 on-time completion, 1 late completion, 1
overdue, 1 upcoming, 1 due_done-not-completed, 1 no-due) + 2 tasks completed
via the move endpoint (which stamps `completed_at` and logs `moved` history).
Fetched `/api/boards/:id/export-data` and independently recomputed every
number before looking at the rendered pages.

## KF-340 — Cycle & lead time: PRESENT-AND-CORRECT

- Menu routing: both Reports-menu paths (`#topbar-reports-btn` and the
  board-menu `#reports-submenu`) navigate to `/b/:id/cycle-lead` with **zero
  toasts**; old "not available" strings for cycle are gone (the 2 remaining
  toasts in `static/app.js` belong to other reports: the Time-in-column
  toast and the generic `default:` branch).
- Rendered: 12 dots, status "2 tasks completed in the last 12 weeks.",
  current-week dot "0 days avg, week of 12. Oct (2 tasks)" — matches the
  independent recomputation (both tasks completed this week; cycle clamped
  from the move-handler's moved-after-completed timestamp ordering, lead
  ≈ 0.0001 days → "0 days" at 1 decimal).
- Metric toggle re-renders (verified Lead time: last dot "0 days avg, week
  of 12. Oct (2 tasks)"); trend line present (1 `.chart-trend`); legend
  "Cycle/Lead time / Trend (moving average)".
- Math unit-tested in node against the shipped logic: `weekStart` is
  Monday-start (2026-10-12 and 2026-10-18 both → Mon Oct 12 2026);
  `movingAverage([0,0,3,0,0])` = `[0,1,1,1,0]` (centered 3-week, edges use
  available points); `daysBetween` clamps negatives to 0.
- 404 on unknown board; empty board shows "No completed tasks in the last
  12 weeks."; close × returns to board.
- KanbanFlow fidelity: compared against kanbanflow.com's
  `CycleAndLeadTime.png` (510×260) — y-axis "Average time (days)", x labels
  "7. Mar" style, green solid line + dots, dashed gray trend line, legend
  "Cycle/Lead time" + "Trend (moving average)". Our page matches all five.

## KF-341 — Due date performance: PRESENT-AND-CORRECT

- Menu routing: both menu paths navigate to `/b/:id/due-date`, zero toasts.
- Rendered summary "2 / On time / 1 / Late / 1 / Overdue / 1 / Upcoming"
  and status "2 of 5 tasks with due dates completed on time." — **exact
  match** with the independent classification:
  - OnTime Task (completed 10-12 ≤ due 10-20) → On time
  - Late Task (completed 10-12 > due 10-01) → Late
  - Overdue Task (not completed, due 10-05 < now) → Overdue
  - Upcoming Task (not completed, due 11-01 > now) → Upcoming
  - DoneDone Task (not completed, due_done=true) → On time
  - NoDue Task + 3 template tasks → excluded
- Table sorted by due date ascending (Oct 1, Oct 2, Oct 5, Oct 20, Nov 1);
  all statuses correct; completed column shows "—" for uncompleted tasks.
- 404 on unknown board; empty board shows "No tasks with due dates on this
  board." with the table hidden; close × works.

## Break-attempts survived

Dark mode + 1100px viewport: zero JS errors, zero horizontal overflow on
both pages (the `.timer-page` shell's lack of dark variants is the
pre-existing KF-349, not a regression). Special characters in task names
(`"quoted, comma"` seeded — no errors). Tasks created directly in the Done
column get no `completed_at` (move endpoint is the only completion path),
so the cycle-fallback branch is defensive-only — verified unreachable via
API, harmless.

## L0 re-run (exact)

`cargo fmt --check` PASS · `cargo build` PASS · `cargo clippy --all-targets`
0 warnings · `cargo test` **86 passed, 0 failed** · `node --check
static/app.js` PASS.

## Release note

Cherry-pick `ab08624` and `0627780` onto **origin/main** (batch-19/20/21/22
pattern — local master `6bb6cd1` is behind origin/main's app tree); never
merge.
