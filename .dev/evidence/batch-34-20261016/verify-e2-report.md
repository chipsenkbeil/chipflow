## verify-e2 report: fix/timer at 57f7699

**Result: 5 PASS, 3 FAIL, and the regression sweep passes.**
- The six defects that failed on 17b590b were re-checked. Every original repro is gone on 57f7699, and the KF-006 skew regression is fixed.
- **KF-001** still fails through a second path: the task is deleted from another tab or by an agent.
- **KF-103 and KF-279** fail only against the new 10-17 goldens. Their earlier failures are fixed. You may prefer to file those two gaps as new defects instead of reopening.
- **KF-412 and KF-413** pass.

In this report, `$S` = `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`.

### Environment
- **Worktrees** under `/Users/senkwich/projects/chipflow/.dev/worktrees/`:
  - `verify-e2` at 57f7699 (fix)
  - `verify-e2-prev` at 17b590b (previous attempt)
  - `verify-e2-base` at ffdce86 (still origin/main after a fetch)
  - fix-e was not touched, and no tracked files changed.
- **Diff 17b590b..57f7699** touches only: agent-guide.md, src/{db,models,routes}.rs, static/{app.js,style.css}, tests/settings_delete_board.rs.
- **Release binaries** (all three embed static/):

| Tree | md5 | `elapsed_seconds` count | `watchSelectedCard` count |
|---|---|---|---|
| fix | acf4606b9b90493f44396a3ba1a8b905 | 4 | 2 |
| prev | b87bf36d96c1c9dc7477f57571185c3f | 0 | 0 |
| base | 7dcdbf865ac709f914969a533553b9e0 | 0 | 0 |

- **Servers:**
  - Ports 3345 (fix), 3346 (prev) and 3347 (base), each with a fresh DB at `/tmp/chipflow-verify-e2-<port>.redb`, TZ=America/Chicago, and the golden-master seed.
  - Server clock: libfaketime with `FAKETIME_TIMESTAMP_FILE`, `NO_CACHE` and `DONT_FAKE_MONOTONIC`.
  - Browser clock: Playwright 1.63 `context.clock`, paused and advanced in 1 s lockstep with the server. Skew is set with `skewMs`.
  - All servers were killed by PID.
- **Viewport:** 1905x1000/998 at DPR 1, or 1440x612 for golden comparisons. Animations off, a fresh context per scenario. macOS headless fonts.
- **Scripts:** `$S/r2/pw/e2-*.mjs` and `$S/verify-e2/*.sh`.
- **Evidence:** 41 files in `$S/verify-e2/shots/`.

### Gates on the fix (`$S/e2-gates.log`)
| Gate | Result |
|---|---|
| fmt | rc 0 |
| build | ok |
| clippy --all-targets (after touching a source file) | 0 warnings |
| `TZ=UTC cargo test` | 108 passed, 0 failed |
| jsc checkSyntax | rc 0 |

The release build prints one `unused import ServeDir` warning. It appears identically on prev and base, so it predates this work.

### KF-006: PASS
| Claim | Baseline / prev | Fix |
|---|---|---|
| Browser clock 6 min ahead | base: pill 24:00 at t+60, break runs. prev: pill 19:00, short break killed at +2 s (server went idle) | pill 24:00 = server 1440; break still running at +299, idle at +301 |
| Skew −6 min, +1 h, −1 h | n/a | identical to the 6 min ahead row in all three |
| Take break appears exactly at server 0 | prev: shown at server rem=1 | fresh load at 1499: Stop and "00:01"; at 1500: Take break |
| Full cycle with skew +6 min, and a pomodoro stopped at 10 min in the middle | n/a | breaks short/300, short/300, short/300, long/900; each break ends at server time; the stopped pomodoro is not counted; 0 console errors |

**Break-attempt (survived):** the browser wall clock jumps +6 min, then −1 h, in the middle of a break (`setSystemTime`). The break keeps counting from the server; the base behaves the same.

**Harness note:** in the lockstep loop, a poll answered in the middle of a step can put the client 1 s ahead. This made Take break show at rem=1 twice. A fresh load proves the boundary is exact.

