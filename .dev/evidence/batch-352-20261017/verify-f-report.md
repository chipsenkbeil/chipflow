## verify-f report: fix/board-visual-f @ e6b2847 (19 commits on 1194d71)

**Verdict: 15 PASS, 4 FAIL.** Three of the four FAILs come from the 10-16 goldens disagreeing with the tracker or the 09-30 capture. The fourth is a duplicate "Today" header that the fix introduces and the baseline doesn't have.

### Environment
- **Worktrees:** fix tree `.dev/worktrees/verify-f` (detached e6b2847), baseline tree `.dev/worktrees/verify-f-base` (detached 1194d71). The diff touches only the 11 files expected for these defects.
- **Binaries:** both release builds with `--config 'build.build-dir="target/build"'`.
  - fix md5 a3cd1fb7c749c682c773324659cd181c, baseline md5 d650e832b499f07320dcab4ab9bbf21f.
  - `rg -a -c addTaskDialog` finds 32 in fix and 0 in baseline; `new_tasks_at_top` finds 5 and 0.
- **Ports and databases:**
  - fix on 3374 with `/tmp/chipflow-verify-f-3374.redb`, baseline on 3375 with `/tmp/chipflow-verify-f-3375.redb`.
  - The fix database is a byte copy (md5 2a5a3b00…) of the database the baseline binary wrote.
  - Seeded data: the golden-master fixture, the 3 Done tasks set to grouping date 2026-09-30, legend on, plus extra "VF Test" and "VF Lanes" boards.
