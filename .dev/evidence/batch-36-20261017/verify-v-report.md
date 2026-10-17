All four defects PASS and the regression sweep and gates are clean. The one deviation is that the Boards dropdown now sits at y 34, 4 px higher than the golden (obs 1).

## Environment
- **Candidate:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-v`, HEAD bc117c6 (a76aff7, 1d04f97, bc117c6 on a980a7c). The diff touches only `static/app.js` (+2), `static/style.css` and `templates/board.html`, with no Rust.
- **Baseline binary:** the disk was full (217 MB free), so the a980a7c baseline could not be built. I used `release-352/target/debug/chipflow` instead. `git diff a980a7c..9bb6959 -- src templates static Cargo.*` is empty, so its app tree is identical to a980a7c. The `verify-v-base` worktree at a980a7c is only the cwd and seed-script source.
- **Binaries:** md5 fix `aa00f8fb6dd11b95c9db7c9bde442cb4`, base `9dd39e568b592e22911891efe9f8f74f`. `timer-pill--idle` appears in the fix binary (`rg -a -c` = 1) and not in the base. The served `style.css` has it on 3406 and not on 3407.
- **Servers:** fix on 3406 (PID 46534), base on 3407 (PID 47816). Both ran under libfaketime at 2026-10-26 09:00 America/Chicago, with Playwright `context.clock` in lockstep. Databases are `$S/db-3406.redb` and `$S/db-3407.redb`, fresh and seeded with `seed-golden-master.sh`. Both PIDs are killed and no other process was touched.
- **Viewports:** 1440x612, 1905x1000 and 1120x900 at DPR 1, plus DPR 2 for the chip.
- **Evidence:** all under `$S/verify-v/`, where `$S=/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`.
  - `shots/`: screenshots.
  - `chip-3406*.out`, `chip-3407.out`: chip measurements.
  - `reg-340{6,7}.out`, `pages-340{6,7}.out`: regression and per-page data.
  - `gate-*.log`: gate logs.
  - `*.mjs`: scripts.
  - `build-fix.log`: fix build.

## KF-233 PASS
| Claim | Baseline | Fix | Golden |
|---|---|---|---|
| Top bar | 1440x36, #333 plus 3 shade rows | 0,0,1440,34, #333333, box-shadow none | board/01: 1440x34 |
| Boards button | 111x28 at (11,5) | 100x26 at (10,4), #515151 | 100x26 at (10,4) |
| Board header row | y 36 | y 34 | y 34 |
| Column header | y 82 | y 80 | y 80 |
| Dark bar | not measured | #222222 | #222222 |
| Board settings container | content at y 74 | content at y 74 (`.bshell` at 74; page 34 + 40 px padding) | 74 |

- The dark Boards button is `rgba(255,255,255,.15)` over #222222. Light-mode #515151 is the 0.15 white over #333333.
- Break-attempt: the 1905 and 1120 widths and `/settings`, `/settings/colors` and `/settings/delete` pages. Below y 80 the colors and delete pages are pixel-identical to the baseline.

## KF-239 PASS
- **Top bar:** no `#timer-pill` in `.topbar`.
- **Idle:** the clock icon is at 1218,40,34,34, black at opacity 0.7, with the funnel at 1258,40, matching `shortcut0` and `shortcut1`. In dark it is white.
- **Panel:** 992.5,40,260 wide, against the golden's 992,40,260. Heights are 304 idle and 334 running, against the golden's 310 idle.
- **States:** these were measured via `chip.mjs`, `flow.mjs` and `stop2.mjs`, each with 0 console errors and 0 failed requests.

| State | Result |
|---|---|
| Idle, running pomodoro | pass |
| Waiting 00:00 | chip stays, title shows `--------` |
| Short and long break | chip with green ▶ |
| Break overtime | pass |
| Stopwatch | pass |
| Panel open and close | pass |
| Log, settings overlays | pass |
| Stop with reason "Email" | chip returns to the clock icon at 1218; log entry has badge title `Stopped with reason "Email"` |
| Reload mid-session | pass |

- **Layout edit:** the pill is hidden in layout-edit mode.
- **Break-attempts:** a 1:23:47 stopwatch grows the chip to 147.9 px without moving the chevron. Chip hover is #404040 in dark too, so no `body.dark` rule leaks through.

## KF-463 PASS
- **Chip:** 1106,40,145,34 in every state, identical to golden timer/22 and 28 (also at DPR 2).
- **Digits:** 20 px bold, box 1150,42,57,30, identical to golden `clockText`.
- **Stop square:** 20x20 at 1119, 13 px in.
- **Break triangle:** 16x19 at 1123, 17 px in.
- **Chevron:** 25x25 at 1216, 44.5 px from the top (golden 45), ending 10 px from the right edge.
- **Baseline:** 96.3x32 with 13.6 px digits.
- **Visual:** side-by-side with golden 28 (`shots/pair28.png`) matches.

