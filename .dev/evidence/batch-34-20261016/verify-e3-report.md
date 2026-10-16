## verify-e3 report: fix/timer-r3 at d79de3d (timer release candidate)

**Result: KF-001, KF-103 and KF-279 PASS. The 18-defect timer sweep PASSES. The batch-32/33 integration checks PASS.**

**About the network drop:** your note said I was cut off before the residual-gap repros on the previous tree. I wasn't: they had already finished, and their outputs are intact (`kf001-prev.out`, `live-s1-prev.out`, `live-s2-prev.out`). After your message I checked my ports. One stale fix server of mine (PID 49775, port 3345) was still running; I killed it by PID. Nothing else was listening on 3345 or 3346. The empty-environment test run was the only step left, and it is now done.

In this report, `$S` = `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`. Outputs are in `$S/verify-e3/`, scripts in `$S/r2/pw/e3-*.mjs`, screenshots in `$S/verify-e3/shots/`.

### Environment
- **Worktrees** under `/Users/senkwich/projects/chipflow/.dev/worktrees/`, all detached and clean:
  - `verify-e3` at d79de3d (the fix)
  - `verify-e3-base` at 1194d71 (the baseline)
  - `verify-e3-prev` at 9f1352e: the tree just before the 3 new commits, used to reproduce the residual gaps
- timer-r3 was not touched. No tracked files were edited.
- The 3 new commits change only `static/app.js` (+72/−28). The full 1194d71..d79de3d diff covers agent-guide.md, src/{db,models,routes}.rs, static/{app.js,style.css}, 4 templates and tests/settings_delete_board.rs.
- **Release binaries:**

| Tree | md5 | `drawTodayHeading` count | `dropSelection` count | `elapsed_seconds` count |
|---|---|---|---|---|
| fix | 0d43a80c45548a52fc600c01bb6dc4a1 | 3 | 3 | 4 |
| prev | 6fb2faa00880f4d052a8c5e986d9adcc | 0 | 0 | 4 |
| base | a304578c49886a045e3e48a36069e08d | 0 | 0 | 0 |

- **Servers:**
  - Fix on port 3345. Baseline, then prev, on port 3346, one at a time.
  - Fresh DBs at `/tmp/chipflow-verify-e3-<port>.redb`, always started with `--database-path`. TZ=America/Chicago, golden-master seed.
  - Server clock: libfaketime with `FAKETIME_TIMESTAMP_FILE`, `NO_CACHE` and `FAKETIME_DONT_FAKE_MONOTONIC=1`.
  - Browser clock: Playwright 1.63 `context.clock`, paused and advanced in 1 s lockstep with the server.
  - Every server was killed by PID. Both ports are free.
- **Viewport:** 1905x998/1000 at DPR 1. 1440x612 for the golden-25 geometry. 1905x600 for the sticky-header check. Animations off, a fresh context per scenario.

### Gates on the fix (`gates.log`)
| Gate | Result |
|---|---|
| fmt | rc 0 |
| build | ok |
| clippy --all-targets (after touching src/main.rs) | 0 warnings |
| `TZ=UTC cargo test` | 128 passed, 0 failed, 1 ignored |
| jsc checkSyntax | rc 0 |

### Empty-environment runs and the two due-date tests
The two tests are `format_due_card_weekday_near_absolute_far` and `format_due_modal_relative_and_done_marker`.

| Run | fix | base |
|---|---|---|
| `TZ=America/Chicago`, the two tests | both FAIL | both FAIL |
| `TZ=Europe/London`, the two tests | both FAIL | both FAIL |
| `TZ=UTC`, the two tests | pass | pass |
| macOS `env -i` (no HOME, no TZ, empty TZDIR), full suite | 71/73 in the binary; only these two fail | 66/68; only these two fail |
| Linux container (`rust:1.98-bookworm`), full suite | 128 passed, 0 failed | 122 passed, 0 failed |

- **The failures are pre-existing.** They fail identically on the baseline, so they are not caused by this tree.
- **macOS can't reproduce the sandbox.** With TZ unset, macOS reads `/etc/localtime`, which is Europe/London on this machine. That run is a non-UTC run, not a sandbox one.
- **The Linux container does reproduce it** (`emptyenv-linux.log`):
  - `env -i`, no HOME, no TZ, `/etc/localtime` and `/usr/share/zoneinfo` removed, an empty TZDIR, `--offline`.
  - `date` in that environment reports UTC.

