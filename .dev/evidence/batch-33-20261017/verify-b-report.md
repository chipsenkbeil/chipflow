## verify-b report: KF-358 and KF-223

**Verdicts:** KF-358 passes. KF-223 fails. Every KF-223 claim that can be measured against the references passes, but a break-attempt found a problem this fix introduces: the vertical column name in a collapsed column is not clipped to its cell. When the last row is short, the name runs out of the board frame and into the page gutter, and a long name adds a phantom scroll area.

The combined tree also does not compile its tests. The stream B commits add two required fields (`collapsed`, `column_name`) to `CellView`, and stream A's KF-359 test helper builds a `CellView` without them. Release builds still work.

### Environment
- Fix tree: `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-b`, which is origin/main d332e14 plus cherry-picks 4c5da04 → 3d2f46a and 4bc68f1 → ac7c1a5. Both applied cleanly. The diff touches only `src/routes.rs`, `static/app.js`, `static/style.css` and `templates/board.html`.
- Baseline tree: `.dev/worktrees/verify-b-base` (plain d332e14).
- Combined tree: `.dev/worktrees/verify-b-combo`, which is d332e14 plus e3dd656, 34397f9, 259033d, 4c5da04 and 4bc68f1, ending at HEAD 399c2dd.
- Binaries (in `$S/verify-b/`, where `$S` is the scratchpad folder `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`):
  - fix: md5 667fd19ee7ca4c8802f493957c545b2c
  - baseline: md5 15e1a61487530fa793254781533db562
  - combined: md5 f9a5391f076f3fe478e9f6ad95d4b130
  - `rg -a -c collapsedColumn-name` finds 2 matches in fix and combined, 0 in baseline.
- Instances:
  - Ports 3315 (fix), 3316 (baseline), 3317 (combined).
  - Each started with a fresh DB `/tmp/chipflow-verify-b-<port>.redb`, `TZ=America/Chicago` and `faketime '2026-09-30 15:22:00'`. The HTTP Date header read Wed, 30 Sep 2026 20:23 GMT.
  - Each seeded with `seed-golden-master.sh --as-of 2026-09-30`, with every column set to `prop_subtasks: show`.
- Capture:
  - Linux container `mcr.microsoft.com/playwright:v1.63.0-noble@sha256:eff16c…` at 1919x998, DPR 1, browser clock pinned with `context.clock`, animations frozen, a fresh context per scenario.
  - Interactions ran in host Playwright 1.63.0.
  - Differing pixels were counted with a difference composite and threshold, not `compare -metric AE`.
- Cleanup: all servers killed by exact PID, container removed, DBs deleted.
  - One process slip: `faketime` forks a child, so my first kill left two servers running on their old DBs. The interaction rerun therefore used those processes, with the same binaries and freshly seeded boards. All were killed by child PID at the end, and nothing is listening on 3315 to 3317.

### Gates
- **Fix tree:** all clean.
  - fmt rc 0, build rc 0.
  - clippy `--all-targets` rc 0 with 0 warnings.
  - `TZ=UTC cargo test`: 97 passed, 0 failed, including the new `collapsed_column_renders_count_header_and_body_name`.
  - jsc `checkSyntax` rc 0.
  - The release build's `unused import ServeDir` warning also appears on the baseline.
- **Combined tree:** one conflict, and the test build fails.
  - Cherry-picking 4bc68f1 conflicted in the `routes.rs` tests module, where both streams append tests at the same spot. I resolved it by keeping both blocks.
  - fmt rc 0 and build rc 0, but clippy `--all-targets` and `cargo test` both fail with rc 101: `E0063 missing fields collapsed and column_name in initializer of CellView` at `src/routes.rs:8529`, inside KF-359's `board_grid` test helper.
  - As an experiment I added `collapsed: col.collapsed, column_name: col.name.clone()` there. With that, clippy is clean and 102 tests pass. I then reverted it with `git checkout`, so the worktree is clean.
  - Whoever releases both streams together must make that change.

### KF-358: board frame and full-height column bodies

