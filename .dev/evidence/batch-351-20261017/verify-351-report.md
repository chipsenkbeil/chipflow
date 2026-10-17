## verify-351: blind verification of ChipFlow batch-35.1, candidate integ/batch-35.1 at 6468308

**Verdict: PASS.** All three streams still hold together on the combined tree, and the release-safety checks pass. I found no new defect in the candidate. There are 4 low-severity observations; 3 are pre-existing and 1 is a nit.

**Environment**
- Worktrees:
  - candidate `.dev/worktrees/verify-351` at 6468308 (31 commits on 7817bce)
  - baseline `.dev/worktrees/verify-351-base` at 7817bce
- The diff touches only the files these defects need: csrf.rs, db.rs, routes.rs, models.rs, main.rs, app.js, style.css, templates, tests and the agent docs.
- Release binaries differ:
  - candidate md5 7b16c088…
  - baseline md5 0ebfa012…
  - "cross-site request refused" appears 3 times in the candidate binary and 0 times in the baseline.
- Ports 3386 and 3387, fresh `/tmp/chipflow-verify-351-*.redb` files, always `--database-path`. Playwright 1.63.0 at DPR 1, 1905x1000 (1440x612 for golden comparisons). libfaketime ran the server with TZ America/Chicago.
- Both servers were stopped by exact PID, both ports are free, and the Linux container was removed.
- Evidence is all under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-351/` (screenshots in `shots/`).

**1. Gates**
- `fmt --check`, `clippy --all-targets` (0 warnings) and `jsc checkSyntax` all exit 0.
- Plain libtest under TZ=UTC, America/Chicago and Pacific/Chatham: 167 passed, 0 failed, 1 ignored in each.
- `env -i`: all 13 test binaries exit 0, 167 passed.
- Linux sandbox (`linux-sandbox.out`): rust:1.98.0, crates fetched first, then network disconnected (no DNS), `--offline`, no zoneinfo or /etc/localtime, no HOME, empty TZDIR, uid 1000. Exit 0, 167 passed.

**2. Release safety**
- **Upgrade from a baseline-written database.** Setup on the baseline:
  - seeded the golden master
  - renamed colour labels to "2 Pomodoros", ">3 Pomodoros" and " 3 Pomodoros", plus a custom "2 Pomodoros!"
  - added a unicode comment
  - logged a 66 s interrupted pomodoro and left a second pomodoro running
  - sent SIGTERM; it shut down cleanly and the file was already stamped 1
- **Under the candidate:**
  - it opens at schema version 1 and the old session cookie still works
  - the running timer keeps the same `started_at`
  - the full API dump diff (`dump-base.txt` vs `dump-fix.txt`) shows only expected changes:
    - the exact labels became Pomodori and "2 Pomodoros!" was left alone
    - KF-429 border/light colour values
    - straight quotes in stop reasons
    - a new additive `seconds` field in the timer log
  - all modal, history and time HTML differences trace to those same changes
- **Break attempts on the upgrade:**
  - I set a label back to "2 Pomodoros" after the upgrade and restarted; it stayed, so the restore runs once.
  - A baseline file left dirty by SIGKILL is repaired and opened with the write before the kill preserved.
- **Refusal of a newer file:** a copy stamped schema 2 is refused with exit 1 and the clear "schema version 2" message. md5 is cb30c110… before and after, and nothing listens on the port.
- **Shutdown:** idle SIGTERM exits 0 at once. With a stalled 5000-byte POST open, the log says "requests still open after 10s", the process exits at 10 s with code 0, and the file is clean.

**3. KF-420 cross-site guard**
- **Read-only GETs.** I enumerated the router myself and found 61 GET routes (`routes.txt`). Each got GET and HEAD with the cookie, `Sec-Fetch-Site: cross-site` and `Origin: https://evil.example`:
  - 122 requests, 0 database writes by md5
  - 14 query-string variants (28 requests), also 0 writes
