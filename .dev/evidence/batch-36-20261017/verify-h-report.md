## verify-h report: board-h (68c9897) against baseline 1194d71

**Verdicts: 9 PASS, 4 FAIL.** The four failures are KF-215, KF-350, KF-371 and KF-379.

### Environment
- **Trees:** fix worktree `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-h`, detached at 68c9897. Baseline worktree `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-h-base`, detached at 1194d71. board-h was not touched and is still at 68c9897.
- **Diff scope:** agent-guide.md, src/routes.rs, static/{app.js,style.css}, templates/{board,new_board,task_card,timer_log}.html. Nothing outside what the 13 defects need.
- **Binaries:** release builds copied to the scratchpad.
  - fix md5 `691835a5e22b2829cbcd77623aaee866`
  - base md5 `179b15b24119e8c9e83264b57a705438`
  - `rg -a -c`: `help-dialog` and `spent-bookmarks-toggle` appear 2 and 3 times in fix, 0 in base.
- **Instances:** fix on port 3376 with `/tmp/chipflow-verify-h-3376.redb`; base on 3377 with `/tmp/chipflow-verify-h-3377.redb`. A copy, `/tmp/chipflow-verify-h-mig.redb`, was used for the migration test. Both servers were killed by PID afterwards.
- **Data:** golden-master seed boards, plus extra seeded boards for lanes and for the cumulative flow chart.
- **Browser:** Playwright 1.63.0, DPR 1, browser timezone UTC, a fresh context per run, animations off. Viewport 1440x612 for golden comparisons, 1905x1000 elsewhere.
- **Evidence:** everything is under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-h/`: per-scenario `*.json` files and `shots/`. Playwright scripts are in `.../scratchpad/r2/pw/vh/`.

### Gates (fix tree)
- `fmt --check`: 0.
- `build`: 0. The release build has one warning, `unused import ServeDir` (routes.rs:26), and baseline has the same one.
- `clippy --all-targets`: 0 warnings.
- `TZ=UTC cargo test`: all pass, including 70 in the main suite (baseline has 68).
- `jsc checkSyntax` on app.js: 0. Both inline scripts in new_board.html also pass.
- **Empty-environment test** (`env -i`): the same two tests fail on fix and on baseline, `format_due_card_weekday_near_absolute_far` and `format_due_modal_relative_and_done_marker`. This host is on CDT, so these are the known non-UTC failures and not new.

### Per-defect results (claim → baseline → fix)

**KF-215 — FAIL** (filter panel; golden board/11 at 1440x612)
- Panel box: base 1195,75,245x537 → fix 1190,36,250x576. Golden is 1190,34,250x578; ChipFlow's top bar is 36px tall against KF's 34, so this matches.
- Header 35px `#ecebeb`, User label and select (1208,109.5,214x30), labels field (214x30) and list button (23px, `#f5f5f5`) all match within 1–2px.
- Bookmarks box 1190,566,250x46 matches exactly.
- Switch track and knob colours match the golden, both off (`#9e9e9e`/`#fafafa`) and on (`#f9c243`/`#f5a033`).
- Tracker numbers at 1905: base panel y=75 and funnel x=1719, so the bug reproduces.
- **Header controls:** fix moves the funnel, pencil and Menu left of the panel (funnel x=1004 at 1440, 1469 at 1905). In golden board/11 and in kf-reference 05-filter-panel.jpg (1568 wide) the panel covers these controls and nothing appears to its left. The tracker's 09-30 line ("funnel moved to x≈1514") conflicts with both newer references, and the builder followed the tracker line.
- **Remember row:** 10px too high. Its separator is at y=352 against the golden's 356, its text centre at 371 against 381, and the row is 37px tall against 34.
- **Bookmarks footer:** a static div with no chevron, and clicking it does nothing. The golden shows a collapsible section with a chevron.
- **Labels field:** clearing it and pressing Enter does not remove the label filter; the field snaps back to "bug". The list has no "- No labels -" entry and no Done or OR/AND footer (golden 15b).
- **Dark-mode regression:** with `body.dark`, the panel turns `#111827` but the section labels stay `rgba(66,66,66,.8)` (about 1.5:1 contrast) and "Bookmarks (0)" stays `#444` (about 1.8:1). The header stays light. Baseline labels were readable at `#9ca3af`, and KanbanFlow keeps this panel light in dark mode (golden dark/11). Evidence: `shots/darkPanel215-fix-dark.png`.

**KF-350 — FAIL** (cumulative flow after a column rename)
- Tracker case (rename "In Review" to "Reviewing", no history in the window): base shows legend +"In Review", "22 tasks across 6 columns", "In Review: up to 0 tasks". Fix shows 5 columns. This narrow case is fixed.
- **Break:** a task created in "In Review" on Oct 16, the column renamed on Oct 16, and the task moved to Done at 00:00:15 UTC on Oct 17. Fix and baseline both show legend `...,Done,In Review`, "22 tasks across 6 columns" and "In Review: up to 1 tasks".
- The fix only drops old names that have zero tasks across the window. Any renamed column with traffic in the last 30 days still gets a phantom band. Evidence: `cfd350-fix-midnight.json`, `shots/cfd350-fix-midnight-cfd.png`.

