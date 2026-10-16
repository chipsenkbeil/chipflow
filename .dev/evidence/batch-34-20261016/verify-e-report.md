## verify-e report: timer batch, fix/timer at 17b590b

Result: **10 PASS and 6 FAIL**. Every defect's main repro is gone on the fix, but six of them broke under break-attempts or against the KanbanFlow (KF) references. One of the six (KF-006) is a regression: the baseline handles that case correctly.

### Environment
- Worktrees:
  - Fix: `.dev/worktrees/verify-e`, detached at 17b590b.
  - Baseline: `.dev/worktrees/verify-e-base` at ffdce86. ffdce86 is still origin/main after a fetch.
- The diff touches 10 files, all on the timer surface: agent-guide.md, src/{db,models,routes}.rs, static/{app.js,style.css}, and templates {board,modal,time_entries,time_log_view}.html.
- Debug binaries (static files are served from each worktree):
  - Fix md5 `543b02b4c96c85cdda5580d46f299b2c`. The string "Manually added time" occurs 2 times in its app.js and 1 time on the baseline. `1 Pomodoro`/`NO_TASK_NAME` occur 2 times in the fix binary and 0 times in the baseline.
  - Baseline md5 `0bd4cd6538434c22e36fffdc1af6d4c1`.
- Instances: fix on port 3345 with DB `/tmp/chipflow-verify-e-3345.redb`, baseline on port 3346 with DB `/tmp/chipflow-verify-e-3346.redb`. Both use TZ=America/Chicago and the golden-master seed. All servers have been killed by PID.
- Clocks:
  - Server: libfaketime with `FAKETIME_TIMESTAMP_FILE` (rewritten atomically), `FAKETIME_NO_CACHE=1` and `FAKETIME_DONT_FAKE_MONOTONIC=1`. Without the last flag, parking_lot panics.
  - Browser: Playwright 1.63 `context.clock` install plus `pauseAt`, advanced with `runFor` in lockstep with the server file.
- Viewport 1905x1000, or 1905x998 for comparisons with the golden. DPR 1, a fresh context per scenario, animations off.
- Fonts are macOS headless. I did not use the Linux container, so text extents can differ by about 1 px.
- Scripts: `$S/r2/pw/ve-*.mjs`. Evidence: `$S/verify-e/shots/` (48 files) and `$S/verify-e/s3-fix.out`. Here `$S` is the scratchpad.

### Gates on the fix
| Gate | Result |
|---|---|
| fmt | rc 0 |
| build | rc 0 |
| clippy --all-targets | 0 warnings (re-run after touching a source file) |
| `TZ=UTC cargo test` | rc 0: 10, 54, 3, 12, 8, 6, 4, 2, 2, 2, 3 passed |
| jsc checkSyntax | rc 0 |

### Per-defect claims (claim: baseline value, then fix value)

**KF-001 (FAIL)**
- Idle popup:
  - Baseline: Task and Duration selects, "Start Pomodoro", "Time log".
  - Fix: "Pomodoro | Time until break | 25:00 | ▶ Start | No task selected | Select task" with the footer row. It matches refs 01 and 03.
- Running panel counts down: fix 24:00 at t+60 s.
- Idle pill "▶ 25:00 ▾": fix OK.
- TODAY list populated: fix OK.
- Card indicators: fix OK.
- **Break-attempt that failed:** select a task, delete it, then reload. The panel still shows "VE temp 2 ×". Start then gives `POST /api/timer/start` 404, a console error and the toast "Could not start timer: task not found". This comes from the fix's new localStorage selected-task.

**KF-004 (PASS)**
- Log rows have `data-entry-id`: baseline no, fix yes.
- 24 s entry:
  - Baseline: the PUT is rejected with "End time must be after start time".
  - Fix: the dialog loads From 16:06:05, To 16:06:29, Duration "24s". Update with no changes saves, with seconds:24.
- Edit to 08:00:00–08:20:00:
  - Baseline keeps `duration_display` "24s".
  - Fix shows "20m", minutes 20, seconds 1200.
- Update button: fix shows "Updating…" and is disabled while the PUT is held.
- Reversed range: preview "—", plus an error.

