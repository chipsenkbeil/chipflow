## verify-a2: KF-334 (and KF-141), with regression checks on KF-304 and KF-359

**Verdict: KF-334 PASS.** Every item in the KanbanFlow reference checks out on the fix, and the baseline reproduces the bug. One server gap is still open: it predates this diff and was not caused by it (candidate C1 below).

### Environment
- **Fix tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-a2`, HEAD 821bbef. That is origin/main fe23cc2 plus the cherry-picks e3dd656, 34397f9, 259033d and 12ea131, which applied cleanly.
- **Baseline tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-a2-base` (fe23cc2).
- **Files in the diff:** agent-guide.md, agents/skill.md, src/db.rs, src/routes.rs, static/app.js, static/style.css, templates/board.html, templates/board_delete.html, tests/task_move_position.rs. All are expected for KF-304, KF-334 and KF-359.
- **Binaries** (debug builds, copied to the scratchpad):
  - fix md5 52c5499ca3e127ceef7563adbc2815d6
  - base md5 a85e0645f9faf63d6481bcb507fa02dc
  - `rg -a -c 'lane_label_column|swimlane-gutter'` finds 2 in the fix binary and 0 in base.
- **Instances:** fix on port 3307 with `/tmp/chipflow-verify-a2-3307.redb`, baseline on 3308 with `/tmp/chipflow-verify-a2-3308.redb`. Both were fresh DBs, seeded with seed-golden-master.sh plus my own fixture boards "Lanes" and "Target" (`$S/verify-a2/fixtures.sh`). Both servers are now stopped by PID.
- **Browser:** Playwright 1.63 Chromium at 1905x1000, DPR 1, animations frozen, a fresh context per scenario. The edge-case scenario ran at 1100x700.
- **Evidence:** `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-a2/` (out-fix/, out-base/, results.json, extra.json, api-cross-3307.txt, api-cross-3308.txt, pw/kf334.mjs, pw/extra.mjs).

### Gates (fix tree)
- fmt --check: rc 0
- build: rc 0
- clippy --all-targets: rc 0, 0 warnings
- `TZ=UTC cargo test`: rc 0, 102 passed, 0 failed
- jsc checkSyntax on static/app.js: exit 0

### KF-334 / KF-141: claim, baseline, fix

| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| Card menu items, in order, each with an icon | Same 7 items | Start timer, Move ▸, Color ▸, Assign members, Copy here, Task URL, Delete; all 7 have an svg | OK |
| Move submenu items | Only "Move advanced" (bug reproduced) | Move right, Move to bottom, Move advanced; all 3 have an svg | PASS |
| Parent menu stays visible while the submenu is open | Parent hidden | Parent at x 1333–1517 and submenu at x 1513–1697 are both visible, side by side (they overlap by 4 px); see out-fix/s01-submenu.png | PASS |
| Move right | Not offered | In Review card ends up last in Done; request is `POST /move` with position 3 | PASS |
| Move right in the last column | n/a | Not offered: the Done submenu shows only Move to bottom and Move advanced | PASS |
| Move to bottom = bottom of the card's own column | Not offered | In Progress first card goes to last; x stays 768–1136 and y goes from 147 to 282 | PASS |
| Move to bottom on the last card | n/a | Order unchanged | PASS |
| Move to bottom in Done | n/a | First Done card goes to last; the last card is a no-op | PASS |
| Move advanced opens a "Move task" popover at the card | Columns fill a hidden submenu, so no dialog appears (bug reproduced) | Popover at x 1149–1409, y 178–420, directly under the card (card y 147–174, left edge 1149). Overlay is transparent. It has the title "Move task", a × button, a Board select (current board) and a Column select (current column). The Move button is green rgb(22,163,74), 229 px wide inside a 260 px popover, i.e. full width | PASS |
| Advanced move within the board | n/a | Card moves to the end of To Do | PASS |
| Advanced move to another board | Without a lane, the card lands in a lane from the source board and is invisible on the target board (API cases A and D) | The Column list switches to Target's columns, and "In Progress" is preselected because the name matches. The card lands in Target's Default lane, visible at x 630–1242 under the In Progress header (624–1249) | PASS |
| `POST /api/tasks/{id}/move` across boards | See row above | Without swimlane_id, or with null, the card goes to the target board's first lane. With the target's own lane ("Second") it is honoured. Same-board moves keep the lane. Moving back to the source board takes its Default lane | PASS |
| Swimlane board | n/a | Move right from Lane Two stays in Lane Two. Move to bottom stays within its lane. An advanced move on the same board keeps Lane Two. A cross-board advanced move from Lane Two is visible under Zeta | PASS |
| Collapsed columns | n/a | Move right into a collapsed Done works, and the header count updates to 5. An advanced move into a collapsed In Progress works | PASS |
| Task-modal Move button and the V shortcut | Column is a static SPAN | Both open the same dialog, centred with a dimmed overlay (rgba(0,0,0,.55)) at x 743–1163. Column is a SELECT. Moving closes the modal and the card lands in the chosen column | PASS |
| Closing | n/a | Escape (also from inside the select), ×, and a click outside all close the popover. A click inside keeps it open. Escape or an outside click with the submenu open closes both menus. Board state is unchanged afterwards | PASS |
| Keyboard focus | n/a | Focus lands on the Column select, Tab goes to Move, and Enter moves the card | PASS |
| Console | 0 errors | 0 console errors and 0 page errors in all 14 scenarios plus the extra checks | PASS |

