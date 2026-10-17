verify-w2 report: candidate fix/ui-w2 @ 6078db2 vs baseline 2c1773b. Nine of the ten defects PASS; KF-505 FAILS on the Edit-from-chip anchor only. Regression and gates are clean apart from the observations listed.

ENVIRONMENT
- Worktrees: .dev/worktrees/verify-w2 (detached 6078db2) and verify-w2-base (detached 2c1773b). Diff touches only src/routes.rs, static/app.js, static/style.css and templates/board.html (4 files, +377/-160).
- Release builds with --config 'build.build-dir="target/build"'. Binaries copied to the scratchpad:
  - candidate: md5 ec186157d9500f5cbe6c20c5662217db
  - baseline: md5 dcfe326a8b30c7e24cc03b5c1815095e
  - `rg -a -c addMembersPop`: 17 in the candidate, 0 in the baseline.
- Ports: candidate 3410, baseline 3411. Databases are cand.redb and base.redb under $S/verify-w2/, each seeded with the golden-master board. Both servers are killed by exact PID.
- Viewport 1440x612 DPR 1 (quick-add Members at 1440x435). Playwright 1.63 via node.
- libfaketime notes:
  - `FAKETIME_TIMESTAMP_FILE` must be written without a trailing newline. Offsets take no `s` suffix: `+1495` works, `+1495s` crashes the server.
  - Playwright `clock.install` was set to the same offset as the file.