**KF-369 — PASS** (swimlane menu)
- Base: Rename on a folded strip opens no dialog, and Move up on "Lane Two" leaves the order unchanged. Both bugs reproduce.
- Fix:
  - Rename works from a folded strip.
  - Move up puts "Lane 2" above Default.
  - The first lane hides Move up and the last lane hides Move down.
  - A non-empty lane offers only Rename and Down, so Delete is not offered.
  - Delete of an empty folded lane works, and its confirm no longer says "the server will refuse".
- **Break:** a lane named `Lane Three ✨ & <b>"quoted"</b>` keeps its exact name in the rename prefill and in the delete confirm.
- `ERR_ABORTED` shows on PATCH, move and DELETE because the page reloads after a successful fetch. Baseline does the same on column PATCH, and the server-side changes all landed.

**KF-370 — PASS** (User filter)
- Unassigned: base 22 of 22 visible → fix 17. Named user: base 22 → fix 5. Timer users: 0 on both.
- Combined filters work: Unassigned + Blue gives 3, and the named user + Overdue gives 2.
- **Breaks:**
  - A remembered filter saved before the fix with `user:"me"` leaves the select blank and the funnel active while all 22 cards show (cosmetic).
  - Assigning a member from the card's right-click "Assign members" leaves the card visible under Unassigned until reload: `data-member-ids` stays stale and the avatar does not update either. The avatar staleness is also on baseline, so this is not new.

**KF-371 — FAIL** (Task sorting, Column sum, Archiving)
- Baseline repro: a column saved with sorting "name" stays in board order.
- Fix: due date sorts ascending with no-due tasks last. Colour follows palette order. Name is case-insensitive. Created and Added to column sort oldest first. Hours shows "17h", and "17h 6m" with a 0.1h task. Archiving groups a non-Done column under "Today". Done groups by default and stops when saved false.
- **Column sum:** "None" and "Task count" render the same badge ("5"), so Task count still has no effect. Golden board/02 shows no count on non-WIP headers when Column sum is None; KF-316 tracks that count cell separately.
- **Archiving:** the dialog still says "Loads the 20 most recent tasks from the start", and that limit is not implemented.
- **Sort direction:** ascending cannot be checked against KanbanFlow. Sorting is premium on this account, and the free-tier goldens (columns/03) only show the options marked "[premium feature]". The option lists match KanbanFlow's.
- **Break:** dragging a card within a sorted column shows the new order, and the next load silently reverts it with no toast.
- **Baseline-written DB (copy read by the fix binary):**
  - `column_sum:true` reads as "Task count".
  - `sorting:"name"` and `"created"` work.
  - Two behaviour changes come from `group_by_date` values the baseline dialog stored by name:
    - **(a) Renamed Done column:** a Done column renamed "Finished" and then edited once in baseline had `false` stored, so it stops grouping by date after the upgrade. Baseline always grouped Done columns.
    - **(b) Name containing "done":** a column "Undone ideas" edited in baseline had `true` stored, so it starts grouping by date after the upgrade.

**KF-379 — FAIL** (help dialog)
- Base: clicking "?" opens nothing. Fix: "?" and Menu > Help & feedback both open the dialog, Esc and × close it, search filters the topics, the tabs switch, and focus lands in search.
- **Position against account/09:** the dialog box is 370,91.2,700x489.6, while KanbanFlow's is 370,61,700x490. KanbanFlow's y=61 is exactly centred, since (612−490)/2 = 61. Fix is 30px low because `margin:60px auto 0` sits inside the flex-centred overlay.
- **Tab bar:** 39.3px tall against 52; flex-shrink squeezes it, so the tabs and search sit about 20px off.
- **Other differences:** the backdrop is 0.55 against KanbanFlow's about 0.2 (`#737373` vs `#CCCCCC` on white). The disclosure triangles sit on the left where KanbanFlow puts chevrons on the right.

**KF-381 — PASS** (Date month/year options)
- Base has no month or year options. Fix appends "Due in October/November/2026/2027" after "Due in 90 days", in the golden board/11 order.
- October shows 3 cards, November 1, 2026 4 and 2027 0.
- **Break:** with Remember on and "Due in October" selected, a reload restores the value and 3 cards.

**KF-382 — PASS** (template delete error)
- Deleting an already-removed template: base throws `ReferenceError: showError is not defined` and shows nothing. Fix shows "Could not delete the template.".
- The remaining console 404 is deliberate (the template was already gone).
- **Break:** a connection-refused network failure also shows the error on fix; base gets an unhandled `TypeError`.

