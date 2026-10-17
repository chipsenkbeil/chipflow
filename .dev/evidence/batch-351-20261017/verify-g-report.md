## verify-g report: branch fix/dark-settings at 405da66 (origin/main 1194d71 plus 10 commits)

**Result: 8 PASS, 2 FAIL.** KF-373 fails because the dark layout editor has unreadable section headings. KF-368 fails because a long board name overflows the rebuilt delete page. Everything else matches KanbanFlow or the references.

### Environment
- **Worktrees:**
  - fix: `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-g`, detached at 405da66.
  - baseline: `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-g-base`, detached at 1194d71.
  - dark-g was not touched.
- **Diff scope:** db.rs, models.rs, routes.rs, app.js, style.css, the two new `_topbar_*.html` partials, board.html, board_delete.html and board_settings.html. All expected for these defects.
- **Binaries (release, build-dir target/build):**
  - fix md5 b85e41a6307db19d9594fa936adb8f82; baseline md5 76a6de93d110e66d21bcca89625ebf40.
  - `rg -a -c` finds `pomodori_restore_done`, `initTopbarOnly` and `kf-alert-danger` in the fix binary (counts 1, 3, 2) and none of them in the baseline binary.
  - Copies used for testing: `$S/vg/chipflow-{fix,base}`.
- **Instances and data:**
  - fix on 3372 with `/tmp/chipflow-verify-g-3372.redb`; baseline on 3373 with `/tmp/chipflow-verify-g-3373.redb`.
  - Both seeded with seed-golden-master.sh, plus VG board shapes (lanes, twelve lanes, tall, collapsed, legend, drag, long name, "Dale fidelity test").
  - KF-296 used its own `/tmp/chipflow-verify-g-3373-pomo.redb`.
  - Every run passed `--database-path`. All servers were killed by PID and my container is removed.
- **Viewports and capture:**
  - Host Playwright 1.63.0 at 1905x1000, DPR 1, a fresh context per scenario, animations off.
  - Linux container (chipflow-verify-g, the pinned image) at 1919x998 for the light diffs and 1905/1920x1000 for the settings references.
- Evidence lives in `$S/vg/` (`$S` = `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`): `shots/`, `lin-*/`, `set-*/`, `gates.log`.

### Gates
- fmt: exit 0.
- build: exit 0.
- clippy `--all-targets -D warnings`: exit 0, no warnings.
- `TZ=UTC cargo test`: 125 passed, 0 failed, 1 ignored.
- jsc checkSyntax: exit 0.
- **Empty environment** (no HOME, no TZ, empty TZDIR): 2 failures, `format_due_card_weekday_near_absolute_far` and `format_due_modal_relative_and_done_marker` ("got Sunday/Saturday 12:00 PM").
  - The baseline fails the same 2 tests with the same output in the same environment.
  - The diff does not touch `format_due`, so this tree did not cause it.
  - It is the same timezone-test problem the fix/tz-tests and fix/tz-test-sandbox branches are working on.
- The new restore test `starter_board_keeps_pomodori_across_restarts` passes in both environments.
- Console errors: 0. Failed requests: 0. Errors on any page I exercised on the fix: 0.

