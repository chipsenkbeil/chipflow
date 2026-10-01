# Final golden-master battery — 2026-10-01

- **Build under test:** `c3307b73137b8d6570f5ce699f0dfddf9a810b9f` (green-box production, verified via `chipflow.service` port 3000 `/api/v1/version` 2026-10-01 ~01:26 UTC; local test server ran the same tree from `/home/hatch/.cache/chipflow-check-target/debug/chipflow` on `127.0.0.1:3101` with fresh DB `.dev/test-dbs/final-battery.redb`)
- **Golden-master board:** "ChipFlow Golden Master" (`8c5ed78a-9070-4b17-a3f4-91faed6e0b27`), replicated via API from `.dev/golden-master-board.json`: 5 columns (Backlog, To Do, In Progress wip_limit 3, In Review, Done), 22 tasks with colors/labels/estimates/dues/descriptions/subtasks/comments/assignee, one 2.5h manual entry 2026-09-29 on task #11
- **Harness:** Playwright (playwright-core) + Chromium, 1440x900; SortableJS 1.15.2 injected locally (sandbox egress blocks jsdelivr; curl works — test-harness artifact only, not an app issue)
- **Timestamps:** all times UTC 2026-10-01 (~01:55–02:40)

## battery1 — board, modal, manual dialog, drag, filter, reports, shortcuts (~01:55 UTC)

- PASS login (user `battery`)
- PASS board render: 5 columns; To Do header "2 overdue tasks" (correct for 2026-10-01: tasks due 2026-09-25 and 2026-09-30 17:00 — the spec's "1 overdue" was captured 2026-09-30); In Progress "5 / 3" red WIP warning; 22 cards; color tints; estimate/due icons; assignee badges
- PASS task modal (#11 "Build payment webhook handler"): 4 subtasks (2 done), 8h estimate, 2h30m logged, 2 comments (author "Chip Senkbeil"), labels, due date
- PASS manual-time dialog: opens via `y` key; 22 task datalist options; calendar popup opens/closes/day-pick; comment toggle; From/To → Duration auto-compute; future-time error overlay ("Cannot add time / You can not enter a time in the future / OK") with dialog state preserved after dismiss; valid submit closes and the entry appears in the log
- PASS drag Backlog → To Do: counts 4/6, persists after reload
- PASS filter panel opens; board menu opens; Reports submenu opens with exactly 15 items; shortcuts dialog via `?`; layout-edit view; add-column dialog (6 columns); no page errors

## battery2 — timer, columns, filter, legend, template page (~02:00 UTC)

- PASS why-stop menu: 17 reasons in KanbanFlow order (15 defaults, "Add new reason…", "Task done" last) — screenshot `.dev/evidence/debug-why-stop.png`
- PASS stop-with-reason via API (`POST /api/timer/stop {"completed":false,"reason":"Meeting"}` after 25s) → log entry `interrupted:true`, `interrupt_reason:"Meeting"`, `badge_title:"Stopped with reason 'Meeting'"`
- PASS add column via layout-edit view ("Battery Col")
- **KF-229 (filed):** timer pill never shows countdown. KF-209 replaced the pill contents with a clock icon (KanbanFlow parity) but `TimerUI.updatePill` (static/app.js ~2389) still writes to `#timer-pill-icon` / `#timer-pill-time` children that no longer exist in templates/board.html (only the SVG remains); null-guarded so it silently does nothing. Card outline + live badge + tab title still work — only the board-bar pill is dead.

## battery3 — edit-entry, color filter, legend, column delete, template board (~02:30–02:40 UTC)

- PASS interrupted-pomodoro row renders in the `/timer/log` UI ("Stopped with reason 'Meeting'")
- PASS task-modal time-log subview: 1 entry with `[data-entry-id]` edit affordance; Edit dialog opens with title "Edit Pomodoro entry"; Date ✓ validation, calendar popup opens and closes via day-pick; comment toggle reveals field; Update closes the dialog — screenshot `.dev/evidence/final-battery-edit-entry.png`
- PASS color filter: select `red` → 2/22 cards visible; clear → 22/22 restored (screenshot `final-battery-filter-red.png`). Note: default color labels are the board's Pomodoro renames ("1 Pomodoro"…), so the test selects by option value — this is KF-217 design, not a defect.
- PASS color legend toggle (KF-222): footer appears with all 10 palette segments (screenshot `final-battery-legend.png`)
- PASS column menu opens on right-click (KF-038 parity)
- **FAIL → KF-230 (filed):** column menu offers only Edit / Collapse / Show details — no Delete. `deleteColumn()` (static/app.js:814, with KF-047 confirm dialog) has zero callers; the layout view has no delete affordance either. `DELETE /api/columns/:id` works but is unreachable from the UI. Screenshot `final-battery-column-menu.png`.
- PASS new-board template picker; Pomodoro template creates a board with columns "Work To-do", "Do today", "In progress" (WIP 3), "Done" (screenshot `final-battery-pomodoro-board.png`)
- PASS no page errors across battery3

## Persistence across server restart (~02:30 UTC)

- Killed the test server (SIGTERM), restarted with the same `.dev/test-dbs/final-battery.redb`
- Before: 3 boards, 6 columns, 22 tasks, 5 timer-log entries. After: identical (verified via API)
- Browser reload of the golden master post-restart: 6 columns, 22 cards, In Progress "5 / 3", 0 page errors

## Test-harness artifacts (not app defects)

- Sandbox Chromium cannot reach jsdelivr (ERR_EMPTY_RESPONSE; curl fine) → SortableJS injected locally for drag tests
- `/timer-log` 404s; the real route is `/timer/log`
- Timer log page is JS-rendered (curl shows shell only)
- Manual dialog field ids are `mt-*` / edit dialog `ee-*`, not `#manual-time-dialog`
- Column ⋮ button is `display:none` until hover (KF-038: menu is right-click only) — tests use right-click
- Calendar popups overlay their toggle buttons; tests close them via day-pick

## Observations (not filed)

- Dark-themed manual/why-stop/edit dialogs on the light board — unverified against KanbanFlow, not filed
- Color legend defaults to hidden behind a per-board toggle (KF-222 design choice)
- New template-created board sorts before existing boards in the board list (lowest `position`) — unverified against KanbanFlow, not filed

## Verdict

**battery_passed: false** — 2 new defects filed (KF-229, KF-230). Everything else in the battery passed.
