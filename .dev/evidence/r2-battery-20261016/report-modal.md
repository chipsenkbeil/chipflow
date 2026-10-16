# R2 battery, batch `modal` (port 3260)

Scope: every `text`, `functional`, `none` and `n/a` row in the inventory
sections Task modal, Timer / pomodoro, Calendar and Board menu / legend. 68
rows: 46 PASS, 18 FAIL, 2 BLOCKED, 2 N/A.

Paths below are relative to
`/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/r2/results/modal/`
(`shots/` for screenshots, `sNN-*.mjs` for drivers, `sNN-*.log.json` for each
driver's notes, console errors, failed requests and HTTP >= 400, `sNN.out` for
stdout).

## 1. Environment

- Binary: `.dev/worktrees/r2-main/target/release/chipflow` (origin/main
  62d573d, not rebuilt). Server PID 27680, started and killed by exact PID.
- Port 3260, DB `/tmp/chipflow-r2-3260.redb` (fresh). Log `server.log`.
- Host timezone Europe/London (BST, UTC+1). UK DST ends 2026-10-25, which the
  DST edge cases use. Browser contexts use the host timezone.
- Setup: `POST /setup` then `POST /login` as `audit` (cookie jar
  `cookies.txt`). Admin initials render "AU" (the GM profile's "CS" is fixture
  identity only).
- Fixture: `seed-golden-master.sh http://127.0.0.1:3260 cookies.txt`, default
  `--as-of 2026-10-16` (a Friday). Board `930aa16d-1ea8-4cfa-8676-c28f430ba17d`.
  Task-number map in `env.json` (built from fixture order against the
  export-data names). Seed output `seed.out`, `seed.err`.
