verify-37b report: UI half of batch-37, candidate integ/batch-37 313c7a0 vs baseline 9548374. Every claimed UI fix passed, one real regression found (new defect A), and one acceptance check (KF-481) is only partly met.

ENVIRONMENT
- Worktrees: .dev/worktrees/verify-37b (313c7a0) and verify-37b-base (9548374). Release builds, built one cargo run at a time.
- Binary md5: cand 2a01869dd07abf3a017e30e229f0e403, base 39447dcdbfcb4f54613330cdcb765eae. `rg -a -c data-overdue`: cand 2, base 1.
- Ports: cand 3388, base 3389, fresh DBs under $S/verify-37b/ ($S = the session scratchpad). Timed runs used libfaketime with `FAKETIME_TIMESTAMP_FILE` (no trailing newline, NO_CACHE, DONT_FAKE_MONOTONIC) and Playwright `context.clock` in lockstep.
- Viewport 1440x612 DPR1, light and dark, unless stated. Playwright 1.63.0 via node only.
- Evidence: $S/verify-37b/ holds the scripts (w1*.mjs, w2*.mjs, t*.mjs, sx*.mjs, r*.mjs, sw*.mjs, dd1.mjs) and shots/*.png.
- Servers: both killed by PID at the end. `git status` is clean in my worktree.

GATES (cand)
- fmt: exit 0.
- clippy --all-targets: exit 0, 0 warnings.
- `TZ=UTC cargo test`: exit 0, 16 `test result` lines, all "0 failed".
- jsc checkSyntax on `static/app.js`: exit 0.

W1 BOARD GRID (numbers are baseline -> candidate)
- KF-483: title colour rgb(31,41,55) -> rgb(51,51,51) = #333. PASS.
- KF-482: avatar ring is now the card border colour for all 8 standard colours in light and dark (22 cards, 0 mismatches; baseline 22 of 22 mismatched). Green is #59d600. PASS.
- KF-474: dark lane-menu hover renders (84,84,84) -> (88,88,88) = #585858. PASS.
- KF-486: strip box y 118 (h28) -> 119 (h27); warning line bottom is 119. Pixel (296,118) is (243,243,243) in light and (68,68,68) in dark. PASS (that pixel was already the strip colour on the baseline; the fix is the box geometry).
- KF-487: collapsed "In Progress" name box 20x122.14 -> 20x118.39 (golden 118). PASS.
- KF-503: all columns collapsed gives 28 px strips at 1440 and 1920 (baseline 284 and 380). One column expanded leaves the others at 28. PASS.
- KF-445: header rule at (80,117) is now rgb(195,195,195) under the lane corner (baseline white); the dark rule is present too. PASS. I did not test scrolled lane labels.
- KF-477: Done group labels with grouping dates 2025-10-16 / 2027-01-02 / 2026-10-14 now read "Thursday, 16 October 2025", "Saturday, 2 January 2027" and "Wednesday, 14 October" (unchanged for the current year); baseline had no year. PASS.
- KF-478: after 11 random subtask toggles on one card, folded label equals the label after reload 11 of 11 times (baseline 9 of 11 drifted). Enter and Space on `.card-sub-more` toggle it (baseline did nothing). PASS.
- KF-476:
  - "Bookmark filter" button 41.6 px high -> 26.6 px, `white-space: nowrap`.
  - Remembered `{user:"me"}` now selects the current user with the funnel active (baseline left the select blank).
  - After right-click "Assign members" the card leaves the Unassigned filter with no reload and shows its avatar (baseline stayed visible, stale).
  - PASS.
- KF-440: with To Do and In Progress collapsed, filtering to blue gave a count badge of 0, no darkred, and no warning line on the candidate. Baseline kept darkred and the line stale.
  - With `prop_due_dates:"hide"` on To Do, filtering to red keeps "1 overdue task" and filtering to blue hides the strip with the text "0 overdue tasks".
  - PASS.
- KF-441:
  - Dropping a card on a collapsed strip made 0 move requests and left the order unchanged (baseline: `POST /move`, card reordered in its source column).
  - Strip has role=button, tabindex=0, aria-expanded=false; Enter and Space expand it.
  - PASS.
- KF-406 UI:
  - Main lane renamed "Work" at position 0 plus a user lane "Default": Work is an unlabelled gutter, Default is labelled with a menu (baseline was inverted).
  - A copy of the baseline DB with that lane state, opened on the candidate, adopts Work by position (startup log "got main swimlane … (KF-406)").
  - PASS.

W2 TIMER AND DIALOGS
- KF-485:
  - Start button 112x46 / 17 px -> 106x44 / 20 px.
  - Stop button 104x44 at (1133,100) / 20 px.
  - Take break button 104x44 at (1133,100), 16 px with a two-line label, bg #49a849.
  - Idle task row y165 h28 and "Select task" y200.
  - Running task row 1007,164,230,30. Panel 992,40,260x310.
  - PASS.
- KF-491: General settings box 416.17 -> 441 (also 441 on the Task settings and Advanced tabs). The Layout tab is 461.64 on both builds (unchanged, not in this stream). PASS.
- KF-496: every `.log-when` matches `^\d+:\d\d [AP]M - \d+:\d\d [AP]M$` (en dash before). PASS.
- KF-497:
  - Relative to the dialog's left edge, the board select is at 15; the period select is 180x30 at 135.
  - Entry type is at 325, i.e. golden x 710 after the dialog offset.
  - Four 30x30 icons at 505/540/575/610 (golden 890 is dialog-left 385 + 505).
  - PASS.
- KF-501: deleting a timed task from the card menu, the pill is idle after 46 ms and the panel reads "No task selected". Baseline never went idle in 1.5 s. The modal path was already fine (155 ms on baseline, 180 ms on candidate). PASS.
- KF-504:
  - Due dialog with its alert on top: Escape closes the alert only. With the task modal and its dialog on top: Escape closes the dialog only.
  - Timer log with the entry dialog on top: Escape closes the entry dialog only (baseline closed the log and dialog with the first Escape).
  - The due-dialog/alert and modal/dialog cases did not reproduce on the baseline; the timer-log case did.
  - PASS.
- KF-505 from the task modal's due chip (modal left 415.5):
  - Popover 280x440 at x = modal-left + 27, top 172.
  - Internals match the acceptance list exactly: date (10,65,125x30), calendar (12,96,256x201), target select (10,324,260x30), Delete (10,400,125x30), Update (145,400,125x30).
  - No console errors. PASS on geometry. See observations B and C for styling and the Add-menu geometry.
- KF-506: 9000 s entry reads "2h 30m" in the log (baseline "150m"). A running stopwatch at 85 min reads "0h + 1h 25m / 8h" on the card and "1:25:02" on the pill. PASS.
- KF-481 (partial): at 1440x435 the quick-add Members popover is 250x103 at (500,205).
  - Clicking Members saves nothing (task count unchanged; baseline saved the task and opened details).
  - Enter in the search does nothing; the first Escape closes the popover.
  - A member picked in the popover is assigned on save. Ctrl+Enter saves and closes; Ctrl+Shift+Enter saves and keeps the dialog open; Shift+Enter saves and opens details.
  - PARTIAL because of observation A.
- KF-488: stop at 5 s and 19 s discards (`discarded:true`, no reason menu). Stop at 20 s and 21 s shows "Why did you stop?". `agents/skill.md:301` documents the 20 s discard. PASS.

S/X UI
- KF-407: with one active board plus one archived, the delete page carries "cannot delete the last active board; unarchive another board first" up front and in the error line. API DELETE gives 400 with the same text. Baseline offered the confirm dialog. Deleting a non-last board still works. PASS.
- KF-492: the Archive button on that page shows "Could not archive board: cannot archive the last active board". API PUT archived gives 400 with the same text. Archiving works when two boards are active. PASS.
- KF-460: with same-named tasks on two boards, a manual entry added from the modal lands only on the dialog's own task (baseline put it on the other board's task). Typing the shared name in the timer popup "Add time" dialog is refused with 'Several tasks are named "…"; add the time from the task itself.' (baseline saved it silently). A unique name works. I did not test the edit-entry dialog. The Rust test was not inspected, but cargo test is green. PASS.
- KF-455: `POST /api/boards` with 101 characters gives 400 "Board name must be 1-100 characters." (baseline 200). 100 characters is accepted; blank is refused. The new-board page also refuses 101 characters, but with a generic message (observation D). PASS on the API.

REGRESSION: PASS
- KF-457 widths with the legend pinned:
  - 1920/1440/1120/900 give columns 380/284/220/220 and cards 363/267/203/203 (last column −1), identical to the JYv2f3u dumps and to the baseline.
  - Legend at y877, h23, full width.
- Header and sum badges at 1905/1440/1240/1120, light and dark, hours and count sums on: 0 overlaps among header children and column name/badge/add. `board-header-row` is at y34 and the menu stays inside the viewport.
- V top bar: pill at x1218 idle, running chip 145x34 at (1106,40), bg #484848. Stop square 1119,47,20,20 and clock text 1150,42,57,30 match golden timer/22. Idle panel 992,40,260x310.
- Timer states:
  - Break panel: title "Pomodoro", label "Break time", Start 106x44.
  - Break chip: 145x34, bg #484848, icon at (1123,47.5).
  - 00:00 hold holds at "00:00".
  - Tab title alternates "00:00 …" / "-------- …".
- G board-only dark mode: `body.dark` only on the board and `/`; settings, reports and timer pages stay light.
- Due dialog: future date saves, past date shows the confirm with Cancel focused, Esc keeps the stored value, OK saves, Delete clears. Same behaviour on candidate and baseline.
- Help dialog 370,61,700x490 and filter panel 1190,34,250x578, both match the goldens.
- Drag and drop, cand and base: cross-column, reorder, drop into Done, and a move into another swimlane each make exactly one `POST /move` with the expected result.
  - Done keeps the "Today" group; candidate shows `completed_at` stamped (the baseline task GET does not expose `completed_at`; KF-494 adds it).

UI-SWEEP: PASS
- Light and dark, 26 pages (board, `/`, 11 report pages, 8 board-settings pages plus delete, `/settings`, `/timer/log`, `/timer/statistics`, `/boards/new`, `/boards/archived`, `/device`, `/agents.md`): 0 console errors, 0 failed requests.
- 12 overlay and menu states, light and dark: 0 errors. The exception is the known delete-reload `net::ERR_ABORTED` on `/api/tasks/<id>`.
- The only HTTP 404 was my own guess `/timer/time-spent`, which is not a route. Expected 400s from refusal tests are not defects.

BREAK-ATTEMPTS I TRIED
- KF-441: Space key and a drop from another column.
- KF-440: a column with due dates hidden; filtering by all 7 colours.
- KF-503: 1920 width, and one column expanded.
- KF-477: dates in 2025 and 2027.
- KF-478: 11 toggles including expand/fold in between.
- KF-406: a copy of the baseline DB, and an empty "Default" lane.
- KF-501: both delete paths.
- KF-504: three overlay combinations.
- KF-488: boundary at exactly 20 s.
- KF-460: shared name typed in the popup dialog.
- KF-455: 100, 101, blank, and padded names.

OBSERVATIONS (candidate defects)
A. Quick-add dialog stays open after the Members popover closes (found by testing KF-481).
- Evidence: with the Members popover opened then closed by Escape, three more Escapes do not close the quick-add dialog (the popover's search input was focused, and after close focus falls to BODY, which the dialog's Escape path does not reach). This did not occur on the baseline. Script `w2k.mjs`.
- Acceptance: open quick-add, click Members, press Escape twice with no click in between, and `#add-task-overlay` is hidden.
B. KF-505 popover internals differ from the golden's styling (geometry matches). `shots/w2g-cand-chip-l.png`, `shots/w2g-cand-chip-d.png`.
- Candidate: dialog shadow `rgba(0,0,0,0.5) 0 16px 48px`, title bar has no #f4f4f4 fill, calendar day cells have no #f5f5f5 fill, Repeat checkbox is 17.59 px.
- Golden dark/modal/23: shadow `rgba(0,0,0,0.18) 0 2px 9px 4px`, header #f4f4f4, day cells #f5f5f5, checkbox 15x15.
- Acceptance: `.duedate-dlg` computed `box-shadow` is `rgba(0,0,0,0.18) 0 2px 9px 4px`, `.dlg-title` bg is #f4f4f4, and `#dd-repeat` is 15x15.
C. KF-505 Add-state geometry. Add menu, Due date: candidate popover is 280x440 at (1080.5,61), and it includes Target column and Repeat. Golden modal/22 shows 280x350 at (712,104) with Date/Time, calendar and Add only. `shots/w2g-cand-add-l.png`.
- Acceptance: the "Add due date" popover is 280x350 at modal-left+297, top 104, or the golden's Target column/Repeat presence is captured.
D. New-board page, KF-455. A name over 100 characters shows only "Could not create the board." (`shots/sx2-cand-l.png`). The page itself is unstyled (the diff does not touch that template, so likely pre-existing).
- Acceptance: the page shows "Board name must be 1-100 characters." for a 101-character name.
E. Timer panel task link text is 13 px. "Change task" is 73x16 at x1085.5 versus golden 77x16 at x1083 (font 13.3333 px in the golden); "Select task" is 63.6 wide, centred.
- Acceptance: `#tp-change-task` is 77x16.
F. Overdue strip inner text box (low confidence, no pixel proof). `.columnOverdue-text` is at y118 h27; golden `overdueInd` is at y119 h27 (the strip box itself is 119 with h27).
- Acceptance: `.columnOverdue-text` top equals the warning line bottom (119).
G. Unexplained, not reproduced. On the baseline server, after many of my PATCH `/api/columns` calls, In Progress lost its `{"prop_subtasks":"show"}` config. The same call sequence on the candidate kept it. I could not reproduce it in isolation, so it is probably an artefact of my own scripts; low priority.

DONE: BATCH-37-B PASS (UI half) — all claimed UI fixes verified; one new regression logged (observation A, quick-add Escape after the Members popover) plus the styling and geometry observations B and C
DONE: W1-UI PASS — KF-483, 482, 474, 486, 487, 503, 445, 477, 478, 476, 440, 441, 406 all pass the tracker acceptance lines on the candidate; baseline reproduces each
DONE: W2-UI PASS — KF-485, 491, 496, 497, 501, 504, 505, 506, 488 pass; KF-481 is PARTIAL (popover geometry, Enter, Escape-first and shortcuts pass; Escape-after-close does not, observation A)
DONE: S-X-UI PASS — KF-407, 492, 460 and 455 verified (new-board message wording is observation D)
DONE: REGRESSION PASS — widths, header/badges, V bar and timer goldens, dark mode, due dialog, help dialog, filter panel and drag-and-drop all match the goldens and the baseline
DONE: UI-SWEEP PASS — 26 pages and 12 overlay states, light and dark, 0 console errors and 0 failed requests (only the known delete-reload ERR_ABORTED)
