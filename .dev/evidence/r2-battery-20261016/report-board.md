I couldn't write `$R/results/board/report.md`: the harness blocks report files from subagents, so the full report is below. Please save it to that path. Everything else (drivers, per-scenario JSON, 103 screenshots, server logs) is in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/r2/results/board/`. The server on 3250 was stopped by its exact PID.

**Result:** 54 rows checked: 29 PASS, 23 FAIL, 0 BLOCKED, 2 N/A. Two failures are HIGH: the swimlane grid shifts one column left (R2-015), and the card Move submenu does nothing (R2-077, a regression of RELEASED KF-334 and KF-141).

All paths below are relative to `$R/results/board/` unless absolute.

## 1. Environment
- **Binary:** `/Users/senkwich/projects/chipflow/.dev/worktrees/r2-main/target/release/chipflow` (62d573d; `/api/v1/version` reports `build_sha: unknown`).
- **Instance:** port 3250, fresh DB `/tmp/chipflow-r2-3250.redb`. Host timezone is Europe/London (BST +01:00), today is Friday 2026-10-16. Logs and PID are in `server.log` and `server.pid`.
- **Admin:** `audit`, created via `/setup`. Session jar `cookies.txt`; a throwaway jar `cookies-logout.txt` was used for the sign-out test.
- **Seed:** `seed-golden-master.sh http://127.0.0.1:3250 cookies.txt` with the default as-of date, 2026-10-16 (`seed.out`, `seed.err`).
- **Boards:**
  - GM: `4d9cc613-02c2-4e27-aeb1-b1cc16f75fd2`
  - General (created by setup): `80907e88-6378-4cb4-ba29-d2f421f72ee3`
  - POMO (built-in Pomodoro template, created through the UI): `66c09c8f-8d3f-4653-b3b0-2b0cf55541c4`
  - "From GM template" (GM saved as a template and re-created; used for destructive tests): `dcc90d77-e556-4494-93a4-3bbe667eeb0c`
  - "Done groups R2": `30a89d70-0a31-4ad5-b941-a0f13574212d`
  - "All folded R2": `aff834ed-f846-402f-9e56-b1c49e92167c`
  - MULTI boards (unicode name, 150-character name, archived then unarchived) are listed in `s3-boards.json`.
- **Done tasks on several days:** I restarted the server twice on the same DB under `faketime -f -3d` and `faketime -f -1d` (`/opt/homebrew/bin/faketime`), moved tasks into Done each time, then restarted on the real clock (`server-fake3d.log`, `server-fake1d.log`).
- **Browser:** Playwright 1.63.0 Chromium, viewport 1905x1000, DPR 1, reduced motion with transitions off, a fresh context per scenario. The shared helper is `lib.mjs`; drivers are `s1-topbar.mjs` through `s10-popups.mjs`. Each driver writes `<name>.json` with its results plus console errors, page errors, failed requests, 4xx/5xx responses and native dialogs, and saves screenshots as `shots/<name>-NN-*.png`.
- **Tracker excerpts:** the tracker sections for every covering KF id are in `kf-sections.md`; my rows are in `myrows.txt`.
- **Console and network:** the only page error was the R2-029 ReferenceError (`s3`). Console errors appeared only where a row below calls them out. Requests reported as `net::ERR_ABORTED` came from calls whose response body is never read, or that race a `location.reload()`. I checked server state each time and every one had succeeded.

