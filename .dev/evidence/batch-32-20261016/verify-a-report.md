## verify-a report: KF-304, KF-334 (+KF-141), KF-359 (+KF-152)

**Verdicts:** KF-304 passes. KF-359 passes, and the single-Default case (KF-152) is unchanged. KF-334 fails. The move actions now work, so KF-141 is functionally fixed, but "Move advanced" and the submenu do not match the fresh KanbanFlow reference.

### Environment
- **Tree under test:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-a`, HEAD `f42d2a3`. It is origin/main `d332e14` plus cherry-picks of e3dd656 (now b994c3c), 34397f9 (now a897a1b) and 259033d (now f42d2a3). All three applied cleanly.
- **Files changed:** only the five expected: `src/routes.rs`, `static/app.js`, `static/style.css`, `templates/board.html`, `templates/board_delete.html`.
- **Baseline:** `.dev/worktrees/verify-a-base` at origin/main `d332e14`.
- **Binaries** (debug, served from snapshot copies):
  - `$S/verify-a/chipflow-fix` md5 `577fd946e047c9b7e7458daa94f250f3`
  - `$S/verify-a/chipflow-base` md5 `392194cad17da83f0223870baa3c91ce`
  - Strings unique to the fix: `swimlane-gutter` and `data-last-board` each appear once in the fix binary and zero times in the baseline.
- **Ports and DBs:** fix on 3305 (`/tmp/chipflow-verify-a-3305.redb`), baseline on 3306 (`/tmp/chipflow-verify-a-3306.redb`). Both seeded fresh with `seed-golden-master.sh --as-of 2026-09-30` plus scenario boards. Both servers were killed by PID afterwards.
- **Browser:** Playwright 1.63.0 Chromium at 1905x1000, DPR 1, a fresh context per scenario, animations off.
- `$S` = `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`. All evidence is under `$S/verify-a/`; JSON is named `{lanes,ctx,del,regress,last}-{base,fix}*.json` and screenshots are in `shots/`.

### Gates (fix tree; every cargo command used `--config 'build.build-dir="target/build"'`)
| Gate | Result |
|---|---|
| fmt --check | exit 0 |
| build | ok |
| clippy --all-targets | 0 warnings |
| `TZ=UTC cargo test` | all ok (10/50/3/12/8/6/4/2/2/1/3/0) |
| jsc checkSyntax `static/app.js` | exit 0 |

The five new tests ran and passed: `board_delete_page_flags_only_the_last_board` plus the four `board_*lane*` tests.

### KF-304: delete a board from `/b/:id/settings/delete`
| Claim / check | Baseline | Fix | Verdict |
|---|---|---|---|
| Clicking "Delete board" opens a dialog | `ReferenceError: showConfirmDialog is not defined`, no dialog, no request | Styled `#confirm-dialog` 420x144 titled "Delete board", text `Delete the board "DelA"? This cannot be undone.`, focus on Delete | reproduced, fixed |
| Cancel | n/a | closes, board kept | ok |
| Escape | n/a | closes | ok |
| Confirm deletes | n/a | DELETE returns 200, page goes to `/boards/new`, board gone from `/api/boards` and from the board page HTML | ok |
| Visiting the deleted board's URLs | n/a | `/b/:id` and `/b/:id/settings/delete` both return 404 "board not found" | ok |
| Last remaining board | ReferenceError | Inline "Could not delete board: cannot delete the last board", no dialog, no request; the API also refuses (400) | ok |
| Archive | works | works, lands on `/boards/archived` | ok |
| Console errors / failed requests | 1 page error | 0, apart from the 404s I triggered on purpose | ok |

Evidence: `del-base.json`, `del-fix.json`, `last-*-solo.json`, `shots/del-fix-dialog.png`.

Break-attempts:
- **Board deleted in another tab, then Confirm:** the page shows "Could not delete board: board not found" and the button re-enables.
- **Board name with quotes and HTML** (`Q "quoted" <b>bold</b> & amp`): rendered as literal text, no markup injected.
- **Overlay click:** does not dismiss the dialog. That is a design note, not a failure.

### KF-334 (+KF-141): card context menu and Move submenu, judged against `kf-reference/card-context-menu-20261016.md`
| Check | KanbanFlow reference | Baseline | Fix | Verdict |
|---|---|---|---|---|
| Main menu order and icons | Start timer, Move ▸, Color ▸, Assign members, Copy here, Task URL, Delete, each with an icon | same | same | ok |
| Submenu items | Move right, Move to bottom, Move advanced | only "Move advanced" | all three | ok |
| Submenu icons | each item has an icon | none | none on all three (`icon:false`) | **deviation** |
| Move right | moves to the next column | nothing happens | POST /move; card goes to column 1 (bottom), same lane; works into a collapsed column and inside a swimlane | ok |
| Move to bottom | bottom of the card's own column | nothing happens | rank 0 of 4 becomes rank 3 of 4, same lane; on a card already last it is a harmless no-op POST | ok |
| Move to bottom in the last (Done) column | own column, so it should be offered | absent | **absent**: only "Move advanced" is shown, because it sits inside the `has a column to the right` branch | **deviation** |
| Move advanced | a "Move task" popover anchored at the card, with title, × close, Board select, Column select and a green full-width Move button; "NOT an inline list of columns inside the submenu" | timeout, nothing moves | the submenu is replaced in place by a plain list of the five column names; no title, no ×, no Board or Column select, no Move button | **FAIL** |
| Move advanced moves the card | n/a | no | yes: POST /move to the picked column, lane kept (also checked on a 3-lane board) | functional |
| Escape / click-away close | n/a | ok | ok | ok |
| Console errors / failed requests | n/a | 0 | 0 | ok |