**Observation, pre-existing on base:** `POST /api/timer/stop {"completed":true}` after only 2 min is logged as a completed pomodoro and counts toward the long-break cycle. The server trusts the client's flag. The UI only sends it at server-anchored 0, so this is reachable only through the API.

### KF-001: FAIL
| Claim | prev | Fix |
|---|---|---|
| Delete the selected task, then reload | still shows "E2 temp one ×"; Start fails | "No task selected", localStorage cleared; Start runs a task-less pomodoro, no 404 |
| Delete through the card context menu in the same tab, no reload | selection stays | selection dropped |
| Delete the running task, then reload | — | server goes idle; panel shows "No task selected" |

**Break-attempt that failed:**
- Steps: tab B (or an API agent) deletes the selected task, then tab A clicks Start.
- Result in tab A: `POST /api/timer/start` returns 404, a console error is logged, and the toast reads "Could not start timer: task not found".
- The selection is kept after the failure, so every later Start also 404s until a reload.
- Cause: `watchSelectedCard` only re-checks when the card leaves this tab's DOM.
- Evidence: `node e2-kf001.mjs`, scenario 3.

### KF-103: FAIL (new golden; the original bug is fixed)
| Claim | prev | Fix |
|---|---|---|
| Badge after a logged 25 min session | stale "2h 30m / 8h" until reload | "2h 55m / 8h", matching the server |
| After a 10 min stop with a reason | — | "3h 5m", matching the server |
| After a discard under 20 s | — | reverts to "3h 5m" |
| Retarget in the middle of a session | — | A unchanged; B "0m + 10m", then "10m" |

**Failed against golden 25:**
- Golden 25: a task with no time, 1:35 into a pomodoro (clock 23:25), shows the card badge "⏱ + 2m".
- Fix in the same state: "0m + 1m".
- The fix adds a "0m" base and rounds the minutes down. Golden 25 (1:35 → 2m) and golden 26 (≥25:00 → 25m) both point to rounding to the nearest minute.
- Evidence: `shots/g25-card.png` and `shots/fix-kf325-2-running-1m35.png`. The colour (#8b0000) matches.

**Minor:** when another tab stops the session, tab A's badge stays stale until a reload (10m against the server's 14m). There is no live board sync, so this predates the fix.

### KF-067: PASS
| Claim | prev | Fix |
|---|---|---|
| Delete the only completed entry | "1 Pomodoro 8m" stays | "33m" becomes "8m" and the count disappears |

**Break-attempt:** reassign the completed entry to task B through `PUT /api/time/entries/:id`.
- prev: A keeps "1 Pomodoro"; B shows no count.
- Fix: A loses the count; B reads "1 Pomodoro 25m".

### KF-279: FAIL (new goldens; the earlier failures are fixed)
| Claim | prev | Fix |
|---|---|---|
| Day with only a 30 s stopped entry (ref 07) | "TODAY 30s" | "TODAY" |
| Order after edits | creation order | an entry moved to 8:18 PM sorts first; moved to 7:00 AM sorts last; moved to yesterday leaves the list |
| After a completed pomodoro (goldens 27/29) | — | "TODAY 26m 1 Pomodoro", matching |

**Failed against goldens 25 and 26:**
- I rebuilt golden 25's state: a 30 s stopped pomodoro, a 50 s stopwatch, and a running pomodoro at clock 23:25.
- The goldens show "TODAY 2m" with zero completed pomodori. Golden 27 shows the count animating up from 0.
- The fix shows a bare "TODAY", both while running and while waiting at 00:00.
- Evidence: `shots/fix-kf279-C-panel.png`.

### KF-325: PASS
- Selected card: `3px dashed`, `border-radius: 0`, card colour, `-1px` margin. The outer edge sits 1 px outside the card box, as KF's `::after` inset does.
- Pixel map of the running card (`$S/verify-e2/edge.lua`): 3 rows thick, 6 on / 3 off along each edge, and an L-shaped dash covering each corner, so corners are solid. The dash gaps show the card's background, like KF's `::before`.
- Orange uses #FAA200. The dashes persist in the idle-selected, running, waiting and break states (golden 27).
- **Break-attempt:** after a Select-task-mode click, the card does not have focus and its outline is none (the 57f7699 change).
- **Separate palette issue:** the yellow border is #EECD45 (the existing `.taskBorderColor-yellow`), while KF's yellow taskBorderColor is #F5CC00 (theme vars, golden 22).