### KF-373: dark mode themes only the board
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| Top bar #222222, no bevel | not themed | #222222, box-shadow none; KF dark PNG shows no bevel (y33 #212121, then #363636) | ok |
| Board bg #363636, table #4a4a4a, main border #222 | #0f172a, white column bodies | #363636 / #4a4a4a / #222222 | ok |
| Column header #3e3e3e, top #222, left #2b2b2b; name #eee 15/700; count #ccc | dark-on-dark names | exact | ok |
| WIP over count and warning line #ff8080 | darkred | #ff8080 (4.41:1, same as KF) | ok |
| Overdue text #ff8080; Done label and line #eee; add "+" #36b345 | — | exact | ok |
| Board name #eee; Menu/filter/pencil white at 0.7 | name light on light | exact | ok |
| Column details popover | white | #4a4a4a, 1px #222, head #3e3e3e 13/700 #eee, divider #2b2b2b, shadow 0 5px 10px #0000004d, white × | ok |
| Collapsed column name #ccc, badge #eee, over #ff8080 | — | exact | ok |
| Cards and legend stay light | card titles 1.0:1 | card face and text identical to light; legend segments light | ok |
| Text below 4.5:1 on the GM board | 40/68 (34/68 below 3:1, matching R2) | 13/68; only the "+" icons, pill icons and red due text, all identical in light | ok |
| Surfaces that must stay light: modal, card menu, column menu, Boards dropdown, Menu, filter panel, timer panel, timer settings, timer log, bell, account, reports menu | menus, Menu, filter, timer settings, bell and account turned dark; modal text 1.0:1 | light-vs-dark element shots, corners excluded: **0 px diff** everywhere except the Menu's Dark mode switch (36x20, expected) | ok |
| Layout editor: header buttons #555555 | — | #555555; h2 #eee | ok |
| **Layout editor headings "Columns" and "Swimlanes"** | readable on baseline (light --ink) | **#1f2937 on #363636 = 1.21:1, invisible** | **FAIL** |

- **Board shapes in dark:** plain, collapsed, legend, lanes with folded strips, 12 lanes, tall scrolled, Done groups, WIP violation and collapsed-column WIP all render correctly. The sticky header holds when scrolled. No horizontal scroll at 700 or 420 px.
- **Baseline dark problems reproduced:** white column bodies, invisible card titles and board name, menus and filter turned dark, modal text 1.0:1.
- **Minor:** the swimlane "⋮" icons are #6b7280 on #4a4a4a (1.83:1).
- Evidence: `shots/3372-dark-layout.png`, `shots/surfaces-3372.json`, `shots/shapes-3372-3373.json`.

