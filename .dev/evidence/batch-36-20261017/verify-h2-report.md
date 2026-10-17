verify-h2 report: fix/board-h e60efda vs baseline 68c9897

## Environment
- Worktrees (detached): fix `.dev/worktrees/verify-h2` @ e60efda, baseline `.dev/worktrees/verify-h2-base` @ 68c9897, production `.dev/worktrees/verify-h2-prod` @ d230d76. All three are debug builds with `--config 'build.build-dir="target/build"'`.
- Binary md5: fix 4ea84e803a5fd540902d55d32b6198a2, baseline 7bdae032ce3c5741e867e9a362bcf6d4. `rg -a -c filter-bookmarks-toggle` finds 1 hit in the fix binary and 0 in the baseline.
- Ports: fix 3376, baseline 3377. Port 3377 was reused one after another for the production and data checks.
- DBs: `/tmp/chipflow-verify-h2-{3376,3377,prod,prodcopy,basecopy,fixcopy}.redb`, each seeded with `seed-golden-master.sh`.
- Browser: Playwright 1.63 Chromium, DPR 1, animations frozen, a fresh context per scenario. Viewports 1440x612 and 1905x1000, light and dark.
- Diff scope is as expected: agent-guide.md, db.rs, models.rs, routes.rs, app.js, style.css, board.html, task_card.html.
- All servers were killed by PID. `lsof` shows no listener on 3376 or 3377.
- Evidence is in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-h2/` (JSON there, screenshots in `shots/`, dumps in `data/`). The harness is in `.../scratchpad/r2/pw/vh2/`.

## Gates (fix tree)
- `fmt --check`: exit 0.
- `clippy --all-targets -- -D warnings`: exit 0, 0 warnings.
- `env -i TZ=UTC /Users/senkwich/.cargo/bin/cargo test`: exit 0, 126 passed, 0 failed, 1 ignored.
- jsc `checkSyntax(app.js)`: exit 0.
- Logs: `gates.log`, `test-utc.log`, `clippy.log`.

## KF-215 filter panel: PASS
All numbers are at 1440x612 unless noted. JSON: `fix-215-geom.json`, `base-215-geom.json`, `fix-215f.json`.

| Claim | Golden | Baseline | Fix |
|---|---|---|---|
| Panel box | 1190,34,250x578 | 1190,36 | 1190,36,250x576 (+2) |
| Header funnel, pencil, Menu | covered by the panel | moved left to x=1004 and 1088, not covered | stay at 1254, 1296, 1338; `elementFromPoint` hits the panel (covered) |
| Separator | y 356 | none | y 358, 1px, #e6e6e6 |
| Remember row | y 364, height 34 | y 352, height 37 | y 366, height 34 |
| Bookmarks footer | 1190,566,250x46 with chevron | box present, no chevron | 1190,566,250x46, chevron at 1413,585 |

- **1905x1000:** panel at 1655,36; separator y 358; Remember row 366 / 34 px; footer 1655,954,250x46; all three header controls covered.
- **Colours:** pixel samples match the golden. Panel background 247 vs 247, header 236 vs 235, footer 239 vs 239.
- **Dark mode:** with body.dark the panel stays light (rgb 247,247,247), as in dark/11. On the baseline it is rgb(17,24,39). Labels, selects and the bookmark input stay dark-on-white and readable (`shots/crop-215-dark-lb.png`).
- **Bookmarks:** a bookmark saved with color Green plus label "frontend" re-applies both after a filter reset. It survives a reload and applies immediately after it. Delete brings the count back to 0, and a blank name is ignored.
- **Labels clear:** with "bug" checked, clearing the labels field and pressing Enter shows all 22 cards. On the baseline it stays at 3.
- **Close paths:** Esc and × both close the panel. 0 console errors and 0 failed requests.
- Break-attempts:
  - Applying a bookmark immediately after a reload, before the labels finish loading: labels restored.
  - Mixed-case typed list "feature, Bug": parsed correctly.
  - Unknown label typed: filter cleared silently (see observations).

## KF-350 cumulative flow: FAIL
- **Method:** real data on both servers, crossing a day boundary with the browser timezone set to Etc/GMT+1, so local midnight is 01:00 UTC.
  - Phase 1 (00:31 UTC): create the tasks.
  - Phase 2 (01:00:34 UTC): rename Alpha to "Alpha Renamed", then move T1 to Done. Rename Beta to B2, move T3 out, rename again to B3. Rename Temp to Temp2 (with T4 created before the rename and T6 after), move both out, delete the column.
  - JSON: `fix-cfd-Etc_GMT+1.json`, `base-cfd-Etc_GMT+1.json`.
- **Hard case passes.** Baseline legend has 8 entries, including phantom "Alpha" and "Beta" bands; status reads "6 tasks across 8 columns". On the fix, Alpha Renamed is [2,1] (yesterday, today) and B3 is [2,1]. There is no Alpha, Beta or B2 band.
- **Break found (deleted column that had been renamed).** One deleted column shows as two bands, "Temp" [1,0] and "Temp2" [1,0]. The legend reads `To-do, Alpha Renamed, B3, Done, Temp2, Temp`, and the status says "6 tasks across 6 columns" for 4 live columns plus 1 deleted one. The deleted column's former names and id can no longer be resolved, so each event falls back to the name it recorded. Severity LOW. Evidence: `shots/fix-cfd-Etc_GMT+1.png`.
- **Renames made before this release (report only).** I opened the baseline DB with the fix binary. Phantom "Alpha" and "Beta" bands remain, because old events have no column_id and former_names is empty. Status: "6 tasks across 8 columns" (`legacy-cfd-Etc_GMT+1.json`). This limitation is not documented in agent-guide, skill.md or agents.json; it appears only in the model doc comments.

## KF-371 column settings: FAIL
JSON: `fix-371.json`, `fix-371d.json`, `base-371d.json`.

Passing parts:
- **Column sum None:** no badge and no separator. The baseline shows the count.
- **Task count:** shows the count and updates after drags (Counted 2 to 1, OldBool 2 to 3).
- **Old DB values:** old `column_sum:true` reads as count, both on the board and in the Edit column dialog.
- **WIP columns:** still show "1 / 3".
- **Hours badge type:** stays "3h 30m" after a drag in and under a color filter. The baseline turns it into the count "3".
- **Load more:** an archiving column of 45 tasks shows 20, then 40, then 45, and the button then hides. Newest first, no task lost, 45 cards in the DOM. The control looks reasonable (`shots/zoom-loadmore.png`).
- **Sorted columns, drop inside:** snaps back at once with the toast "This column is sorted, so its order follows the sorting.". No /move request is sent, the order is unchanged after 3 s and after a reload. The baseline saves the drop, then silently reverts on reload.
- **Sorted columns, drop from another column:** a task dropped at the top lands in its sorted spot after the reload.
- **Stored group_by_date:** false on a Done column gives no grouping; true on a non-Done column groups.

Failures:
1. **Hours badge overlaps the + button.** "3h 30m" starts at x 661.9 while the + button ends at 677.1, so about 15 px of the badge sits under it. The separator line also runs through the text. Measured at 1905x1000 (`shots/zoom-hours-badge.png`). The baseline overlaps identically, so this predates the new commits, but the hours display is part of this defect and is illegible.
2. **Hours badge goes stale after a drag.** Dragging H1 (1.5h) out of the Hours column leaves "3h 30m"; only a reload shows "2h" (`kf371h.mjs` output). This contradicts "stays right after drag".

Break-attempts: a column that is both archiving and sorted (date label inside the list) snaps back correctly; a drop with a filter active.

## KF-379 help dialog: PASS
- **Box:** golden 370,61,700x490; baseline 370,91.2 with a 39.3 px tab bar and a 0.55 backdrop; fix 370,61.2,700x489.6 (centred).
- **Tab bar:** 52 px. Backdrop rgba(0,0,0,0.2); pixel sample 204,204,204 in both golden and fix.
- **Chevrons:** on the right via `::after`; the native disclosure marker is gone, and the chevron flips when a subject opens. The baseline uses the left disclosure marker.
- **Focus:** lands in `#help-search`.
- **Close paths:** backdrop click closes (it did not on the baseline). Esc closes, including with text in the search field. × closes, and a click inside keeps it open. Reopening resets to Quick help.
- **1905x1000:** centred at 602.5,185.5. **Dark:** same.
- Evidence: `fix-379.json`, `base-379.json`, `shots/cmp-379.png`.
- Break-attempt: a drag-select from the search field released over the backdrop closes the dialog (see observations).

