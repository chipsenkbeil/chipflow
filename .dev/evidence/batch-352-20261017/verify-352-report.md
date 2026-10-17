## verify-352: blind release-candidate verification of batch-35.2 (resumed after the network drop)

I resumed from the evidence already on disk: both release binaries had built, and no checks had run yet. I then ran every check from the start. Evidence is in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-352/`; screenshots are in its `shots/` folder.

**Environment**
- **Worktrees:** the candidate is `.dev/worktrees/verify-352` at ee0b86f (git status clean at the end). The baseline is `.dev/worktrees/verify-352-base` at 657ec5e.
- **Release binaries:** candidate md5 `84f726dd…`, baseline md5 `d6e70b15…`. The string `board-scroll` appears 10 times in the candidate and 0 times in the baseline; `new_tasks_at_top` appears 5 and 0 times.
- **Ports and databases:** candidate on 3386, baseline on 3387. Every database lives in the verify-352 folder: `db-3386.redb`, `db-3387.redb`, `safety.redb`, `rollback.redb`.
- **Browser:** Playwright 1.63.0, device pixel ratio 1, animations off. `SHOWBARS=1` re-enables real scrollbars, because headless Chromium hides them and they had to be visible to measure the 10 px bar.
- **Cleanup:** both servers were killed by PID and nothing is listening. The colima container `v352-linux` was removed.
- **Fixture changes on my test instances only:** I enabled inline subtasks on three columns, set grouping dates on the Done tasks, and seeded boards 2 to 4 (collapsed column, swimlanes, collapsed WIP column).

### 1. Gates: PASS
- **fmt:** exit 0.
- **clippy --all-targets:** exit 0 with 0 warnings. The release build shows one `unused import ServeDir` warning, which the baseline has too.
- **jsc checkSyntax:** exit 0.
- **Tests on macOS:** `/Users/senkwich/.cargo/bin/cargo test` passed under TZ=UTC, America/Chicago and Pacific/Chatham: 174 passed, 0 failed, 1 ignored in each. Each of the 13 test binaries also passed run directly under `env -i`.
- **New tests:** the new tests for KF-457, KF-365, KF-403, KF-364, KF-245 and KF-362 all ran and passed.
- **Linux sandbox:** rust:1.98.0, crates fetched first, then the network disconnected (DNS fails). Run as uid 1000 with no HOME, no zoneinfo, an empty TZDIR and `--offline`. Exit 0, all suites ok (`linux-sandbox.out`).

### 2. Release safety: PASS
- **Upgrade from production:** the baseline seeded the golden master, started a pomodoro, then took SIGTERM ("shut down cleanly"). The candidate opened that database at schema version 1 with no restamp.
  - The full API dump of 3262 lines is identical between the two (`dump.diff` is empty).
  - The old cookie still works (board 200), and the timer is still running with the same `started_at`.
- **Rollback:** after the candidate wrote `new_tasks_at_top` to that database, the baseline still opens it at schema v1: board 200, timer still running.
- **Cross-site GET/HEAD sweep:** I enumerated the router myself and found 61 GET routes. Each got GET and HEAD with the cookie, `Sec-Fetch-Site: cross-site` and a foreign Origin. That is 122 requests with 0 database writes by md5, plus 28 query-string variants with 0 writes. The session survived.
- **Hostile writes:** every POST/PUT/PATCH/DELETE route got six hostile header sets, 282 requests in all. All 282 returned 403, with 0 writes. Same-origin writes (attachment upload, feed token) still succeed.

### 3. Stream F on the combined tree, light and dark: PASS
KanbanFlow numbers come from `board/02` and `columns/08`, at 1440x612 with ChipFlow 2 px lower throughout.

| Defect | KanbanFlow | Candidate | Baseline |
|---|---|---|---|
| KF-361 header row | 46 px row, title ink x 18–225, invite + x 285–295, rule y 80 | 46 px row (y 36–82), title x 18–225, + x 285–295, rule #C3C3C3 at y 82 | — |
| KF-311 column header | name 11 px in, 37 px band, + at the right | names x 22/306/590/874/1158 (exact), 37 px, + ink centre within 1 px, 0 count cells | — |
| KF-363 cards | 267x48 at 19,126, 8 px gap, one-line 30 px, title ink x 312–545, time #606060 10 px | 19,128 267x48, 8 px gap, 30 px, title x 312–545, time #606060 10 px | — |
| KF-316 count cell | none | none | — |
| KF-362 overdue strip | text at 298 with a text-wide bar (304–387 for "2", 587–663 for "1"), warning line at y 116 | bar x 304–379 for "1" (76 px), #8b0000 10 px bold, warning line at y 118; dark #444 / #ff8080 / #2b2b2b | — |
| KF-241 folded strip | name box 710,128 20x118, line 710,116 21x3, count #8b0000 | 709.5,130 20x122, line 710,118 21x3, #8b0000 | name at y 160 (bug reproduced) |
| KF-245 borders | — | all 8 card borders and faces match the KF values | — |
| KF-354 legend | 0,589 1440x23, items 180 px, text at 72 | 0,589 1440x23, 180 px, text at 72.9 | — |
| KF-333 colour submenu | 140x34 tinted rows, current bold | tinted, current bold, 140x34 rows, 140x272 overall | — |
| KF-302 quick-add dialog | 470,25 500x300 #ffffe0, name field 500,55 440x28, colour select 544,113 396x34, buttons at y 266 | identical numbers; 8 colour rows at a 36 px pitch, default bold | — |
| KF-364 inline subtasks | 3 subtasks all shown; "2 hidden (2 done)"; 16 px boxes at a 25 px pitch | same; label becomes "Hide" when expanded and toggles back | "1 hidden (1 done)" on 3-subtask cards (bug reproduced) |
| KF-365 Done groups | — | Today first (also when today's card is placed last), then Tue 20 Oct (future), Yesterday, Mon 12 Oct; header #888 bold 12 px, rule #d8d8d8, 1156,128 264x21 (KF 1156,126) | future group first, Today second |
| KF-367 board menu | 1190,34 250x578 #f7f7f7 | 1190,36 250x576 #f7f7f7, shadow match, 35 px head, Get Premium 1200,499 #49a849; switches on #f9c243/#f5a033, off #9e9e9e/#fafafa | — |
| KF-376 icons | 16 px | #808080, 16 px at a 24 px pitch, ink x 28–87 (KF 28–88) | — |
| KF-377 avatars | 537,178 | "AU" (header initials) at 537,180 | "A" (bug reproduced) |
| KF-386 drag placeholder | #fafafa, 1px #fafafa border, child hidden | #fafafa light, #444444 dark (KanbanFlow's theme value is #444) | — |
| KF-385 column overflow | — | the overflowing column gets its own 7 px bar, its cards lose 7 px (440 vs 447), the board region does not scroll | — |
| KF-403 new tasks | — | bottom by default (live and after reload); top when set; the settings page reads "Top of column" | — |
| KF-405 column delete | — | column menu shows only Edit/Collapse/Show details | — |

KF-405 detail:
- In the layout view, deleting non-empty To Do refuses until a target is chosen.
- With Backlog chosen as the target, the tasks are appended in order, and nothing is lost.
- On the swimlane board, deleting Backlog into Done keeps each task in its own lane (22 tasks before and after).

Dark mode themes only the board:
- Column header #3e3e3e, names #eeeeee, table #4a4a4a, borders #222222, gutter #2b2b2b, WIP count #ff8080, Done label #eeeeee, header row #363636.
- Cards, modal, menus, card menu and filter panel stay light.

There were 0 console errors at 1905x1000, 1690x1000, 1440x612, 1240x850 and 1120x900, in both themes.

### 4. KF-457 responsive: PASS
- **Column width:** (W−20)/N, compared with the KanbanFlow dumps. Widths were 380 at 1920, 284 at 1440, 252 at 1280 and 220 at 1120. Below that they stay 220 at 1024, 900 and 640, matching KanbanFlow exactly. Card widths and first-card position also match (+2 px vertical).
- **Breakpoint on the plain board:** 1120 has no scroll; 1119 overflows by 1 px. Overflow is 96 px at 1024, 220 at 900, and 420 at 640, where the page stays 700 px wide.
- **Collapsed columns:** with one collapsed column the breakpoint is 928/927 (28 px strip).
- **Swimlanes:** with a lane column the breakpoint is 1270/1269, from the 150 px lane column.
- **Folded swimlane strip:** a board with a folded strip still scrolls correctly, with no clipping.
- **Scrollbar:** the bar is 10 px. Light track #eee / thumb #bbb; dark #555 / #777, as KanbanFlow's theme values specify.
- **Scrolling:**
  - A wheel scroll moves only the board region.
  - Keyboard and wheel never scroll the page.
  - The top bar, board header and Menu keep viewport width.
  - The legend is full width, 23 px tall, with 8 items of W/8, and stays at x=0 after scrolling.
- **Baseline reproduction:** the baseline keeps 254 px columns, Done sits off screen at 1240, and the legend moves to x −52 / −268 when the board scrolls.

### 5. KF-484 debug-profile change: PASS
- **Release identity:** a clean release build without the Cargo.toml change and a clean build with it (separate build dirs) both give md5 `84f726dd…`. That equals the candidate binary (`kf484-md5.txt`). An incremental build does not even recompile.
- **Debug cost:** only argon2 and blake2 recompile. The login test costs 0.24 user CPU-s with the change versus 1.35–2.3 without. The full unit-test binary costs 8.4 versus 36.0 user CPU-s.

### 6. Interactions: PASS
- **Timer:** the chip, idle panel and running panel work.
  - The panel sits at x 992.5 (KanbanFlow 992, baseline 988, so the integrator's +4.5 px holds).
  - The Stop button centre is (1182.5, 124) versus KanbanFlow (1185, 122), so about −2.5 px horizontally once the 2 px offset is removed. The baseline was −7.
- **Drag and drop:** works into Done while the board is scrolled at 1024. A card dropped beside the "Yesterday" group is re-grouped under Today (completed today), the same live and after reload. It also works across swimlanes while scrolled.
- **Other flows:** quick add, column delete from the layout view, and the swimlane board all work.
- **Errors:** 0 console errors, 0 failed requests, 0 HTTP errors across every script.

### 7. Integration commit ee0b86f: PASS
- `columnHeader-separator`, `color-legend-band` and `color-legend-rule` have no references left in templates, app.js or src.
- `columnHeader-overdue` survives only in a Rust test that asserts it is absent.
- The dark drag placeholder computes to #444444, matching KanbanFlow's dark value of #444.

### 8. UI sweep, light and dark: PASS
- Covered: board, modal, filter, reports menu, board menu, card menu, timer, layout view, and all 23 settings, report and timer pages (every page 200).
- The "Position of new tasks" dialog was included.
- 0 console, page or HTTP errors. Dark mode stays off on settings pages, as in KanbanFlow.

### Observations (possible new defects; none blocks the release)
1. **Timer buttons:** Start/Stop measure 112x46 versus KanbanFlow's 106/104x44, and the Stop centre is −2.5 px. The baseline has the same size.
2. **Card avatar ring colour:** it is a 75% mix (green computes to about #79e030), but the new `board/02` styles.json gives KanbanFlow's computed ring as #59d600. The KF-243 CSS comment contradicts the new golden. This predates the batch.
3. **Overdue strip box:** it starts at y 120, on the last row of the 3 px warning line, while KanbanFlow's starts just below it. The text and bar positions still match.
4. **Collapsed-strip name box:** 122 px tall versus KanbanFlow's 118.
5. **640 px window:** a script call to `window.scrollTo` can still shift the 700 px document by 60 px. The wheel and keyboard cannot. KanbanFlow's mechanism allows the same.
6. **Stop reason menu:** stopping a pomodoro a few seconds in showed no reason menu on either the candidate or the baseline. This predates the batch; I did not look into it.
7. **API quirks:** `GET /api/tasks?board_id=` returned a task from another board, and `POST /api/tasks/:id/move` accepted a move into a different board. Both predate the batch.
8. **Overdue due line on cards:** it is bright red with no icon, versus KanbanFlow's #8b0000 with an icon. Outside this batch.

DONE: BATCH-35.2 PASS: all gates green (macOS and the offline Linux run as uid 1000), the production database upgrades with no loss, Stream F, KF-457 and KF-484 hold together on 657ec5e, and only small observations remain.
DONE: GATE PASS: fmt, clippy (0 warnings) and jsc clean; 174 tests pass under 3 TZs and `env -i`; the Linux no-network, no-zoneinfo, no-HOME run passes as uid 1000.
DONE: SAFETY PASS: the baseline database opens at schema v1 with an identical 3262-line API dump, cookie and timer survive, rollback works, 122+28 cross-site GET/HEAD requests write nothing, and 282 hostile writes all get 403.
DONE: STREAM-F PASS: all 19 defects measured on the combined tree against the goldens within 0–2 px (uniform +2 px vertical), light and dark; three reproduced on the baseline (KF-241, KF-364, KF-377).
DONE: KF-457 PASS: widths, breakpoints (+28 collapsed, +150 lane), 10 px themed bar, page never scrolls, pinned 23 px legend with W/8 items all match the KanbanFlow dumps; the bug reproduces on the baseline.
DONE: KF-484 PASS: clean release builds with and without the change are byte-identical (84f726dd…); the debug login test costs about 5x less CPU and the unit binary 8.4 versus 36 CPU-s.
DONE: INTERACTIONS PASS: timer panel 0.5 px from KanbanFlow, Stop −2.5 px; drag into Done and across lanes while scrolled, quick add, layout-view delete and swimlanes all work with 0 console errors.
DONE: UI-SWEEP PASS: board surfaces and 23 pages in light and dark, 0 console, page or HTTP errors, dark limited to the board.