- **Bug reproduced on the baseline:**
  - a calendar-feed GET on a new board creates a token
  - a status read after deleting the timed task writes
  - the first colours GET on a colourless board writes; I stripped the board with my own redb tool
- **On the candidate:**
  - the feed GET returns `path: null` with no write
  - the timer is cleared at delete time
  - the stripped board is backfilled at startup and the GET after it writes nothing
- **Hostile writes.** All 47 POST/PUT/PATCH/DELETE method-routes were tried with six header sets: cross-site, same-site, foreign Origin only, `Origin: null`, the prefix trick `127.0.0.1:3386.evil.example`, and https-vs-http. All 282 got 403 with no write.
  - The baseline wrote: task PATCH, comment, settings PUT, attachment delete, and a cross-site `/logout` that killed the session.
- **Allowed requests:**
  - same-origin, `Sec-Fetch-Site: none`, own Origin (case-insensitive) and requests without these headers all write
  - a Bearer token sent with cross-site headers works: PATCH, comment, timer
  - a Bearer token still cannot mint tokens (403)
- **Bypass attempt:** a bogus Bearer plus a valid cookie, cross-site, gets 401 and does not fall back to the cookie.
- **Real browser:** a same-site page on 127.0.0.1:3999 auto-submits a form and fires a no-cors fetch.
  - Baseline: the board was renamed to "PWNED form".
  - Candidate: 403 "cross-site request refused", cookie was sent, nothing was created.

**KF-421, KF-422, KF-423**
- **KF-421 move race**, 50 rounds of a move while the 5 neighbours are PATCHed:
  - baseline loses 85 of 250 writes, candidate 0 of 250
  - variant with two concurrent neighbour moves plus a rename: baseline loses 142, candidate 0
  - positions stay dense (0–5)
  - an extra colour reorder-vs-relabel race lost nothing on either build
- **KF-422 column PATCH**: the baseline renamed the column and flipped `is_done` on requests it rejected with 400. The candidate writes nothing on any invalid combination and applies a valid one in full.
- **KF-423 subtask 404**: on the candidate, unknown-subtask PATCH and DELETE and an unknown task all leave the md5 unchanged. The baseline wrote on each.

**4. Full UI sweep, light and dark, on the candidate**
- Sweep A, 31 steps per theme, 0 failures:
  - login, including a wrong password
  - card create, title, description, subtasks, comment, labels, estimate, due date, colour
  - manual time entry, time log and history subviews, all six modal menus, watch
  - the attachment entry is present and disabled (no attachment UI; the API upload, download and delete were exercised in check 3)
  - drag move, context-menu Move right, context-menu delete
- Sweep B, 26 steps per theme:
  - filter, column edit, details, collapse and expand, layout view adding a column and a swimlane, column delete
  - board menu: legend, large names, members, recycle bin, dark toggle, save as template; reports menu; boards dropdown
  - pomodoro start, stop with reason after more than 20 s, stopwatch, timer log, timer settings tabs
- Sweep C, 40 steps per theme, 0 failures:
  - all 8 board settings tabs, rename board, and on Colors: Edit dialog, drag reorder, copy from board, disable/enable, set default
  - delete page Cancel, archive then unarchive, delete board, board from template and template delete
  - account settings, token create and revoke
  - all 11 report pages plus `/timer/log` and `/timer/statistics`
  - the calendar feed toggle POSTs and the feed URL returns 200
- Result: 0 console errors and 0 failed requests apart from my own deliberate probes.
  - Chromium labels the empty-body 200 DELETE fetch as ERR_ABORTED. The baseline does the same; I classified it as harness noise.
- The one B failure in each theme is lane Rename/Delete on a folded strip. This is pre-existing KF-369 and reproduces on the baseline. There are no device-pairing pages in this tree.

