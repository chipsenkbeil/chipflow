verify-f2 report: blind verification of fix/board-visual-f 271a281 against e6b2847

All six defects pass. I found no regressions, and no gate fails apart from the three tests already known to fail outside UTC. Evidence is in /private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-f2/ (screenshots in shots/, crops and diff masks in crops/, JSON logs as del-*.json, drag-*.json, sub-*.json, reg-*.json).

## Environment
- **Fix tree:** worktree .dev/worktrees/verify-f2, detached at 271a281. Release binary md5 a95e3c9b01aa74ebd717586270a99e65; the string `columnOverdue-text` occurs 3 times in it.
- **Baseline:** worktree .dev/worktrees/verify-f2-base at e6b2847. md5 294ccc22983e67ca81964a62eb69bb72; `columnOverdue-text` occurs 0 times.
- **Diff scope:** e6b2847..271a281 touches only src/routes.rs, static/app.js, static/style.css and templates/board.html. db.rs and models.rs are unchanged.
- **Servers:** fix on port 3374 with /tmp/chipflow-verify-f2-3374.redb, baseline on 3375 with /tmp/chipflow-verify-f2-3375.redb. Both are killed by PID and both ports are free.
- **Seed:** seed-golden-master.sh with `--as-of 2026-09-30`, so the dates are literal and the overdue counts match golden 02 (2 in To Do, 1 in In Progress). I then set grouping_date 2026-09-30 on the 3 Done tasks and turned legend_visible on.
- **Browser:** Playwright 1.63.0 headless Chromium, DPR 1, animations frozen, a fresh context per run, timezone America/Chicago. The host clock was 2026-10-16 20:15 CDT, so the server's "today" is 16 Oct.
- **Tools:** start.sh, mkb.sh, donescen.sh, mksub.sh, regions.sh and outside.sh in the folder above; Playwright scripts are r2/pw/vf2-*.mjs.

## Gates (fix tree)
- `cargo fmt --check`: exit 0.
- `clippy --all-targets`: 0 warnings, 0 errors (clippy.log).
- `TZ=UTC` plain libtest via /Users/senkwich/.cargo/bin/cargo: every suite passed (12, 73 + 1 ignored, 3, 13, 8, 6, 4, 2, 2, 2, 3, 0).
- Local timezone (known, not counted): 3 failures, `format_due_card_weekday_near_absolute_far`, `format_due_modal_relative_and_done_marker` and `statistics_best_day_is_null_without_pomodori`.
- jsc `checkSyntax`: exit 0. `node --check`: OK.
- Log: gates.log in the evidence folder.

## Per-claim results

**KF-311 (column names left-aligned)**
- On the baseline the names are centred: padding 42 px and the To Do ink at x 420.
- On the fix, at 1440x612, the name ink lands on golden 02's x for every column to within 1 px at thresholds 120, 180 and 210: Backlog 22/22, To Do 306 vs 305–306, In Progress 590 vs 590–591, In Review 874/874, Done 1158 vs 1157–1158.
- Golden 08 also matches to within 1 px for Backlog, To Do, In Review and Done. Golden 01 puts the name box 11 px inside the header; ours is too (box at x 21).
- The header is 37 px, the "+" is unchanged (x 258.6 and the other columns, 34x34), and "5 / 3" still sits 8 px after the name, exactly as on the baseline.
- The collapsed strip header shows no pixel change.

**KF-362 (overdue strip)**
- Bars are at x 305–387 and 588–663, y 146–148. Golden 02 has 304–387 and 588–663 (587–664 at the claimed edges), y 144–146 plus the 2 px offset.
- Our span box is 298.6,120,95.1x27; golden overdueInd is 298,119,95x27. The bar is the text box plus 1 px each side.
- Light colours are unchanged: #f3f3f3, #8b0000, #d7d7d7 rule, #ff8080 bar.
- Dark mode now gives a #444444 background, #ff8080 text and a #2b2b2b rule, matching KanbanFlow's theme variables. The baseline stayed light in dark mode with a centred 120 px bar.

