## verify-b2 report: KF-223 and KF-358

**Results:** KF-223 passes. KF-358 fails on one regression that shows once a board is taller than the viewport and scrolls.

### Environment
- **Fix tree:** worktree `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-b2`, detached at fa408bcb26d30ceaf6e766dd219485fad3e09848. It is ffdce86 plus 921e4e2, a2bc94f, 483a842, bd2178a and fa408bc. The diff touches only `src/routes.rs`, `static/app.js`, `static/style.css` and `templates/board.html`, which is what these two defects need.
- **Baseline tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-b2-base` at ffdce86.
- **Binaries (debug):**
  - fix md5 1329671d6309b52212c2e6e9431143a0; baseline md5 abb64bb6987025d44943e9d8f6fc50cd.
  - `rg -a -c collapsedColumn-nameBox` finds 1 match in the fix binary and 0 in the baseline.
  - Copies are in `$S/vb2/chipflow-{fix,base}`.
- **Servers:** port 3315 runs the fix and 3316 the baseline, with fresh DBs `/tmp/chipflow-verify-b2-3315.redb` and `/tmp/chipflow-verify-b2-3316.redb`. Both have been killed by PID.
- **Captures:**
  - Viewports were 1919x998 for the golden comparisons, 1905x1000 for the others, plus 1100x700. DPR 1, animations frozen, a fresh context per scenario.
  - Pixel claims were measured on Linux captures in my own container (playwright v1.63.0-noble, pinned sha), which has since been removed. DOM metrics came from macOS Chromium 1.63.0.
- **Fixtures:**
  - The golden-master seed.
  - Swimlane boards whose main lane is renamed "Default": "VB2 long lanes" (4 lanes), "VB2 many lanes" (12 lanes), "VB2 one char", and "VB2 tall column 45" (45 cards in one lane).
- **Evidence:** `$S/vb2/shots/` and `$S/vb2/crops/`, where `$S=/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`. Each PNG has a `.json` beside it with DOM metrics and the console and network log.

### Gates (fix tree)
- `cargo fmt --check`: exit 0.
- `cargo build`: OK.
- `cargo clippy --all-targets`: exit 0, 0 warnings.
- `TZ=UTC cargo test`: 103 passed, 0 failed, including the new `collapsed_column_renders_count_header_and_body_name`.
- `jsc checkSyntax(static/app.js)`: exit 0.

### KF-223: collapsed strip and "Show details" popover
KF references are the 09-30 goldens; "Fix" values are from the Linux captures.

| Claim | KanbanFlow ref | Baseline | Fix | Verdict |
|---|---|---|---|---|
| Strip width and separators | 27 px (x 479-505), separators at 478/506 | 21 px (x 474-494) | 27 px (x 479-505), separators 478/506 (`#D7D7D7` in header, `#E4E4E4` in body) | PASS |
| Header holds only the count | count "5" `#111111`, ink x 489-495, 9 tall | count, vertical red "2 overdue tasks" and name in a white header | count only; green-channel glyph grid matches KF (x 489-495, 9 rows) and contains `#111111` | PASS |
| Header height unchanged | normal height | header stretched to y 241 | all 5 headers 47 px; after the filter removes the WIP line, all 41 px | PASS |
| Name colour, direction, ink box | `#555555`, top to bottom, 12x54 at x 487-498, 248 px of ink | bottom to top, inside the header | `#555555`, top to bottom, 12x54 at x 487-498, 248 px of ink (identical count) | PASS |
| Name starts 29 px below the header rule | 29 px | n/a | 29 px | PASS |
| Second strip ("IN PROGRESS") | ink 12x120 | n/a | 12x120 at the same x and y as KF | PASS |
| Overdue shows only a line | 3 px `#FF8080`, 21 px wide, across the rule | `#DC2626` vertical text | 3 px `#FF8080` across the rule (y 122-124 on rule 123), 21 px (x 482-502), no text | PASS |
| Over-WIP strip count colour | `#8B0000` | n/a | `#8B0000`, plus the line | PASS |
| Popover box | 377 px (x 56-432), `#C3C3C3` border, `#EEEEEE` band 30 px, `#D7D7D7` rule, 34 px body | 223 px rounded card (x 4-228) | 377 px (x 56-432), same border, band, rule and body | PASS |
| Popover text | title ink 106x12 at x 184-289, body 71x9 at x 209-279 | n/a | identical boxes; title `#666666` | PASS |
| Clicking expands the column | expands | n/a | clicking the body cell or the header expands it after reload; collapsing from the menu works; 0 failed requests | PASS |
| Long name clipped to its cell | n/a | n/a | ends in a clean "…" inside each cell on every row; ink stays 4 px above the frame bottom (ink to y 985, frame 990); no ink below the frame | PASS |
| No phantom scroll height | n/a | n/a | board-wrap scrollHeight equals clientHeight (925) when the board fits; with 12 lanes, scrollHeight is 1206 whether the name is long, "X", or the column is expanded, so the name adds 0 px | PASS |

Break attempts:
- A long name in narrow 96 px cells across 12 lanes: shows "THIS …".
- A one-character name "X".
- Collapsed first column and collapsed Done (with a done task) at the frame's right edge.
- Two adjacent strips.
- The strip with the "Overdue" filter on: the count updates to 2, the line stays, and the headers stay equal.
- A strip column scrolled under the sticky header: the name never paints over the header.
- The popover on a 28 px strip: centred, x 450-827.
- The popover for a column partly off-screen at 1100 px: kept in the viewport, x 719-1096.
- Markup in the name (`<i>x</i> & "q"`): escaped.
- Dragging a card into the stretched empty bottom of a body: it moves to the right column and lane.
- Dark mode: no new breakage. The strip count is `#111111` on `#1F2937`, the same as every other count badge in the baseline.