## 2. Verdicts
| Row | Verdict | Evidence | Break-attempt behind the PASS | KF |
|---|---|---|---|---|
| R2-002 | FAIL | ChipFlow always shows the idle pill "▶ 25:00 ▾" in the dark top bar at (1799,2), and the board header row has no clock button (`shots/s1-01-idle.png`). All four current KanbanFlow captures (09-30, 10-05 while timing, 10-12 x2) show no pill and a clock icon left of the funnel in the board header row. | | KF-239 KF-098 KF-132 KF-024..026 |
| R2-003 | N/A | Single admin with no notification source. The bell contains only its SVG and no badge element (`s10-popups.json`). | | KF-030 KF-310 |
| R2-004 | PASS | The bell opens a "Notifications / No notifications" panel. Re-click, Esc and an outside click each close it (`s1-topbar.json`). | 7 rapid clicks leave it open; opening the reports icon closes it. | KF-310 |
| R2-005 | FAIL | The top-bar "?" (`#help-btn`) opens nothing: the shortcuts dialog stays hidden (`shots/s1-03-help.png`). `static/app.js` has no listener for it. The inventory note "opens the shortcuts dialog" is wrong. | | KF-031 |
| R2-006 | FAIL | The menu itself works: "Signed in as audit / Sign out", which POSTs `/logout`, lands on `/login`, and afterwards `/api/boards` returns 401. But `/login` logs two console errors on every load (401 on `/api/timer/settings` and `/api/timer/status`), reproduced on a fresh load with no cookie (`shots/login-fresh.png.log.json`). | | KF-195 KF-029 KF-234 |
| R2-007 | PASS | The menu lists KF-236's 15 items verbatim and in order. 13 of them navigate with a clean console; Print calls `window.print()`; "Time in column" shows a "not available" toast, which is a deliberate omission. | Clicked every item in a fresh page. | KF-236 |
| R2-008 | PASS | White dropdown containing search, a "+" button, the board list with the current board highlighted, "View archived boards", and a blue "Create board" button. This matches KF-330's verified structure (`shots/s3-02-dropdown.png`). | The 150-character name is truncated with an ellipsis; the unicode/emoji name renders. | KF-330 KF-240 KF-088 KF-172 KF-174 KF-023 KF-196 |
| R2-009 | PASS | Search is case-insensitive, handles unicode, trims surrounding spaces, empties the list on no match, and switching boards works. The Favorites half is N/A (see observations). | Unicode, padded and no-match queries. | KF-088 KF-240 |
| R2-012 | N/A | Single admin. The invite "+" opens "Invite to board / ChipFlow is single-user…". | | KF-237 |
| R2-015 | FAIL | Board with lanes Default and "Lane Two": the unlabeled Default row renders one column to the left. Backlog cards sit under the empty corner cell, Done cards sit under "In Review", and Done is empty for that row (`s4b-grid.json`, `shots/s4-03-lane-two-unfolded.png`). | | KF-131 KF-162 KF-152 |
| R2-016 | PASS | Each folded strip is 32px wide with a vertical label rotated 180°, weight 400, color rgb(17,17,17); the first lane folds to the left side. | A 60-character unicode lane name; all lanes folded. | KF-201 KF-202 KF-314 KF-248 |
| R2-017 | FAIL | (a) On a folded strip, the ⋮ menu's Rename and Delete do nothing, so an empty lane can't be renamed or deleted from the board. (b) "Move up" on a lane below Default silently does nothing. (c) Delete on a non-empty lane shows a confirm that itself says the server will refuse, then toasts "Could not delete swimlane." (`s4-swimlanes.json`). | | KF-156 KF-160 |
| R2-018 | PASS | A card in the first lane is clickable under the sticky header. On a fresh board with every lane folded, "+" creates a task in Default and it shows immediately. | All lanes folded on an empty board. | KF-147 KF-206 |
| R2-019 | PASS | The layout view shows "Layout: …", the column and lane lists, a green "+ Add column" at the end of the column list, and a blue "+ Add column" in the header. The pill is hidden while the view is open and comes back when it closes. | Opened the dialog from the end button. | KF-180 KF-246 KF-247 |
| R2-020 | PASS | Column drags persist through the API (Done dragged to first keeps `is_done`). A lane drag also persists. | Done moved to first and back; a rapid triple drag still left a valid column order. | KF-180 KF-048 |
| R2-021 | FAIL | Labels is a checkbox list where KanbanFlow has an "Add labels…" input, and Remember filter is a checkbox where KanbanFlow has a toggle. The panel docks at y=76 and covers the Done column, while KF-215's verified KanbanFlow panel starts at y≈36 with the funnel moved to x≈1514; ChipFlow's funnel stays at x=1719 (`shots/s5-01-panel-open.png`). | | KF-096 KF-134 KF-175 KF-215 |
| R2-022 | PASS | Color filter shows exactly the expected cards, column counts follow it, and the WIP warning clears. | Full and partial hide inside the WIP column. | KF-217 KF-296 KF-161 KF-297 |
| R2-023 | FAIL | "Timer users" works. "Unassigned" shows 22 cards where 17 are expected, and the named user shows 22 where 5 are expected (`s5-filter.json`). | | KF-096 |
| R2-024 | FAIL | The date ranges are correct, but the "Due in <month>" and "Due in <year>" options are missing: `addDateOptions` targets `#filter-date-options`, an element that no longer exists. | | KF-159 KF-134 |
| R2-025 | PASS | Label filtering is correct with AND logic. Clicking a label chip on a card opens the panel with that label checked. | Chip clicked in a column that shows labels while other columns hide them. | KF-165 |
| R2-026 | PASS | The Remember filter survives a reload; unticking it resets the filter. "Bookmarks (0)" is static text, and KanbanFlow's bookmark behavior was never captured. | Switched boards (the filter leaks, see observations). | KF-176 |
| R2-027 | PASS | The board list and template picker render; Delete appears only on custom templates; an empty name gives an error. | Whitespace-only name; long names. | KF-190 KF-155 KF-163 |
| R2-028 | FAIL | Columns and starter tasks match GM-141. The palette doesn't: 10 colors, all enabled, plain names (`s3-boards.json`). The 09-30 batteries verified 4 named pomodoro colors (recorded in `.dev/fidelity-status.md`), the template's own description promises pomodori colors, and the KF-331 capture shows 4 named enabled plus 6 disabled. This is a regression. | | KF-307 KF-308 KF-190 |
| R2-029 | FAIL | The styled confirm, cancel, backdrop close and built-in protection all work. Deleting a template that was already removed behind the page's back returns 404 and throws `ReferenceError: showError is not defined`, with no message to the user (`s3-boards.json`). | | KF-112 |
| R2-030 | PASS | Save as template, then create a board from it: all 22 tasks match field by field (`instantiate.mismatches: []`). | Unicode template name; full diff of every task. | KF-163 KF-164 |
| R2-031 | PASS | The empty state, archive, the hidden list, `?include_archived` and unarchive all work. | Opened the archived board's URL directly. | KF-305 |
| R2-032 | PASS | The dialog shows exactly KF-095's 9 rows. | Checked that the old Y/E rows are gone. | KF-095 KF-186 KF-276 |
| R2-033 | FAIL | With the task modal open, T, Y, E, V, ".", the two-stage Esc, Ctrl/Meta+Enter and Delete all work. P opens the stale global `#reports-menu` (2 items) instead of the modal's own Reports menu (`s7-keys.json`). This is the same class of bug as KF-298. | | KF-094 KF-298 KF-299 KF-273 |
| R2-034 | PASS | The quick-add popup drags, is clamped to the viewport, and still adds the task. Toasts appear fixed at the bottom center. | Dragged off-screen; a 300-character name. | KF-209 |
| R2-035 | PASS | Data digests for all 9 boards, templates, settings and the timer log were identical across 3 restarts (the only change was the board I was editing), and the session survived (`snap-*.txt`). | Two of the three restarts ran under a shifted clock. | none |
| R2-038 | PASS | "0 / 3" renders in rgb(91,91,91). "3 / 3" shows no warning. "4 / 3" is darkred with the *LIMIT EXCEEDED* title and the warning line. | At-limit and over-limit states. | KF-035 |
| R2-045 | PASS | A collapsed column placed after Done renders as a 22px strip at the right edge with an uppercase vertical name, and expands when clicked. | Done is no longer the last column. | KF-039 |
| R2-046 | FAIL | The menu has Edit / Collapse / Show details / Delete, with Delete in red. KF-223's live-verified KanbanFlow menu has 3 items; KF-230 asserts a delete item without a capture. Because the references conflict, I proposed no defect. | | KF-038 KF-223 KF-230 |
| R2-048 | FAIL | The dialog fields and copy match the references, and saved values reload. But the Archiving default is keyed to the column *name* containing "done": a Done column saved unchecked reopens checked, Done renamed "Finished" opens unchecked, and "Undone ideas" opens checked. | | KF-042..046 KF-315 |
| R2-049 | FAIL | Deleting an empty column works. Deleting a non-empty column with a move target fails: four move requests return 400 (missing `position`), the delete returns 400, and the tasks stay where they were. This is a regression of KF-306 (`s8b-cols.json`). | | KF-047 KF-230 KF-306 |
| R2-050 | PASS | Position offers "At the end" and "At the beginning", and "beginning" is honored. KanbanFlow's option list was never captured. | Unicode name; position changed after the dialog opened. | KF-041 KF-181 |
| R2-051 | FAIL | Mid-drag only the header goes blank; the column body stays visible and no gap appears at the drop position (`shots/s8d-03-col-middrag.png`). Playwright never fired `dragend` for header drags, so the drop itself wasn't exercised here. | | KF-048 |
| R2-052 | FAIL | The empty "Do today" column shows a "0" count cell and separator. In the reference capture this header has no count; the "0" belongs to the collapsed strip next to it (`shots/ref-general-headers.png`). | | KF-316 |
| R2-053 | FAIL | Overflow scrolls the whole board area (`main.board-wrap`), not the column body. The reference shows a scrollbar on the Done column only. | | none |
| R2-056 | FAIL | Groups render as Today / Yesterday / "Tuesday, 13 October". Two problems: (a) a task completed today and placed below older groups creates a second "Today" group (`shots/s8c-03-done-dup-today.png`); (b) when nothing was completed today, no empty "Today" header renders, which the reference capture shows (`shots/s8c-04-done-no-today.png`). | | KF-326 |
| R2-057 | FAIL | There is no UI to edit the grouping date. Setting `grouping_date` via PATCH is stored but has no effect on which group the task appears in. | | KF-140 KF-050 |
| R2-058 | FAIL | The Labels and Due dates display settings take effect. Sorting, Column sum and Archiving are saved but nothing reads them (`s8b-cols.json`). | | KF-043 KF-045 |
| R2-059 | PASS | The drop position persists after reload. Deleting from the context menu updates the count and the WIP warning live. | Double-clicked confirm; deleted across the WIP boundary. | KF-193 KF-120 |
| R2-071 | PASS | With Labels set to Show, chips render only in that column. | Columns with mixed show/hide settings. | KF-043 |
| R2-072 | PASS | No tomato badge appears; a completed pomodoro shows "1m". KF-055's tomato comes from old (v1-era) video. The current 10-05 and 10-12 captures show a card after a completed pomodoro with no tomato, and ChipFlow matches them. | Completed vs. stopped session. | KF-055 KF-200 |
| R2-075 | PASS | 7 items, each with an icon; Delete is black; Start timer has no submenu arrow. | Done-column card; POMO board. | KF-050 KF-140 KF-329 |
| R2-076 | FAIL | The submenu shows the 8 names (10 on POMO), and recoloring persists and updates the title. But the rows aren't tinted: each has a 0.9rem colored bar on the left, where KF-333 says the whole row is tinted (`shots/s9-02-color-sub.png`). | | KF-333 KF-142 |
| R2-077 | FAIL | Every card offers only "Move advanced", because the code reads `card.dataset.columnId` and the card template never renders it. "Move advanced" fills the submenu but leaves it hidden, so the card can't be moved (`s9-cards.json`, `s9c-advanced.json`). This is a regression. | | KF-334 KF-141 |
| R2-078 | PASS | Start timer acts directly: the timer starts and the card gets the orange dashed outline (rgb(250,162,0)). There is no "Select in timer". | Started on a board with swimlanes. | KF-327 KF-050 |
| R2-079 | FAIL | Mid-drag the placeholder keeps the card's own solid 1px border; the dashed placeholder rule loses on specificity (`s8d-cols.json`). The drop itself persists. | | KF-120 |
| R2-080 | PASS | After editing in the modal and pressing Ctrl+Enter, the card updates without a reload. Dragging the card while its timer runs keeps the outline and live badge. | Unicode rename; card moved while timing. | KF-185 KF-225 KF-199 |
| R2-081 | PASS | The styled confirm deletes the task and the count updates live. | Double-clicked OK. | KF-148 KF-184 |
| R2-082 | PASS | Task name font goes from 12.48px to 19.2px, the setting persists across reload, and it applies on every board. | Reload; checked another board. | KF-353 |
| R2-083 | PASS | All 5 tooltips match verbatim. Icon toggles persist across reload, and the comments icon opens the modal. | Reload persistence. | KF-224 |