**KF-241 (folded column name)**
- The rotated name's ink profile matches golden 08 row for row, shifted down 2 px. First ink is at y 131; golden's is 129.5, partly anti-aliased.
- The baseline's ink starts about 18 px lower. Ellipsis still works at 1440x380, and a long name fits.

**KF-365 (Done column groups)**
- I built 9 data shapes on both servers: empty; all done today; a mix of future (+3, +10), today, yesterday, -5, -16 and undated; past-only; future-only; an explicit grouping date of today; undated-only; undated plus today; and a swimlane board including dates ±365 days away.
- The fix shows exactly one "Today" header, always first, in every Done cell of every shape. The baseline repeated "Today" in the mix and swimlane shapes.
- Order is Today, then future groups newest first (26 Oct before 19 Oct), then past newest first, then "Completed" for undated tasks.
- Real mouse drags, each followed by a reload: within Today; within a future group; within Yesterday (which now sits after the future groups); from To-do into Today; from To-do onto a future group; and Today onto Yesterday. In every case the order after reload matched the order right after the drop. Each move returned 200 with 0 errors.
- A task dropped onto another day's group goes back to its own group (top of Today), which is the KF-387 behaviour.
- My judgement on the builder's choice: it is reasonable. I found no KanbanFlow evidence about future grouping dates, and the order is consistent with "Today pinned, otherwise newest first".

**KF-405 (deleting a non-empty column from the layout view)**
- On the baseline every move returned 400, the DELETE returned 400, the column stayed and the console showed 5 errors.
- On the fix, in the UI:
  - **Plain board:** B's tasks were appended to C at positions 3–6, in their original order. Positions in C are 0–6, the column is gone, the count stays 7→7, with 0 failed requests and 0 console errors.
  - **3-lane board:** each task stayed in its own lane, appended at the end, positions dense.
  - **Other cases:**
    - Target is the Done column: tasks are appended at positions 2–4, all under Today.
    - Deleted column is collapsed: works.
    - Empty column: a single DELETE, which returns 200.
    - Last remaining non-empty column: refused, with an empty target list and the toast "Choose a column to move the tasks to."
    - Last remaining empty column: deleted, leaving a board with no columns. The baseline behaves the same, so this is not new.
- I also checked that the server clamps a move position of 99 to the end with dense positions, so a stale count in the page cannot produce invalid positions.

**KF-364 (Hide label)**
- Clicking "2 hidden (2 done)" changes it to "Hide" and the title to "Click to hide done subtasks"; clicking again restores both.
- Unchecking a hidden done subtask while expanded keeps "Hide"; folding then shows "3 hidden (2 done)", which matches the rows actually hidden.
- Checking an open subtask while folded leaves the label unchanged.
- A card with 3 subtasks shows all three, and clicking the label does not open the modal.
- On the baseline the label disappeared once expanded, so the list could not be folded again.