**5. Stream G (dark mode and settings)**
- **Dark board vs the golden** (`shots/pair-dark-board.png`):
  - top bar #222222, header row #363636, table #4a4a4a, borders #222222
  - column header #3e3e3e, names, board title and Done labels #eeeeee
  - WIP and overdue #ff8080, add-task plus #36b345, details popover #4a4a4a/#3e3e3e
- **Stays light in dark mode:** cards, modal, card/column/board menus, filter panel, boards dropdown, timer settings and timer log. Settings pages, reports, `/settings` and `/boards/new` do not get `body.dark`.
- **Baseline reproduced KF-373:** dark menus, light-on-light card titles, a dark modal title, and no top bar on settings pages.
- **KF-357**: the top bar is now on all 8 settings pages; the container starts at y 74, as in KanbanFlow.
- **KF-111**: Custom fields, Custom roles, API & Webhooks and Add task from email are at opacity 0.6 with no pills.
- **KF-356** colours tab:
  - the "Enabled colors" heading is on the same row as the three buttons
  - header row #f5f5f5, row rules #dddddd, Edit/Disable are bordered buttons
  - info box #d9edf7 face with #bce8f1 border
- **KF-368** delete page:
  - page #D3E8F2, 600 px box, #E8E8E8 title band
  - alert #F2DEDE
  - Cancel, then Delete #D9534F, then Archive #2185D0
  - footer #333333 at y 970–999
  - a 103-character name wraps to 3 lines inside the band at 1905, 1100 and 640 px with no horizontal overflow
- **KF-394, KF-395, KF-082** (all reproduced on the baseline):
  - the Colors tab Edit Cancel and ×, the premium Close and the token revoke Cancel all close
  - "Pick a board to copy the palette from." now shows inside the dialog
  - drag reorder moved Red to the top and the new order persisted after reload
- **KF-296**: filter options and the Colors tab show "2 Pomodori", "3 Pomodori" and ">3 Pomodori".

**6. Stream C (timer goldens)**
- libfaketime on the server and Playwright `clock` were advanced in lockstep. The baseline reproduced every item below.
- **KF-424, break panel** (`shots/pair-timer-break.png`):
  - title "Pomodoro", label "Break time"
  - "Start" button with a #4cd964 triangle 17x20 (`tp-break-start`)
  - the task row keeps its ×
  - the panel matches golden 27
- **KF-425, finished break:**
  - status stays `short_break` with 0 remaining
  - the panel stays "Break time 00:00" with Start, also 10 s later and after a reload; matches golden 29
  - the baseline went back to idle "Time until break 25:00"
  - Start skips to a new pomodoro; reopening the panel shows "Time until break" with Stop
- **KF-426, header chip during a break:** `timer-pill-playbox`, 16x19, borders 10/9/16 px in #4cd964, 128 exact #4cd964 pixels. It holds after a reload and at 00:00 (`shots/pair-chip.png`).
- **KF-427, waiting blink:** the clock visibility and the tab title alternate between "00:00" and "--------" about once a second.
- **KF-428, log header:** "Today 25m 1 Pomodoro"; a day with only a manual entry shows "2h 30m" with no count. Stop reasons use straight quotes: "Boss interrupted".
- **KF-430, completion rule, via the API:**

| Elapsed | Client claim | Baseline | Candidate |
|---|---|---|---|
| 120 s | completed | completed, count 1 | interrupted |
| 1497 s | completed | — | interrupted |
| 1498 s | not completed | — | completed |
| 1500 s | not completed | — | completed |

- **KF-429, task colours:** every card's background and border match the theme psv: yellow #f5cc00, green #59d600, blue #70b0ff, red #ff858f, orange #faa200, purple #c994ff, magenta #ff85ff, cyan #00d6d6. The baseline had #eecd45, #7cd340, #7faef9 and #ee8b91.
- The same flow in dark mode is identical, with no errors.