**KF-006 (FAIL)**
- Interrupted pomodoro counts toward the cycle: baseline yes (count went 1→2); fix no (stayed 5).
- Task-less sessions: baseline doesn't log them; fix logs them as "No task".
- At 00:00: fix waits with Take break, including after a reload.
- Overtime is logged as completed: fix logs 42 m, 10:00–10:42.
- Break order with every-4: fix gives short, short, short, then long at the 4th completed pomodoro, then short. The interrupted pomodoro is excluded.
- Task kept across the break: fix OK.
- **Regression (failed break-attempt):** set the browser clock 6 min ahead of the server.
  - Fix: the pill reads 18:59 while the server's `remaining_seconds` is 1499. A 5-minute short break is stopped by the client 2 s after it starts (server phase goes idle).
  - Baseline: the pill reads 24:59 and the break runs on (04:58).
  - Cause: the fix computes the countdown from `Date.now() − started_at` instead of the server's remaining time.
- Also, pre-existing: `pomodoro_count` comes from the UTC day, so the long-break cycle and the TODAY list reset at 19:00 Chicago. I saw the count drop to 0 at 19:05 local.

**KF-010 (PASS)**
- With the panel open, opening another task's modal: baseline hides the popup; the fix panel stays and its link reads "Select open task".
- Clicking it selects that task. While running, it retargets the session: `started_at` is kept and remaining is 1380.
- The pill is still unclickable under the modal overlay when the modal is opened first. That is the same on both builds.

**KF-015 (PASS)**
- Completed pomodoro sent with reason "completed":
  - Baseline tooltip: "Stopped with reason 'completed'".
  - Fix: reason is null and the tooltip reads "Pomodoro".
- Stopwatch row in the sub-view:
  - Baseline: "Stopped Pomodoro with reason 'Other'".
  - Fix: no such line.
- Stopped pomodoro: fix "Stopped with reason 'Email'".
- The timer log says "Successful Pomodoro" for the same entry, which is consistent.

**KF-067 (FAIL)**
- Count beside Time spent:
  - Baseline: only "28m", no count.
  - Fix: "● 1 Pomodoro 58m" and "5 Pomodori 5h 2m". At zero it shows just "0m".
- **Break-attempt that failed:** I deleted the task's only completed pomodoro entry. Time spent went 58m→33m but the label still reads "1 Pomodoro". The count is a counter, not derived from the entries.

**KF-103 (FAIL)**
- Live part:
  - Baseline: a separate "2h 30m + 0m" span, and nothing while waiting.
  - Fix: inline "2h 30m + 42m / 8h", red rgb(139,0,0). It stays through the waiting state and disappears during the break.
- **Failed:** after a logged 42 m session the badge falls back to "2h 30m / 8h". After 2h 32m logged in total, it still read "2h 30m / 8h" until a reload; the server renders "5h 2m / 8h". KF reverts only on discard.

**KF-278 (PASS)**
- Native `title`: baseline has "You are selecting this task"; fix has none.
- Hover: the fix shows the tooltip and the menu ("Add time entry / Open time log").
- Nodes after 5 modal opens: baseline 3→4 and climbing; fix 1 tooltip and 1 menu.
- Minor: pressing Escape with the pointer on the menu closes the modal but leaves the menu floating until the pointer leaves it.

**KF-279 (FAIL)**
- Heading:
  - Baseline: "TODAY 0M"/"TODAY 25M", green `#4ade80` dots, "— pending" with no time, ×.
  - Fix: "TODAY 42m 1 Pomodoro".
- Dots are `#FAA200`, which equals the golden.
- Pending row reads "10:00 AM – pending".
- ⋮ menus: Edit/Delete on logged entries; the delete confirm offers Cancel/Delete, and Cancel deleted nothing.
- "•••" expands to all 12 rows.
- **Failed against ref 07:** on a day holding only a 30 s stopped entry the fix shows "TODAY 30s". Ref 07, the same state, shows "TODAY" with no total.
- Also: an edited entry keeps its creation position in the list (4:59 PM sits between 12:32 and 11:57).