## 3. FAIL summaries
- **R2-015 [HIGH]** On a board with a "Default" lane plus another lane, the Default row is missing its label cell, so its cards shift one column left. No tracker defect covers this.
- **R2-077 [HIGH]** The card Move menu is dead. Regression of RELEASED KF-334 and KF-141.
- **R2-049 [MEDIUM]** Moving tasks while deleting a column fails with 400s. Regression of RELEASED KF-306.
- **R2-017 [MEDIUM]** Folded-lane menus do nothing, and Move up/down ignore the Default and folded lanes.
- **R2-023 [MEDIUM]** The "Unassigned" and named-user filters are no-ops.
- **R2-056 [MEDIUM]** Duplicate "Today" group; no empty "Today" header. KF-326 (RELEASED) only covered the single-group case.
- **R2-058 [MEDIUM]** Task sorting, Column sum and Archiving have no effect on the board.
- **R2-002 [MEDIUM]** The pill sits where current KanbanFlow shows a clock icon in the board header row. KF-239 relied on old-era video.
- **R2-028 [MEDIUM]** The Pomodoro template palette regressed (verified correct 09-30).
- **LOW:**
  - R2-005: the "?" button is dead.
  - R2-006: the login page logs 401 console errors.
  - R2-021: the filter controls and panel position differ from KF-215 (which is FIX).
  - R2-024: the month and year date options are missing.
  - R2-029: a failed template delete throws a ReferenceError.
  - R2-033: P opens the wrong menu.
  - R2-046: the column menu has a 4th "Delete" item (references conflict).
  - R2-048: the Archiving default is keyed to the column name.
  - R2-051: mid-drag the column body stays visible.
  - R2-052: the empty column shows a count. KF-316 (RELEASED) likely misread the adjacent collapsed strip.
  - R2-053: overflow scrolls the whole board.
  - R2-057: grouping_date is inert.
  - R2-076: color submenu rows aren't tinted.
  - R2-079: the card drag placeholder isn't dashed.