- Extra boards: "R2 Pomodoro" `b2e357d0-07df-4d4d-af0e-05f4e57aa89a` from the
  built-in "Pomodoro board" template, then palette set to the POMO profile via
  `PATCH /api/boards/{id}/colors/{id}` (Yellow "1 Pomodoro" default, Green "2
  Pomodori", Blue "3 Pomodori", Red ">3 Pomodori", other six disabled). The
  template itself seeds the plain 10-color palette, all enabled. Also "R2 Pomo"
  `ac6dbe26-…` from "Kanban basics" (created by mistake, left untouched) and
  the default "General" board.
- Viewport 1905x1000, DPR 1, Playwright 1.63.0, Chromium 153.0.8010.12,
  reducedMotion, animations frozen, fresh context per driver (`lib.mjs`).
- Timer durations shortened through `PUT /api/settings`
  `{"pomodoro_minutes":1,"short_break_minutes":1,"long_break_minutes":1,"long_break_every":2}`
  (s08), `long_break_every:4` (s09 onward). The API accepts 1 minute although
  the Work time dropdown starts at 15. `pomodoro_minutes:0` returns 200 and is
  ignored.
- Drivers run: `s01-modal-structure.mjs` through `s17-misc.mjs` (`node sNN-….mjs`).
  s03, s05, s09, s10, s16 died partway on my own selectors. Each was resumed in
  a follow-up driver (s04, s06, s10, s11, s17) and no verdict rests on a step
  that did not complete.
- Console/network: zero console errors and zero page errors in every driver
  except s12 (two `400 /api/time/manual`, the future-time validation path,
  see R2-137). Every "failed request" logged elsewhere is one of two harness
  artifacts. A board reload I triggered (`ERR_ABORTED` on `/b/…`) or a UI
  `DELETE` that got a 200 and was then reported `net::ERR_ABORTED` because the
  client never reads the body (proved in `s04-abort.log.json` events: `res 200`
  precedes `fail ERR_ABORTED`, server state changed).

## 2. Verdicts

| Row | Verdict | Evidence | Break-attempt (PASS) | KF ids |
|---|---|---|---|---|
| R2-084 | PASS | #11 modal x648–1248 (600px), top 25, bottom 961 of 1000, bg rgb(255,234,194) orange tint, row1 Color/Members/Labels/Time spent, row2 Time estimate/Due date. `s01.out`, `shots/s01-modal-01-modal-11.png` | Added a 4,994-char comment: modal stays 25–961 and scrolls inside, no horizontal overflow (`s03-edits.log.json` comments-modal) | KF-250 251 319 323 073 |
| R2-085 | PASS | Subline "In Progress · Created: Today" plus 3 `.tm-subline-ico` | Re-read after a rename and reload, unchanged | KF-057 255 256 |
| R2-086 | PASS | Iconbar x1259 (modal right edge 1248), order Add, Move, Timer, Reports, More, Delete, label pill right of circle, Delete glyph rgb(102,102,102) | Opened each menu and pressed Esc: menu closes, modal stays | KF-056 252 266 267 320 321 |
| R2-087 | PASS | Description is a `<div>`, zero textareas. Add > Description opens `#description-dialog`, Save writes "Edited description line one.\nLine two." to export-data and the modal | Clicking the static text also opens the editor. Newline survives the round trip | KF-063 253 322 |
| R2-088 | PASS | Color row "Orange" with rgb(250,162,0) dot. Picker shows 8 swatches titled Yellow…Cyan. Picking Purple re-tints the modal to rgb(237,219,255) and sets `color_id` | Reload, modal still Purple | KF-073 257 214 |
| R2-089 | PASS | Members row "AU" disc plus ⊕. Dialog: "Members", ⚙, `Search...`, row "● audit ✓". Toggle off sets `member_ids:[]`, on restores | Toggled off and on, checked API after each | KF-060 261 328 |
| R2-090 | PASS | Chips rgb(255,245,225) bg, dark text. Dialog has no Save. Adding "urgent" and removing it both hit the API before the dialog closes. Empty board: "No recently used labels exist" (`shots/s16-misc-01-labels-empty.png`) | Removed a label by its × with the dialog open, API updated at once | KF-061 157 260 268 269 317 328 |
| R2-091 | FAIL | See FAIL 1–3 | | KF-062 145 227 259 270 |
| R2-092 | PASS | "Add time estimate ×", placeholder "🕐 Enter time estimate", green Add. "1h 30m" shows "1h 30m", API 1.5 | Inputs "abc", "0h", "2.5": all rejected with a toast, value unchanged (`s17.out`) | KF-066 072 216 |
| R2-093 | FAIL | See FAIL 4 | | KF-013 067 068 258 278 |
| R2-094 | FAIL | See FAIL 5 | | KF-014 |
| R2-095 | PASS | Header "Subtasks 2 / 4", ⋮ menu, ⋮ handle per row, no ×, done rows `text-decoration: none`, "Add subtask..." border 0 | Added a 5th, toggled, renamed (Meta+A), deleted through the styled confirm. Count tracked 2/5, 3/5, back to /4 | KF-058 262–265 |
| R2-096 | PASS | ↓ moves focus to the next name. Ctrl+↑ and Cmd+↓ reorder, `PUT …/subtasks/order` persisted (API order matches DOM) | Both modifier keys, checked API after each | KF-273 |
| R2-097 | PASS | "Comments 2", "AU" avatars, "Today 3:44 PM", single-line "Add comment..." with no button, ⋮ > Delete through styled confirm | 4,994-char comment wraps at 518px. 20,000 chars via API: 400 "comment is too long (max 5000 chars)" | KF-064 254 |
| R2-098 | PASS | Add > Attachment disabled, rgb(156,163,175). No Attachments section. API upload 200, download byte-identical with `content-disposition: attachment`, 11 MB upload 413 | Oversize upload | KF-189 332 |
| R2-099 | PASS | Add menu: Description, Member, Label, Subtask, Due date, Time estimate, Manual time entry, Comment, Attachment, Relation, each with an icon | Clicked every item. Relation only toasts (observation) | KF-059 064 065 066 144 |
| R2-100 | PASS | "Move task ×", Board select (General, ChipFlow Golden Master), static Column "In Progress", green Move | Esc dismisses the dialog first, modal stays | KF-069 070 274 |
| R2-101 | PASS | Timer menu: Start Pomodoro, Start Stopwatch, Time log. Start Pomodoro starts the timer on #11 (`s08.out` after-start) | Started from this menu, status API showed #11 | KF-298 |
| R2-102 | PASS | Reports: Time log, Print, History, Time in column. Time in column toasts "not available" | Opened each item | KF-100 275 |
| R2-103 | FAIL | See FAIL 6 | | KF-101 115 116 277 015 016 |
| R2-104 | PASS | History sub-view lists every edit I made, newest first, author and timestamp (`shots/s06-more-02-history-subview.png`) | Comment add/delete, subtask rename/delete, due done all appear. Raw event codes noted under observations | KF-100 |
| R2-105 | PASS | More: Watch, Task URL, Copy, Keyboard shortcuts. Watch sets `watched:true`, label "✓ Unwatch". Task URL copies `/b/{board}#task-{id}`, and loading that URL opens the modal (`s16.out` deep-link). Copy creates "… (copy)". Shortcuts dialog opens | Followed the deep link in a fresh load | KF-071 117 |
| R2-106 | PASS | Delete shows styled "Delete task / Delete this task and its time entries?". Cancel keeps the task. Delete: card gone, `GET /api/tasks/{id}` 404 | Cancel first, then confirm | KF-148 184 188 |
| R2-107 | FAIL | See FAIL 7 | | KF-299 094 |
| R2-108 | FAIL | See FAIL 8 | | KF-118 302 208 |
| R2-110 | FAIL | See FAIL 9 | | KF-001 138 177 |
| R2-111 | PASS | Running popup: "Time until break", "00:55", purple dot + task name, red Stop, "Change task" (`shots/s08-timer-03-popup-running.png`) | Dot follows the task color after I recolored #11 | KF-001 280 138 |
| R2-115 | FAIL | See FAIL 10 | | KF-279 |
| R2-116 | FAIL | See FAIL 11 | | KF-010 |
| R2-117 | FAIL | See FAIL 12 | | KF-006 283 |
| R2-118 | PASS | Stopwatch idle: "Stopwatch / Session time", big "00:00", green "▶ Start" | Switched tabs both ways | KF-281 |
| R2-119 | PASS | "Stopwatch / Session time / STOPWATCH", count-up, red Stop. Pill "00:04 ▾" with stop icon, title "00:04 ChipFlow Golden Master" | Stopped at 25s, logged kind stopwatch "S" | KF-008 281 |
| R2-120 | PASS | Toast "Session discarded / Session lasted less than 20 seconds", bg rgb(31,41,55), no why-menu, log total unchanged | Stopped at 3s and at 2.5s in two drivers | KF-007 |
| R2-121 | PASS | Dark menu, header "Why did you stop?", 17 items in KanbanFlow order ending "Add new reason…", "Task done" | Reset reasons to the 15 defaults first so a custom reason could not inflate the count | KF-005 011 021 194 077 |
| R2-122 | PASS | "Email" logs reason Email, interrupted, red rgb(248,113,113) "Stopped Pomodoro with reason 'Email'", orange dot in the log modal | Also "Task done" (logged as a reason, task stays in In Progress) and "Add new reason…" (logs "R2 new reason") | KF-012 198 002 |
| R2-123 | PASS | Right-click on the running clock: Comment, Label, Edit, Delete. Comment opens #11 with the composer focused | Exercised Comment end to end | KF-282 |
| R2-124 | PASS | Idle pill "▶ 25:00 ▾", green ▶ rgb(22,163,74). Stopwatch idle "■ 00:00 ▾", red ■ rgb(220,38,38) | Reload in stopwatch mode returns to pomodoro (observation) | KF-098 026 025 229 |
| R2-125 | PASS | Ticks "00:58", "00:57", "00:56" one second apart, zero-padded, red stop square (`crop-pill-running.png`) | Read three consecutive seconds | KF-027 028 197 231 |
| R2-126 | FAIL | See FAIL 13 | | KF-232 020 |
| R2-128 | PASS | General: "Pomodoro durations", Work 25, Short 5, Long break interval "Every 4th break", Long 10, PiP on. Save persists Work 30 (`s16.out` save-api) | Value outside the dropdown (API 1 min) shows "15 minutes" (observation) | KF-074 075 283 328 |
| R2-129 | PASS | 15 reasons with ⋮⋮ handles, ×, green "Add reason". Click on a label gives a yellow rgb(254,243,199) input, rename persists. Drag of row 2 above row 1 persists | Rename plus drag, read back through the API | KF-076 106 |
| R2-130 | PASS | Info text verbatim, Examples box (Meditate, Take a short walk, Switch sitting/standing at your desk), green "Add activity". Dialog "Add break activity", placeholder "e.g. Meditate, Stretch, Drink water, Take a walk", goal 1, "No limit", green button. Added "Stretch" persisted | Added an activity and reloaded | KF-078 284 285 |
| R2-131 | PASS | Ticking Always/Timer start/Never, 9 sounds incl. "Grandpa clock", Bell default, Alarm 70%, Points 70%, Sounds on, PiP on. Save persists grandpa_clock | Changed and saved, read API | KF-079 |
| R2-132 | BLOCKED | No reference audio exists | | KF-079 |
| R2-133 | BLOCKED | PiP toggle persists, the window was never observed in either app | | KF-075 |
| R2-134 | N/A | KanbanFlow has no counterpart. ChipFlow's timer settings and `templates/settings.html` carry no notification control (`rg -i notification` empty) | | KF-105 |
| R2-135 | FAIL | See FAIL 14 | | KF-003 018 102 119 137 271 |
| R2-136 | PASS | Calendar popup "< October 2026 >", "Sun Mon Tue Wed Thu Fri Sat", selected day rgb(37,99,235), Today / yesterday / This month. "yesterday" sets 2026-10-15 | Used the quick button, read the field | KF-291 |
| R2-137 | FAIL | See FAIL 15 | | KF-272 |
| R2-138 | FAIL | See FAIL 16 | | KF-004 017 271 102 |
| R2-139 | FAIL | See FAIL 17 | | KF-090 286 288 113 318 019 |
| R2-140 | PASS | Board filter General: "No entries exist for the given filter" (`shots/s12-log-10-log-empty.png`) | Board with zero entries reached through the real filter | KF-090 |
| R2-141 | FAIL | See FAIL 18 | | KF-093 289 |
| R2-142 | PASS | Stepping onto "Custom (absolute)" opens a dialog with ←, ×, Start/End date, October and November calendars, green Done. 2026-10-15..15 shows only that day | Single-day range | KF-290 |
| R2-143 | PASS | "Custom (relative)" shows "Last N days/weeks". N=1 shows only today | N=1 at the boundary | none |
| R2-144 | PASS | `.card-live` "4h 34m + 0m" while running, hidden after the discard toast (`s17.out`) | Discard path specifically | KF-103 |
| R2-145 | PASS | Status `{active, phase, mode, mode_title, task_id, task_name, task_color, task_url, started_at, duration_secs, remaining_seconds, total_seconds, pomodoro_count}`. Stop `{discarded, minutes, completed}`, 404 "no active timer" when idle. `/api/timer/settings` equals `/api/settings` | Two tabs, a replacement start, a deleted task, a pomodoro left past 0:00 (`s11.out`) | KF-001 002 127–130 |
| R2-168 | PASS | Monday-start October grid, "13 tasks with due dates in view", chips on the right days, Today returns from November, Color=Green and Label=backend filter correctly | Due probes across the 2026-10-25 DST change land on 25 and 26 | KF-338 |
| R2-169 | PASS | #6 with `due_repeat:"every week"` shows on Oct 16, 23, 30, Nov 6, 13, 20, 27, Dec 4. Feed carries `RRULE:FREQ=WEEKLY` | Expansion across a month boundary and the DST change | KF-352 |
| R2-170 | PASS | Subscribe href `webcal://…/api/calendar-feed/{token}.ics`, Copy link "Feed link copied.", Reset link issues a new token | Reset under automation (native confirm noted under observations) | KF-351 |
| R2-171 | PASS | Cookieless GET 200 `text/calendar; charset=utf-8`, 10 VEVENTs. After reset the old token 404s and the new one 200s. Bogus token 404 | Old link after rotation | KF-351 |
| R2-174 | FAIL | See FAIL 19 | | KF-353 |
| R2-175 | PASS | Settings item goes to `/b/{pomo}/settings`, heading "Board settings: R2 Pomodoro" | Opened from the non-default board | KF-154 |
| R2-176 | N/A | Members: dialog "audit — Board owner." Recycle bin: toast "ChipFlow has no recycle bin — deleted tasks are removed permanently." Get Premium: "ChipFlow is free and open-source…" | | none |
| R2-177 | PASS | Help & feedback opens `#shortcuts-dialog` | | none |
| R2-180 | PASS | 4 segments "1 Pomodoro / 2 Pomodori / 3 Pomodori / >3 Pomodori", 476px each across 1905, footer 967–1000 flush. Yellow/green/blue bg equal the 2026-10-12 capture samples exactly (255,255,222 / 220,252,190 / 201,221,253) | Disabled colors absent, renamed labels used | KF-335 121 |
| R2-181 | PASS | Yellow segment `title="Low effort work"` from its description. Segments without a description have no title | Only one color given a description | none |
| R2-182 | PASS | POMO legend on survives reload. GM board stays off, then toggles independently. Config API `{"legend_visible":true}` | Two boards toggled in sequence | KF-222 |

## 3. FAILs

1. **R2-091, due dialog writes every task in the target column** (HIGH).
   Adding today 17:00 to #11 through Add > Due date set the same `due_at` on
   all five In Progress tasks (`s03-edits.log.json` inProgress-due-before /
   after). Cause visible in `static/app.js:4497` `applyToColumn`, which PATCHes
   every card in the selected "Target column". Picking another column would
   overwrite that column and skip this task. Not covered by any tracker entry.
   KF-270 (FIXED) shipped this dialog.
2. **R2-091, relative date words** (MEDIUM). Today 17:00 renders "Friday 5:00
   PM", 5 days past renders "Sunday 5:00 PM". KanbanFlow
   (`kanbanflow-golden-master-board-20260930.png`) shows "Today 5:00 PM",
   "Friday 5:00 PM" for +2 days and "25 September 5:00 PM" for 5 days past, and
   the 2026-10-12 capture shows "2 October 5:00 PM". `format_due_card`
   (`src/routes.rs:971`) applies a weekday to ±7 days. The harness suspect is
   confirmed. Same text in the modal and on cards. KF-259 (RELEASED) owns this
   row's format.
3. **R2-091, "(Done)" missing on seeded dues** (MEDIUM). KanbanFlow shows
   "(Done)" after all three seeded due dates, including the overdue #7 (red !)
   and the overdue #11 in the 10-12 capture. ChipFlow shows it only when
   `due_done` is true, and no UI sets that (only the API). Inference, flagged:
   the span is KanbanFlow's `task-dueDateColumn`, so "(Done)" is likely the
   target column's name, which also explains FAIL 1's dialog field. Regression
   of KF-259's intent.
4. **R2-093, Time spent menu needs a click** (LOW). Hover shows only the
   tooltip, the "Add time entry / Open time log" menu opens on click. The cell
   also has a native `title`, so two tooltips stack. Each modal open appends
   another tooltip and menu node to `<body>` (4 of each after 4 opens,
   `s17.out`). KF-278 (RELEASED) specified a hover menu, so this is a partial
   regression.
5. **R2-094, no pomodoro counter in the modal** (LOW-MEDIUM). With 1
   successful and 9 stopped pomodori on #11 the modal shows only "4h 4m" and
   no "Pomodoro" text anywhere (`s12.out` modal-pomodoro-counter null).
   KanbanFlow shows "🍅 1 Pomodoro" / "● 2 Pomodori" next to Time spent (KF-014,
   KF-067, frame review §c v3-01323).
6. **R2-103, wrong labels in the task time log** (MEDIUM). The completed
   pomodoro's P badge tooltip reads "Stopped with reason 'completed'" (the
   client sends `reason:'completed'` and the server stores it). Stopwatch
   entries read "Stopped Pomodoro with reason 'Other'" in the sub-view
   (`s12.out` subview). KF-015 wants "Pomodoro". The timer log modal gets the
   completed one right ("Successful Pomodoro"), so the two views disagree.