Evidence: `ctx-base.json`, `ctx-fix.json`, `shots/crop-fix-ctx-all.png` (submenu, advanced list, last-column submenu), `shots/ctx-fix-move-advanced.png`.

A matching dialog already exists: `templates/board.html:987` `#move-task-dialog` has the "Move task" title, × close, a Board select and a green Move button (its Column field is static text, not a select). It is used by the task modal and the "V" shortcut, and the fix does not use it.

### KF-359 (+KF-152): Default lane row alignment
Each permutation was measured as the x-offset (dx) and width difference (dw) of every row cell against its column header, plus a check that every card sits inside its own column header.

| Board | Baseline | Fix |
|---|---|---|
| Default + Lane Two, both with tasks | Default row has no label cell; 5/5 cells misaligned; 22/25 cards under the wrong header | blank gutter cell; 0 misaligned; 0/25 cards outside |
| 3 lanes, all with tasks | 5 misaligned, 22/25 outside | 0 misaligned, 0/25 outside |
| Default + empty (folded) Lane Two | corner cell present, 5 misaligned, 22/22 outside | no corner cell, 0 misaligned |
| 3 lanes, one folded | 5 misaligned | 0 misaligned |
| Default folded, two other lanes with tasks | aligned | aligned, pixel-identical (AE 0) |
| Default renamed to "Main" + Lane Two | aligned | identical (AE 0) |
| Only lane renamed to "Main" | corner + "Main" label | identical (AE ≈0) |
| Collapsed column | aligned | identical (AE 0) |
| Second lane also named "Default" | misaligned | aligned |
| KF-152: lone Default lane, golden-master board | — | pixel-identical to baseline (max channel delta 2/255, confined to the top bar at x 1646–1780, y 2) |

Evidence: `lanes-{base,fix}.json`, `lanes-{base2,fix2}.json`, `shots/lanes-fix-LANES3.png`, `shots/diff-lanes-*.png`.

Break-attempts: the renamed and only-renamed lanes, a second lane named "Default", and the folded main lane. The swimlane ⋮ menu on a labelled lane still opens (Rename / Move up / Move down / Delete).

Caveat: the tracker's fix direction says "never key layout on the lane's name", and the fix still decides which lane is the main one with `lane.name == "Default"` (`MAIN_SWIMLANE_NAME`). Visible effects, all present on the baseline too:
- A lane the user names "Default" renders as an unlabelled row with no ⋮ menu.
- A renamed main lane gets a label.
- A lone renamed lane still shows the swimlane column, unlike the lone-Default case covered by KF-152.

### Regressions from making `[data-close-dialog]` global
- **Board page:** I opened every overlay that has a `[data-close-dialog]` (14 of them) and each closes from it. The real openers also behave: card Delete → Cancel keeps the card, column Edit → Cancel, Assign members → Done, `?` → shortcuts → Close, and the task modal opens and closes.
- **Other pages:** `/settings`, `/boards/new`, `/boards/archived`, colors, timer log and timer stats all load with 0 console errors and 0 failed requests.
- **Side effect worth recording:** on the baseline, Cancel in the `/settings` API-token Revoke dialog does nothing (`afterCancel: true`). On the fix it closes, and the token is kept until you confirm. Confirming still revokes.

No regressions found.

### Possible new defects (not caused by this diff)
1. **Parent menu disappears when a submenu opens** (baseline and fix): clicking Move ▸ hides the card menu, so only the submenu stays visible (`shots/crop-fix-menu-vs-sub.png`). The KanbanFlow reference does not say whether the parent stays open, so this is unconfirmed.
2. **A user can end up with zero active boards:** the last-board check counts archived boards. With one active board and one archived board, deleting the active one is allowed, and `/` then redirects into the archived board (`last-fix.json`, `last-base.json`).
3. **Delete page differs from KanbanFlow** (`golden-masters/kanbanflow-delete-board-dialog-20260929.png`): KanbanFlow's page is itself the confirmation, titled "Delete board: <name>" with pink alert text and Cancel on the left, Delete board then Archive board on the right. ChipFlow's text and button order differ and it adds a second dialog. Whether KanbanFlow shows a second confirm can't be told from the static golden.
4. **A board whose lanes are all empty renders a table with no rows**, only folded strips (baseline and fix).

DONE: KF-304 PASS styled confirm works end-to-end (baseline ReferenceError reproduced); Cancel/Escape, confirm-delete with 404 and list removal, last-board refusal, archive, deleted-elsewhere and quoted-name probes all correct; 0 console errors
DONE: KF-334 FAIL Move right / Move to bottom / Move advanced now move cards (KF-141 fixed), but Move advanced is an inline column list instead of KanbanFlow's "Move task" popover (Board/Column selects, × close, green Move button), submenu items have no icons, and Move to bottom is missing in the last column
DONE: KF-359 PASS every row aligns under its header in all 11 lane permutations (baseline 5/5 cells misaligned reproduced); lone-Default board (KF-152) pixel-identical; caveat: main lane still identified by the name "Default", contrary to the fix direction