### KF-412: PASS (TZ=America/Chicago)
| Check | prev | Fix |
|---|---|---|
| 19:05 after a 6:30–6:55 PM pomodoro | TODAY empty, count 0 | TODAY lists it, count 1 |
| Day key of a 7:10 PM entry | 2026-10-17 | 2026-10-16 |
| Log filter `from=to=16th` | 1 entry | 2 entries |
| Log filter `17th` | 1 entry | none |
| time-spent for the 16th | 25 | 50 |
| time-spent for the 17th | 25 | 0 |
| Just before and after local midnight | — | 23:59:50 still lists 3 entries; 00:10 next day: TODAY empty, count 0 |

**Break-attempt:** the DST fall-back day (2026-11-01). Entries at 00:30 CDT, 03:00 CST and 23:30 CST all bucket to 2026-11-01, and TODAY resets at 00:05 on the 2nd.

### KF-413: PASS
- Fresh DB on the fix: `long_break_minutes` is 15, and the settings modal shows "15 minutes". The fourth break runs 900 s.
- Upgrade cases (`$S/verify-e2/kf413.sh`):

| DB written by base | Reads on the fix |
|---|---|
| Settings never saved | 15 |
| Long break saved as 10 | 10 |
| Only the work time saved | 10 (the stored row carries the old default) |

- On the fix itself: save 20, then a partial PUT of another field, then a restart → still 20. A `/settings` form post without the field keeps 20.

### Regression sweep (fix): all pass, 0 console errors, 0 failed requests
| Defect | Probe result |
|---|---|
| KF-004 | 24 s entry loads 16:00:00–16:00:24 "24s"; Update with no changes saves seconds 24 |
| KF-010 | "Select open task" shows beside another task's modal |
| KF-015 | badge titles: Pomodoro / Stopped with reason 'Email' / Stopwatch |
| KF-278 | hover shows the tooltip and "Add time entry / Open time log"; 1 menu node after 3 opens; no native title |
| KF-289 | menu reads "Download Excel file", "Download CSV file" |
| KF-366 | panel is 260 px wide, rgb(62,62,62) |
| KF-375 | title reads "24:00 …" at t+60 with the popup open |
| KF-378 | `avatar-timing` class on the header and card avatars |
| KF-380 | /login signed out logs 0 errors |
| KF-390 | header "Thursday, 15 October"; row "Manually added time" |

### Decision (a): tab title "25:00 <board>" when idle with a task
- There is no conflict with the references; they support the builder.
  - Ref 03 shows the idle-with-task title "25:00 <board> - KanbanFlow".
  - Ref 07 shows KF going back to that same idle-with-task state after a stop.
- So after a stop KF's title is "25:00 <board>".
- KF-375's "returns to normal after a stop" comes from KF-232's 10-01 audit, which predates these references. Its point is that the title must not freeze on the stopped countdown. The fix meets that: "24:00" while running, "25:00 …" after the stop, and the plain title once no task is selected.
- No reference captures the title when idle with no task.
- Golden 26 shows one gap: while waiting at 00:00, KF's title alternates "00:00 …" / "-------- …". The fix shows a steady "00:00 …".
- Suggest rewording KF-375 to "returns to the idle title".