## Regression: PASS
- **Nine earlier defects, fix and baseline identical** (`*-reg.json`, `*-reg2.json`):

| Defect | Probe | Result |
|---|---|---|
| KF-370 | user filter | 17 unassigned, 5 named |
| KF-381 | date options | Due in October, November, 2026, 2027 |
| KF-384 | Done column Edit dialog | Archiving checked |
| KF-388 | Esc during title edit | discarded, modal stays open |
| KF-383 | P with modal open | opens the modal's reports menu, not the global one |
| KF-392 | Time spent total | colour rgb(31,41,55) |
| KF-393 | spent bookmarks | toggle opens |
| KF-369 | Move up on lane below Default | L2 moves above Default |
| KF-382 | template delete error path | not probed in the UI; `new_board.html` is untouched by the diff (template-level check only) |

- **Full-board pixel compare, fix vs baseline:** ran at 1905x1000 and 1440x612, light and dark. Only 101 px differ in light and 25 px in dark. The bounding box is the column-header badge strip (y 85–122), which is the intended count-badge removal (`shots/cmp-header-1440.png`).
- **Drag-and-drop:** an unsorted within-column reorder persists after reload.
- **Console:** 0 errors and 0 page errors in every scenario. The only failed request is an ERR_ABORTED swimlane /move during the page's own reload, and it appears on both builds.