**KF-289 (PASS)**
- Menu: baseline "Excel (Detailed)/Excel (Summary)/PDF (Summary)"; fix "Download Excel file", "Download CSV file".
- Both downloads produce real files with rows.

**KF-325 (FAIL)**
- Running:
  - Baseline: a 1 px solid border plus a 2 px dashed outline.
  - Fix: the border is replaced with a dashed `#FAA200` one.
- Waiting:
  - Baseline: no outline.
  - Fix: the dashed border is kept, as in KF.
- The selected-idle outline appears, as in ref 03.
- **Failed against the golden:**
  - KF's border is **3 px** thick (y153–155, x514–516) with a 6 on / 3 off dash pattern (period 9) and solid corners.
  - The fix draws **2 px** with about 6 on / 4 off (period 10).
  - Evidence: `shots/pair-dash-KF325.png`.

**KF-366 (PASS)**
- Panel size and colour: baseline 320 px wide, `#1F2937`; fix 260 px, `#3E3E3E`.
- Measured relative to the panel's top-left corner:

| Element | KF golden | Fix |
|---|---|---|
| Take break | 108x42, `#49A849`, at (+136, +61) | identical |
| Task row | `#515151`, y +124..+153 | identical |
| "Change task" underline | y +173 | identical |
| Footer band | `#2E2E2E`, 51 px tall | 51 px tall |
| Footer icons | 208x14 | identical |
| Footer labels | 236x10 | 235x9 (font difference) |

- The absolute position is 19 px left and 2 px down, because it follows the funnel and the header, which sit elsewhere in ChipFlow's layout.

**KF-375 (PASS)**
- Popup open at t+60: baseline title stays "25:00 …" (frozen); fix title reads "24:00 …".
- Discard (10 s) with the popup open: the baseline title keeps "25:00"; the fix title resets.
- After a reload while waiting, the fix title reads "00:00".

**KF-378 (PASS)**
- Header avatar: baseline has no dark pixels; fix has a badge with 12 dark pixels and the class `avatar-timing`.
- Card avatar: fix has the badge too. A card without an avatar gets an "AU" avatar with the badge, which survives a drag.
- The badge is removed during breaks and when idle.

**KF-380 (PASS)**
- /login without a cookie: baseline logs 2 console errors (401s); fix logs 0.
- Manual entry with a future start:
  - Baseline: POST returns 400 and logs a console error.
  - Fix: no POST, and the "You can not enter a time in the future" overlay shows.
- /setup on a fresh DB: 0 errors on both builds.

**KF-390 (PASS)**
- Baseline: "Friday, October 16" and "Manual entry".
- Fix: "Today — 4h 31m — 11 Pomodoros", "Thursday, 15 October — …", and "Manually added time".

### Break-attempts run
- Survived:
  - Reload while waiting.
  - Retarget while running.
  - Drag of the timed card.
  - Stopping mid-break (it shows "Why did you stop?", then goes idle).
  - Two tabs: the stop syncs, and the break ends in both.
  - DST spring-forward: "1:50 AM – 3:15 AM", 25m, 1500 s.
  - Manual entry overlapping a running pomodoro (accepted).
  - Legacy reason "completed" now reads "Pomodoro".
- Failed: covered in the FAIL rows above.
- There is no task-archive endpoint, so I tested delete instead.

### Regression sweep (fix, 0 console errors and 0 failed requests on every run)
- Card menu: Start timer, Move, Color, Assign members, Copy here, Task URL, Delete.
- Task modal opens and closes.
- Drag and drop persists across a reload. It needs the real clock, because SortableJS stalls under a paused fake clock.
- Board header: the filter panel opens.
- /login and /setup: 0 errors.

### Builder choices with no KF evidence
| Choice | References |
|---|---|
| (a) Panel stays open on outside clicks | Silent |
| (b) Today's timer-log header reads "Today" | Supported: ref 12 shows "Today 30s" |
| (c) Break titles "Short break"/"Long break" and label "Time left" | Silent: no break-state capture exists |
| (d) A finished break returns to idle with the task selected | Silent for a break ending. Ref 07 shows the same idle-with-task state after a stop. |
| (e) Task-less entries named "No task" | Silent |
| (f) ⋮ items Edit / Delete / Change task | Silent on the items: refs 05, 07 and 12 show ⋮ but not its contents |

