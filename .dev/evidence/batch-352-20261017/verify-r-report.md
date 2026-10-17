verify-r report: KF-457 "fit the board to the viewport like KanbanFlow" (c36ab2b, baseline 271a281)

The fix passes. On 5- and 4-column boards, every number I measured matches KanbanFlow's (KF's) dump at every width that has a KF capture. The bug reproduces on the baseline and is gone on the fix. All gates are green, there are no stored-data changes, and no console errors or failed requests appeared on any page.

## Environment
- **Worktrees:** fix is `.dev/worktrees/verify-r`, detached at c36ab2b. Baseline is `.dev/worktrees/verify-r-base`, detached at 271a281. The diff touches only `src/routes.rs` (template helper and one test), `static/style.css` and `templates/board.html`.
- **Binaries:** release builds with `build.build-dir="target/build"`. Fix md5 0e86e15e88e3ce0c2e0e81b6b2fb8cf7, baseline md5 29be8d41805ec2863e60bc794e03ce93. `rg -a -c board-scroll` finds 10 matches in the fix binary and 0 in the baseline.
- **Ports and databases:** fix on 3378, baseline on 3379. Fresh databases at `$S/verify-r/chipflow-verify-r-{3378,3379}.redb`. Both servers were killed by PID (90342, 90486).
- **Browser:** Playwright 1.63 Chromium (scripts `$S/r2/pw/vr-*.mjs`), DPR 1, fresh context per scenario, animations off. Headless Chromium hides scrollbars by default, so I relaunched without `--hide-scrollbars` for every scrollbar-dependent number.
- **Fixtures:** golden master with the legend on (`--as-of 2026-09-30`), the same plus 2 swimlanes holding tasks, the same with column 2 collapsed, lanes plus a collapsed column, a 4-column board with the legend off and cards only in column 1, a tall board (40 cards in To Do), an all-collapsed board, and a 1-column board.

## Gates (fix tree)
- `fmt --check`: exit 0.
- `clippy --all-targets`: exit 0, 0 warnings.
- `TZ=UTC /Users/senkwich/.cargo/bin/cargo test`: exit 0. Every suite is ok, including the new `board_table_should_carry_expanded_and_collapsed_column_counts`.
- `jsc checkSyntax static/app.js`: exit 0.

## KF-457 width table, golden master (5 columns, legend on)
Columns: KF / fix / baseline, in px.

| Width | KF col / card / board scroll | Fix | Baseline |
|---|---|---|---|
| 1920 | 380 / 362,363 / 0 | 380 / 362,363 / 0 | 379.6 / 362.6,363.6 / 0 |
| 1680 | 332 / 314,315 / 0 | identical to KF | 331.6 |
| 1440 | 284 / 266,267 / 0 | identical to KF | 283.6 |
| 1366 | 269.2 / 251.2,252.2 / 0 | 269.19–269.25 / 251.25,252.19 (subpixel) | 268.8 |
| 1280 | 252 / 234,235 / 0 | identical to KF | 254, scrolls 12 |
| 1180 | 232 / 214,215 / 0 | identical to KF | 254, scrolls 112 |
| 1120 | 220 / 202,203 / 0 | identical to KF | 254, scrolls 172 |
| 1119 | 220 / scrolls 1 | 220 / scrolls 1 | 254, scrolls 173 |
| 1100 | 220 / scrolls 20 | 220 / scrolls 20 | scrolls 192 |
| 1024 | 220 / scrolls 96 | 220 / scrolls 96 | scrolls 268 |
| 900 | 220 / scrolls 220 | 220 / scrolls 220 | scrolls 392 |
| 768 | 220 / scrolls 352 | 220 / scrolls 352 | scrolls 524 |
| 640 | layout 700 wide / scrolls 420 | layout 700 wide / scrolls 420 | scrolls 652 |
| 1440x612 | 284 | 284 | 283.6 |
| 1120x612 | 220 | 220 | 254, scrolls 172 |

- **4-column board vs FQhhSs1:** matches at every captured width: 475 / 415 / 355 / 336.5 / 315 / 290 / 270 / 251 / 220 at 900, scrolls 132 at 768 and 200 at 640. The baseline holds columns at 317.5 and scrolls from 1291.
- **Legend:** 23 px tall, full viewport width (700 px at 640), 8 items of W/8, one row, 0 clipped labels. The baseline legend is 33 px.
- **After scrolling the board fully right:**
  - On the fix, the top bar, board header and legend all stay at x=0 with full width, and the last column ends 10 px from the right edge.
  - The baseline legend moves to x = −scroll (−268 at 1024), leaving a gap at its right. That is the owner's report, reproduced.
- **Which element scrolls:** only `div.board-scroll` scrolls, with a 10 px scrollbar. The document never scrolls.
  - At 640 the page is clipped at 700 px, like KF. The Menu button is cut off at x=690, which KF's README predicts.
  - Wheel and keyboard tests did not scroll the document; the board region scrolled.
- **Vertical:** column bodies fill the height and lose exactly 10 px when the horizontal scrollbar appears (748 → 738, KF 751 → 741).
  - ChipFlow's bodies are 3 px shorter than KF's. 2 px is the top bar, and 1 px is KF counting the bottom border inside the body (both end at y=868).
- **Overflowing column:** shows a 7 px scrollbar and its cards lose 7 px (196 vs 203). The last column's cards are 202. Card gutters are 9 px left and 8 px right.
- **KF's 1120x612 shows 196 px cards in To Do; ChipFlow's golden master doesn't.** KF's cards are taller there ("(Done)" due lines and visible subtasks), so its column overflows. This is a card-content difference, not layout.