7. **R2-107, Esc commits a title edit and closes** (LOW). Typing a new title
   then one Esc closed the modal and saved "UNSAVED TITLE EDIT" (history shows
   RENAMED). KanbanFlow (GM-151 via KF-299): first Esc discards in-progress
   edits and keeps the modal. ChipFlow's KF-299 scope covers only the comment
   and subtask drafts (`static/app.js:4851`), and those work. Ctrl+Enter saves
   and closes.
8. **R2-108, column "+" opens a dot popover** (MEDIUM). ChipFlow: 272x106
   popover at the "+", placeholder "Task name", 8 bare dots, blue Add, Cancel
   (`crop-newtask.png`). KanbanFlow (`kanbanflow-task-modal-color-dropdown-20260930.png`):
   centered 500px tinted modal, "Task name...", "● Yellow ▾" dropdown, member
   row, Save, ×. KF-302 (RELEASED) only bolted a named-color modal behind a dot
   click. KF-208 claims KanbanFlow's quick-add is a popover, but its evidence
   is a ChipFlow screenshot.
9. **R2-110, idle pomodoro popup** (MEDIUM). No big "25:00" readout. It shows
   Task and Duration (15/25/50 min) selects, a blue "Start Pomodoro" and a
   "Time log" button (`crop-popup-idle.png`). KF-001: idle popup is a big
   "25:00" plus green ▶ Start. The stopwatch tab already matches (KF-281).
   KF-001 is a nostat VERIFIED fix, so this reads as a regression.