**KF-274 conflict:** the fix shows Column as a select, in both the popover and the task-modal dialog. That matches the 10-16 live reference but contradicts KF-274/GM-087 ("static label"). As instructed, I did not fail KF-334 on this point. The tracker entries need reconciling.

**Parent-menu reference:** `.dev/evidence/kf-reference-20261016/` contains no card-menu screenshot, only the .md file. The side-by-side claim therefore rests on your live capture; I did not open KanbanFlow.

### Break-attempts
1. **Stale popover target:** opened the popover on card A, pressed Escape, opened card B's modal, pressed V. The dialog was centred, not a popover, and the move went to B; A was untouched.
2. **Board race:** switched Board to Target and straight back. The Column list shows this board's 5 columns with the original column selected.
3. **Viewport edge at 1100x700:** the submenu flips to the left and the popover is clamped inside the viewport (832–1092 × 338–580).
4. **Second right-click with the submenu open:** the new menu opens and the old submenu closes.
5. **Cross-board API with the source board's lane (case C) or a nonexistent lane "nope" (case G):** both return 200, and the card is stored with a lane from the wrong board, so it does not appear on the target board. This is identical on the baseline, so it predates the diff (candidate C1).
6. **Double-click Move:** two identical POSTs are sent, but only one card ends up on the target, in a visible lane. The second reload produced one aborted navigation (`net::ERR_ABORTED`).

### Regressions (short checks)
- **KF-304:** on a throwaway board, the styled confirm shows the text 'Delete the board "Throwaway X4"? This cannot be undone.'
  - Cancel hides it and the board remains.
  - Delete sends one DELETE, redirects to /boards/new, and the board is gone.
  - No native dialogs and 0 console errors. Still PASS.
- **KF-359:** on the Lanes board (Default plus Lane Two), the header has a corner cell and 3 columns, and both rows have 4 cells.
  - The main row starts with swimlane-gutter.
  - All 8 cards sit inside their own header's x range.
  - Still PASS.

### Candidate new defects
- **C1 (MEDIUM, predates this diff):** `POST /api/tasks/{id}/move` accepts a swimlane_id that belongs to another board, or does not exist, and returns 200. The card is then not shown on any board. Evidence: api-cross-3307.txt and api-cross-3308.txt, cases C and G.
- **C2 (LOW):** the Move task popover does not trap focus. Tab after the Move button goes to BODY and then into the board header. Evidence: s05 tabOrder in out-fix/results.json.
- **C3 (LOW):** the Move button is not disabled while its request is in flight, so a double-click sends duplicate POSTs.
- **C4 (unverified against KanbanFlow; the reference is silent):**
  - Hovering "Move" does not open the submenu; it needs a click.
  - Opening the card menu leaves focus on the card.
  - Move right appends the card to the end of the next column.
  - An advanced move into the card's own column sends it to the bottom.

DONE: KF-334 PASS submenu (Move right/Move to bottom/Move advanced, parent menu stays open), popover, cross-board lane rule, swimlanes, collapsed columns, modal Move and V all verified with numbers; baseline reproduces the bug; 0 console errors
DONE: KF-304 PASS regression check: styled confirm, Cancel and Delete work, 0 console errors
DONE: KF-359 PASS regression check: every row has the corner cell's column and all cards sit under their own headers