## Breakpoints (bisected on the fix)
| Board | Fix | Expected |
|---|---|---|
| 5 columns | first fit 1120, scrolls at 1119 | 1120 |
| 4 columns | 900 / 899 | 900 |
| 4 expanded + 1 collapsed | 928 | 28 + 880 + 20 |
| 5 columns + lanes | 1270 | 150 + 1100 + 20 (KF formula gives 1250 because KF's lane column is 130; consistent with ChipFlow's 150, so judged OK) |
| Lanes + collapsed column | 1078 | 150 + 28 + 880 + 20 |
| Baseline, all boards | 1292 | — |

## Break attempts
- I checked every width from T−3 to T+260 for the 5-column, 4-column and lanes boards at DPR 1, 1.25 and 2, resizing the live page without reloading. There were 0 wrong scroll states, 0 columns under 219.5 px and 0 document overflow.
- Lanes whose tasks are gone fold into side strips. Those strips push the breakpoint out (1185) and columns never squeeze below 220.
- Collapsing or expanding a column (it reloads) updates the inline counts correctly (5/0 and 4/1).
- Returning from the layout view gives the same 220 px columns and a 23 px legend.

## Interactions
All checked at 1024x900 on a board scrolled 96 px; the baseline gave the same placements.
- **Sticky headers:** on the lanes board at 1024x600, scrolled to (120, 187), the header row stays at y=82 (the top of the scroll region). Header x positions equal cell x positions (−110, 40, 260…).
- **Drag between columns while scrolled:** In Review to Done moved the task; the API shows the new column.
- **Auto-scroll while dragging:** scrollLeft goes 0 → 96 at the right edge and back to 0 at the left edge.
- **Popup placement:**
  - The column menu opens at the click point, clamped to the viewport.
  - Show details clamps to the right edge, the same as the baseline.
  - The filter panel, board Menu, timer popup, card context menu and add-task dialog all open where they do on the baseline.
  - The task modal is centred, and the scroll position is kept after it opens.
- **Layout view:** pixel-identical to the baseline at 1024, 640 and 1440x612 dark.
- **Dark mode:** scrollbar thumb #777 and track #555, matching KF's dark variables.
- **Cards at 220 px:** avatars wrap the same way as KF's 1120 capture. "Build payment webhook handler" puts its avatar on its own line; "Implement dark mode toggle" and "Fix pagination bug" keep it on the icon row.
- **Console:** 0 errors and 0 failed requests on every page, on both servers.

## Regression (pixel diff, fix vs baseline)
- **gm 1905x1000 light:** 4758 px differ (0.25%), all at y 82–967 near column dividers and card text. Columns moved from 376.6 to 377 px, which shifts content by a fraction of a pixel. The top bar, header and legend are identical.
- **gm 1440x612 light:** same pattern, 4627 px, y 82–579.
- **Dark (gm, 4-col board with a collapsed column):** the same subpixel shifts, plus the 9 px strip between the board frame and the legend, now dark instead of #f2f2f2. That matches KF's dark golden, where the strip is the dark background (crop at `$S/verify-r/crops/kf-vs-fix-dark-bl.png`). The legend is identical.
- **4-column board:** 330–630 px, card text subpixel shifts only.
- **Lanes 1440x612:** the fix shows the 10 px vertical scrollbar on the board region (the lanes overflow vertically, as with KF's swimlane overflow-y), so columns are 252 instead of 253.6.

## Data
No changes to `db.rs` or `models.rs` (0 diff lines). The `routes.rs` change is template helpers and one test only; no API changes.

## Observations (candidate defects, not KF-457 failures)
1. **All columns collapsed:** on a board where every column is collapsed, the 28 px strips stretch to 284 px each at 1440, on both baseline and fix. KF sets the table to `width: auto` in this case. Evidence: `$S/verify-r/m-fix-edge.json`, `m-base-edge.json`.
2. **Swimlane label column:** the label column (150 px vs KF's 130) scrolls off horizontally along with the columns, on both baseline and fix. I don't know whether KF pins it; there is no KF swimlane golden.
3. **Card content at 220 px (already-known area):** KF cards show "(Done)" on due lines and visible subtasks with a "2 hidden" line, which makes KF's To Do column overflow at 1120x612. ChipFlow cards don't show these.

Evidence is under `$S/verify-r/` (`$S` = `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`):
- `width-table.txt`: KF / fix / baseline at every width
- `m-*.json`, `reg-*.json`: raw measurements
- `bisect-*.txt`, `inter-*.json`
- `gates.log`, `clippy.log`, `test-utc.log`
- `shots/`, `crops/`, `diff/`: screenshots, crops and diff masks

DONE: KF-457 PASS columns, cards, board scroll and legend match KF at all 16 sizes for 5 and 4 columns (220 px floor, breakpoints 1120 and 900, only the board region scrolls with a 10 px bar, legend 23 px at full width and pinned); the baseline reproduces the bug (fixed at 254/317.5, scrolls from 1291, legend shifts with a gap)
DONE: INTERACTIONS PASS sticky headers in both directions, drag while scrolled, edge auto-scroll, every popup and the modal placed correctly, layout view identical, dark scrollbars per KF, 0 console errors
DONE: REGRESSION PASS diffs at 1905x1000 and 1440x612, light and dark, are subpixel column-width shifts plus the dark strip under the frame now matching KF's dark background (and the lanes board's new vertical scrollbar); top bar, header and legend pixel-identical
DONE: DATA PASS no db.rs/models.rs/API changes
DONE: GATE PASS fmt, clippy --all-targets (0 warnings), TZ=UTC cargo test, jsc checkSyntax all exit 0