### Decision (b): TODAY total rule
- The goldens can tell the two rules apart, and both fail somewhere.
- **"Completed pomodori only" (the builder's rule)** is contradicted by goldens 25 and 26: "TODAY 2m" while 0 pomodori are completed.
- **"Show the total from 1 minute" (all entries)** is contradicted twice:
  - Golden 24 (stopwatch panel after the stop, same finished entries) shows a bare "TODAY".
  - Golden 27: the pomodoro ran to 00:00, so it is at least 25:00. Adding the other entries (at least about 1 min) cannot round to the "26m" shown, under any rounding.
- **Best fit:** completed pomodori plus the running pomodoro's elapsed time, rounded to the nearest minute, with the total hidden at 0. This explains refs 05 and 07 and goldens 23, 24, 25, 27 and 29.
- Golden 26 ("2m" at ≥25:00 elapsed) fits only if KF's total was a stale render.
- This is a hypothesis. A live KF probe would settle it: a pomodoro with no other entries that day, open the panel at 3 min, and see whether TODAY reads "3m".

### Stored struct fields and redb tables
- No stored struct field was added, removed or renamed, and no table changed in ffdce86..57f7699. `TimeEntryRow` only gained a method; `Settings` keeps the same fields.
- Changes to the meaning of stored values that a schema-version guard should know about:
  1. **bbd95cf (KF-006):** task-less timer entries store `task_id: ""` and get no `ENTRIES_BY_TASK` row. Older binaries render them as "(deleted task)".
  2. **bbd95cf (KF-006):** timer rows' `started_at` is now the true start. Before, it was the stop instant. Old rows are not backfilled.
     - Verified: a base-written 10:00–10:25 pomodoro shows "10:25 AM–10:50 AM" on both base and fix (`$S/verify-e2/legacy.sh`).
     - A version-gated backfill (`started_at -= seconds` on old timer rows) would need the version marker.
  3. **a2fc674 (KF-004):** `update_entry` now writes `seconds` and sets `minutes = seconds/60`.
  4. **6c7129f (KF-413):** the default changes only when no settings row exists. The field has no `serde(default)`, so stored rows are unaffected.

### Candidate new defects (evidence in `$S/verify-e2/shots/`)
1. **Break panel vs golden 27** (`fix-break-27-break.png`).
   - Golden 27: title "Pomodoro", label "Break time", a green "▶ Start" button (skips to the next pomodoro), task row with ×.
   - Fix: "Short break", "Time left", "Stop", no ×.
2. **Break end vs golden 29.** KF stays on "Break time 00:00" with Start. The fix goes back to "Time until break 25:00" (`fix-break-29-break-finished.png`).
3. **Header chip during a break** should show a green ▶ (golden 28). Not measured.
4. **Waiting-state blink:** golden 26's title and clock blink. The fix shows a steady "00:00".
5. **Timer log vs golden 20.**
   - Golden 20's header is just "Today 30s": the total sits beside the date, with no em-dash and no count.
   - The fix shows "… — 0 Pomodoros", and "3 Pomodoros" on a day where 2 of the 3 were stopped.
   - Stop-reason quotes: the fix uses ‘Email’; the golden uses "Email".
6. **Yellow task border colour:** #EECD45 against KF's #F5CC00.
7. **Server trusts an early `completed:true`.** Pre-existing; see KF-006.
8. **Cross-tab staleness:** card badges do not update across tabs. Pre-existing.

DONE: KF-006 PASS server-anchored ticking: ±6 min and ±1 h skews keep the pill equal to the server; breaks end only at server 0 (prev killed a break at 2 s); a skewed full cycle gives short/short/short/long 900 s; a mid-break wall-clock jump survived
DONE: KF-001 FAIL reload and same-tab deletes now clear the selection, but a delete from another tab or the API leaves it: Start gives a 404, a console error and the toast "task not found", and the selection survives the failure
DONE: KF-103 FAIL the stale badge after a logged session is fixed (2h 55m matches the server), but a task with no time reads "0m + 1m" at 1:35 where golden 25 shows "+ 2m"
DONE: KF-067 PASS the count follows entries: deleting the only completed entry drops "1 Pomodoro" (prev kept it), and reassigning it moves the count to the other task
DONE: KF-279 FAIL a 30 s-only day shows a bare "TODAY" and entries sort by start after edits, but golden 25/26's running state shows "TODAY 2m" where the fix shows a bare "TODAY"
DONE: KF-325 PASS 3 px dashed, 6 on / 3 off, solid L-shaped corners, 1 px outside the box, #FAA200 on orange, kept through running, waiting and break; no focus ring
DONE: KF-412 PASS TODAY, pomodoro_count, log day_key, from/to and time-spent use the Chicago day (prev reset at 19:00); resets at local midnight; DST fall-back day correct
DONE: KF-413 PASS fresh default 15 (API, modal, 900 s long break); saved 10 or 20 kept across a base-to-fix upgrade, restarts, partial PUTs and the form post
DONE: REGRESSION PASS KF-004/010/015/278/289/366/375/378/380/390 each pass one probe with 0 console errors and 0 failed requests