## 4. Proposed new defects
Each one is `Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: <path under $R/results/board/>`.

### KF-NEW-board-1 — Default swimlane row shifts one column left when the board has more lanes [HIGH]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s4b-grid.json, shots/s4-03-lane-two-unfolded.png
- KanbanFlow: every card renders under its own column header.
- ChipFlow: the board template emits the corner `<th>` but skips the row-label `<th>` for the lane named "Default", so that row's cells start under the corner cell.
- Fix direction: emit a label cell for every row whenever the corner cell exists.

### KF-NEW-board-2 — Card Move submenu offers only a dead "Move advanced" (regression of KF-334/KF-141) [HIGH]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s9-cards.json, s9c-advanced.json, shots/s9c-01-after-advanced.png
- KanbanFlow: Move right / Move to bottom / Move advanced (GM-067).
- ChipFlow: the submenu code reads `card.dataset.columnId`, which the card template never renders. "Move advanced" fills `#card-ctx-submenu` but leaves it hidden.
- Fix direction: take the column from `card.closest('.task-list')` and unhide the submenu after `hideCardSubmenu()`.

### KF-NEW-board-3 — Deleting a non-empty column with "Move the tasks" fails (regression of KF-306) [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s8b-cols.json
- KanbanFlow: the tasks move to the chosen column and the column is deleted (GM-164).
- ChipFlow: `deleteColumn` POSTs moves without the required `position` (400 x4), then the DELETE returns 400 and shows "Could not delete column."
- Fix direction: send an end-of-column position and stop the chain if a move fails.