10. **R2-115, TODAY list format** (MEDIUM). Against
    `kanbanflow-pomodoro-panel-20261005.png`: heading "TODAY 1M" (CSS uppercase
    of "1m") with no "1 Pomodoro" count, dots hard-coded #4ade80 where
    KanbanFlow uses the task color (orange in the capture), pending row reads
    "— pending" with no start time ("10:35 PM – pending"), × per entry instead
    of ⋮, and × deletes with no confirmation. KF-279 (RELEASED) chose green dots
    from a green-task capture.
11. **R2-116, "Select open task" unreachable** (LOW-MEDIUM). Opening a task
    modal hides the timer popup, and the modal overlay (z 100) intercepts
    clicks on the pill, so the popup cannot be shown beside a modal
    (`s08.out` popup-with-other-modal: popupVisible false, link "Change task").
    KF-010 / frame review §e.4 show the state in KanbanFlow.
12. **R2-117, break cycle and post-break session** (MEDIUM). With
    `long_break_every:2` the first break after the first completed pomodoro
    was a long break, because a stopped pomodoro had already counted. "Back to
    work" started a pomodoro with `task_id:null`, which then completed and was
    never logged (`log_timer_session` skips task-less sessions,
    `src/routes.rs:7015`), and the next break was long again. No short break
    was ever produced. KanbanFlow's setting is "Every 4th break" and its
    take-break panel keeps the task row. KF-006 (nostat VERIFIED) saw a short
    break, so this is a regression.
