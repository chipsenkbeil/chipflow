verify-36b report (UI half of batch-36). Verdict: PASS, with candidate observations. None of the observations is a regression against production a980a7c; most are small deviations from KanbanFlow.

## Environment
- Candidate: worktree `.dev/worktrees/verify-36b` at 2c1773b. Baseline: `.dev/worktrees/verify-36b-base` at a980a7c.
- Release binaries: candidate md5 b34a0a4a710e001b0f9f65891392399d, baseline md5 9dae7a8afa6a8e03c442f6b10cbeb065. `rg -a -c 'Load more tasks'` gives 4 on the candidate and 0 on the baseline.
- Ports and databases: candidate on 3388, baseline on 3389, with `--database-path` DBs `cand.redb` and `base.redb` in the scratchpad `verify-36b/` folder. Both servers are stopped by exact PID (9963 and 14265).
- Gates on the candidate:
  - fmt: OK.
  - clippy `--all-targets`: no warnings (only a `Finished` line).
  - `TZ=UTC cargo test`: all result lines ok, none failed.
  - jsc `checkSyntax` on `app.js`: exit 0.
  - Release build: one compiler warning, identical on the baseline.
- Playwright 1.63.0 driven by node only, 1440x612, DPR 1, real scrollbars on (`--hide-scrollbars` dropped, because the golden captures show scrollbars). Light and dark; dark is `localStorage chipflow-dark=1`.
- Timed timer states used real waits with a 1-minute pomodoro and 1-minute breaks, not libfaketime. This works because the server derives state from `started_at` plus the configured duration.
- Evidence is in the scratchpad `verify-36b/` folder: `shots/*.png`, `t*.mjs`, `sweep.mjs`, `lib.mjs`, `api.sh`, `start.sh`, `dark-montage.png`.

## Findings by area (candidate vs golden or baseline)

