## verify-g2 report: fix/dark-settings at 7e8de26 (405da66 plus 043a276 and 7e8de26)

**Result: KF-373 PASS. KF-368 PASS, with one caveat.** A 100-character name with no spaces still makes the delete page scroll sideways. The overflow comes from the bold board name inside the alert paragraph, not from the title. Production code d230d76 overflows worse on the same name, so this tree did not cause it. I filed it as an observation, not a blocker. If you read "no sideways scroll for any valid 100-character name" strictly, KF-368 is a FAIL; the numbers are below.

### Environment
- **Worktrees:**
  - fix: `.dev/worktrees/verify-g2`, detached at 7e8de26.
  - comparison: `.dev/worktrees/verify-g2-prev` at 405da66.
  - light baseline: `.dev/worktrees/verify-g2-main` at d230d76.
- **Diff 405da66..7e8de26:** only `static/style.css`, +13/−2 lines.
  - `body.dark .swimlane-menu` gets white text at opacity 0.7, with a hover background of white at 0.08.
  - `body.dark .layout-arrange-group h3` gets #eeeeee.
  - `.bshell-header-centered h1` gets `flex: 0 1 auto; min-width: 0; padding: 7px 15px; text-align: center; overflow-wrap: anywhere`.
  - `.bshell-header-centered` is used only by `board_delete.html`.
  - The class `.swimlane-menu` does not match the `#swimlane-menu` popup, which is an id.
- **Release binaries (build-dir target/build):**

  | Binary | md5 | The three new CSS strings |
  |---|---|---|
  | fix | 46a74eacf213279dc5b6b8b09da5f52c | each found once |
  | prev | 19ddead466ea00d7395ae4005e1001ec | none |
  | main | 92c3b8a287cca7d44ba263c8f76c2a35 | none |

  - The strings searched with `rg -a -c` were `body.dark .layout-arrange-group h3`, `body.dark .swimlane-menu {` and "A long board name wraps inside the box".
  - The served `/static/style.css` sha256 (f3e310eb…) matches the tree.
- **Instances:**
  - fix on 3372 with `/tmp/chipflow-verify-g2-3372.redb`.
  - prev on 3373 with `/tmp/chipflow-verify-g2-3373.redb`; afterwards main on 3373 with `/tmp/chipflow-verify-g2-main-3373.redb`.
  - Every run passed `--database-path`.
  - fix and prev were seeded with seed-golden-master.sh plus the same board shapes (lanes with folded strips, twelve lanes, tall, collapsed, legend) and the same delete-test boards.
  - All servers are killed by PID and my container is removed.
  - `~/Library/Application Support/chipflow/chipflow.redb` was not touched (mtime 15:47, before my runs).
- **Capture:**
  - Host Playwright 1.63.0 at 1905x1000, DPR 1, a fresh context per scenario, animations off.
  - Linux container (the pinned image) at 1919x998 for the light diffs and 1920x1000 for the KanbanFlow delete reference.
- **Evidence:** `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/vg2/`, which holds `shots/` (dark-3372.json, dark-3373.json, surfaces-3372-3373.json, del-3372.json, del-3373.json), `lin-3372-light/`, `lin-3373-light{,-run2}/`, `set-3372/`, `set-3373/` and `gates.log`. The scripts are `r2/pw/vg2-*.mjs`.

### Gates (fix tree)
- fmt: exit 0.
- build: exit 0.
- clippy `--all-targets -D warnings`: exit 0.
- `TZ=UTC cargo test`: all pass (70 + 55 in the other suites, 1 ignored).
- jsc checkSyntax: exit 0.
- **Empty environment:** 2 failures, `format_due_card_weekday_near_absolute_far` and `format_due_modal_relative_and_done_marker` ("got Sunday/Saturday 12:00 PM").
  - d230d76 fails the same 2 tests with the same output in the same environment.
  - The diff touches no Rust.
  - This is the open timezone-test issue; the fix/tz-tests branch (940ef11) is not merged.
- **Console errors on the fix: 0. Failed requests: 0.** This covers the dark sweep, all 14 surfaces, the delete pages at 4 widths × 5 names, the flows and the container runs.