13. **R2-126, tab title freezes while the popup is open** (LOW). The pill and
    title stopped at "00:53" while the pomodoro ran to 0:00 with the popup
    open, and still read "00:53 ChipFlow Golden Master" after status went idle
    (`s08.out` title-finished). After a discard with the popup open the title
    stayed "00:56". Both correct themselves once the popup closes (`s10.out`
    A2). KF-020 / KF-232 (RELEASED).
14. **R2-135, zero duration renders "—"** (LOW). From = To shows "—". Frame
    review §c (v3-02777): zero duration is "0h". Rest of the dialog matches
    (Task, Date + 📅, From/To, auto Duration "1h 30m", Add labels/Add comment
    checkboxes, Cancel, green Add). Labels input placeholder is "Type a label
    and press Enter...".
15. **R2-137, future-time validation logs a console error** (LOW, hygiene
    only). Wording "You can not enter a time in the future" + OK and dialog
    state are correct. The check is a server 400 on `POST /api/time/manual`,
    which Chromium logs as a console error, and the spec requires zero.
16. **R2-138, Edit Pomodoro entry loses seconds and duration** (MEDIUM). For a
    24s entry the dialog loads From 16:06:05, To 16:06:05, Duration "—", and
    Update without changes fails "End time must be after start time." Moving
    it to 08:00:00–08:20:00 saved the range but kept `duration_display` "24s"
    (`s13.out` ee-after). Button text never showed "Updating…" (KF-004). The
    Timer log modal rows have no `data-entry-id`, so the dialog opens only from
    the task sub-view.