| Claim | KanbanFlow reference | Baseline | Fix | Verdict |
|---|---|---|---|---|
| 10 px `#F2F2F2` gutter, `#C3C3C3` frame at x 10 and x 1908 | yes | `#EEF0F3`, no frame | gutter x 0-9 and 1909-1918 `#F2F2F2`, frame x 10 and 1908 `#C3C3C3`; 0 of 9,180 px differ on each side | pass |
| Frame bottom at y 988, page below | 988, then 989-997 `#F2F2F2` | none | 988 `#C3C3C3`, 989-997 `#F2F2F2`; 0 of 17,271 px differ | pass |
| Frame top at y 80 | 80 | none | 75 | owned by KF-361 (board bar is 37 px instead of 46) |
| Separators `#E4E4E4` at x 390/770/1149/1529 (body), `#D7D7D7` in header | yes | none | exact | pass |
| Where a separator crosses a rule: top `#CDCDCD`, header rule and bottom `#D3D3D3` | yes | none | exact at all 4 separators | pass |
| Bodies white down to y 987 whatever the card count | yes | stop at about y 505, then `#EEF0F3` | region 0,560,1919,438: base 840,522 of 840,522 px differ; fix 0. Region 11,600,1897,388: base 736,036, fix 0 | pass |
| Below the header row overall | — | 1,315,930 of 1,769,318 px differ | 391,432 differ, all from cards and headers (KF-311, KF-361, KF-363 and others) | pass |
| Legend on: bodies end at the legend rule | rule y 965, gutter 966-974, legend from 975 | bodies end at 505 | bodies end at 964, rule at 965, legend at 975; geometry exact; 0 of 8,850 px differ in the left gutter | pass |

- In the legend-on case, the colours there still differ: the rule is `#BBBBBB` instead of `#C3C3C3` and the band is `#F0F0F0` instead of `#F2F2F2`. That is 1,668 px plus 15,012 px, already filed under KF-354.
- The 4 separator-crossing pixels on the legend rule show `#BBBBBB` where KanbanFlow has `#D3D3D3`.

Break-attempts:
- **Drop into an empty column, low in its body:** I dragged a card into the empty fifth column at y 800. The fix sends a move to that column. The baseline drops the card into the 40-card fourth column instead. The full-height bodies fix a real misdrop.
- **Narrow viewports:** at 1100x900 the board-wrap scrolls (`scrollWidth` 1292). After scrolling right, the frame is at x 1089 with a 10 px `#F2F2F2` right gutter. At 1440x900 the frame is at x 1429 with a 10 px gutter. The page itself never scrolls sideways.
- **Card counts:** an empty board has its body to y 987. With 40 cards, the frame grows and the wrap scrolls, keeping the frame bottom and gutter (column-level scrolling is KF-385).
- **Legend plus a tall board:** the legend stays fixed at y 965 and the frame meets it.
- **Dark mode:** gutter `#0F172A`, frame `#374151`. Column bodies stay white, which is the existing KF-373.
- 0 console errors and 0 failed requests in every fix scenario.

### KF-223: collapsed column strip and "Show details" popover