### KF-373 (dark layout editor headings and lane icons)
| Claim | 405da66 | 7e8de26 | Verdict |
|---|---|---|---|
| "Columns" / "Swimlanes" heading colour | #1f2937 on #363636 (sampled peak 54, mode #1F2937), 1.21:1 | #eeeeee (sampled peak 238, mode #EEEEEE), 10.4:1; same as the "Layout:" h2 | ok |
| Lane "⋮" on a lane label (#4a4a4a) | #6b7280, sampled peak 113, about 1.8:1 | white at 0.7, sampled peak 200 (#c8c8c8), 5.3:1 | ok |
| Lane "⋮" on folded strips (#3e3e3e) | sampled peak 108 | sampled peak 192, 5.9:1 | ok |
| Consistent with the KF dark capture | — | KanbanFlow's dark layout-editor column "⋮" peaks at **192 on #3e3e3e**; ours is **192 on #3e3e3e**, an exact match | ok |
| Lane icon hover | light hover box #e5e7eb (glaring) | #545454 (8% white × 0.7); psv `boardLayout-iconHoverBgColor` is hsla(0,0%,100%,.08) | ok |
| Dark sweep vs 405da66 | — | 0 px on gm, collapsed, legend, tall, tall-scrolled, column popover, WIP strip, collapsed-column WIP and Done groups | ok |
| Where the dark diffs are | — | lanes 42 px, twelve 174 px, hover 441 px, all inside the `.swimlane-menu` boxes (0 outside); layout editor 1177 px, all inside the h3 boxes (x 40–584, y 253–267) | ok |
| 14 surfaces (modal, menus, Boards dropdown, filter, timer, timer settings, timer log, bell, account, reports menu) | — | 0 px in light and dark | ok |

- The modal scenario shows 4 px at max delta 2 on a card corner under the overlay; the modal element itself is 0 px. Same class as the capture noise below.
- The Boards dropdown differs (26238 px) because board order follows the board UUIDs, and the API order differs between the two instances. It is the data, not the code.

### KF-368 (delete page, long name)
| Claim | 405da66 | 7e8de26 | Verdict |
|---|---|---|---|
| 99-character name with spaces at 1905 | h1 x 369–1536 over box 653–1253, 1 line | wraps to 3 lines, text x 711–1195, inside the box; no scroll | ok |
| Same at 1100 / 700 / 420 | starts at x −34 / −234 / −374; scrollWidth 1134 / 934 / 794 | 3 / 3 / 4 lines inside the box; scrollWidth equals viewport; scrollX stays 0 | ok |
| Hostile 100-character name | overflowed (x −293 at 700) | wraps; no scroll; source is escaped (`&lt;script&gt;`, `&quot;`, `&#x27;`, `&amp;amp;`); 0 injected nodes; `window.__pwned` unset | ok |
| 100 fullwidth "Ｗ" | x −823 at 700 | 5–7 lines inside the box; no scroll | ok |
| Short name ("Dale fidelity test") vs 405da66 | — | **0 px at 1905, 1100, 700 and 420** (host); 0 px at 1920 (container) | ok |
| Short name vs the KanbanFlow reference (1920x1000, container) | title band 0, gap 0, buttons 0 | **title band 0/25200, gap 0/9000, buttons 0/23400** | ok |
| Alert vs the reference | AA only, max 16 | same | ok |
| Delete flow | — | the confirm opens; Cancel keeps the board; OK deletes it and goes to /boards/new | ok |
| Archive flow | — | goes to /boards/archived and the board is listed | ok |
| Cancel link | — | goes to /b/<id>/settings | ok |
| 124-character name (POST accepts it) | — | title box 600x92, no scroll | ok |

### Light theme unchanged
- **Container (1919x998), fix vs 405da66:** 0 px on collapsed, legend, lanes, layout editor, modal, filter, timer and column details.
- **Small diffs in the first prev capture:** plain and twelve 48 px, gm, Menu, card menu and column menu 59 px, max delta 2, all on the anti-aliased corners of the top-bar buttons.
  - A second capture of 405da66 differs from its own first capture by the same 48/59 px.
  - The fix against that second prev capture is **0 px** on gm, plain, twelve, card menu, collapsed, legend and lanes.
  - So this is capture noise, not the code.
- **Host light scenarios:** 0 px on every board shape, the layout editor and the lane hover and menu.

### Break-attempts
1. **Unbreakable 100-character name** ("W"×100):
   - The title wraps inside the box (5 lines).
   - The page still scrolls sideways (scrollWidth 1916/1513/1313/1279 at 1905/1100/700/420). The overflowing element is the `<strong>` holding the board name in the first alert paragraph.
   - d230d76 is worse on the same name (2353 at 1905, 1750 at 700), so the overflow already exists in production. 405da66 also overflows there.
   - Evidence: `shots/3372-ovf-wide-700.png` and `shots/3373-del-3373-wide-1905.png`.
2. **Fullwidth CJK-width name, hostile markup and template syntax, a 124-character name, 420 px:** all handled (see the KF-368 table).
3. **Lane icon on hover, with the lane menu open, and on folded strips on both sides; layout editor on the 12-lane board:** all readable, and the diffs stay inside the expected boxes.

### Observations (candidate new defects)
1. **Delete page alert body:** a board name with no break points overflows `.kf-alert p strong` and the page scrolls sideways. This exists in production (d230d76) and in 405da66. A fix would be to let the alert text wrap anywhere (for example `overflow-wrap: anywhere` on the alert).
2. **Hover background on the dark lane icon is about 4 levels dimmer than KanbanFlow's.** We draw #545454 because the 0.08 white hover sits under the button's 0.7 opacity; KanbanFlow would give about #585858. No capture exists to confirm it.
3. **Not from this tree, already seen by verify-g:**
   - `POST /api/boards` accepts names over 100 characters.
   - Deleting a board while others remain redirects to /boards/new.
4. **Not from this tree:** the empty-environment timezone tests fail on production too.

DONE: KF-373 PASS dark layout headings #eee (10.4:1, were 1.21:1) and lane ⋮ white@0.7 (5.3-5.9:1, peak 192 on #3e3e3e = KF's dark ⋮ exactly); dark sweep 0 px vs 405da66 outside those boxes; light unchanged
DONE: KF-368 PASS 100-char names wrap inside the 600px box at 1905/1100/700/420 with no scroll (405da66 overflowed to x −234 at 700); short name 0 px vs 405da66 and 0 px vs KF title band/buttons; hostile chars escaped; delete/archive/cancel work. Caveat: a spaceless 100-char name still scrolls sideways via the alert's bold name, which also happens on production d230d76 (filed as an observation)