17. **R2-139, timer log text** (LOW). Day header "Friday, October 16 — 4m — 9
    Pomodoros" where KanbanFlow writes "Friday, 10 July — 3h 20m — 8 Pomodoros"
    (day first). Manual rows say "Manual entry", KanbanFlow's board log sub-line
    is "Manually added time" (frame review §c v3-02891). Green/red/gray status
    lines, orange dots for stopped, per-day "Add time entry" (opens the manual
    dialog on that date) all pass.
18. **R2-141, timer log export menu** (LOW, regression). Menu is "Excel
    (Detailed) / Excel (Summary) / PDF (Summary)", the Time spent report's
    items. KF-093 / KF-289 (RELEASED 2026-10-12): "Download Excel file" then
    "Download CSV file". Excel (Detailed) downloads `timer-log-detailed.xls`.
19. **R2-174, board menu structure** (MEDIUM). ChipFlow: 218x405 dropdown
    under the Menu button, no "Menu" title or ×, two separators, "Save board as
    template" between the toggles, "Get Premium" plain text, Color legend ON
    switch green (`crop-menu.png`, `crop-menu-on.png`). KanbanFlow
    (`kanbanflow-general-board-menu-open-20260930.png`,
    `kanbanflow-board-menu-open-20261012.png`): full-height right panel titled
    "Menu" with ×, five groups (Filter/Reports | Board layout/Settings/Members
    | Recycle bin | three toggles | Help & feedback), green full-width "Get
    Premium" button, orange ON switch. KF-353 (RELEASED) covered toggles and
    icons only. The extra template item is expected per the inventory.

## 4. Proposed new defects

### KF-NEW-modal-1 — Add due date dialog overwrites every task in the chosen target column [HIGH]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s03-edits.log.json` (inProgress-due-before/after), `shots/s03-edits-05-after-due.png`
- KanbanFlow: "Add due date" sets the due date of the open task. The column list is the due date's target column (KF-062, KF-270, GM-081).
- ChipFlow: `DueDateDialog.applyToColumn` (`static/app.js:4497`) PATCHes `due_at`/`due_repeat` onto every card in the selected column. Adding a due to #11 set it on all 5 In Progress tasks. Picking another column skips the open task. Empty date plus Add clears the whole column.
- Fix direction: PATCH only the open task. Store the target column on the task if it is kept.

### KF-NEW-modal-2 — Due text shows a weekday for today and for past dates [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s03-edits.log.json` after-due, card/modal probes in the report §2 R2-091, `kanbanflow-golden-master-board-20260930.png`
- KanbanFlow: "Today 5:00 PM", "Friday 5:00 PM" (+2 days), "25 September 5:00 PM" (5 days past), "2 October 5:00 PM" (10 days past).
- ChipFlow: `format_due_card` (`src/routes.rs:971`) prints a weekday within ±7 days: "Friday 5:00 PM" for today, "Sunday 5:00 PM" for 5 days past. Cards and modal share it.
- Fix direction: "Today" for today, weekday only for future dates within the week, "D Month" for past dates.

### KF-NEW-modal-3 — "(Done)" missing after seeded due dates [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: GM board cards #6/#7/#11 via `/api/tasks/{id}/card`, `kanbanflow-golden-master-board-20260930.png`, `kanbanflow-board-20261012.png`
- KanbanFlow: every seeded due date reads "… (Done)", including overdue ones with the red !, in `<span class="task-dueDateColumn">`.
- ChipFlow: "(Done)" appears only when `due_done` is true, which no UI control sets. Inference: the parenthetical is the target column name (default the last column).
- Fix direction: confirm on live KanbanFlow, then render the target column name and drop the boolean.

### KF-NEW-modal-4 — Column "+" opens a dot popover, not the centered new-task modal [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `shots/s06-more-05-new-task-modal.png`, `kanbanflow-task-modal-color-dropdown-20260930.png`
- KanbanFlow: centered tinted modal, "Task name..." input, "● Yellow ▾" color dropdown, member row, Save, ×.
- ChipFlow: 272x106 popover under the "+", "Task name", 8 dots, blue Add, Cancel.
- Fix direction: open the centered new-task modal from "+". Recheck KF-208's popover claim, whose evidence is a ChipFlow screenshot.

### KF-NEW-modal-5 — Idle pomodoro popup lacks the big "25:00" and green Start [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `shots/s07-timer-idle-02-popup-idle.png`
- KanbanFlow: idle popup shows a big "25:00" and a green ▶ Start (KF-001, v1 S4, v3 S2).
- ChipFlow: Task select, Duration select (15/25/50 min), blue "Start Pomodoro", "Time log" button, no readout.
- Fix direction: mirror the stopwatch idle layout (KF-281) with the configured work time.

### KF-NEW-modal-6 — Timer popup TODAY list format [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `shots/s09-stops-03-B-today.png`, `kanbanflow-pomodoro-panel-20261005.png`
- KanbanFlow: "TODAY 58m 1 Pomodoro", dots in the task color, two-line entries, pending row "10:35 PM – pending", ⋮ per entry, "..." overflow.
- ChipFlow: "TODAY 1M" (uppercase), no pomodoro count, dots always #4ade80, "— pending" with no start time, × per entry that deletes with no confirmation.
- Fix direction: match the capture. Route deletion through the styled confirm.