## Regression
- **Pixel diff, full GM board:** fix vs baseline at 1905x1000, 1440x612 and 1240x850, light and dark, In Progress expanded and collapsed. Above y 88 and below y 156 the difference is 0 pixels (one 0.09 anti-aliasing speck on the avatar). The only other difference is the folded name, which is the intended change. Every difference region lies in the header names, the strip, or the folded name.
- **The same pixel diff for these states:** menu panel (KF-367: 250 px wide, #f7f7f7, full height), card Color submenu (KF-333: 8 tinted named rows), task modal, quick-add dialog (KF-302), mid-drag placeholder (KF-386, same as baseline), column menu (Edit, Collapse, Show details, so KF-405's earlier fix holds), overflow board (KF-385: lists scroll on their own, the board does not), swimlane board scrolled 456 px with the header fixed at y 82, and a swimlane board with a folded column and overdue tasks.
- **Behaviour checks:**
  - KF-403: quick-add lands at the bottom.
  - Cross-column drag persists.
  - KF-316: expanded columns without a WIP limit show no count.
  - KF-361/363/245/354/376/377: pixels unchanged.
- Console errors were 0, with 0 failed requests and 0 responses of 400 or above, on every fix-side page.

## Data
- No stored-format changes: db.rs and models.rs are not in the diff.
- I opened a copy of the baseline-written DB with the fix binary. The full API dump (boards, export-data, columns, colours, timer log; 334,393 bytes) is byte-identical to the baseline binary's dump.
- Rendered on both binaries, the same DB differs only in the header, strip and folded name.

## Observations (candidate defects; all LOW, none caused by this diff)
1. The API docs still describe Done drop order as "date groups, newest first", but Today is now pinned first. Affected: agent-guide.md:118, agents/skill.md:56, and the MoveTaskInput doc comment (src/routes.rs:2489), which feeds OpenAPI.
2. Done group labels have no year, so 2027-10-16 and 2025-10-16 both read "… 16 October" (shots/done-lanes-3374.png). The label code is older than this diff; the new ordering makes future dates easier to reach.
3. The page's subtask summary drifts from the server's (already true on the baseline). After a toggle, folding can hide an open subtask and show "3 hidden (2 done)", while a reload shows "2 hidden (2 done)" (sub-3374.json).
4. `.card-sub-more` has role=button and tabindex=0 but no keyboard handler, so Enter and Space do nothing.
5. Deleting the last empty column leaves a board with no columns, recoverable only through the layout view (same on the baseline). Evidence: shots/del-lastempty-after.png.
6. Dark mode in general (already true on the baseline): the board title, column names and WIP count are unreadable and the column body stays white, so the now-dark strip sits inside a white column (shots/b-3374-1440-dark.png).
7. Column separators sit 1 px off golden (295/1145 vs 294/1146), on both baseline and fix.
8. The seeded GM board shows no inline subtasks (prop_subtasks defaults to hide), while golden 02 shows them on the In Progress card. This looks like a gap in the seed script's column card properties.

Servers 3374 and 3375 are killed by PID. Both worktrees are left in place as evidence, with no tracked files changed.

DONE: KF-311 PASS name ink within 1 px of golden 02/08/01 in every column (fix 306 vs golden 305–306 for To Do); header 37 px, "+" and "5 / 3" positions identical to baseline; collapsed strip header unchanged; baseline centred.
DONE: KF-362 PASS bars at 305–387 and 588–663, y 146–148 (golden 304–387 and 588–663, y 144–146 +2); span box 298.6,120,95x27 vs golden 298,119,95x27; light colours unchanged; dark #444/#ff8080/#2b2b2b; baseline centred 120 px bar and light strip in dark mode.
DONE: KF-241 PASS rotated-name ink profile matches golden 08 shifted +2 (first ink y 131 vs golden 129.5); baseline 18 px lower; ellipsis still works.
DONE: KF-365 PASS one Today header, first, in all 9 data shapes and every swimlane cell (baseline duplicates it); 6 real drags within and into Done persist after reload; future-after-Today order judged reasonable (no KanbanFlow evidence either way).
DONE: KF-405 PASS plain, 3-lane, Done-target, collapsed-source and empty-column deletes all work (moves 200, positions dense at the end, lanes kept, count conserved, 0 errors); last non-empty column refused; baseline returned 400 on every move and the delete.
DONE: KF-364 PASS label reads "Hide" when expanded and the summary returns on fold; count follows checkbox toggles; baseline hid the label and could not fold.
DONE: REGRESSION PASS fix-vs-baseline pixel diffs at 1905/1440/1240, light and dark, are 0 outside the header/strip/folded name; menu, Color submenu, modal, quick-add (adds at bottom), drag, column menu, overflow scrolling, sticky swimlane header and folded swimlanes unchanged; 0 console errors and 0 failed requests.
DONE: DATA PASS db.rs/models.rs unchanged; baseline-written DB opened by the fix binary gives a byte-identical API dump.
DONE: GATE PASS fmt 0, clippy --all-targets 0 warnings, TZ=UTC cargo test all green (local timezone: the 3 known due/statistics failures, not counted), jsc checkSyntax exit 0.