### Stored data
- No redb table definitions changed: all 19 `Definition::new` names are identical. No stored struct or serde field changed between 1194d71 and d79de3d.
- `models.rs` only gains `impl TimeEntryRow::duration_seconds` and the `Settings::default` long break of 15. `db.rs` gains functions and parameters only.
- These changes in what stored values mean still stand (the earlier verifier listed them):
  - task-less entries store `task_id: ""`
  - timer `started_at` is now the true start time
  - `update_entry` writes `seconds`

### KF-001: PASS
Repro on prev, gone on the fix (`kf001-{prev,fix}.out`):

| Scenario | prev (9f1352e) | fix |
|---|---|---|
| A: an API agent (curl) deletes the selected task; panel open | still shows "E3 temp A ×" after the poll. Start returns 404, a console error, and the toast "task not found" | at +5 s still selected (no poll yet). At +16 s: "No task selected", localStorage cleared, title back to the plain board name. Start runs a task-less pomodoro, with no 404 and 0 errors |
| B: API delete, Start 1 s later, before any poll | 404 and toast; selection survives; a second Start also 404s | Start makes `GET /api/tasks`, finds the task gone, drops it, and posts nothing. No 404, no toast, 0 errors. The second Start runs a task-less pomodoro |
| C: two tabs; tab B deletes via the card menu; tab A clicks Start | 404 and toast; selection kept | dropped before Start; no POST; 0 errors |
| G: the running task is deleted via the API | the panel keeps the task | server goes idle; tab A shows "No task selected" after the poll; 0 errors |

**Break-attempts:**
- **D, a race:** a route hook deletes the task between the pre-Start check and `POST /api/timer/start`.
  - Fix: the 404 path drops the selection. There is no toast and the selection doesn't survive.
  - The browser still logs one built-in "Failed to load resource: 404" console line. This is only reachable in that race. prev showed the toast and kept the selection.
- **E, no false drop:** the selected task sits in the Done column. It stays selected across 2 polls.
- **F, no false drop:** `GET /api/tasks` returns 500 during a poll. The selection is kept (the console error comes from my injected 500).
- **Collapsed column:** the selected, running task's column is collapsed. The selection survives polls, and the panel still names the task.
- **Same-tab UI delete:** dropped within 1 s, both when the task was selected in that tab and when the selection came from storage. A first run looked like a miss, but it was my harness: no clock tick ran between selecting and deleting.

### KF-103: PASS
Live card part, task with no logged time (`live-s1-fix.out`; prev in `live-s1-prev.out`):

| Elapsed | prev | fix |
|---|---|---|
| 0:29 | no badge | no badge |
| 0:30 | no badge | "+ 1m" |
| 1:29 | "0m + 1m" (at 1:30) | "+ 1m" |
| 1:30 | "0m + 1m" | "+ 2m" |
| 1:35 | "0m + 1m" | "+ 2m" (golden 25) |
| 2:30 | — | "+ 3m" |
| 25:00, waiting | "0m + 25m" | "+ 25m" (golden 26) |
| 25:30 | — | "+ 26m" |

- The live part is #8b0000 at 11.36 px.
- On the baseline (1194d71), the same task at 1:35 reads "0m + 1m".
- **Golden 25 rebuild** (`live-s2`): the task holds a 30 s stopped pomodoro and a 50 s stopwatch.
  - Idle: no badge (golden 24).
  - 1:35: "+ 2m" (golden 25). 25:00: "+ 25m" (golden 26).
  - At 1440x612 the live part sits 26 px right of the card's left edge and 40 px down, after the clock icon.
- **Break-attempts:**
  - Reload during the waiting state: "+ 26m", the dashed border and the selection all come back.
  - Take break: the card reads "26m", equal to the server card.
  - Skew ±6 min: badge and server match at 0:29, 0:30, 1:35 and 25:00.
  - Task with existing time: "15h 37m + 2m / 8h" at 1:35, then "16h 2m / 8h" after the break.
  - Stopwatch: "+ 1m" at 0:30, "+ 7m" at 6:40.
  - Midnight: "25m + 10m" at 23:59:30, "50m" after the break.