## Data: PASS
- **Additive, default-safe fields:**
  - `ColumnRow.former_names` (`#[serde(default)]`).
  - `TaskEvent.column_id` (`Option`, `serde(default)`, `skip_serializing_if`).
  - Storage is serde_json and no struct uses `deny_unknown_fields`.
- **Production DB (d230d76) opened by the fix:** every board's export-data and columns, plus /api/tasks, are identical after dropping the new fields. 11 columns gain `former_names: []`, and no event has a column_id. The legacy `column_sum:true` / `group_by_date:true` / `sorting:name` column renders as count, archiving and sorted.
- **Baseline DB opened by the fix:** identical, 22 `former_names: []` added.
- **Downgrade, fix DB opened by the baseline:** reads fine and the exports are identical, with 159 column_id events hidden. An old binary would drop the new fields when it rewrites a row.
- **SCHEMA_VERSION:** these changes will need a bump when merged onto origin/main, which has the guard (`SCHEMA_VERSION = 1` plus fingerprints).
- **Bookmarks:** stored in localStorage under `chipflow-filter-bookmarks:<boardId>`, separate from `chipflow-spent-bookmarks`.

## Observations (candidate defects)
1. KF-350: a deleted column that had been renamed splits into one band per name. Above, and it is the cause of the FAIL.
2. KF-371: the hours badge overlaps the + button, and goes stale after a drag. Above, and the cause of the FAIL. Also, with a filter active the hours badge keeps the total while count badges show the filtered count.
3. Labels field: Enter with an unknown or partial label silently clears the existing label filter.
4. The labels menu differs from board/15b. KanbanFlow has a "- No labels -" option, a Done button and an OR/AND select, with OR as the default. ChipFlow renders an inline list and always ANDs. This predates the branch.
5. Help dialog details vs account/09:
   - Subject rows are 34 px against KF's 38.
   - The chevron and × are smaller and thinner.
   - The search box is 600x36 against 590x38 and shows a blue focus ring.
   - "Restore task" is missing.
   - A drag-select that ends on the backdrop closes the dialog.
6. Column headers on both builds centre the title, and the + sits about 54 px from the right edge, where KF has a left-aligned title and the + at the edge. With badges now hidden this leaves an empty gap. Baseline-identical, so not from this diff.
7. agent-guide.md still documents export-data columns as `[{"id","name"}]`; it now also returns `former_names`, and events carry `column_id`. That breaks the AGENTS.md rule to keep the guide accurate.
8. Bookmark UI: the "Bookmark filter" button wraps onto two lines. There is no golden for the expanded footer body.

DONE: KF-215 PASS The panel covers the funnel, pencil and Menu at both 1440 and 1905. Separator y 358 and Remember row 366/34 match the golden +2. Bookmarks footer 1190,566,250x46 has a chevron, and bookmarks save, apply and persist across reload. Clearing labels + Enter shows 22/22 cards (baseline 3). The panel stays light in dark mode. 0 console errors.
DONE: KF-350 FAIL The hard case passes: rename after creation then a move across local midnight gives one band (Alpha Renamed [2,1], B3 [2,1]), while the baseline adds phantom Alpha and Beta bands. But a deleted column that had been renamed shows two bands, "Temp" and "Temp2" ("6 tasks across 6 columns" on a 4-column board; shots/fix-cfd-Etc_GMT+1.png). Old name-only history keeps its phantom bands, and that is not documented.
DONE: KF-371 FAIL None, Task count, old bool true, Load more (20/40/45, nothing lost) and sorted drops (snap back with a toast and no request; cross-column drop lands in its sorted spot) all pass. The Estimated hours badge "3h 30m" sits 15 px under the + button with the separator through it (zoom-hours-badge.png, baseline identical). It also stays "3h 30m" after an estimated task is dragged out, until a reload shows "2h".
DONE: KF-379 PASS Box 370,61.2,700x489.6 matches the golden 370,61,700x490. Tab bar 52 px, backdrop rgba(0,0,0,0.2), chevrons on the right. Backdrop click, Esc and × close it; focus lands in search. The baseline sits at y 91 with a 39 px tab bar, a 0.55 backdrop and no backdrop close.
DONE: REGRESSION PASS The 9 earlier defects behave the same on fix and baseline. Full-board compares in 4 permutations differ only in the column-header badge strip (101 px light / 25 px dark). Drag-and-drop works, 0 console errors.
DONE: DATA PASS The new fields are additive and serde-default. DBs from production d230d76 and the baseline open with the fix and export identically apart from `former_names: []`, and the downgrade reads too. A SCHEMA_VERSION bump is needed when this merges onto main.
DONE: GATE PASS fmt 0, clippy --all-targets -D warnings 0 warnings, env -i TZ=UTC cargo test 126 passed / 0 failed, jsc checkSyntax 0.