**KF-383 — PASS** (P in the task modal)
- Base: P un-hides `#reports-menu` behind the modal. Fix opens `tm-reports-menu` (Time log, Print, History, Time in column) and leaves the global menu hidden.
- **Break:** typing "pp" in the comment box opens no menu.

**KF-384 — PASS** (Edit column Archiving default)
- Base: a Done column saved unchecked reopens checked, Done renamed "Finished" opens unchecked, and "Undone ideas" opens checked.
- Fix: checked by default, unchecked after saving off, "Finished" with nothing saved opens checked, "Undone ideas" opens unchecked.
- The migration effects above under KF-371 come from this same key.

**KF-388 — PASS** (Esc and title edits)
- Base: Esc saved "UNSAVED TITLE EDIT" and closed the modal. Fix: the first Esc restores the title and keeps the modal open, the server never sees the draft, and the second Esc closes it.
- **Breaks:**
  - Rename + Enter, then Esc: closes and keeps the new name.
  - Rename + blur, then Esc: closes.
  - Commit "Committed B", type "Draft C", then Esc: reverts to "Committed B", not the original.
  - The subtask draft (KF-299) still clears on the first Esc.

**KF-392 — PASS** (Time spent total colour)
- Base: `#f3f4f6` on `#eef0f3`, contrast 1.04. Fix: `#1f2937`, contrast about 12.8.
- **Break:** in dark mode it is `#e5e7eb` on `#0f172a`.

**KF-393 — PASS** (Time spent Bookmarks)
- Base: "Bookmarks (0)" is static text. Fix:
  - Save updates the count to 1.
  - Applying a bookmark restores Board, Period, Entry type, Colour and Group by, then reloads once.
  - It persists across a page reload, and delete brings the count back to 0.
  - An empty name is ignored.
- **Break:** a name of `A <img src=x onerror=alert(1)>` renders as text.
- **Storage:** bookmarks live in this browser only (localStorage), so they don't follow the owner to another device. No reference shows where KanbanFlow stores them; I believe it keeps them per account on the server, but I could not confirm that.

### Regression sweep (fix vs base, ImageMagick AE)
- Pixel-identical (AE 0): the board at rest at 1905x1000 and 1440x612, the open task modal, the board after dragging Backlog→Reviewing and a card into Done (Done grouping under "Today" is identical), the scrolled sticky header, and a collapsed column strip.
- Filter combinations and filtered count badges match.
- 0 console errors and 0 failed requests everywhere, apart from the deliberate KF-382 404, the deliberate refused connection, and the reload `ERR_ABORTED` noted under KF-369.

### Observations (candidate new defects)
1. **Label filter AND vs OR:** the label filter uses AND ("bug, feature" gives 0 cards), but golden 15b shows KanbanFlow defaulting to OR with an OR/AND switch. This predates the fix (KF-165).
2. **Help backdrop:** clicking the backdrop does not close the help dialog.
3. **Stale comment:** the KF-045 comment in `doSaveColumn` ("mapped back to the boolean config bag") is out of date now that the value is a string.
4. **Server copy of the remembered filter:** `group_by_date:"true"` saved as a string falls back to the Done flag. This is an edge case.

DONE: KF-215 FAIL header controls shown left of panel where goldens board/11 and kf-reference 05 show them covered; Remember row 10px high; Bookmarks static with no chevron; dark-mode labels at about 1.5:1 contrast
DONE: KF-350 FAIL phantom old-name band persists whenever the renamed column held a task at any day end in the window (fix shows "In Review: up to 1 tasks", 6 columns, same as baseline)
DONE: KF-369 PASS menu works on folded strips, Move up/down use full lane order, Delete hidden on non-empty lanes; odd-name rename/delete survived
DONE: KF-370 PASS Unassigned 17 and named user 5 of 22 (baseline 22/22); stale card after right-click assign noted
DONE: KF-371 FAIL Column sum None and Task count render identically (golden shows no count for None); 20-most-recent archiving promised but missing; baseline-saved group_by_date changes Done grouping on upgrade
DONE: KF-379 FAIL "?" now opens help, but dialog sits at y=91.2 vs KF's centred 61 and tab bar is 39.3px vs 52px (account/09)
DONE: KF-381 PASS month/year options appended in golden order, filter correctly and restore from Remember
DONE: KF-382 PASS visible error, no ReferenceError, also on network failure
DONE: KF-383 PASS P opens the modal Reports menu; typing P in a field does nothing
DONE: KF-384 PASS Archiving default follows the Done flag, saved value wins
DONE: KF-388 PASS first Esc discards title draft and keeps modal; committed names survive Esc
DONE: KF-392 PASS total is #1f2937 (about 12.8:1) in light, #e5e7eb in dark
DONE: KF-393 PASS save, apply, delete and persist work and are XSS-safe; browser-only storage could not be checked against KanbanFlow