### KF-279: PASS (on the rule you gave: completed pomodori plus live elapsed, nearest minute, hidden at 0)

| State | prev | fix |
|---|---|---|
| 0:29, nothing else today | — | "TODAY" |
| 0:30 | — | "TODAY 1m" |
| 1:29 | — | "1m" |
| 1:30 | — | "2m" |
| 1:35 | "TODAY" | "2m" |
| 2:30 | — | "3m" |
| 25:00, waiting | "TODAY" | "25m" |
| 25:30 | — | "26m" |
| Reload in the waiting state | — | "26m" |
| After Take break | — | "26m 1 Pomodoro", unchanged during the break |
| Golden 25 rebuild (30 s stopped + 50 s stopwatch + 1:35 live) | "TODAY" | "TODAY 2m" |

- Under the alternative "all entries" rule, the golden 25 rebuild would read 3m. The fix reads 2m, as the golden does.
- The baseline shows "TODAY 0M".
- **Geometry at 1440x612:** relative to the panel, "TODAY" sits at x 18 and the "2m" total at x 86, both matching golden 25. The y position is 3 px lower than the golden, inside KF-366's panel layout.
- **Break-attempts:**
  - Skew ±6 min: identical readings.
  - A running stopwatch is excluded: a bare "TODAY" at 6:40.
  - Task with existing time: the total counts only today's pomodoro.
  - **Midnight crossing:**
    - At 23:59:30 the heading reads "35m 1 Pomodoro". At 00:00:00 it resets to "10m", with only the pending row listed.
    - After Take break at 00:15, the entry is filed under the previous day (its start), so TODAY is empty and hidden.