### KF-358: frame, gutter and full-height bodies

| Claim | KanbanFlow ref | Baseline | Fix | Verdict |
|---|---|---|---|---|
| 10 px `#F2F2F2` gutter | x 0-9 and 1909-1918 | `#EEF0F3`, no gutter | x 0-9 and 1909-1918 | PASS |
| Frame sides | `#C3C3C3` at x 10 and 1908 | none | x 10 and 1908 | PASS |
| Frame bottom | y 988, gutter below | none | y 988, `#F2F2F2` at 989-997 | PASS |
| Frame top | `#C3C3C3` across x 10-1908 | n/a | on y 75 (the bar is 5 px shorter; that is KF-361), same edge pattern | PASS at rest |
| Bodies white to y 987 | always | bodies end after the cards; y 600 is `#EEF0F3` | to 987 on the golden board, an empty one-column board, every column and every lane layout | PASS |
| Column separators | `#E4E4E4` at x 390/770/1149/1529 | n/a | `#E4E4E4` at x 390/770/1149/1529 | PASS |
| Legend-on geometry | rule y 965 | n/a | rule y 965, bodies white to 964, frame sides to the rule | PASS |
| `#C2C2C2` divider with "Edit layout" open | n/a | full-width divider | y 75 `#C2C2C2` across x 0-1918; the screen is 0 px different from baseline | PASS |
| Narrow viewport (1100x700) | n/a | n/a | frame wraps the 1270 px table with a 10 px right margin (scrollWidth 1292) | PASS |
| **Scrolled tall board** | n/a | `#C2C2C2` divider at y 75 stays | **no divider and no header rule** | **FAIL** |

The failure:
- **What happens:** once a board is taller than the viewport and the board-wrap scrolls, the frame top scrolls away. The sticky column header then sits flush under the board bar.
- **Fix:** column x 900 reads `#F2F2F2` to y 74, then `#EBEBEB` from 75 to 115, then white. Row 75 is `#EBEBEB` across the header.
- **Baseline:** keeps the `#C2C2C2` divider across all of row 75 in the same state.
- **Also missing:** the header's bottom rule, so cards scroll straight under the header with no line.
- **Where it reproduces:** a 45-card single-lane board and a 12-lane board, on both Linux and macOS.
- **Why it matters:** this is the same regression class as the earlier "Edit layout" divider loss. Only the layout-view case was restored.
- **Evidence:** `$S/vb2/shots/lin-tall-scrolled-3315.png` and `-3316.png`, `$S/vb2/crops/lin-tall-scrolled-fix-top.png` and `lin-tall-scrolled-base-top.png`, `$S/vb2/crops/scrolled-divider-fix-vs-base.png`, `$S/vb2/shots/many-bottom-3315.png`.
- **Severity:** MEDIUM. It appears on any board that overflows vertically.
- **Caveat:** there is no KanbanFlow capture of a scrolled board, so this is judged as a regression against the baseline, not against a reference.

Break attempts: 12 lanes scrolled to the bottom (frame bottom at y 988 is reached and holds), 45 cards, an empty board, the legend on, the filter on, 1100 px wide, dark mode, and layout edit.

### Console and network
24 fix captures: 0 console errors, 0 page errors, 0 failed requests, 0 HTTP errors.

### Observations (candidates, not filed)
1. **Default lane label is blank.** The "Default" lane's label cell renders with no text, in the baseline and the fix alike, so not a regression. If KanbanFlow shows the lane name, that is a KF-359 gap. Evidence: `$S/vb2/shots/long-3315.png`.
2. **Dark mode turns the board area white.** Stretched bodies make it almost all `#FFFFFF`; the baseline was white only behind cards. Dark mode was already broken in the baseline (white bodies, invisible titles, `#111111` counts on `#1F2937`). Evidence: `$S/vb2/shots/dark-3315.png` and `dark-3316.png`.
3. **Two strips sit 1 px right of KanbanFlow.** Fix x 626-652 and 654-680 against KF 625-651 and 653-679; the single strip matches exactly. This comes from the table's fractional column widths. Evidence: `$S/vb2/shots/lin-two-3315.png`.
4. **Labels can show through the corner cell.** The sticky `.corner-cell` is transparent, so a lane label scrolled under it could show between the bar and the header row.
5. **Filtered strip count keeps its over-WIP colour.** Under a filter, `refreshFilteredCounts` does not toggle `columnHeader-countBadge--over`, so a strip whose filtered count drops under its WIP limit stays darkred. This is the same in the baseline.

DONE: KF-223 PASS every measured claim matches the 09-30 goldens (27px strip, count-only header, #555555 12x54 name 29px below the rule, 21px #FF8080 line, 377px popover); long names stop at the cell with an ellipsis, inside the frame, adding 0px of scroll height
DONE: KF-358 FAIL gutter, frame, bodies, legend and layout-view divider all match, but once a tall board scrolls the sticky header sits under the board bar with no #C2C2C2/#C3C3C3 divider and no header rule, while the baseline keeps the y75 divider