### KF-NEW-board-4 — Swimlane menu dead on folded strips; Move up/down skip Default and folded lanes [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s4-swimlanes.json
- KanbanFlow: no reference (swimlanes are a premium feature). ChipFlow's own menu promises Rename / Move up / Move down / Delete.
- ChipFlow: the lane id comes from `closest('.swimlane-label')`, which is null on a folded strip, so every action does nothing. `moveSwimlane` only counts labeled lanes.
- Fix direction: read the id from `data-lane-menu-for` and use the full lane order.

### KF-NEW-board-5 — Filter "Unassigned" and named-user options show all cards [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s5-filter.json
- KanbanFlow: the User filter narrows the board (KF-134).
- ChipFlow: 22 of 22 cards stay visible for both options (17 and 5 expected). `applyFilter` treats them as "show all" even though tasks carry `member_ids`.
- Fix direction: render member ids on cards and filter on them.

### KF-NEW-board-6 — Done groups duplicate "Today" and omit the empty "Today" header [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: shots/s8c-03-done-dup-today.png, shots/s8c-04-done-no-today.png
- KanbanFlow: one group per day, newest first, with an empty "Today" header above older groups (General board and 10-12 captures).
- ChipFlow: groups are built from consecutive card positions, so a today task below older ones forms a second "Today". With nothing completed today there is no "Today" header at all.
- Fix direction: sort by completion date before grouping and always emit "Today".

