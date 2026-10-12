# Batch-24 independent verifier report — KF-336 + KF-338

**Verdicts: KF-336 PRESENT-AND-CORRECT · KF-338 PRESENT-AND-CORRECT.** Neither returns to the build path.

**Verified commits:** `f2b0935` (KF-336), `5b830f2` (KF-338) in worktree `.dev/worktrees/batch-24`
(base `6bb6cd1` + report cherry-picks, detached HEAD). Test instance 127.0.0.1:3242,
throwaway redb (removed), server killed by PID 105421 after verification.

## KF-336 — Board history: PRESENT-AND-CORRECT

- Both Reports-menu paths (topbar `#topbar-reports-btn` → `#topbar-reports-menu
  [data-report="history"]`) navigate to `/b/:id/board-history` with **zero toasts**;
  old "not available" toast string for history is gone from `static/app.js`.
- Seeded 2 tasks → 9 history entries rendered **newest-first**, matching the
  API-computed order exactly (`members_updated, moved, due_updated×2, time_logged,
  comment, renamed, created×2`).
- Headlines in KanbanFlow tone: "Task X was created in column To-do",
  "Task X was renamed." (+ detail 'Renamed to "…"'), "Task X has changed
  properties: Column/Due date/Members.", "Task X: logged 25m",
  "verifier added to task X with text" + comment body.
- `moved` entry renders a native `<details open>` compare table:
  Property|Old value|New value = Column|To-do|In progress.
- Filters: user (`All users`, `verifier`) and type (`Column, Comment, Created,
  Due date, Renamed, Time logged`) both filter correctly ("1 change." for moved).
- Relative timestamps: "Today 03:21" etc. Comment body with `<html> & "quotes"`
  is HTML-escaped, not rendered.
- 404 on unknown board (page + export-data). Empty board: "No changes on this board."

## KF-338 — Calendar: PRESENT-AND-CORRECT

- Menu routing works, zero toasts. Monday-start grid (Mon–Sun headers);
  Oct 2026: 35 cells, Sep 28–30 + Nov 1 grayed (`cal-outside`), today (12)
  highlighted (`cal-today`, blue circle).
- 2 chips on correct due dates with correct sublines: "Yesterday → In progress"
  (due Oct 11), "On today → To-do" (due Oct 12). After re-dating: "On tomorrow →
  In progress" and "On 20. Nov → To-do" (November nav). All four relative branches
  (`On today / On tomorrow / Yesterday / On 12. Oct`) verified live.
- Month nav ‹/Today/› works (Sep → Nov → Oct). All 4 filters present
  (user/color/label/swimlane); user filter populates from task members and
  filters chips correctly (2 → 1 → 2).
- 404 on unknown board. Empty board: "No tasks with due dates in view."
- Chip colors: default yellow for uncolored tasks, task name + subline.

## Break-attempts survived

- Empty boards (both pages), 404s, dark `color-scheme` (no JS errors),
  1100px viewport (0px overflow both pages), special characters in task/comment
  names (escaped), tasks with no members (filter empty by design, not a bug),
  cross-month navigation, due-date re-dating.
- Failed network requests: only CDN htmx/sortable (`ERR_TUNNEL_CONNECTION_FAILED`
  — sandbox proxy blocks jsdelivr; pre-existing environment limitation, not an app bug).

## L0 re-run (worktree)

`cargo fmt --check` PASS · `cargo build` PASS · `cargo clippy --all-targets`
0 warnings · `cargo test` **86 passed / 0 failed** · `node --check static/app.js` PASS.

## Known-gap judgments

1. **Deletion entries (KF-336): ACCEPT.** ChipFlow does not log task deletions,
   so "Task X was deleted" cannot be shown. Core contract (functional history
   destination) is met; deletion logging would need schema work — a future
   enhancement, not a build-path failure.
2. **iCal feed (KF-338): FILE FOLLOW-UP.** KanbanFlow's Calendar offers external
   iCal subscription; not implemented. Security-sensitive (unauthenticated feed
   URLs) — needs a designed approach (e.g. per-board secret tokens), so it
   should be tracked as its own defect, not bolted on here.
3. **`due_repeat` expansion (KF-338): FILE FOLLOW-UP.** Repeating due dates are
   not expanded onto future occurrences — a real but smaller parity gap.

## Release note

Cherry-pick `f2b0935` and `5b830f2` onto **origin/main** (batch-19/20/21/22/23
pattern — local master `6bb6cd1` is behind origin/main's app tree); never merge.

Screenshots: `/tmp/v24t/history-dark.png`, `/tmp/v24t/calendar-dark.png`,
`/tmp/v24t/calendar-light.png`, `/tmp/v24t/history-light.png` (ephemeral).
