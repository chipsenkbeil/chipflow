## verify-b4 report: KF-358 and KF-223 on integ/batch-33 a1baa41

Both defects pass on a1baa41. Every check in the brief holds when measured. Each regression from the earlier attempts reproduces on its own tree (bd37f6c, c17372f) and is gone on a1baa41.

### Environment
- **Tree under test:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-b4`, detached at a1baa41. I did not touch integ-33.
- **Comparison trees:** `-bd37` (bd37f6c), `-c173` (c17372f), `-base` (ffdce86 = origin/main).
- **Diff scope:** `git diff --name-only c17372f a1baa41` gives `static/style.css` only.
- **Binaries:** release builds, copied to `$S/verify-b4/bin/`.

| Commit | md5 | `border-image-outset: 0 0 1px 0` | `folded-strip::before` |
|---|---|---|---|
| a1baa41 | 30cc0a9d… | 1 | 2 |
| bd37f6c | 237a59c8… | 0 | 0 |
| c17372f | 40e3b9c0… | 0 | 0 |
| ffdce86 | 8f065ce1… | 0 | 0 |

The served `/static/style.css` matched on all four ports.
- **Ports and databases:** 3315 / 3316 / 3317 / 3318, fresh databases at `/tmp/chipflow-verify-b4-<port>.redb`.
- **Fixtures:** seed `--as-of 2026-10-16`, built by `$S/verify-b4/fixtures.sh`. Boards: plain, collapsed (To Do), legend, 45-card, 12-lane, folded-right, folded-left, tall folded-right, tall left+right, long collapsed name, tall collapsed.
- **Capture:** my own Linux container (playwright:v1.63.0-noble at the pinned digest). Viewport 1919x998, DPR 1, fresh context per scenario, animations off. Pixels were read with ImageMagick and a luajit run-length helper (`row.sh`, `col.sh`, `diff.sh`).
- **Console and network:** 0 console errors, 0 failed requests and 0 HTTP 4xx/5xx across 42 scenarios on a1baa41. The other three trees were also clean (34, 39 and 31 scenarios).
- **Cleanup:** servers killed by PID and the container removed. The worktrees are still in place.

### Gates (a1baa41)
- fmt passes; debug build passes.
- clippy `--all-targets`: no warnings.
- `TZ=UTC cargo test`: every suite ok (11, 68 with 1 ignored, 3, 13, 8, 6, 4, 2, 2, 2, 3, 0).
- jsc `checkSyntax`: exit 0.
- The release build warns `unused import tower_http::services::ServeDir`. ffdce86 gives the same warning, so it is not new.

### KF-358

| Claim | Baseline | a1baa41 | Verdict |
|---|---|---|---|
| 10 px #F2F2F2 gutter; frame at x 10 / 1908 | ffdce86: no frame, #EEF0F3 page | gutter x 0-9 and 1909-1918; frame x10 and x1908 are #C3C3C3 for y 75-988 | PASS |
| Frame top / bottom (KanbanFlow y 80 / 988) | – | top y75 (the 37 px header row is KF-361), bottom y988 with #D3D3D3 crossings | PASS (top offset belongs to KF-361) |
| #E4E4E4 separators at x 390/770/1149/1529 | – | same, and identical to KanbanFlow at y600 | PASS |
| White bodies run to y987 regardless of card count | ffdce86: body ends at y448, #EEF0F3 below | rows y 560-997 vs the KanbanFlow 09-30 board: **0 px differ** (ffdce86: 840,522) | PASS |
| With the legend on, bodies end at the legend | – | white to y964, legend rule at y965, frame bottom dropped | PASS |
| Scrolled: frame top continuous #C3C3C3 with #CDCDCD crossings | bd37f6c: y75 shows header #EBEBEB, no frame; c17372f: gaps over strips (x 11-42, 1844-1907) | continuous from x10 to x1908 on the 45-card, 12-lane, tall folded-right and tall left+right boards, at scrollTop 300 and at bottom, including over strips and the corner | PASS |
| Scrolled: header rule #C3C3C3 with #D3D3D3 at separators | bd37f6c: rule lost (header runs straight into cards) | y130 #C3C3C3 at mid-column x; #D3D3D3 at x 390 and 510; collapsed board at y123 | PASS |
| At rest, a1baa41 vs bd37f6c, rows y ≥ 75 | – | 0 px on plain, collapsed, legend, Edit layout, folded-right, folded-left, tall folded and 12-lane | PASS |
| Edit layout divider | – | y75 is #C2C2C2 across the full width on all four trees, at rest and scrolled | PASS |
| 45-card board | – | exactly **644 px**, inside x 1300-1518, y 87-99: the "In Review" header text | the accepted flip, unchanged |

- **Noise:** a first capture of folded-right showed 10 px at delta 1. A repeat run of the same tree reproduced them and the cross-tree diff then came out 0, so they are capture noise.
- **The 644 px flip:** a1baa41 has 70 non-grey pixels in that box (the green "+" only), against 682 on bd37f6c. That is still exactly the subpixel-to-grayscale flip. c17372f carries the identical 644 px, and a1baa41 vs c17372f is 0.
- **a1baa41 vs c17372f scrolled:** the only difference on folded boards is 64 px, the two 32 px strip segments of the frame top at y75. Every other board is 0 px.

### KF-223
- **Strip width:** 27 px at x 479-505, with separators at 478 and 506. This matches KanbanFlow at y600.
- **Header:** holds the count only. "5" ink is 7x9 at #111111, same position as KanbanFlow. The ink is not pixel-identical, which is a glyph rendering difference.
- **Name ink:** 12x54, starting 29 px below the rule (ours y152 under a y123 rule; KanbanFlow y146 under y117). It is **pixel-identical to KanbanFlow** (29x70 crop, 0 px differ).
- **Overdue strip:** only a 3 px #FF8080 line across the header bottom, with no text.
- **Long name at 1919x998:** the whole name fits in the strip, so there is no ellipsis to see there. At 1919x600 the name is clipped with "…" and scrollHeight equals clientHeight (525/525), with the legend on as well. The same holds on bd37f6c with 0 px differing.
- **Popover geometry:** 377x67 on the plain, collapsed, collapsed-column, scrolled, first-column, last-column, scrolled-with-strips and tall-collapsed boards:
  - #C3C3C3 border;
  - #EEEEEE band of 30 rows;
  - #D7D7D7 rule;
  - 34-row white body.
- **Popover top line:** a single #C3C3C3 line that coincides with the header rule (y130 plain and scrolled, y123 collapsed). c17372f still shows the doubled #BBBBBB + #C3C3C3 at y 130-131 and y 123-124.
- **Popover vs the KanbanFlow golden:** compared on the same column, "Column: Backlog" on the collapsed board, at x 56-432 against KanbanFlow's x 56-432:
  - border columns are identical;
  - the body text sits at the same position;
  - the title is centred and #666666, one pixel wider and further left;
  - the 1026 differing pixels are all text antialiasing (our title uses subpixel AA, KanbanFlow's is grayscale).

### Dark mode (a1baa41)
- **Corner cell:** #1F2937 (c17372f: #FFFFFF).
- **Frame:** closed. #374151 at x10, x1908 and y75. The frame top over the strips is #374151 when scrolled, where c17372f leaves gaps.
- **Out of scope:** these are part of the separate redesign and I did not fail on them:
  - the board header row stays #F2F2F2;
  - strips stay light (#F3F4F6 / #E5E7EB);
  - the frame bottom keeps light #D3D3D3 crossings;
  - bodies, separators and the lane-label column stay white / #E4E4E4.

### Break-attempts (all passed on a1baa41; c17372f fails the strip ones)
1. **Native drag and drop while scrolled to 300 with strips:** card 9 dragged from Backlog to In Progress on the tall left+right board, plus a drag on the tall folded-right board. The frame top is continuous before, mid-drag and after. A POST to `/move` was sent, and the API confirms card 9 is now in In Progress / Default.
2. **Unfolding while scrolled:** adding a task via Backlog "+" put it in the folded "Personal" lane (confirmed by API). After a reload and scrolling back to 300 the strip had become a row, and the frame top stays continuous with the new crossings.
3. **1100x700 viewport, scrolled both ways:** frame top and header rule are continuous at scrollTop 300 and at bottom, with scrollLeft 0 and fully right, on the tall left+right, 45-card and 12-lane boards.
4. **Popover opened while scrolled:** checked on the 45-card, strips and tall-collapsed boards.
5. **Last-column popover at the right edge:** at 1919 wide it sits at x 1530-1906. At 1100 wide, scrolled right, it is clamped to x 719-1095. Both sit on the y130 rule.
6. **DPR 2:** the 2-device-pixel frame line covers both strips at y 150-151 (c17372f shows strip background there). The header rule is at y 260-261 with #D3D3D3 at the separators.

### Regressions found
None in what I swept: the board layout, legend, layout view, collapsed columns, popover, dark mode, narrow viewport and drag. The changed selectors are used only by `templates/board.html` and `static/app.js`.

### Observations (candidate defects, none caused by this diff)
- **Tall collapsed-column board, at rest:** 2 px at delta 1 against bd37f6c (x518 and x962, y113, the ends of the In Progress WIP bar). It reproduces on every run, is the same on c17372f, and is the same kind of compositing artefact as the accepted flip. Evidence: `out-rest-3315/tallcol.png` vs `out-rest-3316/tallcol.png`.
- **Frame bottom one pixel low on tall boards scrolled to the bottom:** it sits at y989 (8 px gutter) on the 45-card and tall left+right boards, because the header top is at a fractional 75.38. bd37f6c does the same.
- **No header rule under the corner on the 12-lane board:** at rest and scrolled. Identical to bd37f6c; I had no KanbanFlow reference to compare against.
- **Header row height and colour vs KanbanFlow:** our rule moves from y130 to y123 when To Do is collapsed, while KanbanFlow keeps 36 rows at y 81-116. Our header is #EBEBEB where the 09-30 KanbanFlow capture shows #EEEEEE. This belongs to KF-361/KF-362 and KF-355.
- **Legend strip colours vs `kanbanflow-board-menu-open-legend-20260930.png`:** ours is #BBBBBB / #F0F0F0, that capture shows #C3C3C3 / #F2F2F2. KF-354's own measurement differs from this capture.
- **"+" on a column whose first lane is folded:** the card is saved in the folded lane but appears in the Default lane until a reload. This is the KF-208 code path.

All evidence is under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-b4/`. There is one `out-<set>-<port>/` folder per scenario set (rest, scroll, pop, dark, break, long), each with PNGs and a `results.json` holding metrics, console, network and POST bodies.

DONE: KF-358 PASS frame top continuous #C3C3C3/#CDCDCD and header rule #C3C3C3/#D3D3D3 while scrolled on every board incl. folded strips, narrow, DPR2 and mid-drag; at rest 0 px vs bd37f6c on all 8 listed boards (45-card: only the accepted 644 px flip); 0 errors and 0 failed requests
DONE: KF-223 PASS 27 px count-only strip with name ink 12x54 29 px below the rule (pixel-identical to KanbanFlow), long name clipped with ellipsis and 0 extra scroll; popover 377x67 with a single top line on the header rule on the plain, collapsed and scrolled boards (c17372f doubling gone)