- $S = /private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad. Evidence is in $S/verify-w2/: shots/*.png, pw/*.mjs (scripts t1 to t20), start.sh, sweep.txt, sweep-boards.txt, md5-*, gate-*.log, build-*.log.

GATES
- `cargo fmt --check`: exit 0.
- `clippy --all-targets`: exit 0, 0 warnings.
- `TZ=UTC cargo test`: all ok (207 passed, 1 ignored, plus the other suites).
- `env -i` plain cargo test: exit 0 (207 passed, 1 ignored, same suites).
- `db::schema_fingerprint_tests::stored_schema_matches_fingerprint` ok. SCHEMA_VERSION = 2 and the server logs "schema version 2".
- jsc `checkSyntax(static/app.js)`: exit 0.

DEFECT TABLES (baseline -> fix)

KF-485
- Start (idle): 112x46 at (1126,99), 17 px label -> 106x44 at (1133,100), 20 px label. Golden timer/14: 1133,100,106x44.
- Stop (running): 112x46 at (1126,99), 17 px -> 104x44 at (1133,100), 20 px. Golden dark/timer/25 stopBtn: 1133,100,104x44. Chip 1106,40,145x34 matches golden.
- Break Start is 106x44 at (1133,100), matching golden timer/27.
- Break attempt: the clock now sits at x 1007 (golden 1007).

KF-491
- General tab `.bshell` height: 416.2 -> 441 (golden adminBox 441).
- The menu is 270,115,240x400, exactly the golden.
- Task settings, Advanced, API and Email tabs also read 441. Layout stays 443.6, Colors 830, Fields 506.5.
- Light and dark are identical, 0 console errors.

KF-496
- `.log-when` regex `^\d+:\d\d [AP]M - \d+:\d\d [AP]M$`: base false (en dash) -> true for all 8 rows. The API `time_range` also uses " - ".
- A new unit test covers it.
- The panel TODAY list keeps its en dash with nbsp padding. That is correct: golden timer/25 uses "7:53 PM – pending".

KF-497
- Candidate positions, dialog x 390 (golden 385) with dialog-left offsets:
  - board select 110x30 at dx 15
  - Period select 180x30 at dx 135
  - Entry type 180x30 at dx 325 (golden 710-385)
  - icons 30x30 at dx 505, 540, 575, 610 (golden 890 is dx 505, 5 px gaps)
- Baseline: Period stepper, Entry type at 733, five glyph icons.
- The two new icons are links with hrefs /timer/log and /timer/statistics. Clicking them loads "Time spent" and "Pomodoro Statistics".
- Break attempts, all fine:
  - Selecting each of the 7 period options refetches with the right from/to.
  - Custom (absolute) opens the range dialog.
  - Custom (relative) shows the relative inputs; the toolbar wraps to 111.8 px tall.
  - Escape closes the custom dialog first, then the export menu, then the log.

KF-501
- Card menu delete with the panel open: baseline pill still running (00:06), title still has the clock, panel still shows the task after 2.5 s. Candidate: pill idle (25:00), title without clock, panel "No task selected" in 34 to 55 ms.
- Modal delete: both builds go idle within about 50 ms because the modal reloads the page.
- Break attempt: deleting a different task leaves the timer running.

KF-504
- Timer log + edit-entry: baseline Escape closed the log (log false, edit false). Candidate: Esc1 closes only the edit dialog (log stays), Esc2 closes the log.
- Timer log + Add time entry: same result as above.
- Due dialog + alert (Esc1 closes only the alert, then due, then modal) and task modal + labels dialog (closes labels only) already pass on baseline and still pass.
- Break attempts:
  - confirm dialog over the modal: Escape closes only the confirm.
  - past-due confirm over the due dialog: closes only the confirm.
  - the manual-time calendar popup stays open after Escape in both builds (not a regression).

KF-505, FAIL on the Edit anchor only
- Width 280 and height 440.3 pass. Baseline: 420 wide, 559.8 tall, centred.
- Title "Edit due date" is centred (offset 0.0 px).
- Add menu path, light: dialog at (1080.5, 61). Golden modal/23 white region: left 1080, top 59, bottom 499, so dy is +2.0, at the limit of ±2.
- Edit from the due chip at 1440x612: dialog at (472.6, 172) against golden dark/modal/23 at (442, 172). dx is +30.6, outside ±2.
  - The y matches because both clamp to the viewport bottom.
  - The builder centres on the chip. The golden left edge sits about 3 px left of the modal content left (445) or aligns near the chip's right edge. I could not derive the golden rule.
  - ChipFlow's chip also sits elsewhere (x 550.5 vs golden about 524 for the same task), so the layout differs. Even with the chip at the golden x, centring gives about 446, still 4 px off.
- Outside click closes the popover (tested at x 300, 700, 1200 and on the Add circle), and the modal stays open.
- Set, clear, repeat and target "(Done)" all work, with the same results as baseline. The chip text reads "Today 5:00 PM (Done) · every week". Ticking Repeat shows no widened panel in either build.
- Internal layout is not at golden px: date input at y 58 and 35 tall (golden 65 and 30), calendar y 110 (golden 96), select y 310 and 37 tall (golden 324 and 30).

KF-506
- API `duration_display`, baseline -> fix, in the log, `/api/timer/log` and `time-log-view`:

| Minutes | Baseline | Fix |
|---|---|---|
| 9000 s (150) | 150m | "2h 30m" |
| 85 | 85m | "1h 25m" |
| 60 | 60m | "1h" |
| 120 | 120m | "2h" |
| 1440 | 1440m | "24h" |
| 59 | 59m | "59m" |

- Card live part at 85 min (libfaketime +85m and Playwright clock in lockstep): baseline "+ 85m", fix "+ 1h 25m", pill 1:25:11. 30 min reads "+ 30m", 59 reads "+ 59m", 60 reads "+ 1h", 120 reads "+ 2h 1m" (rounded to nearest, so correct).

KF-481 partial
- Quick-add at 1440x435: dialog 470,25,500x300. The Members popover is 500,205 at 250x103 (head 32, search 33, row 38), matching the golden.
- Opening it saves nothing: task count unchanged and no modal.
- Escape closes the popover first, then the dialog.
- Picked members are assigned after Save: Ctrl+Enter, Ctrl+Shift+Enter and Shift+Enter all checked via `member_ids`. Ctrl+Shift+Enter keeps the dialog open with the name "" and the popover closed; the next add has no carried-over member. Shift+Enter saves and opens the modal.
- The Members button toggles, the X closes, and picks reset when the dialog is reopened.

KF-488
- A stop under 20 s: toast "Session discarded / Session lasted less than 20 seconds", no reason menu, log count unchanged (15 to 15). The baseline is identical.
- A pomodoro stop at 21.5 s shows the reason menu (Boss interrupted, Colleague interrupted, Context switch, Dog, Email, ...), and picking one logs the entry (15 to 16).
- The docs (README, agent-guide.md, agents/skill.md) say "under 20 seconds are discarded".

REGRESSION
- Header pill is 1106,40,145x34 when running and 1218,40,34x34 when idle. Topbar is 34 px. Both unchanged.
- Quick-add dialog at 1440x612 is 470,25,500x300 (golden modal/12). Name 500,55,440x28, close 935,30,30x30, first icon 500,172,104.5x34 (golden Members 104x34, to the half-pixel) — identical to baseline.
- Pomodoro to 00:00 (libfaketime, behaviour identical to baseline except sizes):
  - the 00:00 clock blinks and the title alternates "00:00" and "--------"
  - it holds at 00:00 for 8+ s with no auto-break
  - Take break is 104x44 at (1133,100)
  - after Take break the label reads "Break time", Start is 106x44, it counts down to 00:00 and holds, with no auto-start
- Timer log header "Today 32h 54m 2 Pomodoros" is present.
- Console errors: 0 in light and dark across board, timer log, settings, due dialog, quick-add and timer flows. The only request failures are DELETE net::ERR_ABORTED when the page reloads after a task delete, identical on baseline.
- No stored-data change: fingerprint test passes, SCHEMA_VERSION 2.
- No GET writes: I swept 94 route patterns, all GET with cookie plus Origin evil.example, Sec-Fetch-Site cross-site and Referer, with the timer running. That gave 47+7 HTTP 200s, 405 for non-GET routes, and the DB md5 was unchanged (7f1ef1f5... before and after; b883b1e0... before and after for /api/boards/:id).

OBSERVATIONS (candidate defects, each with a one-line acceptance check)
1. Idle panel "Select task" link is 1 px higher than golden. It is at y 199, baseline 200, golden timer/14 200. The popup body is 116 px, baseline 117. The task row sits at y 163, baseline 164, golden 165. This is a regression from the KF-485 clock-row change (margin 1px 0). Acceptance: at 1440x612 the idle panel's "Select task" button top is 200 (golden timer/14).
2. Take break label is 20 px; golden timer/26 is 16 px, 2 lines. Candidate: `#tp-take-break` 104x44 with fs 20px, filling the height. Baseline was 17 px. Acceptance: `#tp-take-break` computed font-size is 16px at 1440x612 (golden timer/26).
3. In the quick-add Members popover, pressing Enter in the Search field saves the task and closes the dialog. Acceptance: with the popover open, Enter in the Search input creates no task and closes nothing (KanbanFlow behaviour unverified). A click on the dialog body does not close the popover (golden unknown).
4. KF-505 Edit-from-chip anchor and internal layout (above). Acceptance: with the golden's long-title task and the due chip clicked at 1440x612, the popover's left is 442 (±2) at top 172; internals match dark/modal/23 relative offsets (date input (10,65,125x30), calendar (12,96,256x201), target select (10,324,260x30), Delete (10,400,125x30), Update (145,400,125x30)).
5. ChipFlow's task modal differs from the golden's layout: single-line truncated title versus the golden's 3 wrapped lines, and the due chip at x 550 versus the golden's ~524. This blocks a like-for-like KF-505 anchor comparison. Acceptance: for the modal/27 task, the "Due date" label is at x 524 and the chip at 524,195,~124x28.
6. The 12 h live-card cap is pre-existing: 25 h of elapsed time shows "+ 12h" on the card while the pill shows 25:00:37. Acceptance: card live part for 1500 minutes elapsed reads "+ 1d 1h" or the capped value is documented.
7. The timer log's default period is "This month". Golden timer/20 defaults to "This + Last week". Pre-existing; not part of this change.

DONE: KF-485 PASS Start 106x44 and Stop 104x44 at (1133,100), 20 px; baseline 112x46, 17 px; break Start 106x44
DONE: KF-491 PASS General box 441 (±0), menu 240x400 exactly golden; baseline 416.2
DONE: KF-496 PASS all `.log-when` match the " - " regex (8 of 8); baseline false
DONE: KF-497 PASS Period 180x30 at dx 135, four 30x30 icons at dx 505/540/575/610, Entry type at dx 325 (golden-matching), both icon links work; baseline had the stepper with Entry type at 733
DONE: KF-501 PASS card-menu delete goes idle in 34 to 55 ms with "No task selected"; baseline stuck; modal delete idle in about 40 ms
DONE: KF-504 PASS Escape closes only the edit-entry or manual-time dialog over the log, while the due dialog + alert and modal + labels cases pass; baseline closed the log
DONE: KF-505 FAIL width 280 and height 440 pass, title centred, Add-menu position within 2 px of golden (dy 2.0), outside click closes, set/clear/repeat intact; but Edit-from-chip popover left is 472.6 vs golden 442 (dx +30.6)
DONE: KF-506 PASS "2h 30m" for 9000 s, "+ 1h 25m" on the card for 85 min, API duration_display "1h 25m"; baseline "150m" and "+ 85m"
DONE: KF-481 PASS (partial scope) Members popover 250x103 at (500,205) at 1440x435, no save on open, Escape closes it first, picked members assigned on Save, and the three save shortcuts behave as specified (see observation 3 for the Enter-in-Search case)
DONE: KF-488 PASS under 20 s discards with a toast and no reason menu, at 21.5 s the menu shows and logs, and the docs say so
DONE: REGRESSION PASS with caveats (timer goldens, header and topbar, quick-add geometry, due dates and 0 console errors hold; observations 1 and 2 are minor deltas from golden; no GET writes; no stored-data change)
DONE: GATE PASS (fmt, clippy 0 warnings, TZ=UTC and env -i cargo test, jsc, fingerprint test, SCHEMA_VERSION 2)