### Timer sweep, one probe each on the fix (0 console errors and 0 failed requests unless noted)
- **KF-001, KF-103, KF-279:** above.
- **KF-004:** a 24 s entry loads 16:00:00–16:00:24, "24s"; Update with no changes keeps `seconds` 24.
- **KF-006:** skew +6 and −6 min. The pill matches the server (24:00 at t+60). Take break appears exactly at server 0. The break runs 300 s and ends at server time (idle at +301).
- **KF-010:** "Select open task" shows beside another task's modal.
- **KF-015:** badge titles read Stopwatch / Pomodoro / "Stopped with reason 'Email'".
- **KF-067:** "1 Pomodoro 33m". Reassigning the entry gives A "8m" and B "1 Pomodoro 25m"; deleting it gives B "0m".
- **KF-278:** hovering shows the tooltip "You are selecting this task" (KanbanFlow's text per the tracker) and the menu. One menu node after 3 opens, no native `title`.
- **KF-289:** the menu reads "Download Excel file" / "Download CSV file".
- **KF-325:** 3px dashed, radius 0, 1 px outside the card box, in the idle-selected, running, waiting and break states.
- **KF-366:** panel 260 px wide, rgb(62,62,62).
- **KF-375:** the title reads "24:00 …" at t+60 with the popup open, and "25:00 …" after a stop.
- **KF-378:** `avatar-timing` on the header and card avatars.
- **KF-380:** a signed-out /login logs 0 errors.
- **KF-390:** the manual row reads "Manually added time"; the header reads "Thursday, 15 October".
- **KF-412:** an entry at 7:10 PM Chicago is filed under the 16th. Totals for the 16th are 50 and for the 17th 0. TODAY empties at 00:10.
- **KF-413:**
  - A fresh DB defaults to 15, and the settings modal shows "15 minutes".
  - Upgrading a baseline DB: unsaved settings read 15; a saved 10 stays 10; a row with only the work time saved reads 10.
  - A saved 20 survives a restart, a partial PUT and the form post.

### Integration with batch-32/33, timer running (`integ-fix.out`, `shots/integ-3345-*`, `shots/sticky-3345*`)
- **Card menu:** reads Start timer / Move ▸ / Color ▸ / Assign members / Copy here / Task URL / Delete. The Move submenu reads Move right / Move to bottom / Move advanced.
- **Move right on the timed card:** the page reloads into In Review with the timer still running (elapsed 187 s). Dashed border, "+ 3m" and the avatar badge are all kept.
- **Done drag** (column located by `data-is-done`): the card lands in Done. Border and live part are kept through the drag and a reload.
- **Due dialog on the timed card:** saving shows "Due: Sunday 3:00 PM" on the card (the server stores 2026-10-25T20:00Z). The border and live part survive the card refresh.
- **KF-400:** a PATCH with an unparseable `due_at` returns 400 and the stored value is unchanged. This is the only console line in the run, and it is the expected 400.
- **Collapsed strip:** collapsing the timed card's column hides the card. The selection and panel task row survive a poll. After expanding, the border is back and the badge reads "+ 4m".
- **Board frame and sticky header:** the timed card was scrolled 40 px under the sticky header in `.board-wrap`. `elementFromPoint` returns the header above its bottom edge and the card below it. The screenshot shows the header painting over the dashed border.
- **KF-358 frame:** with the −1 px margin of a selected card, the next card's top doesn't move (305 px both idle and selected).

### Observations and candidate defects (none blocks the verdicts)
1. **Golden 26 conflicts with the rule.** Golden 26 shows "TODAY 2m" while the pomodoro waits at 00:00. Under your rule the fix shows "25m", and keeps updating while the panel is open. The golden 25 and 26 panel appears to have stayed open throughout, so KanbanFlow may compute the total only when the panel renders. A live probe on KanbanFlow would settle it: keep the panel open past 2 min.
2. **First Start after a remote delete does nothing visible.** It quietly clears the task and starts nothing, with no toast; the second click starts a task-less pomodoro. KanbanFlow's behaviour here is unknown. LOW.
3. **Every idle poll fetches the whole task list.** With a task selected, each 15 s poll requests the full unpaginated `/api/tasks`. Minor load.
4. **Midnight:** a pomodoro that crosses midnight counts toward the new day's heading while it runs, then moves to the previous day once logged (item in KF-279 above). LOW. KanbanFlow's behaviour is unknown.
5. **"+ 0m" in the first 29 s.** A task with existing time shows "+ 0m" for the first 29 s, while a task with no time shows nothing. KanbanFlow's behaviour is unknown.
6. **Yellow border colour:** #EECD45 against KanbanFlow's #F5CC00. Already noted by the previous verifier.
7. **A session left running across days logs everything when stopped.** A pomodoro started on the 22nd and stopped on the 24th logged 48h on the card. Pre-existing server behaviour.
8. **The two due-date tests fail under any non-UTC TZ on both trees.** Pre-existing. A follow-up could pin their zone.

DONE: KF-001 PASS a task deleted from another tab, the card menu or the API is dropped on the next poll or before Start; no 404 or toast and the selection doesn't survive (prev: 404, toast, kept); Done-column task and a 500 from /api/tasks keep the selection
DONE: KF-103 PASS nearest-minute live part with no "0m" base: none at 0:29, "+ 1m" at 0:30, "+ 2m" at 1:30/1:35 (golden 25), "+ 25m" waiting (golden 26); prev and base read "0m + 1m"; survives reload, ±6 min skew and midnight
DONE: KF-279 PASS TODAY = completed pomodori + live pomodoro, nearest minute, hidden at 0: bare at 0:29, 1m at 0:30, 2m at 1:30, "TODAY 2m" in the golden 25 rebuild, 25m/26m waiting, "26m 1 Pomodoro" after break; stopwatch excluded; ±6 min skew identical (golden 26 shows 2m: see observation 1)
DONE: TIMER-SWEEP PASS all 18 timer defects pass one probe each on d79de3d, including the KF-006 ±6 min skew cycle, KF-412 day buckets and the KF-413 upgrade cases, with 0 console errors and 0 failed requests
DONE: INTEGRATION PASS Move submenu, Move right, Done drag, due dialog + KF-400, collapsed strip and sticky header all keep the timer state (dashed border, live part, avatar badge, selection); gates green; sandbox-like Linux run 128/128; no stored struct or table changes; the two TZ-sensitive tests fail identically on the baseline
