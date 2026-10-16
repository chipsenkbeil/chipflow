## verify-b3 report: KF-358 and KF-223 on integ/batch-33 c17372f

**Both verdicts are FAIL.** The fix keeps the frame top and header rule in place on ordinary scrolled boards. It still loses the frame top over folded swimlane strips when the board scrolls. It also adds a KF-223 regression: the Show details popover now draws a doubled line on every board.

### Environment
- **Fix:** worktree `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-b3`, detached at c17372f, port 3315.
- **Prev:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-b3-prev`, at bd37f6c, port 3316.
- **Base:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-b3-base`, at ffdce86 (origin/main), port 3317.
- I never touched integ-33.
- `git diff bd37f6c c17372f` touches only `static/style.css` (+27/−13).
- **Binaries:** debug build md5s are fix 1a7a3cd7…, prev 241adaa8…, base 6627c03a…. Debug builds serve `static/` from disk, so the served CSS is the real proof that the trees differ. Served `style.css` md5s are 3315 eba324d5…, 3316 423cc792…, 3317 fef55880…. `border-image` appears 2× in the fix CSS and 0× in the other two.
- **DBs and fixtures:** fresh DBs at `/tmp/chipflow-verify-b3-{3315,3316,3317}.redb`, seeded identically on all three with seed-golden-master `--as-of 2026-10-16`. Boards: plain, collapsed To Do, legend on, 45 cards in Backlog, 12 lanes, folded strips on the right, folded strip on the left, and tall boards with folded strips (right, and left plus right).
- **Captures:** Linux capture container (playwright v1.63.0-noble, pinned digest), 1919x998, DPR 1, a fresh context per scenario, animations off. Narrow runs at 1100x700.
- **Cleanup:** servers killed by PID, container removed.

### Gates on c17372f (all pass)
- `fmt --check` exit 0.
- `build` OK.
- `clippy --all-targets`: 0 warnings.
- `TZ=UTC cargo test`: all suites ok (11, 68 with 1 ignored, 3, 13, 8, 6, 4, 2, 2, 2, 3, 0), 0 failed.
- jsc `checkSyntax` on `static/app.js` exit 0.

### 1. Scrolled state (rows 73–78 and the header rule)
The header bottom rule is at y 130 on the plain board and y 123 on the collapsed board.

| Probe | ffdce86 | bd37f6c | c17372f |
|---|---|---|---|
| 45-card board, scrollTop 300 or bottom: y 75, mid-column x 200 | C2C2C2 divider | EBEBEB, no line (bug reproduced) | C3C3C3 |
| Same, at the separator x 390 | C2C2C2 | D7D7D7 | CDCDCD |
| Same, header rule y 130 at x 200 / x 390 | header not sticky; cards at y 76 | cards: FFFFE0 / E4E4E4, rule gone | C3C3C3 / D3D3D3 |
| 12-lane board, scrollTop 300 or bottom: y 75 across the corner (x 10–160) and separators 161 / 510 | C2C2C2 | EBEBEB/FFFFFF; at the bottom, lane-label ink bleeds through the corner at y 75–76 | C3C3C3, with CDCDCD at 161 and 510 |
| 12-lane board, header rule y 130 | — | lost | C3C3C3, with D3D3D3 at 161 and 510 |
| Header band y 70–130, at rest vs scrolled, on the same build (4 pairs) | — | 9.3k–10.3k px differ (header shifts up 1 px, rule lost) | 0 px. Only row 131, which is card content, differs |
| Narrow 1100x700, scrolled both ways (12-lane s300 + left 200; 45-card bottom + right) | — | rule lost | frame top and rule intact, crossings correct; header and cell lefts match (−31/193/417/641/865) |
| **Tall board with folded strips, scrollTop 300 or bottom: y 75 over the strips** | C2C2C2 | line gone | **line gone.** Right strips x 1844–1907 show F3F4F6/E5E7EB; left strip x 11–42 also gone |

The last row is the KF-358 failure. The strips' new `border-top` scrolls away with the strips, so the frame top breaks for 32 px per strip under the board bar. Evidence: `$S/verify-b3/crops/cmp-tallfold-tr.png` (rest / fix scrolled / prev scrolled) and `$S/verify-b3/out2-3315/tallfold-s300.png`.

### 2. At rest, c17372f vs bd37f6c (board region y ≥ 75)

| Board | Differing pixels |
|---|---|
| plain | 0 |
| collapsed | 0 |
| legend | 0 |
| Edit layout (y 75 = C2C2C2 on all three builds) | 0 |
| 12-lane | 0 |
| narrow: plain, plain scrolled right, legend, collapsed | 0 |
| 45-card | 646 |
| folded right | 481 |
| folded left | 483 |
| dark mode | 1.7k–2.2k (intended dark border changes) |

**45-card board (646 px).** 644 px fall in the In Review header box (x 1300–1518, y 87–99): the label, "+" and count "4" go from subpixel to grayscale antialiasing, max delta 109. The other 2 px are in the card area.
- The flip is deterministic: a repeat run gives exactly 644 px, and the header band is identical run to run.
- Grayscale labels are already common on scrolling boards. On bd37f6c, 3 of the 5 labels on this board are grayscale. On the 12-lane board every label is grayscale on all three builds. ffdce86 renders In Review grayscale on this board too.
- It never appears on a board that doesn't scroll (all the 0-px rows above).
- **Acceptable.**