### Candidate new defects (not part of these 16)
1. **Timer entries use the UTC day.** `entries_today`, the log's `day_key` and `pomodoro_count` all bucket by UTC.
   - At 19:40 Chicago, a 7:30 PM entry is grouped under "Saturday, 17 October", listed above "Today".
   - The TODAY list and the long-break cycle reset at 19:00 local.
   - This is already true on the baseline.
2. **Long break default conflicts.** Live ref 08 (10-16) shows the default long break as 15 min. KF-283 set it to 10 based on GM-112, and both builds default to 10.
3. **Tab title when idle with a task selected.** Ref 03 has "25:00 <board>" in the title; the fix shows the plain title.
4. **"Why did you stop?" menu shape.** It is 300 px wide against the 260 px panel; in ref 06 it is panel-width. After a reason is picked the panel stays closed, whereas ref 07 shows it back.
5. **Two tabs double-stop at break end.** One tab gets a 404 on `/api/timer/stop`, which logs a console error. The baseline double-stops even with a single tab.
6. **Future end times accepted.** Editing or adding an entry that ends in the future is accepted on both builds. After a successful update the edit dialog keeps a stale error element.
7. **Leftover focus ring.** After selecting a card in Select-task mode, its `:focus` ring (2 px `#2563eb`) wraps the dashed border.
8. **Selected task not synced across tabs.** Tab 2 never sees tab 1's new selection.
9. **Docs.** `agents/skill.md` doesn't mention the new edit-entry PUT or task-less entries.

DONE: KF-001 FAIL deleting the selected task leaves it in the panel; Start then gives a 404, a console error and the toast "task not found"
DONE: KF-004 PASS 24s entry loads 16:06:05–16:06:29 "24s", saves unchanged, and 08:00–08:20 saves as "20m"; "Updating…" shown; baseline rows have no data-entry-id
DONE: KF-006 FAIL regression: browser clock 6 min ahead makes the client stop a 5-min break after 2 s (pill 18:59 vs server 1499 s); baseline is correct. Pacing and task-less logging pass
DONE: KF-010 PASS "Select open task" shows beside another task's modal and selects or retargets the session; baseline popup hides
DONE: KF-015 PASS completed pomodoro tooltip reads "Pomodoro" (baseline "Stopped with reason 'completed'"); stopwatch rows no longer say "Stopped Pomodoro"
DONE: KF-067 FAIL "1 Pomodoro" stays after the only completed pomodoro entry is deleted (time drops 58m→33m); the count is not derived from entries
DONE: KF-103 FAIL inline live part works, but after a logged 42m session the badge reverts to the stale "2h 30m / 8h" until reload (server 5h 2m); KF reverts only on discard
DONE: KF-278 PASS hover shows tooltip and menu, no native title, 1 node each after 5 opens (baseline 4); minor orphan menu after Escape
DONE: KF-279 FAIL heading, dots, pending time, ⋮ and confirm all match, but a 30s-only day shows "TODAY 30s" where ref 07 shows just "TODAY"
DONE: KF-289 PASS menu reads "Download Excel file", "Download CSV file" and both downloads contain rows
DONE: KF-325 FAIL dashed border replaces the solid one and persists while waiting, but it is 2 px with period 10 vs the golden's 3 px with 6 on / 3 off
DONE: KF-366 PASS 260px #3E3E3E panel; button, task row, link and 51px footer match the golden within 1 px relative to the panel; absolute offset comes from the header layout
DONE: KF-375 PASS title ticks with the popup open (24:00 vs baseline frozen 25:00) and resets after a discard
DONE: KF-378 PASS play badge on the header and timed-card avatars (12 dark px vs 0 on baseline); removed on break and idle
DONE: KF-380 PASS /login 0 console errors (baseline 2); future manual start is caught client-side with no POST (baseline 400)
DONE: KF-390 PASS headers read "Thursday, 15 October" and "Today"; manual rows read "Manually added time" (baseline "Friday, October 16" and "Manual entry")