### KF-NEW-modal-7 — "Select open task" cannot be reached [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s08.out` popup-with-other-modal, `shots/s08-timer-04-select-open-task.png`
- KanbanFlow: with the timer popup open and another task's modal open, the task-row link reads "Select open task" (KF-010, frame review §e.4).
- ChipFlow: opening a modal hides the popup and the overlay blocks the pill.
- Fix direction: let the popup stay above the modal overlay.

### KF-NEW-modal-8 — Break flow drops the task, loses the next session, and miscounts long breaks [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s08.out` (break-started, back-to-work, second-break), `/api/timer/log` after the run
- KanbanFlow: the take-break panel keeps the task row (`kanbanflow-pomodoro-panel-20261005.png`). Long break interval counts breaks ("Every 4th break").
- ChipFlow: interrupted pomodori count toward the cycle, so the first break was long with interval 2. "Back to work" starts with `task_id:null`. `log_timer_session` (`src/routes.rs:7015`) logs nothing without a task, so that completed pomodoro vanished and the next break was long again.
- Fix direction: keep the task across the break, log task-less sessions, count completed pomodori for the cycle.

### KF-NEW-modal-9 — Tab title and pill freeze while the timer popup is open [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s08.out` title-finished, `results/modal/s09.out` A-after-stop
- KanbanFlow: the tab title counts down live (KF-020) and returns to normal after a stop (KF-232).
- ChipFlow: with the popup open the title stopped at "00:53" and stayed there after the session ended. After a discard it kept "00:56" until the popup closed.
- Fix direction: tick the title and pill independently of popup visibility.

### KF-NEW-modal-10 — Completed pomodoro tooltip says "Stopped with reason 'completed'", stopwatch rows say "Stopped Pomodoro" [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s12.out` subview, subview-badges
- KanbanFlow: P badge tooltip "Pomodoro" or "Stopped with reason 'X'" (KF-015).
- ChipFlow: `finishSession` sends `reason:'completed'`, stored as `interrupt_reason`, so the tooltip reads "Stopped with reason 'completed'". The task time-log sub-view labels stopwatch entries "Stopped Pomodoro with reason 'Other'".
- Fix direction: send no reason on completion, branch the sub-view text on `kind`.

### KF-NEW-modal-11 — Edit Pomodoro entry drops seconds and never recomputes duration [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s13.out`, `shots/s13-edit-01-edit-entry.png`
- KanbanFlow: From/To with seconds, computed Duration, Update → "Updating…" (KF-004).
- ChipFlow: a 24s entry loads From = To = 16:06:05, Update unchanged fails "End time must be after start time." Saving 08:00:00–08:20:00 kept duration "24s".
- Fix direction: load the true end time, recompute minutes and seconds from the saved range.

### KF-NEW-modal-12 — Timer log export menu regressed to the Time spent report's items [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Regression of KF-289 (RELEASED). Evidence: `shots/s12-log-07-log-export-menu.png`
- KanbanFlow: "Download Excel file", "Download CSV file" (GM-119).
- ChipFlow: "Excel (Detailed)", "Excel (Summary)", "PDF (Summary)" (`templates/board.html` `#log-export-menu`).
- Fix direction: restore the two KanbanFlow items for the timer log.

### KF-NEW-modal-13 — Timer log day header and manual sub-line wording [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s12.out` log-modal
- KanbanFlow: "Friday, 10 July — 3h 20m — 8 Pomodoros", manual sub-line "Manually added time".
- ChipFlow: "Friday, October 16 — 4m — 9 Pomodoros", "Manual entry".
- Fix direction: day-first date, KanbanFlow wording.

### KF-NEW-modal-14 — Task modal shows no pomodoro count [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s12.out` modal-time-spent / modal-pomodoro-counter
- KanbanFlow: "🍅 1 Pomodoro" / "● 2 Pomodori" with Time spent once pomodori exist (KF-014, KF-067, frame review §c).
- ChipFlow: Time spent "4h 4m" only, with 10 pomodoro sessions logged.
- Fix direction: show the count when non-zero, keep "2h 30m" alone when zero (GM-073).

### KF-NEW-modal-15 — Time spent menu opens on click, tooltip doubled, nodes leak [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Partial regression of KF-278. Evidence: `shots/s01-modal-06-hover-time-spent.png`, `results/modal/s17.out`
- KanbanFlow: hover shows the tooltip and the "Add time entry / Open time log" menu.
- ChipFlow: hover shows the custom tooltip and the native `title`. The menu needs a click. `wireTimeSpentHover` appends a tooltip and a menu to `<body>` per modal open.
- Fix direction: open on hover, drop the `title`, reuse one node.