**Folded boards (481 and 483 px).** These boards don't scroll, so the cause isn't the antialiasing flip:
- The Lane Two row moved 0.3 px (y 784.3 to 784.0), which changes the "Lane two card" glyph raster. It is subpixel antialiased on both builds.
- At y 75, the strip crossings are #D4D5D7 on the fix vs a continuous #C3C3C3 on bd37f6c (x 1844, 1875–1876, 1907 on the right board; x 11 and 42 on the left board).
- The board bar shows antialiasing noise of at most 5 levels.

These are minor, but they are not 0.

### 3. KF-358 tracker claims on c17372f (ffdce86 shows the original bug: EEF0F3 page, no frame)
- **Gutter:** 10 px F2F2F2 at x 0–9, x 1909–1918 and y 989–997. **Pass.**
- **Frame:** C3C3C3 at x 10, x 1908 and y 988. **Pass.**
- **Bodies:** white to y 987. **Pass.**
- **Separators:** E4E4E4 at x 390/770/1149/1529. **Pass.**
- **Crossings:** CDCDCD on the frame top, D3D3D3 on the header rule and on y 988. **Pass.**
- **Legend:** rule at y 965 (BBBBBB, x 10–1908), frame wrap ends at 965, bodies white to 964. Identical to bd37f6c. **Pass.**

### 4. KF-223 spot check on c17372f
- **Strip:** 27 px, x 479–505, separators at 478 and 506. **Pass.**
- **Header:** count only ("5"). **Pass.**
- **Name ink:** 12x54 at x 487–498, starting at y 152, which is 29 px below the rule at y 123. **Pass.**
- **Long name:** clipped with an ellipsis; the wrap scrollHeight stays 923, so it adds 0 scroll height. **Pass.**
- **Popover:** 377 px wide (x 56–433). **Pass.**
- **Popover placement: FAIL.** Its top now lands 1 px below the header rule because the sticky header's rect now includes that rule. That stacks a #BBBBBB rule (shadowed) on top of a #C3C3C3 popover border, a visible 2-px line.
  - Plain board: y 130–131.
  - Collapsed board: y 123–124.
  - Also on a scrolled board.
  - bd37f6c draws one C3C3C3 line, matching KanbanFlow (rule = popover top = y 117 in `kanbanflow-column-info-popover-20260930.png`).
  - Evidence: `$S/verify-b3/crops/cmp-pop-plain.png`, `$S/verify-b3/crops/cmp-pop.png`, `$S/verify-b3/inter2-3315/pop-plain.png`.

### 5. Break attempts the builder didn't run
- **Folded strips on a tall scrolled board:** frame top lost over the strips. **FAIL** (above).
- **Show details popover after the header change:** doubled line. **FAIL** (above).
- **Dark mode:** the #374151 borders form a closed frame (x 10, x 1908, y 75, y 988), and the header rule at y 130 stays #374151 when scrolled.
  - The bottom crossings stay light D3D3D3.
  - Bodies and the corner cell are white with E4E4E4 separators, while ffdce86 is dark. That was already true on bd37f6c, so it's an older regression in the KF-358 lineage. c17372f's hard-coded `.corner-cell {background:#fff}` has no dark override.
  - The board bar stays F2F2F2 in dark mode on all three builds.
- **Drag and drop on scrolled boards (fix and prev both OK):**
  - 12-lane board at scrollTop 300: Lane 5 To Do moved to Lane 6 In Review.
  - 12-lane board at the bottom: Lane 12 Backlog moved to Lane 11 To Do.
  - 45-card board at the bottom: Backlog card dropped mid-body in To Do and landed in To Do.
- **Sticky corner cell:** now opaque, so lane-label ink no longer bleeds through as it does on bd37f6c. The lane-label column has no header rule under the corner, at rest or scrolled, on both builds. That isn't new.

### Console and network
0 console errors, 0 page errors, 0 failed requests and 0 responses ≥ 400 on c17372f across 38 captures and both interaction runs.

### Observations (candidate defects; not caused by c17372f unless noted)
1. Dark mode on the KF-358 lineage paints white bodies and a white corner, with light card text on light cards (`$S/verify-b3/crops/cmp-plain-dark.png`). The corner part is entrenched by c17372f.
2. Folded strips: on the fix, the frame top over each strip crossing renders #D4D5D7 at rest. While scrolling, the strip labels and menus scroll away; that behaviour is older than this commit.
3. The lane-label column has no rule under the sticky corner cell, so scrolled lane labels slide under it with no divider.

All files are under `$S/verify-b3/` (= `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-b3/`): captures in `out-331x/`, `out2-331x/`, `inter-331x/`, `inter2-331x/` (each with results.json logs), crops in `crops/`, gate log in `gates.log`, capture scripts in `ctr/`, fixture scripts `fixtures.sh` and `fixtures2.sh`.

DONE: KF-358 FAIL frame top still scrolls away over folded swimlane strips on a scrolled board (y 75 x 1844-1907 / 11-42 lose the line), and the commit makes the column-details popover sit 1 px below the header rule; other claims and the plain/12-lane scrolled states pass
DONE: KF-223 FAIL c17372f regresses the Show details popover: its top border lands 1 px below the header rule, giving a doubled #BBBBBB+#C3C3C3 line on every board (bd37f6c and KanbanFlow draw one line); strip, count, name ink, ellipsis and 377 px width all pass