### KF-NEW-board-7 — Column Task sorting, Column sum and Archiving are saved but have no effect [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s8b-cols.json
- KanbanFlow: these settings change the column (v1-e019, GM-045).
- ChipFlow: nothing in `src/` or `static/app.js` reads `sorting`, `column_sum` or `group_by_date`.
- Fix direction: implement the three settings or remove them.

### KF-NEW-board-8 — Idle timer pill in the top bar; current KanbanFlow shows a clock icon in the board header row [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: shots/s1-01-idle.png plus the 09-30, 10-05 and 10-12 golden-master captures
- KanbanFlow: no top-bar pill; a clock icon at about (1714,57); while timing, the Pomodoro panel drops from that row.
- ChipFlow: a "▶ 25:00 ▾" pill in the dark top bar. KF-239 relied on v1-era frames, which also show board tabs that KF-174 rejected as outdated.
- Fix direction: settle the rule against the current captures, then move the timer control to the board header row.

### KF-NEW-board-9 — Built-in Pomodoro template seeds the plain 10-color palette [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s3-boards.json (pomo.colors)
- KanbanFlow: "1 Pomodoro (Default)", "2 Pomodori", "3 Pomodori", ">3 Pomodori" enabled, 6 colors disabled (KF-331 capture). The 09-30 batteries verified this.
- ChipFlow: Yellow through White, all enabled.
- Fix direction: restore the four renamed colors in `pomodoro_template_snapshot` and disable the rest.

### KF-NEW-board-10 — Top-bar "?" button does nothing [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: shots/s1-03-help.png
- KanbanFlow: "?" opens help (KF-031; destination never captured).
- ChipFlow: no listener is attached.
- Fix direction: call `BoardChrome.openHelp()`.

### KF-NEW-board-11 — Login page logs two 401 console errors [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: shots/login-fresh.png.log.json
- KanbanFlow: n/a (a clean console is a ChipFlow gate).
- ChipFlow: the timer bootstrap fetches `/api/timer/settings` and `/api/timer/status` before the user is logged in.
- Fix direction: skip the timer bootstrap on unauthenticated pages.

### KF-NEW-board-12 — Filter Date lacks the month and year options [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s5-filter.json
- KanbanFlow: offers "Due in October", "Due in 2026" and similar (KF-134).
- ChipFlow: `addDateOptions` targets `#filter-date-options`, which was removed when KF-175 changed the control to a select.
- Fix direction: append the options to `#f-date`.

### KF-NEW-board-13 — Template delete error path throws a ReferenceError [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s3-boards.json
- KanbanFlow: n/a.
- ChipFlow: in `templates/new_board.html`, the second script calls a `showError` that only exists inside the first IIFE.
- Fix direction: define or share the helper.

### KF-NEW-board-14 — P with the task modal open opens the stale global reports menu [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s7-keys.json
- KanbanFlow: P opens the modal's Reports menu (GM-093, GM-089), just as T opens the modal's Timer menu (KF-298).
- ChipFlow: P opens the placeholder `#reports-menu`.
- Fix direction: when the modal is open, click `[data-tm-menu="tm-reports-menu"]`.

### KF-NEW-board-15 — Edit-column Archiving default keyed to the column name [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s8b-cols.json (editDone)
- KanbanFlow: Archiving is checked for the Done column (KF-315).
- ChipFlow: the default checks whether the name contains "done", so the dialog misreports the saved value.
- Fix direction: default from `data-is-done`, and only when there is no saved value.

### KF-NEW-board-16 — Column overflow scrolls the whole board [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: shots/s8d-02-overflow-scrolled.png
- KanbanFlow: the column gets its own scrollbar (General board capture).
- ChipFlow: `main.board-wrap` scrolls all columns together.
- Fix direction: give column bodies a bounded height with `overflow-y: auto`.

### KF-NEW-board-17 — Card drag placeholder keeps the card's solid border [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s8d-cols.json, shots/s8d-05-card-middrag.png
- KanbanFlow: a dashed placeholder (`2px dashed rgba(0,0,0,.2)`, per fidelity-status).
- ChipFlow: the placeholder shows the card's 1px solid color border.
- Fix direction: raise the placeholder rule's specificity.