### KF-NEW-modal-16 — Esc saves an in-progress title edit and closes the modal [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s05-more.log.json` esc1/esc-api-name
- KanbanFlow: first Esc discards in-progress edits, second closes (GM-151).
- ChipFlow: the name commits on change, so one Esc saved "UNSAVED TITLE EDIT" and closed. Comment and subtask drafts behave correctly.
- Fix direction: treat a changed title as a discardable draft.

### KF-NEW-modal-17 — Board menu is a dropdown, not KanbanFlow's right-side panel [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `shots/s15-menu-01-menu-pomo-off.png`, `shots/s15-menu-02-menu-pomo-legend-on.png`
- KanbanFlow: full-height right panel titled "Menu" with ×, five separated groups, green full-width "Get Premium", orange ON switch.
- ChipFlow: 218x405 dropdown, no title or ×, two separators, plain-text "Get Premium", green ON switch.
- Fix direction: render the panel, add the separators, style Get Premium and the switch.

### KF-NEW-modal-18 — Manual time zero duration shows "—" [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (modal). Evidence: `results/modal/s12.out` mt-duration-zero
- KanbanFlow: "0h" (frame review §c, v3-02777).
- ChipFlow: "—" when From equals To. The same "—" appears in the edit dialog.
- Fix direction: render "0h".

## 5. BLOCKED rows

- R2-132 (timer audio): needs reference recordings of KanbanFlow's alarm,
  ticking and volume behavior. ChipFlow's ▶ sample button exists, audio was
  not compared.
- R2-133 (Picture-in-Picture window): needs a capture of KanbanFlow's PiP
  window from Chip's account. ChipFlow's toggle persists through Save.

## 6. Observations (no KanbanFlow reference, or another batch's surface)

- `POST /api/tasks/{id}/time {"minutes":30}` at 16:17 logged "4:17 PM – 4:47
  PM", ending in the future. Harness suspect confirmed.
- `due_at` validation: "2026-10-25 01:30" (ambiguous at the UK DST change),
  "2027-03-28 01:30" (nonexistent), "garbage-date" and "2026-13-45" all return
  201 and are stored verbatim (`normalize_due_input`, `src/routes.rs:2236`,
  `.single()` then raw fallback). The due then vanishes from card, modal and
  iCal feed but still shows on the calendar.
- Date-only dues are stored as 23:59:59 in server local time (`+01:00` in
  BST, `+00:00` in GMT), not UTC. The harness suspect saw a GMT date. They
  render "11:59 PM".
- More > Copy copies only name, color and column. Description, labels,
  subtasks, due, estimate and members are dropped.
- History sub-view headings are raw event codes ("SUBTASK_RENAMED",
  "DUE_DONE_UPDATED").
- Modal footer repeats "Created Oct 16, 2026" below the subline's "Created:
  Today".
- `GET /api/tasks/{id}` has no `attachments` key although agent-guide.md says
  it does. Uploaded attachments are invisible in the UI.
- Estimate dialog opens empty instead of showing the current value. "0h" is
  rejected, so an estimate cannot be cleared from the UI (frame review §c says
  KanbanFlow accepts "0h").
- Timer settings: reasons and activities save instantly, durations and sounds
  need Save, Close discards silently. The idle pill keeps the old duration
  after Save until reload. An API value outside the dropdown (1 min) displays
  as "15 minutes" and Save would overwrite it.
- "Add new reason…": Enter in the input does nothing, only the Add button
  submits.
- Stopping a stopwatch after 20s opens "Why did you stop?". No reference says
  KanbanFlow does.
- Stopwatch mode is forgotten on reload.
- The server never completes a pomodoro by itself. With no browser open the
  status stays active with remaining 0, and an API stop logs it as
  interrupted.
- Replacing a running timer logs "Switched task" even for the same task.
- Deleting a task whose timer runs silently clears the timer and discards the
  elapsed time.
- Breaks have no task and are never logged, so Break activities statistics
  stay 0 (Reports batch, R2-149, KF-347).
- Timer log day totals sum per-entry truncated minutes: "4m" for about 9.5
  minutes of sessions. "9 Pomodoros" counts stopped sessions (frame review
  infers successful only).
- Durations of an hour or more show "150m" / "90m" in the time logs and API
  `duration_display`, while cards and modal use "2h 30m". The task sub-view
  uses "4:14 PM - 4:15 PM" (hyphen), other views use an en dash.
- Calendar: month view only, no week view. Clicking a chip does nothing. Chip
  subtitle format "On 11. Oct".
- iCal "Reset link" uses native `window.confirm` (KF-184/KF-188 class). Under
  automation it was auto-dismissed and silently did nothing (first s14 run).
- Running pill digits are white. Frame review §e.2 reports red digits from
  low-resolution frames.
- KF-283 says KanbanFlow's default long break is 10 min (GM-112), frame review
  §b says 15 (v2-03302). ChipFlow uses 10.
- Members dialog adds a blue "Done" button, KF-060 lists none.
- Card avatar "A" versus "AU" in the header and modal (Cards batch).

DONE: modal 46 PASS 18 FAIL 2 BLOCKED (2 N/A)