## KF-462 PASS (does not reproduce on the baseline)
- In dark, the idle clock, running chip and short and long break chips are all #484848 with #ffffff digits.
- The baseline binary also measured #484848 and #ffffff for the idle and break chips in dark, so the tracker's bug is already gone in a980a7c.
- The baseline `style.css` has no `body.dark .timer-pill` rule. The candidate keeps that rule absent.

## DONE: REGRESSION
Pixel comparison against the baseline, with the candidate cropped from y 80 and the baseline from y 82, so the 2 px shift is removed.

| Region | Light | Dark |
|---|---|---|
| Board body, 1905, 1440, 1120 widths | 20, 18, 0 differing px | 18, 18, 3 differing px |
| Bottom 30 rows (legend pinned) | 0 | 0 |
| Header row, excluding the 3 removed shade rows | 219 px (the 20x20 clock icon) | 219 px (the 20x20 clock icon) |

- The board-body residue is subpixel AA at a delta of 0.3% or less (AU avatar).
- Column widths: 377 at 1905, 284 at 1440, 220 at 1120, so KF-457 is intact.
- The 2 px upward shift is uniform.
- Board Menu panel at 1190,34,250x578, matching the golden. The boards dropdown and the filter panel have anchoring deviations from the golden (obs 1 and 2); both are placed uniformly under the shorter bar.
- Pages: `/settings`, colors, delete, burndown, throughput, calendar, cycle-lead, cumulative-flow, due-date, board-history, timer log and statistics, `/boards/new` and `/boards/archived` all return 200 with 0 console errors, 0 page errors and 0 failed requests, light and dark. Only the three settings pages have a `.topbar`, and they were measured above. The reports, timer-log and boards pages have no top bar on the baseline either.
- No Rust changed, so stored data is untouched. The only JS change is one `classList.toggle`, so there are no GET writes.

## DONE: GATE
All run from the candidate worktree with `--config 'build.build-dir="target/build"'`.
- `cargo fmt --check`: exit 0.
- `clippy --all-targets`: exit 0, 0 warnings.
- `TZ=UTC cargo test` (`/Users/senkwich/.cargo/bin/cargo`): exit 0, every suite ok (24, 101 plus 1 ignored, 3, 13, 3, 8, 7, 4, 2, 2, 1, 2, 4, 0 passed).
- `env -i` plain libtest: exit 0, same counts.
- jsc `checkSyntax` on `static/app.js`: exit 0.

## Judgments
- **(a) Idle shows only the clock icon: agree.** It matches goldens board/01, board/02 and timer/14. Keeping the time out of the idle state is what KanbanFlow does.
- **(b) No timer on non-board pages: agree.** The baseline's `timer-pill` was also only in `board.html`, so nothing is lost. A running timer is still invisible on settings pages. This is shared with the baseline and matches the golden's header-row-only chip.
- **(c) Reports icon stays: disagree, it should go.** The golden top-bar right cluster is bell, help and avatar only, and the KF-233 tracker section lists the reports button as a deviation. The candidate still has `#topbar-reports-btn` at 1258,1,32,32. Reports are reachable from the board Menu.

## Observations (candidate defects, outside the claims)
1. **Boards dropdown flush against the bar.** It opens at (10,34); golden board/09 has it at (10,38) (260 wide). The baseline was at 37 under a 38 px bar, so the gap is now lost. Acceptance: at 1440x612 clicking `.boards-btn` gives `#boards-dropdown` rect top 38 and left 10.
2. **Filter panel position.** It opens at 1195,80,245x532; golden board/11 has 1190,34,250x578. This predates the change and the baseline was uniformly 2 px lower, but it is the same panel. Acceptance: `#filter-panel` top 34, left 1190, width 250.
3. **Reports icon remains in the top bar.** Acceptance: `.topbar` contains no `#topbar-reports-btn` and its right cluster is bell, help and avatar at x 1302-1337, 1347-1383 and 1394-1429 at 1440 wide (golden icons are 35 px wide; the candidate's are 32 px, with the avatar 24 px).
4. **Idle panel height.** It is 304 against the golden's 310 (`expandedTimer` 992,40,260x310 in timer/14). Acceptance: the idle `#timer-popup` height is 310 at 1440x612.
5. **Half-pixel panel x.** The panel left edge is 992.5 against the golden's 992, and the chevron top is 44.5 against 45. Acceptance: panel left is exactly 992 and chevron top exactly 45.

DONE: KF-233 PASS 34 px #333/#222 bar, Boards button 100x26 at (10,4), header row y 34, column header y 80, settings content y 74; baseline was 36/111x28/36/82.
DONE: KF-239 PASS no top-bar timer; clock icon at 1218,40,34,34 left of funnel at 1258; chip and panel (992.5,40,260) work in every state with 0 console errors.
DONE: KF-463 PASS chip 145x34, 20 px bold digits at 1150, stop square 13 px in, triangle 17 px in, all matching the goldens; baseline was 96x32.
DONE: KF-462 PASS dark chip is #484848 with #fff digits; it also measured so on the baseline, so the bug is already gone there.