| Claim | KanbanFlow reference | Baseline | Fix | Verdict |
|---|---|---|---|---|
| Strip width | 27 px (x 479-505), separators at 478 and 506 | 21 px (474-494) | 27 px (479-505), separators 478 and 506 | pass |
| Strip header keeps the normal header height | other headers unchanged | every header grows to 144 px; menu collapse takes headers from 41 to 68 px | all headers equal (47 or 41); menu collapse leaves them at 41 | pass |
| Header holds only the count, `#111111`, 7x9 | count ink x 489-495, y 96-104 | 6x9 `#1F2937`, plus overdue text and the name | ink x 489-495, 7x9, `#111111`; text is "5" only | pass |
| Count is darkred over the WIP limit | yes | darkred | `rgb(139,0,0)` | pass |
| Name `#555555`, top to bottom, in the white body | ink 12x54 at x 487-498, mass 39,544 | in the header, bottom to top | ink 12x54 at x 487-498, mass 39,544 (same ink); starts 29 px below the header rule, as in KanbanFlow | pass |
| Overdue shows only a 3 px `#FF8080` line at the header bottom | line 21 px wide, inset 3 px | vertical red text | line 21x3 at x 482-502 with `#D3D3D3` crossings, matching the reference | pass |
| Strip body vs reference (B), offset 6 px | — | 17,722 of 24,998 px differ | 0 of 24,998 | pass |
| Two adjacent strips (C) | — | 32,561 of 49,134 | 6,136 of 49,134 | pass (only text antialiasing; ink boxes identical; KanbanFlow's own two captures differ in name ink mass, 39,304 vs 39,544) |
| Popover: 377 px box, `#C3C3C3` border, `#EEEEEE` band, centred `#666666` title | x 56-432, 67 px tall | 224 px rounded card | x 56-432, 67 px; band 30 px, rule `#D7D7D7`, body 34 px | pass |
| Popover text | title ink 106x12, count 71x9, close X 12x14 | — | title 106x12, count 71x9, close X 14x14; 1,026 of 25,259 px differ (antialiasing plus the 1 px wider X); border rows 0 differ | pass |

- The remaining 6 px vertical offset and the `#EBEBEB` header colour are owned by KF-311 and KF-361.
- **Clicking expands the column:** clicking the strip body and the strip header each send `PATCH {collapsed:false}` and reload. The baseline body click does nothing.
- **Failed requests fixed:** the baseline logs `net::ERR_ABORTED` on every collapse or expand PATCH. The fix logs none.

**Break-attempt that fails:**
- **Long name:** a long column name collapsed on a board with two lanes. The name span is 1,622 px tall and is not clipped. In the last row it paints over the frame bottom (y 988, `#565656`) and into the gutter (989-993). It also grows the board-wrap scroll height from 923 to 2,129 px.
- **Realistic version:** "In Progress" collapsed, a main lane with 25 cards and a short Lane Two. Scrolled to the bottom, "IN PROGRESS" runs about 51 px out of the frame into the gutter. In upper rows the name is cut off by the next row with no ellipsis.
- The baseline does not have this, because its name sat in the header with `max-height` and `overflow: hidden`.

Break-attempts that pass:
- **All five columns collapsed:** each strip stays 379 px wide with a 372 px warning line. The baseline does the same and no reference shows this state, so I'm listing it under observations.
- **Done column collapsed:** a 28 px strip at x 1880 inside the frame.
- **Over the WIP limit and overdue on the same collapsed column:** the count is darkred and there is one line. The tooltip reads "Overdue tasks: 1 / Total tasks: 5 / Click to expand".
- **Counts:** a two-digit count ("40") is 13 px wide and centred.
- **Popover placement:** under Done it is centred at x 1486. On the strip it is centred at x 304.
- **Narrow and dark:** strips look right at 1440 and 1100. In dark mode the strip is readable apart from the existing KF-373 header text.
- **Combined tree, swimlanes:** Default plus 1 lane and Default plus 2 lanes, each with the extra lanes folded and unfolded, plus 2 lanes with two collapsed columns. Every row cell lines up with its header and 0 cards sit outside their column. Each strip header holds only its count, and each row's collapsed cell holds only its own column's name. No console errors.

### Regressions and observations
- **Layout view loses its divider (low, new):** removing the board bar's `border-bottom` also removes the `#C2C2C2` divider at y 75 when Edit layout is open, because no frame is drawn there. `$S/verify-b/crops/sweep-R_layout.png`
- **Dropping a card on a strip (existing):** the drop is refused, but the card gets reordered within its source column, to position 3 on fix and position 1 on baseline. The strip has no tabindex, so it cannot be reached or expanded from the keyboard.
- **Filter on a collapsed column (low, new):** with a filter that drops the column's count to 1, at or under its WIP limit of 2, the count stays darkred (that part is pre-existing). The overdue line also stays, because it is driven by the server-side overdue count, while the baseline hides its line.
- **Narrow viewport with legend (existing):** at 1100 px, after scrolling right, the legend stays 1,100 px wide and scrolls with the content, leaving a 192 px gap at the bottom right.
- **Combined vs fix captures:** board region 0 px differ (B: 0 of 1,915,162). Board A has 81 px of differences in the top bar and the invite "+", outside the board.

### Evidence
All under `$S/verify-b/`:
- Captures and per-scenario logs: `shots/{fix,base,combo}-*.png` and `*.log.json`
- Crops: `crops/popover-full.png`, `crops/popover-x.png`, `crops/long-name-overflow.png`, `crops/tall-name-overflow.png`, `crops/sweep-*.png`
- Interaction results: `inter-fix.json`, `inter-base.json`
- Gate logs: `gates-*.txt`, `fix-*.log`, `combo-*.log`, `combo-patched-*.log`
- Scripts: `setup.sh`, `drive.mjs`, `scenarios.mjs`, `inter.mjs`, `filt.mjs`, `tools.sh`

DONE: KF-358 PASS — gutter, frame and full-height bodies match the 09-30 reference (0 px differ in gutters, frame lines and y 560-997); legend-on geometry is exact; baseline repro 840,522 px and a misdrop into the wrong column, both gone on the fix.
DONE: KF-223 FAIL — strip and popover match the references (strip body 0 of 24,998 px differ, same ink boxes, headers stay equal height), but the new vertical name is not clipped to its cell: it runs out of the frame into the gutter on a short last lane and adds up to 1,206 px of phantom scroll.