**1. Top bar and timer home — PASS**
- Top bar is 34 px, #333333 (dark #222222). The baseline is 36.
- Boards button is 100x26 #515151 at (10,4); dark is #ffffff at 0.15 alpha, matching the dark golden.
- Bell, help and avatar boxes are 36x26 #515151 at x 1302, 1348 and 1394, y 4. There is no reports icon in the top bar; reports open from the board Menu.
- Boards dropdown is at (10,38), 260 wide.
- Idle clock icon is 1218,40,34,34 and the funnel is 1258,40.
- Idle panel is 992,40,260x310 with the chevron at 1216,45,25x25.
- Running chip is 1106,40,145x34, 20 px bold, #484848 in both light and dark. The stop-box icon, time text and chevron match the golden/timer/22 boxes exactly.
- Timer states exercised:
  - idle, running and waiting (the panel clock blinks visible/hidden);
  - short break and long break (green ▶ on the chip);
  - break finished (Break time 00:00, Start #4e4e4e);
  - stopwatch running;
  - stop-reason menu, log modal, settings tab.
- Break-attempt: dark-mode chip, and the long break reached through `long_break_every=2`.

**2. Column header — PASS (layout judged)**
- Name starts 11 px in and the + sits at the right edge.
- Column sum None shows no badge. Task count shows e.g. "5". Estimated hours shows e.g. "17h". A thin vertical rule sits before the +.
- Across 1905, 1440, 1240, 1120 and 900, in light and dark, there are 0 overlaps. The badge-to-+ gap is 6 px, the name-to-badge gap is at least 8 px, and a long name ellipsizes before the badge.
- Badges update live on drag (Backlog 5→4, To Do 17h→21h and back), create (5→6), delete (6→5) and the Blue colour filter (Backlog 1, To Do 8h).
- Judgement: clean grey right-aligned numerals before the +, readable in dark. It reads as a restrained addition, with no KanbanFlow golden to compare against.
- A column with a WIP limit shows only "n / m" and no sum badge.

**3. Stream H UI — PASS**
- KF-215:
  - Panel is 1190,34,250x578 and covers the header controls, matching golden board/11 (label at 1208,87,214x20; Remember row at 364,34; Bookmarks header at 566,46).
  - It stays light in dark mode. The Bookmarks chevron expands and collapses it. An unknown label plus Enter keeps the Blue filter and toasts "Label not found".
- KF-369: on folded strips the menu shows Rename, Move up, Move down and Delete.
  - Move up, rename and delete worked.
  - The menu on the lane that contained a task offered Rename and Move up only, with no Delete.
- KF-370: Unassigned gives 17 cards and the named user gives 5 (22 total). The baseline Unassigned shows all cards.
- KF-371:
  - Name sorting works.
  - The Done column shows 20 cards plus "Load more tasks", then 27 after clicking.
  - A drop inside a sorted column snaps back with the toast "This column is sorted, so its order follows the sorting."
  - A drop into a sorted column from another column lands in sorted place.
- KF-379/475: dialog is 370,61,700x490, tab bar 52 px, rows 38 px, search 590x38 with no outline or shadow, chevrons on the right, and a "Restore task" topic exists.
  - A drag from the search box ending on the backdrop leaves the dialog open, and a drag from the backdrop into the dialog leaves it open. A plain backdrop click closes it.
- KF-381: Date select has month and year options.
- KF-383: P with the modal open opens `tm-reports-menu`. The baseline opens the stale global menu.
- KF-384: Done renamed "Finished" with no saved value opens checked; non-done "Undone ideas" opens unchecked.
- KF-388: the first Esc reverts the title and keeps the modal, and the second Esc closes it. The baseline saved "UNSAVED" and closed.
- KF-392: Total colour is rgb(31,41,55) on #eef0f3.
- KF-393: bookmark save and recall of Period and Entry type works and persists across reload.
- KF-350: cumulative flow shows one legend entry per current column, 7 columns and 7 legend entries, with no phantom band for the renamed columns. A renamed-then-deleted column leaves no band.

**4. Due dialog — PASS**
- Add is 130x30, bold 14 px, #49a849. Delete and Update are 125x30, bold, #d9534f and #49a849.
- Invalid alert is 290x124 over a 0.25 dim with a 75x30 OK, and the first Esc closes the alert before the dialog.
- Past-date confirm is 340x146 with 75x30 buttons. Cancel keeps the dialog and nothing is saved; OK saves. An unchanged overdue date saves with no prompt.
- Delete (KF-401) set `due_at` to null on the open task only; another task was unchanged.
- KF-404: default time is 17:00.
- KF-259: card shows "Due: Today 5:00 PM (Done)" and the modal shows "(<target column name>)". After I renamed Done the modal read "(Finished…)", which matches the target-column semantics in the tracker.

**5. Pages — PASS**
- KF-473: a 100-character name gives `scrollWidth` equal to the viewport at 1905, 1100 and 640. The baseline overflows (1916, 1513 and 1283).
- KF-447: footer is 30 px, rgb(51,51,51) on all 8 tabs. At 1440x612 it sits at y 582-612 on the short tabs.
- KF-416: seven equal 130 px columns, long chip truncated, no horizontal scroll at 1440 or 1100. The baseline scrolls horizontally (1726 and 1556).

**6. Timer UI — PASS**
- KF-461: modal is 660 wide with white #fff groups (border #ccc), dashed rows, italic 11 px #ff0000 stopped status, and no dark backgrounds. A yellow task's dot is rgb(245,204,0) = #f5cc00.
- KF-465: from a waiting break, Stopwatch then Start gave `/api/timer/status` mode "stopwatch", and no select is rendered in the popup.
- KF-450: after a remote delete, Start toasts "The selected task was deleted / Press Start again to run without a task". The baseline silently started a task-less timer.
- KF-452: menu is 260 wide, #3e3e3e, has a back button, row hover #717171, and the panel returns after a reason. The baseline menu is 300 wide.
- KF-453: no console errors or failed requests in either tab, and tab 2 shows tab 1's new selection. I did not reproduce the original double-stop on the baseline.
- Break-attempt for KF-450/452/453: two-tab sync, and a task-less stopwatch.

**7. Regression — PASS**
- KF-457, clean candidate board, legend on: 1920x900, 1440x612, 1120x612 and 900x900 match the responsive/ dumps.
  - Column x/width pairs are identical (380 / 284 / 220 / 220).
  - The legend is pinned at 23 px with 8 equal segments (240 / 180 / 140 / 112.5).
  - The top bar is 34 and the header row is 46.
  - Column body heights are 1 px less than the dump, and the baseline is also 2 px lower overall because its top bar is 36.
- Board geometry (header row, title, card rects, overdue strip, icon grey #808080, "Today" label #888 bold, avatars "AU", column menu = Edit/Collapse/Show details, new tasks at the bottom) is identical to the baseline. One difference is intentional: the overdue due-line colour went from #dc2626 to #8b0000.
- Dark mode board is identical to the baseline apart from the 2 px offset.
- Full sweep of 26 pages in light and dark, plus interactions (layout editor, modal and its menus, filter, board menu, account, notifications, boards dropdown, column and card context menus): 0 console errors and 0 failed responses.
- Sweep overflow: `/timer/log`, `/timer/statistics` and `/boards/new` overflowed horizontally. That came from my 100-character test board, appears identically on the baseline, and was gone after I deleted it. See observation 7.
- Only non-board pages ignore dark mode, which is the existing design.

## Candidate observations
None of these blocks the batch; items 8 and 9 are pre-existing.
1. **Empty archiving column renders an empty "Today" header.**
   - Evidence: `shots/empty-arch.png`. KanbanFlow's empty Done in timer/27 shows none.
   - Acceptance: an archiving column with 0 tasks has no `.done-group-label`.
2. **Timer "Start" and "Take break" button geometry differs from golden.**
   - Start is 112x46 at (1126,99) versus golden 106x44 at (1133,100). Take break stays one line where the golden wraps to two.
   - Acceptance: at 1440x612 `#tp-start` measures 106x44 at x 1133 on the idle panel (golden timer/14).
3. **Timer log overlay and header differ from golden timer/20.**
   - Overlay is rgba(17,24,39,.55) versus golden rgba(0,0,0,.3); the head is transparent where the golden uses #f4f4f4.
   - Acceptance: `.timer-log-overlay` background is rgba(0,0,0,0.3) and `.timer-log-head` is #f4f4f4.
4. **Board settings box is 25 px shorter than golden.**
   - It is 270,74,900x416 versus golden 441, and ends at y 490 versus 515. Nav items sit 1 px higher.
   - Acceptance: `.bshell` height is 441 on the General tab at 1440x612 (golden settings/02).
5. **Hovering "Reports" in the board Menu opens the submenu, and a click then closes it.**
   - Cause: the click handler toggles after the mouseenter has opened it. Clicking without hover works.
   - Acceptance: `[data-bm=reports]` hover followed by click leaves `#reports-submenu` visible.
6. **Long name beside a sum badge ends flush against it.**
   - Gap is 0 px (name right edge 151, badge left edge 151).
   - Acceptance: gap between `.columnHeader-name` and `.columnHeader-countBadge` is at least 4 px.
7. **Long board names stretch the board filter selects.**
   - The selects `#spent-board` and `#stats-board-filter` push the page past the viewport. The baseline does the same.
   - Acceptance: a 100-character board name leaves `scrollWidth` equal to the viewport on `/timer/log`, `/timer/statistics` and `/boards/new`.
8. **Pre-existing: `net::ERR_ABORTED` on delete and lane move.**
   - It appears on task delete, lane delete and lane move, so those pages do not give a strict "0 failed requests". The baseline shows the same on task delete.
   - Acceptance: no `requestfailed` event when deleting a task from the card menu.
9. **Pre-existing: stopwatch Stop shows "Why did you stop?".**
   - golden INVENTORY line 117 says KanbanFlow has no stop-reason menu for the stopwatch. The baseline behaves the same.
   - Acceptance: stopping a stopwatch from the panel ends it with no reason menu.

DONE: BATCH-36-B PASS UI half (KF-233, 239, 330, 366, 463, 462, 369, 370, 371, 379, 475, 381, 383, 384, 388, 392, 393, 350, 215, 401, 432, 436, 437, 259, 404, 447, 473, 416, 450, 452, 453, 461, 465) verified. Nothing found to block the batch; 9 non-blocking observations, none a regression.
DONE: TOPBAR PASS 34 px bar, 100x26 Boards, 36x26 icon boxes, dropdown (10,38,260), clock icon, 145x34 chip with 20 px bold digits, idle panel 992,40,260x310 with 25x25 chevron, dark chip #484848, all timer states
DONE: HEADER PASS name 11 px in, no overlap at 1905/1440/1240/1120/900 in light and dark, Column sum None shows no badge, badges live on drag/create/delete/filter, long name ellipsizes (gap 0 px, see observation 6)
DONE: STREAM-H-UI PASS KF-215, 369, 370, 371, 379/475, 381, 383, 384, 388, 392, 393 and cumulative flow KF-350 verified; empty "Today" header on archiving columns is a minor deviation (observation 1)
DONE: DUE-DIALOG PASS KF-401 Delete, KF-436 buttons, KF-437 alert (290x124, dim 0.25, Esc order), KF-432 past confirm, KF-259 "(Done)", KF-404 default 5:00 PM
DONE: PAGES PASS KF-473 no horizontal scroll at 1905/1100/640, KF-447 footer 30 px #333333 on all tabs, KF-416 calendar 7 equal columns; settings box 25 px short of golden (observation 4)
DONE: TIMER-UI PASS KF-461 log modal 660 px, KF-465, KF-450, KF-452, KF-453; log overlay and header differ from golden (observation 3)
DONE: REGRESSION PASS KF-457 widths and legend match the dumps at 1920/1440/1120/900; stream F and G surfaces identical to the baseline except the intended due-line colour change
DONE: UI-SWEEP PASS 26 pages and interactions in light and dark, 0 console errors and 0 failed responses (only the pre-existing ERR_ABORTED, observation 8)