- **Browser:** Playwright 1.63 Chromium, DPR 1, animations frozen. Viewports 1440x612 (golden size), 1905x1000, 1690x1000, 1440x900 and 1240x850.
- Both servers were killed by PID (20722, 20703); nothing is left listening on 3374/3375.
- **Evidence** is all under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-f/`: `shots/`, `crops/`, `board-probe.json`, `sub-probe.json`, `quick-3374.json`, `menus.json`, `coll.json`, `drag.json`, `widths.json`, `gates.log`, `test-utc.log`. The Playwright scripts are `scratchpad/r2/pw/vf-*.mjs`.

### Gates (fix tree)
- `fmt --check` exits 0.
- `clippy --all-targets` exits 0 with 0 warnings.
- `env -i TZ=UTC ~/.cargo/bin/cargo test`: 128 passed, 0 failed, 1 ignored (the existing DST test).
- `jsc checkSyntax(static/app.js)` exits 0.

### Per defect: claim → baseline → fix → golden
Coordinates: the golden y value plus our uniform 2 px top-bar offset is called "golden+2" below.

- **KF-361** board header row
  - Baseline: row 39.4 px, title 17.6 px.
  - Fix: row 0,36,1440,46; title 17,42 at 18px/700/lh34; disc 242,45 (28 px); "+" 276,45; Menu 1350,42,80,34; table top 82.
  - Golden board/01 is 34/46/17,40/disc 16 px after the title/+ 6 px later/1350,40 — all match at golden+2.
  - Break attempt: the longer title "ChipFlow Golden Master" still keeps the 16 px and 6 px gaps.
- **KF-311** column header
  - Baseline: header 55.6 px, name 17 px, count cell and separator present.
  - Fix: header 37 px, background #eee, top rule #c3c3c3, name 15px/700/lh22.5 #111 with ink y 94-107 (matches golden), "+" 22x22 inset 2 px from the right, WIP line 303,118,267,3 (golden 303,116 → +2).
  - But the name is centred: "Backlog" ink starts 113 px into the header. Goldens 01, 02, 08 and the owner's Arc shot all show it left-aligned 11 px in.
- **KF-363** cards
  - Baseline: 4 px gap, one-line card 26.6 px, padding 4.8/6.4.
  - Fix: 8 px gap, one-line card 30 px, padding 5px 7px 0, title 12px/lh18 #222.
  - "Write API documentation…" ink x 6-240 is identical to golden; icon row ink is identical (98x14); the time text sits 1 px lower than golden.
  - At 1240 px the two-line title pitch is 18 px (baseline 15 px), which fixes the cramped look the owner saw.
- **KF-316** count cell
  - Baseline: 4 count badges and 4 separators.
  - Fix: no badges or separators on expanded headers; inline "5 / 3" kept; collapsed strips keep their count.
- **KF-362** overdue strip
  - Baseline: overdue text sits inside the header in #DC2626.
  - Fix: strip below the header at 295,120,283,28; background #f3f3f3; 10px/700 #8b0000; 1 px #d7d7d7 bottom rule; the header gets the #ff8080 line.
  - But the strip text is centred and its red bar is a fixed 120 px at x 377-496.
  - Golden 02 has the text at the left with a bar the width of the text: x 304-387 (84 px) for "2 overdue tasks" and 587-664 (78 px) for "1 overdue task", at y 144-146.
- **KF-241** folded strip
  - Fix: header 28x37; count #8b0000 12px/700 at the golden position; 21 px #ff8080 line 4 px in (golden+2).
  - But the rotated name ink starts at y 147 (offset-adjusted), where golden columns/08 has it at 130 (styles box 128-246). It is 17 px too low.
  - The fix matches the tracker's 09-30 "y 147-266", which the 10-16 golden contradicts. The baseline has the same 17 px offset.
- **KF-245** border shades
  - Baseline: #eecd45, #7cd340, #7faef9, #ee8b91.
  - Fix: #f5cc00, #59d600, #70b0ff, #ff858f. All 10 standard faces and borders now match the palette file (`kf-css-theme-vars.psv`).
  - Break attempt: these are rows written by the baseline binary (still storing the old hex) read by the fix; the colours API and cards both return the new values.
- **KF-354** legend
  - Segments use the card faces with card-border top edges; rule #c3c3c3; band #f2f2f2; 180x23 boxes at y 589. Exact match to golden 02.
  - Break attempt: a custom label ("Custom Yellow Label") and 10 enabled colours render correctly.
- **KF-333** colour submenu
  - Baseline: plain rows with a coloured bar.
  - Fix: 140x34 rows tinted with the card face, current colour bold, padding 0 25 0 15, 15 px text. Matches golden cards/03.
- **KF-302** quick-add dialog
  - Baseline: anchored 272x106 popover.
  - Fix geometry matches golden modal/12 exactly: dialog 470,25,500,300, background #ffffe0, golden shadow; name field 440x28 20px/700 with the yellow underline; colour disc 34 px at opacity .6; select 396x34; icon buttons at y 172; Save buttons at y 266; hints 10px/700; close at 935,30. Overlay is rgba(0,0,0,.6).
  - Colour menu matches golden 13: 396 wide, 36 px rows, 16 px dots at opacity .8, default bold. The tracker says 14 px dots; the golden's 16 px wins and the fix uses 16.
  - Picking Blue tints the dialog #cce3ff.
  - Break attempts, all behave as expected: empty name doesn't create a task; Ctrl+Shift+Enter keeps the dialog open and clears the name; Ctrl+Enter closes; Esc closes; reopening resets the colour; Members saves and opens the task modal. 0 console errors.
- **KF-364** inline subtasks
  - Baseline: always 2 shown plus "N hidden".
  - Fix:
    - 3 subtasks with 1 done: all 3 shown.
    - 4 with 0 done: all 4 shown.
    - 5 with 1 done: 4 open shown, "1 hidden (1 done)".
    - 6 with 3 done: 3 open shown, "3 hidden (3 done)".
    - Golden card #11: 2 open plus "2 hidden (2 done)", box 16 px at a 25 px pitch, identical x.
  - Expanding and toggling a subtask both work.
- **KF-365** Done groups
  - Baseline: no "Today" header when nothing was done today.
  - Fix: an empty "Today" header, then "Wednesday, 30 September". #888 12px/700 with #d8d8d8 rules at y 146/181 and the first card at 192 — pixel-identical to golden at +2. Moving a card to the bottom of Done puts it under "Today".
  - **Break found:** give a Done task a future `grouping_date` (2026-10-20, settable only through the API). The fix renders Today (empty), Tuesday 20 October, Today — "Today" appears twice. The baseline renders a single Today for the same data.
- **KF-367** board menu
  - Baseline: 218x405 white dropdown.
  - Fix: panel 1190,36,250,576 #f7f7f7 with the golden shadow; 35 px #ecebeb title band, bold centred "Menu"; × at 1407,39; 34 px items with 8px 18px padding; #e6e6e6 separators at golden+2; switch on #f9c243/#f5a033, off #9e9e9e/#fafafa; Get Premium 1200,499,230,34 #49a849 at weight 900.
  - Closes with ×, Esc and an outside click; the Reports submenu opens on hover as on the baseline.
- **KF-376** card icons
  - Baseline: #4B5563 outline glyphs; the #11 icon row is 175 px wide.
  - Fix: white-filled #808080 glyphs at a 24 px pitch; #11 row 170 px against golden 169.
- **KF-377** card avatars
  - Baseline: "A", disc centre (20.9, 19.3) from the card's top-right.
  - Fix: "AU", the same initials as the header; disc centre (19.5, 19.5) against the claim's (19, 19).
- **KF-386** drag placeholder
  - Baseline mid-drag: solid orange card face and border.
  - Fix mid-drag: #fafafa fill, 1 px #fafafa border, children hidden, 30 px tall. Matches golden cards/07.
  - Golden vs tracker: the golden shows a blank #fafafa slot, the tracker says dashed. The golden wins and the fix follows it.
- **KF-385** column overflow
  - Baseline: a wheel over the long column scrolls the whole board wrap (scrollTop 441).
  - Fix: only that column's task list scrolls (scrollTop 600); other columns and the wrap stay at 0.
  - Break attempt: on a swimlane board the lane cells are unbounded, so the whole board scrolls and lane 2 stays reachable; the sticky header stays at y 82.
- **KF-403** new-task position
  - Baseline: the quick-added card appears at the top, then jumps to the bottom after a reload.
  - Fix: bottom by default, both immediately and after reload.
  - Setting "Top" in the Task settings dialog puts new cards at the top, with positions lo-1 (-1, -2) for both the UI and `POST /api/tasks`.
  - A wrongly typed value (`"yes"`) returns 400. A board config without the key reports `false`.
- **KF-405** column menu
  - Baseline: menu has Edit / Collapse / Show details / Delete.
  - Fix: Edit / Collapse / Show details only. An empty column deletes from the layout view.
  - A non-empty column can't be deleted in either build (pre-existing; see candidate 2).

### Judgements
- **Column names and the overdue strip:** the 10-16 goldens and the owner's Arc shot show both at the left (name 11 px in; strip text at the left with a text-width bar). The builder's centring follows the 09-30 capture and loses. I count this against KF-311 and KF-362.
- **KF-386:** the golden's blank #fafafa slot beats the tracker's "dashed"; the fix follows the golden.
- **KF-364** (more than 3 subtasks with any done hides the done ones): consistent with both data points (3 shown in full; 4 with 2 done folded). There is no KanbanFlow evidence for long lists with nothing done, or for lists of 5 or more. Plausible but unproven.
- **KF-302** (Members/Labels/Description/More save the task and open its details): no evidence. The goldens show only the buttons; I believe KanbanFlow edits these inside the dialog. Unsupported; capture KanbanFlow's click states before relying on it.
- **KF-403** (affects new tasks only, missing key = bottom): supported by golden settings/13 ("Position of new tasks: Bottom of column"). Wording mismatch: the ChipFlow dialog says the setting affects the column add button only, but `POST /api/tasks` also follows it, as `agent-guide.md` documents.
- **KF-367** ("Save board as template" moved to the layout view): no evidence for any location. KanbanFlow's menu and layout-mode header (settings/01: Done, Layout title, Add column, Add swimlane) have no template control, so this is a ChipFlow-only addition either way.
- **KF-405** (Delete in the layout view): supported by golden columns/01, where KanbanFlow's layout ⋮ menu includes Delete. ChipFlow uses a red "Delete" text button per row instead of that ⋮ menu.

### Owner's Arc shots (renders at 1690x1000, 1440x900 and about 1240 px; the Arc shot is about 1240 CSS px by its measured scale)
- Matches KanbanFlow in the fix: "+" at the right and no count cell; red rule under headers of columns with overdue tasks; card spacing and two-line line height; inline subtasks with "2 hidden (2 done)"; an empty "Today" header then "Wednesday, 30 September" (the "Completed" header is gone); grey icon glyphs; "AU" two-letter initials on cards.
- Still differs from KanbanFlow: titles and the overdue row are centred where KanbanFlow puts them at the left.
- Fit (not a stream-F failure; measured only): both builds are identical.
  - At 1690 and 1440 the columns shrink to fit (334 and 284 px) and nothing overflows.
  - At 1240 the columns stop at a 254 px minimum, the table is 1270 px wide, and the Done column is 52 px off screen.
  - After scrolling right the legend moves with the board but is only viewport-wide, leaving a 52 px gap at its right.
  - KanbanFlow shrinks the columns to fit.

### Candidate defects (observations, evidence in `verify-f/`)
1. Fix regression in KF-365: a future `grouping_date` produces two "Today" headers (API only).
2. Deleting a column that has tasks fails in both builds. The move call omits `position` and gets 400 ("missing field position"), then the delete gets 400. This is now the only delete path.
3. Card avatar ring is a lightened mix (rgb(121,224,48) on a green card) instead of the card border colour #59d600 that KanbanFlow uses. Pre-existing.
4. The palette's light values for purple, magenta, cyan and brown differ from KanbanFlow's (#f2e6ff, #ffebff, #e6ffff, #f9e8dc). Pre-existing.
5. Dark mode doesn't use KanbanFlow's dark values (pre-existing):
   - cells stay white;
   - column names are #111 on #1f2937, nearly invisible;
   - the board title is #e5e7eb on #f2f2f2, invisible.

   The new overdue strip has no dark style (KanbanFlow dark: #444 strip, #ff8080 text). Apart from that, dark matches the baseline: the top bar is pixel-identical in all four width/theme combinations. Card titles now stay #222 in dark, which matches KanbanFlow, where cards stay light.
6. Board title colour is #1F2937 against KanbanFlow's #333; the WIP "5 / 3" glyphs sit about 1 px lower than golden. Minor.
7. Plain Enter in the add-task name saves and closes the dialog; KanbanFlow's behaviour is unverified.
8. After expanding a card's hidden subtasks the label still reads "2 hidden (2 done)".
9. `src/routes.rs:7699`: the KF-053 doc comment now sits above the new KF-364 test, so the two doc comments run together.

DONE: KF-361 PASS header row 46px at golden+2; title, disc (16px gap), + (6px), Menu and table top all match board/01
DONE: KF-311 FAIL 37px band, 15px #111 name, 22px + and WIP line all match golden+2, but names are centred where goldens 01/02/08 and the owner shot have them 11px from the left
DONE: KF-363 PASS 8px gap, 30px one-line card, padding 5/7, title ink identical to golden (x 6-240), two-line pitch 18px; time text 1px low
DONE: KF-316 PASS no count badge or separator on expanded headers (baseline had 4 each); WIP "n / m" kept
DONE: KF-362 FAIL strip, colours, rules and header line are right, but text is centred with a fixed 120px bar (x377-496) where golden 02 has it at the left with a text-width bar (x304-387, 84px)
DONE: KF-241 FAIL count #8b0000 and 21px line match columns/08, but the rotated name starts 17px low (ink y147 vs golden 130); fix follows the tracker's 09-30 y147, which the golden contradicts
DONE: KF-245 PASS all 10 borders match the palette file, including rows written by the baseline binary read back by the fix
DONE: KF-354 PASS legend faces and edges match card palette, rule #c3c3c3, band #f2f2f2, exact golden 02 geometry; custom label survives
DONE: KF-333 PASS rows tinted with card face, current colour bold, 140x34, 15px text, matches cards/03
DONE: KF-302 PASS dialog, field, select, buttons, hints and colour menu (16px dots, 36px rows) match modal/12 and 13 exactly; save, add-more, Esc and empty-name break attempts pass
DONE: KF-364 PASS 3 subtasks shown in full, longer lists fold done ones; #11 shows 2 open plus "2 hidden (2 done)" at 16px/25px pitch, identical to golden
DONE: KF-365 FAIL empty Today header and day groups are pixel-exact to golden, but a future grouping_date (API) renders "Today" twice, which the baseline does not
DONE: KF-367 PASS 250px #f7f7f7 full-height panel, title band, separators, orange switches and Get Premium all match board/03 at golden+2; closes with ×, Esc and outside click
DONE: KF-376 PASS #808080 white-filled glyphs at 24px pitch; #11 icon row 170px vs golden 169
DONE: KF-377 PASS card avatars show "AU" like the header; disc centre (19.5,19.5) vs claim (19,19)
DONE: KF-386 PASS placeholder is #fafafa with #fafafa border and hidden children per golden cards/07 (tracker's "dashed" overridden by the golden); baseline showed a solid card
DONE: KF-385 PASS overflowing column scrolls on its own (600) while the board stays put; swimlane board still reachable, sticky header holds
DONE: KF-403 PASS bottom by default in DOM and after reload; Top setting works from UI and API; missing key = bottom; bad type gives 400
DONE: KF-405 PASS column menu is Edit/Collapse/Show details; empty column deletes from layout view (non-empty delete broken in both builds, pre-existing)
DONE: REGRESSION PASS 1440 and 1905 in light and dark: top bar pixel-identical to baseline, dark unchanged apart from changed elements (new strip has no dark style); card drag within and between columns persists; collapse/expand, swimlanes, sticky header, modal open/close and quick add work; 0 console errors and 0 failed requests in every fix run except the delete-with-move flow (2×400, also on the baseline)
DONE: DATA PASS DB written by the baseline opens in the fix; colour labels and descriptions untouched, standard borders read from the new palette, a board without new_tasks_at_top adds at the bottom as before
DONE: GATE PASS fmt 0, clippy --all-targets 0 warnings, TZ=UTC cargo test 128 passed/0 failed/1 ignored, jsc checkSyntax 0