### KF-NEW-board-18 — grouping_date is stored but never used [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (board). Evidence: s8d-cols.json
- KanbanFlow: has an "Edit grouping date" action (v1-S2, KF-050). KF-140's live menu omits it, so it may only appear on Done-column cards; this needs a capture.
- ChipFlow: the value is stored but the group still comes from `completed_at`.
- Fix direction: use `grouping_date` when it is set.

## 5. BLOCKED rows
None. These would benefit from a fresh capture from Chip's KanbanFlow account:
- the column right-click menu (R2-046)
- the color submenu rows (R2-076)
- a column mid-drag frame (R2-051)
- "Edit grouping date" on a Done-column card (R2-057)
- the palette of a freshly created Pomodoro board (R2-028)

## 6. Observations
- **Favorites came from a browser sidebar.** The v4 video "sidebar" behind KF-088 lists browser tabs, so it isn't KanbanFlow UI. `static/app.js` still has dead favorites code aimed at a missing `#bs-favorites`.
- **KF-236 and KF-239 rest on old video.** The chart icon and the pill both come from v1-era frames; no current capture shows either.
- **Due-line wording (for pixel rows R2-065/066).** `format_due_card` renders any date within ±7 days as a weekday. Today's 17:00 due date reads "Friday 5:00 PM" where the capture says "Today 5:00 PM", and an overdue date 5 days back reads "Sunday 5:00 PM" where the capture would say "25 September 5:00 PM".
- **Column counts (for R2-036).** Normal column headers show count badges; the GM and General captures show none on non-WIP columns.
- **Inline subtasks (for R2-068).** The column default Subtasks = Hide, but the GM capture shows inline subtasks.
- **Template-created Done tasks.** Tasks with a null `completed_at` group under a flat "Completed" header.
- **Board list order.** Lists are ordered by board UUID, so a new board lands at a random position.
- **Archived boards by URL.** An archived board opens normally from its URL with no indication that it is archived.
- **Remember filter leaks across boards.** It's one global localStorage key: a red filter remembered on GM hid all 3 cards on General.
- **Overdue indicator ignores the filter.**
- **Swimlane delete confirm.** On a non-empty lane it offers a Delete that is certain to fail.
- **Layout view.** The hint still refers to "⋮ menus", which have been removed, and Esc doesn't close the view.
- **Context submenus.** Opening a card submenu hides the main context menu.
- **Quick-add placement.** New cards are inserted at the top of the column.
- **Large task names.** The setting is global, not per board.
- **Avatar initials.** Card avatars show "A" while the header shows "AU".
- **Aborted requests.** Calls whose response body is never read, or that race a reload, show up as `net::ERR_ABORTED` and would trip a strict "zero failed requests" gate.

DONE: board 29 PASS 23 FAIL 0 BLOCKED

**Top findings:**
- **Grid shift (R2-015, HIGH).** On any board with a "Default" lane plus a second lane, the Default row loses its label cell and its cards display one column left. Nothing in the tracker covers it.
- **Card Move menu is dead (R2-077, HIGH).** "Move right" and "Move to bottom" never appear because the code reads a `data-column-id` the card template doesn't render, and "Move advanced" builds its list into a hidden submenu. Regression of RELEASED KF-334 and KF-141.
- **Column delete with move fails (R2-049).** Moving tasks while deleting a column returns 400 because `position` is missing. Regression of RELEASED KF-306.
- **Swimlane menu (R2-017).** Rename and Delete do nothing on folded lanes, so an empty lane can't be deleted from the board, and Move up/down ignore the Default and folded lanes.
- **Inert settings.** The "Unassigned" and named-user filters do nothing (R2-023); column sorting, sum and Archiving are saved but never applied (R2-058); `grouping_date` is ignored (R2-057).
- **Done grouping (R2-056).** A second "Today" group appears when a card is placed below older groups, and the empty "Today" header is missing. Several Done days were seeded by restarting the server under faketime.
- **Pomodoro template palette (R2-028).** It regressed to 10 plain enabled colors.
- **References built on old video.** The current captures contradict KF-239 (timer pill), KF-236 (chart icon), KF-088 (Favorites, from a browser sidebar) and probably KF-316 (column count). These affect the pixel rows R2-001, R2-036 and R2-065/066.