### Inferred values (no capture)
- **Separator crossings #262626: supported.**
  - In KanbanFlow's lossless light capture, a crossing is the average of the two lines: #cdcdcd = (#c3c3c3 + #d7d7d7)/2 and #d3d3d3 = (#c3c3c3 + #e4e4e4)/2.
  - The same rule on the psv dark pair gives (#222222 + #2b2b2b)/2 = #262626.
  - The JPEG captures read about 20 levels darker at crossings in both themes (light #bdc3c1 against the true #d3d3d3; dark #131313 to #171717), which fits #262626.
  - The fix draws #262626 at both the header-top and the cell-top crossings.
- **Folded strips and lane labels with header colours: plausible, not proven.**
  - Name #eee, strip edge #222 and border #2b2b2b follow the psv mapping of their light values.
  - The strip background is a judgement call. In light it is #f3f4f6, which is nearer KanbanFlow's #f3f3f3 cell-header colour (dark #444) than its #eee header colour (dark #3e3e3e). So #444444 is an equally defensible choice; the difference is 6 levels.
- **No top-bar bevel in dark: supported.** The dark PNG goes straight from #222 to #363636, and neither theme's styles.json shows a shadow on the top bar.
- **Boards button #ffffff at 0.15: captured, not inferred.** It is in dark/board/01 styles.json. The PNG shows about #434343 with no emboss bands.

### Light theme unchanged (Linux container, 1919x998)
- Fix against baseline:
  - **0 px diff:** lanes, Menu, column menu, column details, filter panel, timer and layout editor.
  - **48–59 px diff, max channel delta 2:** plain, collapsed, legend, twelve lanes, GM board and card menu. These are anti-aliasing at the corners of the top-bar icon buttons.
  - **20 px diff, max delta 3:** the modal.
  - **16126 px diff:** the Boards dropdown. Its list order follows /api/boards order, which depends on board UUIDs, and that order matches the API on each instance. It is the data, not the code.

### KF-349: reports stay light (resolved by the captures)
- With the dark preference set, all 13 report pages are light on both builds (`body.dark` absent) and **pixel-identical to the baseline (0 px)**, with 0 errors.
- The fix only removes the dead `body.dark` report CSS.
- **The captures support this:** dark/account/01 and dark/account/05 (reports unchanged, white).
- Structural note: ChipFlow reports are standalone pages, while KanbanFlow's are a full-screen dialog over the board. That difference already existed.

### KF-357: settings top bar
- **Bar:**
  - Baseline: no bar, container at y40.
  - Fix: a 36 px #333 bar on the settings, colors and delete pages, container at **y74 (KanbanFlow y74)**.
- **Diffs at 1905 px against the KF-331 reference:**
  - The y0–33 bar differs in 6697 of 64770 px, all logo and icon differences.
  - The gap, the H1 band and the nav are **0 px**.
  - The y34–39 strip differs because the bar is 2 px taller and has a 3 px bevel. That geometry is the global bar and is already tracked by the open KF-233.
- **What works on the settings pages:** the Boards dropdown (open, current board marked, search filters to 1, Escape closes, navigation works), the bell (opens and closes on an outside click) and the account menu ("Signed in as audit", Sign out).
- Help does nothing, the same as on the board page (no handler exists).

### KF-394: dialog close
- Edit, Premium, Set default and Copy dialogs: × and Cancel/Close now close all four. On the baseline they did not close Edit or Premium.
- Escape closes all of them.
- A backdrop click does not close Edit or Premium on either build, so this is not a regression; the tracker's claim that backdrop clicks closed them is not true on the baseline.
- The /settings token Revoke Cancel works on both builds.

### KF-395: copy error
- Baseline: the error goes into `#bc-error` behind the overlay (not topmost).
- Fix: `#bc-copy-error` shows inside the dialog, is topmost, and the page-level error stays hidden.
- Note: `.form-error` has no styling, so the message is plain light text, not red. That was already true of the page-level error.

### KF-082: colour drag reorder
- Baseline: Sortable never binds, nothing is sent, the order never changes.
- Fix: Sortable binds. Dragging Blue to the top sends PATCH sort_order updates, and the new order survives a reload (API 0..9).
- A second drag (Red into position 2) also persists. 0 errors.

### KF-296: Pomodori restore
- Old DB built with the baseline binary, then started on the fix:
  - Exactly "2/3/>3 Pomodoros" became "Pomodori", on both the General and VG boards.
  - These were untouched: "2 Pomodoros!", "Pomodoros galore", "2 pomodoros", ">3  Pomodoros" (double space) and "x 2 Pomodoros".
- After I set a label back to "2 Pomodoros" by hand, the 2nd and 3rd restarts left it alone. The restore runs once.
- The filter panel shows "1 Pomodoro / 2 Pomodori / 3 Pomodori / >3 Pomodori".
- Older DBs load fine (`/tmp/chipflow-audit-test.redb` and `chipflow-hotfix-3280.redb`, run on copies; the originals' md5 is unchanged): login works and board pages return 200.

### KF-111: premium items dimmed
- All four items are at opacity 0.6 with no pills.
- **Nav region diff against the KF-331 reference: 0 of 96000 px.**
- The captures support this; settings/02 records "premium (opacity .6)".
- Observation: on the API & Webhooks or email tab, the *active* item is dimmed too, including its white background and blue bar.

### KF-356: Colors tab
- Content diff against the reference is **12840 of 580800 px; the baseline battery measured 258142**.
- The heading row and table header are **0 px**.
- The info box and upsell colours match exactly (#d9edf7, #bce8f1, text #295f7a). What remains is text anti-aliasing (max delta 19–32) and the reference's mouse cursor.
- Disabled colours are now in alphabetical order, as KanbanFlow shows them.

### KF-368: delete-board page
- **Against the reference (1920x1000, board "Dale fidelity test"):**
  - Title band, buttons and gap: **0 px**.
  - Alert colours exact (#f2dede, #eed3d7, #b94a48); text anti-aliasing max delta 16.
  - Footer is #333, with ChipFlow text where KanbanFlow has its copyright.
  - Baseline: almost every region differs.
- **Flows:**
  - Delete opens the confirm; Cancel keeps the board; confirming deletes it and goes to /boards/new.
  - Archive goes to /boards/archived and the board is listed there.
  - The Cancel link goes to /settings.
- **FAIL:** a valid 100-character name (the limit) overflows.
  - The h1 does not wrap: at 1905 px it spans x 288–1617 over a box at 653–1253.
  - At 700 px it starts at x −314, so the start of the name is cut off where it cannot be scrolled to, and the page scrolls horizontally (1014 wide in a 700 viewport).
  - The baseline wraps. Evidence: `shots/3372-long-settings_delete-700.png` and `shots/3372-delete-longname.png`.

### Break-attempts
- **Dark toggle:**
  - Turning dark on, reloading, turning it off and reloading again gives 0 px against a fresh light load.
  - Without the reload, the only difference is the toast.
- **Dark with a modal open:** the modal is light and the overlay is unchanged.
- **Dark at 700 and 420 px:** no overflow.
- **Inline add-task form and a filtered board in dark:** no text below 3:1.
- **Long name with HTML and quotes:** it is escaped correctly, but it overflows (see KF-368).
- **KF-296 with "2 Pomodoros!":** unchanged.
- **Deleting through the rebuilt page:** works.

### Observations (candidate new defects)
1. The dark layout editor headings are 1.21:1 (KF-373 above).
2. The delete-page title overflows with long names (KF-368 above).
3. `POST /api/boards` accepted a 124-character name although `validate_board_name` caps names at 100. This is not about this tree; it is an existing hole in the create path.
4. The swimlane "⋮" icons are 1.83–2.21:1 in dark.
5. The Menu's Dark mode switch is green when on; KanbanFlow's is #f9c243 track with a #f5a033 knob.
6. Dimmed premium tab items stay dimmed while active (KF-111 note).
7. Copy and edit dialog errors are unstyled.
8. `~/Library/Application Support/chipflow/chipflow.redb` was created at 15:36 and modified at 15:47. That was before any of my runs (my binaries were built at 16:35, and my one bare `--help` run exits before touching a DB), so another agent created it.

DONE: KF-373 FAIL dark board values match KanbanFlow exactly and every light surface stays light (0 px), but the layout editor's "Columns"/"Swimlanes" headings are #1f2937 on #363636 (1.21:1), readable on baseline
DONE: KF-349 PASS report pages stay light under dark pref, pixel-identical to baseline, dead CSS removed; KanbanFlow dark captures account/01 and 05 confirm reports stay light
DONE: KF-357 PASS global top bar on settings/colors/delete with container at y74 (gap, H1 and nav 0 px vs KF-331 ref); dropdown, bell and account work; bar height deviation belongs to open KF-233
DONE: KF-394 PASS ×/Cancel/Close now close all four Colors dialogs (Edit and Premium did not on baseline); Escape works; token Revoke Cancel works
DONE: KF-395 PASS no-pick error renders inside the Copy dialog and is topmost (baseline wrote it behind the overlay)
DONE: KF-082 PASS Sortable binds after deferred load; two drags persist across reload (API sort_order 0..9); baseline drag does nothing
DONE: KF-296 PASS only exact "2/3/>3 Pomodoros" restored to Pomodori; "2 Pomodoros!" and other user labels untouched; marker keeps restarts idempotent; old DBs load
DONE: KF-111 PASS four premium items at opacity 0.6 with no pills; nav region 0/96000 px vs KF-331 reference, supported by settings/02 capture
DONE: KF-356 PASS Colors tab rebuilt: heading row and table header 0 px vs reference, colours exact, content diff 12840 px (was 258142)
DONE: KF-368 FAIL page matches reference (title band and buttons 0 px) and delete/archive/cancel work, but a 100-char board name overflows the 600 px box and goes off-screen at 700 px (baseline wraps)