**7. Interactions**
- Deleting the task under a running timer, from the context menu or from another tab, clears the server timer at once.
- Deleting the board clears a timer running on one of its tasks.
- Deleting an unrelated task leaves the timer alone.
- After its next status poll the UI shows the idle flow: "▶ 25:00", "No task selected", and Start works.
- The KF-421 race and the colours page after the startup backfill (KanbanFlow palette, Pomodori labels) are covered above.

**Observations (candidate defects to consider; none blocks the release)**
1. **LOW, pre-existing (identical on the baseline).** After deleting the task whose timer is running, the open board keeps counting down that pomodoro for up to 15 s, until the status poll. This affects the pill, the tab title, and the panel, which still shows Stop and the deleted task's name. Evidence: `inter-3386.out`, `inter-3387.out`.
2. **Pre-existing KF-369.** New swimlanes render as folded strips, and the lane menu's Rename and Delete do nothing there, so lanes cannot be removed from the UI.
3. **LOW, pre-existing.** `POST /api/boards` accepts names over 100 characters (a 103-character name was stored), while the settings form enforces 1–100.
4. **Nit, not exploitable.** `normalize_origin` strips a ":80" suffix from any origin, so the malformed `Origin: http://127.0.0.1:3386:80` is accepted. Browsers never send that.
5. **For information.**
   - The top bar is 36 px tall on both builds; KanbanFlow's is 34 px.
   - The release build warns about the unused `ServeDir` import, as the baseline does.
   - The timer log uses "Pomodoros" for 2 or more. The goldens only show "1 Pomodoro"; KanbanFlow's reports use "Pomodoros".

DONE: BATCH-35.1 PASS all three streams hold on the combined tree; gates clean (167/167 in 3 TZs, env -i, offline no-zoneinfo uid-1000 Linux); release safety verified; no new candidate defects (4 low observations, 3 pre-existing)
DONE: GATE PASS fmt, clippy --all-targets 0 warnings, jsc rc 0; 167 passed / 0 failed / 1 ignored under UTC, Chicago, Chatham, env -i (13 binaries) and the Linux sandbox (rust:1.98.0, offline, no zoneinfo, no HOME, uid 1000)
DONE: SAFETY PASS baseline DB opens at schema 1 with no loss (API dump diff only expected changes); session and running timer survive; "2 Pomodoros" became "2 Pomodori" once and the custom label was untouched; newer stamp refused, md5 identical; SIGTERM rc 0, 10 s drain
DONE: KF-420 PASS 61 GET routes x GET/HEAD cross-site plus 14 query variants: 0 writes (baseline wrote feed token, timer clear, colour backfill); 47 write routes x 6 hostile header sets = 282 refused with 403 and no write; same-origin works; Bearer unaffected; real-browser same-site form blocked (baseline renamed the board)
DONE: UI-SWEEP PASS light and dark, 97 steps per theme across login, board, cards, modal, filters, columns, layout, board menu, timer, settings incl. colours, templates, archive, account, reports, calendar feed; 0 console errors and 0 failed requests; only failure is pre-existing KF-369 lane menu on folded strips (same on baseline)
DONE: STREAM-G PASS dark mode themes only the board with KanbanFlow's values; modal, menus, filter, timer, settings and reports stay light; KF-357, 111, 356, 368 (incl. long names), 394, 395, 082, 296 verified with numbers, baseline reproduced
DONE: STREAM-C PASS KF-424 break panel, KF-425 00:00 hold, KF-426 16x19 #4cd964 chip, KF-427 blink, KF-428 count shown and hidden at 0, KF-429 psv colours, KF-430 120 s and 1497 s interrupted, 1498 s and 1500 s completed (faketime lockstep, light and dark, baseline reproduced)
DONE: INTERACTIONS PASS task or board delete clears the timer and the UI resyncs to the idle flow within the 15 s poll (same latency as baseline); KF-421 race 0/250 and 0/200 lost (baseline 85 and 142); colours page after startup backfill shows the KanbanFlow palette and Pomodori labels
