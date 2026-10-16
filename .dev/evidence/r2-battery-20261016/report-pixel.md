# R2 battery, batch `pixel` (43 rows), 2026-10-16

Result: 2 PASS, 41 FAIL, 0 BLOCKED. Every board, card, column, timer, menu and
settings surface with a recovered KanbanFlow capture deviates from it by more
than 1 px or by exact hex. The capture host itself is faithful: KanbanFlow's
own capture-to-capture noise is 328 px (cursor only), and ChipFlow's legend
labels render with ink boxes and ink mass identical to KanbanFlow's on the
same Linux Chrome.

All paths below are relative to
`/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/r2/results/pixel/`
(`$P`). References are in `/Users/senkwich/projects/chipflow/.dev/evidence/golden-masters/`
(`$G`) plus `/Users/senkwich/projects/chipflow/.dev/evidence/kf331-kanbanflow-settings-colors-recovered-20261016.png`.

## 1. Environment

- Instance: release binary `/Users/senkwich/projects/chipflow/.dev/worktrees/r2-main/target/release/chipflow`
  (origin/main 62d573d, not rebuilt), `--host 127.0.0.1 --port 3240 --database-path /tmp/chipflow-r2-3240.redb`,
  `TZ=America/Chicago`. Server log `$P/server.log`. Killed by exact PID
  (33275, 56826, 60209) at each restart and at the end. Port 3240 no longer listens.
- Admin `audit` (renders "AU" in the header and "A" on cards, see FAIL R2-063).
- Boards: GM `f2de8265-4720-4f0d-b65f-5c1e54a7f4db` (seeded with
  `seed-golden-master.sh --as-of 2026-09-30`), POMO `42ed021b-d9d3-4101-b8ca-1b13f78afa92`
  ("Pomodoro"), POMO-General `b74be172-ee0e-4755-97bf-f6da2a363d13` ("General"),
  Dale `edcb257d-5e17-4412-9a75-1b7ea7f6b8c5` ("Dale fidelity test"). Ids in `$P/boards.env`.
- Fixture adjustments, all through the API (`$P/api.sh`):
  - every GM column `config_json={"prop_subtasks":"show"}` (the 09-30 captures show
    inline subtasks, ChipFlow's column default hides them)
  - To Do `collapsed:true` for every capture except A, In Progress collapsed only for C
  - legend toggled through `PUT /api/boards/{id}/config` (on for F and I, off otherwise)
  - card #1 sections collapsed through localStorage `chipflow.cardsec.<id>` for B to F
  - POMO palette set by PATCH (Yellow "1 Pomodoro" default, Green "2 Pomodori",
    Blue "3 Pomodori", Red ">3 Pomodori", other six disabled), because the built-in
    Pomodoro template produced 10 enabled plain colors
  - G only: `pomodoro_minutes` 1, then a pomodoro on #11 run to "Take break". The
    two test entries were deleted afterwards (`DELETE /api/time/entries/...`).
- Clock: option 1, libfaketime 0.9.13 (installed with `brew install libfaketime`,
  which also pulled coreutils 9.12). `faketime '<start>' $BIN ...` sets a starting
  time that then runs forward. Proof it took effect: the HTTP `Date` header read
  `Wed, 30 Sep 2026 20:22:02 GMT` (15:22 CDT), and the rendered board showed the
  Done group "Today" and #6 not yet overdue. Three server starts on the same DB:
  `2026-09-30 15:22` (A to F, J, K), `2026-10-05 22:25` (G), `2026-10-12 12:00` (H, I).
  The browser clock was pinned with Playwright `context.clock.install()` to the
  server's `Date` header, `timezoneId: America/Chicago`. Chosen over `--as-of`
  masking because it reproduces relative due text, overdue counts and Done groups
  exactly, so none of that text needed masking, and one DB served all three dates.
- Linux capture host: `mcr.microsoft.com/playwright:v1.63.0-noble`,
  digest `sha256:eff16c30e6f3f4af0a03fa4b706120d5e9b0891c344a27d64559aff5900a4a27`
  (created 2026-09-04), Ubuntu 24.04.4 aarch64 under colima. Chromium 153.0.8010.12
  headless, DPR 1, `reducedMotion`, transitions frozen, scrollbars hidden. The
  container reached the Mac server through `--add-host=host.docker.internal:host-gateway`
  (resolves to 192.168.5.2) with the server bound to 127.0.0.1. No 0.0.0.0 bind was needed.
  colima does not mount `/private/tmp`, so scripts went in with `docker cp` and shots
  came back the same way (`$P/run.sh`). Container removed at the end.
- Fonts (`fc-match` in the container): Arial -> Liberation Sans, Helvetica -> Liberation Sans,
  "Open Sans" -> WenQuanYi Zen Hei (not installed), sans-serif / system-ui /
  "Helvetica Neue" / Roboto / "Segoe UI" / BlinkMacSystemFont -> WenQuanYi Zen Hei.
  What Chrome actually used (CDP `CSS.getPlatformFontsForNode`, in each `.log.json`):
  card titles, column names, due lines -> Liberation Sans, `.boards-btn` -> the
  self-hosted Open Sans Bold.
- macOS captures: the two 10-12 references are Chip's macOS screenshots, so H and I
  were captured with the host Playwright in `$R/pw` (same Chromium build) at the
  inferred viewports 1380x1030 (board, page starts at y 5 in the reference, bottom
  5 rows cut) and 1374x1030 (menu, page starts at y 2). These two references are
  colour-managed: KanbanFlow's `#333333` top bar reads `#2D2D2D`, `#F2F2F2` reads
  `#F0F0F0`, `#C3C3C3` reads `#BBBBBB`. Only geometry is compared against them.
- Frames: 1919x998 for the 11 Linux 1919x998 references (they contain no
  scrollbar strip, the top bar spans the full 1919), 1905x1000 for the KF-331
  reference (its 15 px scrollbar at x 1905-1919 is masked), 1920x1000 for the
  delete dialog (no scrollbar).
- Pixel counting: ImageMagick 7.1.2-34's `compare -metric AE` returned 6.67 for a
  10x10 image that differs in all 100 pixels, so it is not a count. Every "diff N"
  below is an exact count of pixels where any channel differs, from
  `-compose difference -composite -separate -evaluate-sequence max -threshold 0`
  (`aereg` in `$P/tools.sh`). Edge scans and green-channel ink boxes come from
  `$P/scan.lua` (`hrow`, `vcol`, `ink`).
- Console: every scenario log (`$P/shots/*.log.json`) has 0 console errors,
  0 page errors, 0 failed requests and 0 HTTP errors, except one discarded run
  that hit a 404 because my runner passed an empty board id (rerun clean).
- Scripts: `$P/drive.mjs` (driver), `$P/scenarios.mjs` (states A to K, Z),
  `$P/m_*.sh` (measurements, outputs in `$P/measure/`), `$P/crops/` (side-by-sides,
  KanbanFlow on top, ChipFlow below, red rule between).

Captures: A `A_gm0930.png`, B `B_todo_collapsed.png`, C `C_cols_collapsed.png`,
D `D_popover.png`, E `E_modal_dropdown.png`, F `F_menu_legend.png`,
G `G_pomodoro.png` and `G_running.png`, H `H_menu1012.png`, I `I_board1012.png`,
J `J_delete.png`, K `K_settings_colors.png` and `K_settings_colors_general.png`,
Z `Z_swimlane_dom` (DOM probe), all in `$P/shots/`.

## 2. Verdicts

| row | verdict | evidence | break-attempt (PASS) | KF ids |
|---|---|---|---|---|
| R2-001 | FAIL | Top bar KF `#333333` y 0-33 (34 px), CF `#333333` y 0-35 plus `#2C2C2C`/`#696969` rows 36-37 (38 px). Boards button KF `#515151` x 10-109 y 4-29 (100x26), CF `#525252` x 11-121 y ~4-33 (111x30, rounded). Right cluster KF bell x 1781-1816, ? 1827-1862, avatar 1873-1908. CF adds a reports button and a "25:00" pill (1814-1902). Masked diff 15852/72922. `measure/A-ae.txt`, `crops/R2-001-topbar.png` | | KF-233 KF-213 KF-234 KF-235 |
| R2-010 | FAIL | Header row KF `#F2F2F2` y 34-79 then `#C3C3C3` at 80 (46 px), CF y 38-74 then `#C2C2C2` at 75 (37 px). Title ink KF x 18-227 y 50-66, CF x 20-224 y 48-64 (2 px right, 2 px up, 5 px narrower). Avatar disc KF x 243-272, CF 239-268. Invite "+" KF 277-306, CF 273-302. KF clock icon at x ~1705-1724 absent in CF. Diff 26907/92112. `crops/R2-010-header-left.png`, `-right.png` | | KF-237 KF-238 KF-309 KF-204 |
| R2-011 | FAIL | KF board-header "CS" avatar carries a black play badge at (268,67) while timing. CF pixel there `#F0F0F0`, no badge. `measure/G.txt`, `crops/R2-011-header-avatar.png` | | none |
| R2-013 | FAIL | KF board frame: 10 px gutter, `#C3C3C3` border at x 10 / 1908 and y 80 / 988, column separators `#E4E4E4` at 390/770/1149/1529, white bodies to y 987. CF: no gutter, no frame, bodies end at y 505 after the last card, page `#EEF0F3` below. Row y 700: KF 13 runs, CF one run of `#EEF0F3`. Diff 1,308,830/1,769,318 (y 76-997). `crops/R2-013-board-bottom.png` | | KF-211 KF-355 KF-126 |
| R2-014 | PASS | No swimlane header row: DOM probe at the header bottom +0/+1/+3 px hits `TD.board-cell`, the only lane element is the body `TR.swimlane-row` (`shots/Z_swimlane_dom.log.json`). The band itself differs (KF `#C3C3C3` rule then 8 px white, CF `#E8EAEE` cell border then 5 px white), owned by R2-013 and R2-036. `crops/R2-014-lane-band.png` | Probed the DOM below the header for any lane-name row at three offsets and listed every `*lane*` element: one body row, no header | KF-152 KF-149 |
| R2-036 | FAIL | Header KF `#EEEEEE` y 81-116 (36 px) then `#C3C3C3`, CF `#EBEBEB` y 86-139 (54 px) after a 10 px `#EEF0F3` gap. "In Review" ink KF 68x11, CF 78x12 (bigger font). "+" KF 22x22 at x 360-381 y 89-110, CF ~11x11 at x 324-335 y 98-108. CF adds a count cell with `#D4D7DC` separators at x 350/383 that KF does not draw. Diff 130064/134330. `measure/R2-036.txt`, `crops/R2-036-colheader.png` | | KF-311 KF-313 KF-203 KF-192 |
| R2-037 | FAIL | "5 / 3" is `#8B0000` 23x9 in both but at y 101-109 (CF) vs 96-104 (KF). Warning line 3 px `#FF8080`: KF x 779-1140 at y 116-118 on the header edge, CF x 778-1139 with anti-aliased round ends at y 121-123 inside the header. Diff 23396/23400. `crops/R2-037-wip.png` | | KF-312 KF-034 |
| R2-039 | FAIL | KF: red `#FF8080` header line y 116-118, then a `#F3F3F3` strip y 119-143 with "1 overdue task" `#8B0000` 69x8 at y 128-135, then a 120 px `#FF8080` underline x 520-639 y 144-146 on a `#D7D7D7` rule. CF: text `#DC2626` 76x8 at y 121-128 inside the grey header, no strip, no lines. `measure/R2-039.txt`, `crops/R2-039-overdue.png` | | KF-218 KF-219 |
| R2-040 | FAIL | 10-12 macOS: KF strip under In Progress (y 124-148, colour-shifted reds above and below), text 74x8 left-aligned at x 381. CF text 79x8 centered at x 491-569 inside the header, no strip. `measure/I.txt`, `crops/R2-040-ip-overdue-1012.png` | | KF-218 |
| R2-041 | FAIL | Collapsed To Do: KF strip inner x 479-505 (27 px), header `#EEEEEE` y 81-115, count "5" `#111111` 7x9 at y 96-104, "TO DO" `#555555` 12x54 in the white body y 146-199 reading top-to-bottom. CF strip x 474-494 (21 px), header white and stretched to y 229, count 10x9, "TO DO" `#4B5563` inside the header y 178-215 reading bottom-to-top. `measure/B.txt`, `crops/R2-041-042-strip.png` | | KF-039 KF-040 KF-125 KF-223 |
| R2-042 | FAIL | KF shows only the count plus a 3 px `#FF8080` line at y 116-118. CF prints vertical "1 overdue task" in `#DC2626` (y 98-167) overlapping the count, and draws no line. The capture settles KF-223's open question: no text. `crops/R2-041-042-strip.png` | | KF-223 |
| R2-043 | FAIL | KF In Progress strip: count `#8B0000` 6x9 at x 664-669, 21 px `#FF8080` line x 656-676 y 116-118, "IN PROGRESS" 12x120 in the body y 147-266. CF: 3 px `#FF8080` line at y 99-101 running through the count, count box 16x9, name in the white header y 105-199. `measure/C.txt`, `crops/R2-043-044-strips.png` | | KF-241 |
| R2-044 | FAIL | KF adjacent strips x 625-651 and 653-679 (27 px each, separators 624/652/680). CF 625-645 and 647-667 (21 px each). | | KF-039 KF-241 |
| R2-047 | FAIL | KF popover box x 56-432 (377 px, `#C3C3C3` border) y 117-183, `#EEEEEE` title band y 118-147, centered title `#666666` 106x12, centered body 71x9. CF box x 5-227 (223 px) y 234-309, white title, left-aligned `#111827` title 117x14, body 90x9. `measure/D.txt`, `crops/R2-047-popover.png` | | KF-223 |
| R2-054 | FAIL | KF "Today" bold `#888888` 35x12 at y 130-141, rule `#D8D8D8` x 1539-1898 y 146, first card at 157. CF "Today" 30x10 at y 152-161 in a lighter grey, rule `#D1D5DB` y 166, first card at 171. Diff 17388/23340. `measure/R2-054.txt`, `crops/R2-054-done-today.png` | | KF-124 KF-326 |
| R2-055 | FAIL | KF 10-12 shows an empty "Today" header (35x12) above "Wednesday, 30 September" (161x12). CF shows only "Wednesday, 30 September" (146x11, lighter). Same in the 10-05 state (`shots/G_pomodoro.png`). `crops/R2-055-done-groups-1012.png` | | KF-326 |
| R2-060 | FAIL | Faces match (`#CCE3FF` etc). Borders differ for 4 of 8 colors (R2-061). Card gap KF 8 px, CF 4 px. One-line card KF 30 px, CF 26 px. Title inset KF (+9,+10), CF (+8,+8). Title width "Write API documentation for v2 endpoints" KF 220 px, CF 229 px. Diff 37632/78000 on the Backlog card region. `crops/R2-060-card-base.png` | | KF-245 KF-191 KF-179 KF-166 KF-167 |
| R2-061 | FAIL | All 8 face colors exact. Borders: Blue KF `#70B0FF` CF `#7FAEF9`, Green `#59D600` vs `#7CD340`, Yellow `#F5CC00` vs `#EECD45`, Red `#FF858F` vs `#EE8B91`. Cyan `#00D6D6`, Purple `#C994FF`, Orange `#FAA200`, Magenta `#FF85FF` exact. Vertical scans x 330 and x 700. `crops/R2-061-colors-todo.png` | | KF-051 KF-191 |
| R2-062 | FAIL | KF icons are grey `#808080` glyphs (lines, grid calendar, checklist, dotted bubble), CF uses outline icons in `#4B5563`. #2 description icon KF 16x12, CF 12x15. #11 icon row KF 164x14 at y 153-166, CF 169x14 at y 172-185. `measure/R2-062.txt`, `crops/R2-062-iconrows.png` | | KF-224 KF-242 KF-054 |
| R2-063 | FAIL | Disc size matches (27 px). Initials: KF "CS" (two letters, header and cards). CF cards show one letter ("A") while the CF header shows "AU" for the same user, so this is not the username: `src/routes.rs:842` takes `chars().next()`, the header uses `user_initials`. Centre offset from card edge KF (19,19), CF (21,19). `crops/R2-063-avatar.png` | | KF-243 KF-029 |
| R2-064 | FAIL | "0h / 4h" KF 30x8 at x 93, CF 34x8 at x 79. "2h 30m / 8h" KF 52x8, CF 59x8. KF text grey, CF darker. `crops/R2-064-time-badge.png` | | KF-054 KF-114 |
| R2-065 | FAIL | Clock pinned to 09-30. #6 KF "Due: Today 5:00 PM (Done)", CF "Due: Wednesday 5:00 PM". #11 KF "Due: Friday 5:00 PM (Done)" 127 px, CF "Due: Friday 5:00 PM" 111 px. CF never says "Today" (`format_due_card`, `src/routes.rs:965`) and has no "(Done)" link. `crops/R2-065-due-relative.png` | | KF-244 KF-053 |
| R2-066 | FAIL | #7 KF: red "!" disc plus "Due: 25 September 5:00 PM (Done)" in `#BE0000`/`#8B0000`, ink 184x16. CF "Due: Friday 5:00 PM" `#DC2626`, ink 112x10, no icon, weekday instead of the date. `crops/R2-066-due-overdue.png` | | KF-219 KF-244 |
| R2-067 | FAIL | 10-12: wording now matches ("2 October 5:00 PM") but CF has no "!" disc, no "(Done)", colour `#DC2626` vs KF dark red, ink 141x9 vs 178x16. `crops/R2-067-due-absolute.png` | | KF-244 |
| R2-068 | FAIL | KF #6 lists all 3 subtasks (done one checked and greyed), 14 px boxes at 25 px pitch. CF lists 2 plus "1 hidden (1 done)", ~11 px boxes at 17 px pitch. #1 same pattern. "Wire up toggle control" KF ink 116x12, CF 115x10. "2 hidden (2 done)" on #11 is in both. `crops/R2-068-subtasks.png` | | KF-224 |
| R2-069 | FAIL | #1 collapsed: KF card x 19-469 y 126-173 (451x48), CF x 7-467 y 237-282 (461x46). Border `#70B0FF` vs `#7FAEF9`. Icon row KF at +27 px, CF +25 px. `measure/B2.txt`, `crops/R2-069-card-collapsed.png` | | KF-224 |
| R2-070 | FAIL | #8 KF 30 px tall (y 418-447), CF 26 px (y 374-399). Title width +4% as R2-060. `crops/R2-070-plain-card.png` | | KF-170 |
| R2-073 | FAIL | Matched state (waiting for break): KF "2h 46m + 42m / 8h" with red "+ 42m" inline, CF "2h 30m / 8h" with no live part. Break-attempt state, timer running (`shots/G_running.png`): CF renders a separate bold line "2h 30m + 0m" under the icon row. `crops/R2-073-074-card11.png` | | KF-103 |
| R2-074 | FAIL | KF dashed `#FAA200` outline (6 on, 3 off) replaces the card border in the waiting-for-break state. CF draws none in that state. While running CF draws `2px dashed rgb(250,162,0)` outside a solid border with a 1 px gap (`G_running.log.json`). `crops/R2-073-074-card11.png` | | KF-052 KF-325 KF-327 |
| R2-109 | FAIL | KF: column "+" opens a centered yellow `#FFFFE0` modal (x 710-1209 y 25-~325) whose color field opens an inline list x 784-1180 y 150-437, 36 px rows, 14 px saturated round dots (`#F7D633`...). CF: "+" opens an anchored popover with 8 swatches, a swatch click opens a separate white "Task color" dialog x 750-1169 y 262-735, 44 px rows, 18 px square face-colour swatches. Overlay KF `#616161`, CF `#6B6C6E`. `measure/E.txt`, `crops/R2-109-color-modal.png` | | KF-302 |
| R2-112 | FAIL | Panel KF x 1472-1731 (260 px) y 40-438 `#3E3E3E`, CF x 1591-1910 (320 px) y 43-320 `#1F2937`. Take break KF `#49A849` 108x42, CF `#16A34A` 98x26. CF adds big "00:00" digits and a "Skip" link. `measure/G.txt`, `crops/R2-112-113-114-panel.png` | | KF-006 |
| R2-113 | FAIL | KF task row `#515151` band y 164-193 with dot and bold name, "Change task" link under it. CF has no task row and no "Change task" in this state. | | KF-010 |
| R2-114 | FAIL | KF footer `#2E2E2E` band y 388-438, labels ink 236x10 at y 422-431. CF no band, labels 258x12 at y 292-303. | | KF-009 KF-158 |
| R2-127 | FAIL | KF top bar x 1500-1780 is all `#333333` (no pill) with the popup open. CF shows the pill at x 1814-1902 next to the open popup. `crops/R2-127-pill-topbar.png` | | KF-097 |
| R2-172 | FAIL | 10-12 menu: KF full-height side panel x 1124-1373 (250 px) `#F6F6F6` with "Menu" title band and close X, green Get Premium button x 1134-1363. CF floating dropdown x 1138-1353 (216 px) y 77-480, white, no title, extra "Save board as template", "Get Premium" as plain text. "Filter" ink KF 33x12, CF 29x10. `measure/H.txt`, `crops/R2-172-menu-1012.png` | | KF-353 KF-099 KF-135 KF-204 |
| R2-173 | FAIL | 09-30: KF panel `#F7F7F7` x 1669-1918 y 34-998, Color legend toggle orange (`#F9C243`/`#F5A033`). CF dropdown white x 1683-1898 y ~74-480, toggle green `#56B74F`. `measure/F.txt`, `crops/R2-173-menu.png` | | KF-353 KF-183 |
| R2-178 | PASS | Legend off: neither frame draws a legend strip, rule or band at y 965-997 (KF board border at 988 then page, CF page). No `class="color-legend"` in the HTML of the new "Dale fidelity test" board or the GM board with legend off. Residual diff in that band is the board body (R2-013). | Turned the legend on (F, I) to confirm the off state is a real toggle state: on renders the strip at the exact KF 09-30 geometry, off removes it. Checked a freshly created board for the default | KF-183 KF-222 |
| R2-179 | FAIL | Geometry exact at 1919: segments 240 px at identical boundaries, rule y 965, band 966-974, edge 975, face 976-997, labels identical ink (Yellow 34x9 at x 103-136, same mass 20153). Colours differ on all 8: KF faces equal the card faces (`#FFFFE0 #DBFFC2 #CCE3FF #FFCCD0 #FFEAC2 #EDDBFF #FFE0FF`) and edges equal the card borders (`#F5CC00 #59D600 #70B0FF ...`), CF uses `#FFFFDE #DCFCBE #C9DDFD #F7C9CA ...` and `#EDC82F #6BCA2D #71A4FB ...`, rule `#BBBBBB` vs `#C3C3C3`, band `#F0F0F0` vs `#F2F2F2`. Diff 54892/55044 in x 0-1667. At 1380 the boundaries differ by up to 2 px (KF 171/173 split, CF 173/172). `crops/R2-179-legend-0930.png`, `-1012.png` | | KF-354 KF-249 KF-210 KF-212 KF-205 |
| R2-183 | FAIL | Shell is pixel-exact once shifted 34 px down: nav (General to Advanced) diff 0/49440, H1 area diff 0/27060 (board named "General"), page and shadows diff 133 each side (container bottom, longer CF content). Absolute frame fails because CF has no top bar (everything 34 px up, KF-357). Also the "View board" chevron differs (11x16 box, max delta 85, sits ~1 px lower). `measure/K2.txt`, `crops/R2-183-184-shell.png`, `crops/R2-183-chevron.png` | | KF-080 KF-153 KF-331 |
| R2-184 | FAIL | KF `#333333` top bar y 0-33, CF none, diff 64770/64770. OPEN KF-357 covers it. | | KF-357 |
| R2-185 | FAIL | KF dims Custom fields, Custom roles, API & Webhooks, Add task from email (ink 108/105/135/157 px wide, grey). CF adds "Premium" pills to the first two (ink 168/165 px) and renders the last two at full `#444444` (ink mass 92314 vs 51702). `crops/R2-185-dimmed-nav.png` | | KF-111 |
| R2-186 | FAIL | Content diff 258142/584100 (shift-compensated). OPEN KF-356 covers it. `crops/R2-186-colors-content.png` | | KF-356 KF-081..KF-087 KF-107..KF-110 KF-300 KF-301 KF-303 |
| R2-199 | FAIL | KF: dark top bar, `#D3E8F2` page, settings-style box x 660-1259 with `#E8E8E8` title band, centered title 314x21, `#F2DEDE` alert y 131-242, Cancel left, Delete `#D9534F` then Archive `#2185D0` right, footer `#333333` y 970-999. CF: page-specific "ChipFlow - Delete board" bar, `#EEF0F3` page, white box, left title 261x15 in red, no alert box, Cancel / Archive `#2563EB` / Delete `#DC2626`, no footer. `measure/J.txt`, `crops/R2-199-delete.png` | | KF-089 KF-304 KF-305 |

## 3. FAILs

- R2-001: top bar 38 px vs KF 34, Boards button 111x30 vs 100x26, extra reports button and pill. KF-233 (RELEASED) recorded KF as 38 px with a 47 px button. The recovered capture contradicts both values, so the release matched a wrong reference. MEDIUM. `crops/R2-001-topbar.png`
- R2-010: header row 37 px vs 46, everything in it shifted 2 to 4 px, KF clock icon missing. MEDIUM. `crops/R2-010-header-left.png`
- R2-011: no timing badge on the header avatar. LOW. `crops/R2-011-header-avatar.png`
- R2-013: no 10 px gutter or `#C3C3C3` frame, column bodies stop after the last card instead of running to y 988, page `#EEF0F3` vs `#F2F2F2`. Confirms the harness worker's gap. HIGH (whole-board silhouette). KF-211/KF-355 regress in the sense that the bodies are white only behind cards. `crops/R2-013-board-bottom.png`
- R2-036: header 54 px vs 36, name larger, "+" 11 px vs 22 px, extra count cell. KF-311 (RELEASED) raised the row to ~41 px from a lost GM-031 claim; the capture shows 36 px. KF-313/KF-203 count cells have no counterpart in any recovered capture. MEDIUM. `crops/R2-036-colheader.png`
- R2-037: WIP count +5 px low, line inside the header with round ends. LOW. `crops/R2-037-wip.png`
- R2-039: overdue text inside the header in `#DC2626` instead of KF's separate strip with `#8B0000` text, red header line and 120 px underline. MEDIUM. `crops/R2-039-overdue.png`
- R2-040: same as R2-039 on In Progress at 10-12. MEDIUM. `crops/R2-040-ip-overdue-1012.png`
- R2-041: collapsed strip 21 px vs 27, header white and 145 px tall, vertical name in the header and rotated the other way. HIGH (breaks the whole header row height). Regression against KF-223/KF-241. `crops/R2-041-042-strip.png`
- R2-042: strip prints "1 overdue task" text over the count and omits the red line. KF-223's "verified live" claim is contradicted by the capture. MEDIUM. `crops/R2-041-042-strip.png`
- R2-043: warning line drawn through the count near the top instead of a 3 px line at the header bottom. RELEASED KF-241 regresses. MEDIUM. `crops/R2-043-044-strips.png`
- R2-044: strips 21 px vs 27 px. MEDIUM. `crops/R2-043-044-strips.png`
- R2-047: popover 223 px vs 377 px, no title band, left-aligned, placed 117 px lower. LOW. `crops/R2-047-popover.png`
- R2-054: "Today" header smaller, lighter, rule and spacing differ. LOW. `crops/R2-054-done-today.png`
- R2-055: no empty "Today" header above older Done groups. Predicted by the inventory, now proven. MEDIUM. `crops/R2-055-done-groups-1012.png`
- R2-060: 4 border shades wrong, gap 4 vs 8 px, one-line card 26 vs 30 px, title 4% wider. MEDIUM. `crops/R2-060-card-base.png`
- R2-061: Blue, Green, Yellow, Red borders off. KF-245 (FIX) regresses for those four. MEDIUM. `crops/R2-061-colors-todo.png`
- R2-062: different icon set and colour. LOW. `crops/R2-062-iconrows.png`
- R2-063: cards show one initial while the header shows two. LOW. `crops/R2-063-avatar.png`
- R2-064: time text wider and darker. LOW. `crops/R2-064-time-badge.png`
- R2-065: same-day due says the weekday, no "(Done)" link. MEDIUM (wording visible on every board). `crops/R2-065-due-relative.png`
- R2-066: overdue line lacks the "!" disc, uses the weekday for a past date and `#DC2626`. MEDIUM. `crops/R2-066-due-overdue.png`
- R2-067: absolute date matches, icon, "(Done)" and colour do not. LOW. `crops/R2-067-due-absolute.png`
- R2-068: always truncates to 2 subtasks, smaller boxes and pitch. MEDIUM. Confirms the harness worker's gap. `crops/R2-068-subtasks.png`
- R2-069: collapsed card 46 vs 48 px, wider, insets differ. LOW. `crops/R2-069-card-collapsed.png`
- R2-070: plain card 26 vs 30 px. MEDIUM (every one-line card). `crops/R2-070-plain-card.png`
- R2-073: live time is a separate bold line while running and absent while waiting for break. MEDIUM. `crops/R2-073-074-card11.png`
- R2-074: outline vanishes in the waiting-for-break state, and while running it sits outside a solid border. KF-325/KF-327 RELEASED partly regress. MEDIUM. `crops/R2-073-074-card11.png`
- R2-109: two-step popover plus separate dialog instead of KF's single centered modal with an inline named dropdown. KF-302 RELEASED does not match the capture. MEDIUM. `crops/R2-109-color-modal.png`
- R2-112: panel size, colours and button differ, extra digits and Skip. MEDIUM. `crops/R2-112-113-114-panel.png`
- R2-113: no task row or "Change task". MEDIUM. same crop
- R2-114: footer has no dark band, labels larger. LOW. same crop
- R2-127: pill stays visible with the popup open. LOW. `crops/R2-127-pill-topbar.png`
- R2-172: floating dropdown instead of KF's full-height side panel. KF-353 RELEASED covers only the toggles. MEDIUM. `crops/R2-172-menu-1012.png`
- R2-173: same panel difference, toggle green vs orange. MEDIUM. `crops/R2-173-menu.png`
- R2-179: legend colours were copied from Chip's colour-managed macOS screenshot. KF-354's own KanbanFlow line lists `#ffffde/#dcfcbe/#c9ddfd`, which are the shifted values. Geometry is exact. MEDIUM. `crops/R2-179-legend-0930.png`
- R2-183: exact apart from the missing top bar (OPEN KF-357) and a ~1 px chevron offset. LOW once KF-357 lands. `crops/R2-183-chevron.png`
- R2-184: OPEN KF-357. `crops/R2-183-184-shell.png`
- R2-185: Premium pills added, two items not dimmed. LOW. `crops/R2-185-dimmed-nav.png`
- R2-186: OPEN KF-356. `crops/R2-186-colors-content.png`
- R2-199: delete page shares nothing with KF's: no global top bar, no settings chrome, no alert box, button order and colours, no footer. MEDIUM. `crops/R2-199-delete.png`

## 4. Proposed new defects

### KF-NEW-pixel-1 — Board has no framed gutter and column bodies stop after the last card [HIGH]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-013-board-bottom.png`, `$P/measure/A-ae.txt`
- KanbanFlow: at 1919x998 the board sits in a 10 px `#F2F2F2` gutter with a 1 px `#C3C3C3` frame at x 10 / 1908 and y 80 / 988. Columns are 379 px with `#E4E4E4` separators at x 390/770/1149/1529, and the white bodies run to y 987 whatever the card count.
- ChipFlow: the table spans x 0-1918 with no frame, the bodies end at y 505 (last card plus padding), and `#EEF0F3` page shows below. Diff 1,308,830 of 1,769,318 px below the header row.
- Fix direction: add the gutter and frame, and stretch the column bodies to the viewport bottom (or to the legend when it is on).

### KF-NEW-pixel-2 — Top bar is 38 px with a larger Boards button and an extra cluster [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-001-topbar.png`
- KanbanFlow: `#333333` bar 34 px. Boards button `#515151` 100x26 at (10,4). Right side holds only bell (x 1781-1816), help (1827-1862) and avatar (1873-1908) buttons, no pill in any idle capture.
- ChipFlow: 36 px of `#333333` plus 2 shaded rows (38 px), Boards button 111x30 with rounded corners at (11,4), plus a reports button and the timer pill. KF-233 targeted 38 px and a 47 px button from a lost GM-001 note that the recovered capture contradicts.
- Fix direction: 34 px bar, 100x26 button, and settle the pill rule (inventory R2-002) before restyling the right cluster.

### KF-NEW-pixel-3 — Board header row is 37 px and lacks the clock icon [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-010-header-left.png`, `$P/crops/R2-010-header-right.png`
- KanbanFlow: `#F2F2F2` row y 34-79 (46 px) with a `#C3C3C3` rule at 80. Title ink x 18-227 y 50-66. Right group: clock, funnel, pencil, Menu.
- ChipFlow: row y 38-74 (37 px), title ink x 20-224 y 48-64, avatar and invite discs 4 px left, no clock icon.
- Fix direction: 46 px row, title and discs at KF positions, add the clock (time) icon left of the funnel.

### KF-NEW-pixel-4 — Column header is 54 px with a small "+" and an extra count cell [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-036-colheader.png`, `$P/measure/R2-036.txt`
- KanbanFlow: `#EEEEEE` header y 81-116 (36 px) with a `#C3C3C3` bottom rule, bold name ("In Review" ink 68x11), 22x22 green "+" flush right (x 360-381 in Backlog), no count cell in expanded headers.
- ChipFlow: 10 px gap then `#EBEBEB` header y 86-139 (54 px), name ink 78x12, "+" about 11x11, a count cell with `#D4D7DC` separators. KF-311 and KF-313 were built from lost GM-031/032 notes.
- Fix direction: match the capture (36 px band, KF name size, 22 px "+", drop the count cell or confirm it from a fresh capture).

### KF-NEW-pixel-5 — Overdue indicator renders inside the header instead of KF's strip [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-039-overdue.png`, `$P/crops/R2-040-ip-overdue-1012.png`
- KanbanFlow: the column header gets a 3 px `#FF8080` bottom line, then a `#F3F3F3` strip (25 px) holds "1 overdue task" in bold `#8B0000`, underlined by a 120 px `#FF8080` bar on a `#D7D7D7` rule.
- ChipFlow: "1 overdue task" in `#DC2626` inside the grey header, no strip, no lines.
- Fix direction: render the strip below the header with KF's colours and lines.

### KF-NEW-pixel-6 — Collapsed column strip is broken: tall white header, overlapping text, wrong rotation [HIGH]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Regresses KF-223 and KF-241. Evidence: `$P/crops/R2-041-042-strip.png`, `$P/crops/R2-043-044-strips.png`, `$P/measure/B.txt`, `$P/measure/C.txt`
- KanbanFlow: strip 27 px wide. Header cell `#EEEEEE` holds only the count (black, or `#8B0000` when over WIP) and a 3 px `#FF8080` line at its bottom when the column is over WIP or holds overdue tasks. The column name is vertical in the white body, reading top to bottom.
- ChipFlow: strip 21 px. The header cell is white and grows to ~145 px, which stretches every column header. "N overdue task(s)" prints vertically in red over the count, the over-WIP line crosses the count at y 99-101, and the name sits in the header reading bottom to top.
- Fix direction: keep the strip header at the normal header height, move the name into the body with KF's rotation, drop the overdue text, draw the line at the header bottom.

### KF-NEW-pixel-7 — Card borders, spacing and title metrics differ [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Regresses KF-245 for four colours. Evidence: `$P/crops/R2-061-colors-todo.png`, `$P/crops/R2-060-card-base.png`, `$P/crops/R2-070-plain-card.png`
- KanbanFlow: borders Blue `#70B0FF`, Green `#59D600`, Yellow `#F5CC00`, Red `#FF858F`. Cards 8 px apart, one-line card 30 px, title inset (9,10).
- ChipFlow: `#7FAEF9`, `#7CD340`, `#EECD45`, `#EE8B91` (the palette `border` values). Cards 4 px apart, one-line card 26 px, inset (8,8), title text about 4% wider (220 vs 229 px on #8).
- Fix direction: use KF's border hexes, 8 px gap, KF card padding and title font size.

### KF-NEW-pixel-8 — Card icon set and colour differ [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-062-iconrows.png`
- KanbanFlow: grey `#808080` glyphs (lined page, grid calendar, checklist, dotted speech bubble, clock), description icon 16x12.
- ChipFlow: outline icons in `#4B5563`, description icon 12x15.
- Fix direction: KF glyph shapes and grey.

### KF-NEW-pixel-9 — Card avatars show one initial [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-063-avatar.png`
- KanbanFlow: card avatars show the same two initials as the header ("CS").
- ChipFlow: card chips use the first character only (`src/routes.rs` MemberChip, around line 842), so the same user is "AU" in the header and "A" on cards.
- Fix direction: use `user_initials` for card chips.

### KF-NEW-pixel-10 — Card due line wording and decoration differ from KF [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-065-due-relative.png`, `$P/crops/R2-066-due-overdue.png`, `$P/crops/R2-067-due-absolute.png`
- KanbanFlow (clock 2026-09-30 15:22 CDT): "Due: Today 5:00 PM (Done)", "Due: Friday 5:00 PM (Done)", overdue "Due: 25 September 5:00 PM (Done)" with a red "!" disc and `#BE0000`/`#8B0000` text. #4 (due 30 October) shows no due line under the default "active due in 7 days".
- ChipFlow: "Due: Wednesday 5:00 PM" (no "Today"), past dues within 7 days as a weekday ("Friday"), no "!" disc, `#DC2626`, no "(Done)" link, and #4 shows "Due: 30 October 11:59 PM" because the card line ignores the 7-day window.
- Fix direction: "Today" for same-day, absolute date for past dues, the "!" disc and KF reds, the "(Done)" link, and honour the 7-day window on the card line.

### KF-NEW-pixel-11 — Inline subtasks always truncate to two and are smaller [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-068-subtasks.png`
- KanbanFlow: a card with 3 subtasks lists all 3, done ones checked and greyed. #11 with 4 (2 done) lists the 2 open ones plus "2 hidden (2 done)". Checkboxes 14 px at 25 px pitch.
- ChipFlow: always the first 2 plus "N hidden (M done)", checkboxes about 11 px at 17 px pitch.
- Fix direction: KF's truncation rule (all when the list is short, hide done ones first) and its row metrics.

### KF-NEW-pixel-12 — Done column omits the empty "Today" header and styles group headers lighter [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-055-done-groups-1012.png`, `$P/crops/R2-054-done-today.png`
- KanbanFlow: "Today" always heads the Done column, empty if nothing finished today, then older groups ("Wednesday, 30 September"). Headers bold `#888888`, rule `#D8D8D8`.
- ChipFlow: groups only from task completion dates, so no "Today" on 10-05 or 10-12, headers smaller and lighter, rule `#D1D5DB`.
- Fix direction: always emit "Today", KF header style.

### KF-NEW-pixel-13 — Column details popover styling and anchor differ [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-047-popover.png`
- KanbanFlow: 377 px box anchored at the header bottom, `#EEEEEE` title band, centered grey title and body, close X.
- ChipFlow: 223 px rounded white card with shadow, left-aligned near-black title, below the (taller) header.
- Fix direction: KF box, band and alignment.

### KF-NEW-pixel-14 — New-task colour picker is a popover plus a second dialog, not KF's modal [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). KF-302 RELEASED does not match the recovered capture. Evidence: `$P/crops/R2-109-color-modal.png`, `$P/measure/E.txt`
- KanbanFlow: column "+" opens one centered modal tinted with the default colour (`#FFFFE0`, x 710-1209), "Task name..." field, colour row whose dropdown lists 8 rows (36 px pitch, 14 px round dots, bold default, ⋮ per row).
- ChipFlow: "+" opens an anchored popover with 8 swatches, clicking one opens a white "Task color" dialog (44 px rows, 18 px square swatches in face colours).
- Fix direction: replace the popover with KF's centered modal and inline dropdown.

### KF-NEW-pixel-15 — Board menu is a floating dropdown instead of KF's side panel [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-172-menu-1012.png`, `$P/crops/R2-173-menu.png`
- KanbanFlow: full-height 250 px panel on the right (`#F7F7F7`), "Menu" title band with close X, grouped items, orange toggles, green "Get Premium" button.
- ChipFlow: 216 px white dropdown under the button, no title, green toggles, an extra "Save board as template" item, plain-text "Get Premium".
- Fix direction: KF panel layout and toggle colour, decide where "Save board as template" lives.

### KF-NEW-pixel-16 — Legend colours were copied from a colour-managed screenshot [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-179-legend-0930.png`, `$P/measure/F.txt`, `$P/measure/I.txt`
- KanbanFlow (Linux 09-30 capture): segment faces equal the card faces (`#FFFFE0 #DBFFC2 #CCE3FF #FFCCD0 #FFEAC2 #EDDBFF #FFE0FF`), top edges equal the card borders (`#F5CC00 #59D600 #70B0FF #FF858F #FAA200 #C994FF #FF85FF`), rule `#C3C3C3`, band `#F2F2F2`.
- ChipFlow: `#FFFFDE #DCFCBE #C9DDFD ...`, edges `#EDC82F #6BCA2D #71A4FB ...`, rule `#BBBBBB`, band `#F0F0F0`. These are the pixel values of Chip's 10-12 macOS screenshot, which shows KF's `#333333` top bar as `#2D2D2D`, so they are display-profile shifted. KF-354's KanbanFlow line records the shifted values. Geometry matches exactly.
- Fix direction: derive legend colours from the board palette (card face and border) and the frame from `#C3C3C3`/`#F2F2F2`. Treat any hex sampled from the 10-12 screenshots as untrusted.

### KF-NEW-pixel-17 — Pomodoro popup layout, colours and content differ [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-112-113-114-panel.png`, `$P/measure/G.txt`
- KanbanFlow (pomodoro done, waiting for break): 260 px `#3E3E3E` panel, "Time until break" with a 108x42 `#49A849` Take break button, a `#515151` task row with "Change task", a TODAY log, a `#2E2E2E` footer band with four icon tabs.
- ChipFlow: 320 px `#1F2937` panel, large "00:00" digits, a 98x26 `#16A34A` button, a "Skip" link, no task row, no footer band.
- Fix direction: KF panel width, colours and sections.

### KF-NEW-pixel-18 — Timer pill stays visible while the popup is open [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-127-pill-topbar.png`
- KanbanFlow: no pill in the top bar while the popup is open.
- ChipFlow: the pill (x 1814-1902) stays next to the open popup.
- Fix direction: hide the pill while the popup is open.

### KF-NEW-pixel-19 — Timing indicators on cards and avatars differ [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Partly regresses KF-325/KF-327. Evidence: `$P/crops/R2-073-074-card11.png`, `$P/crops/R2-011-header-avatar.png`, `$P/shots/G_running.log.json`
- KanbanFlow: while timing (including waiting for break) the card keeps a dashed `#FAA200` outline in place of its border, the time readout shows a red inline "+ 42m", and the header and card avatars carry a small play badge.
- ChipFlow: outline only while the pomodoro runs, drawn outside a solid border, the live time is a separate bold line ("2h 30m + 0m"), and no avatar badges.
- Fix direction: keep the outline through the break wait, inline red "+ Nm", add the avatar badges.

### KF-NEW-pixel-20 — Settings nav adds Premium pills and dims the wrong items [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-185-dimmed-nav.png`
- KanbanFlow: Custom fields, Custom roles, API & Webhooks and Add task from email are dimmed, with no badges.
- ChipFlow: "Premium" pills on the first two, API & Webhooks and Add task from email at full `#444444`.
- Fix direction: drop the pills and dim the four items alike, or record the deliberate difference for the two ChipFlow implements.

### KF-NEW-pixel-21 — Delete board page does not use KF's settings chrome [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (pixel). Evidence: `$P/crops/R2-199-delete.png`, `$P/measure/J.txt`
- KanbanFlow: global dark top bar, `#D3E8F2` page, 600 px box with `#E8E8E8` title band and centered "Delete board: Dale fidelity test", `#F2DEDE` alert with three paragraphs, Cancel left, Delete board `#D9534F` and Archive board `#2185D0` right, `#333333` site footer.
- ChipFlow: a page-specific "ChipFlow - Delete board" bar, `#EEF0F3` page, white box with a red left-aligned title, no alert box, Cancel / Archive `#2563EB` / Delete `#DC2626`, no footer.
- Fix direction: build the page on the board-settings shell (after KF-357) with KF's alert and button order.

## 5. BLOCKED

None. Rows that need KanbanFlow account state were handled by masking or
did not depend on it: the bell badge and cursor were outside every measured
region, initials were compared by letter count only, and the 10-05 wall-clock
times and "58m"/"+ 42m" numbers were not compared.

## Observations (no KanbanFlow reference, or outside these rows)

- The built-in "Pomodoro board" template creates 10 enabled plain colours, not the 4 renamed plus 6 disabled palette the KF-331 reference shows. I configured the palette by PATCH.
- The KF-331 reference's H1 "Board settings: General" names the board. A ChipFlow board named "General" renders that H1 with 0 differing pixels. The inventory's note that the H1 does not name the board is wrong.
- The 10-12 macOS captures left-align column names and the overdue strip text, the 09-30 Linux captures center them. ChipFlow centers.
- ChipFlow's column default hides inline subtasks (`prop_subtasks` unset), while the KF Golden Master board shows them. Whether KF's default shows them is unverified.
- Timer log `day_key` is the UTC date: an entry at 22:26 CDT on 10-05 was keyed `2026-10-06`.
- Card due line shows a bare-date due (#4, `2026-10-30`) as "30 October 11:59 PM" (fixture gap 9, KF-348).
- On macOS both KanbanFlow (10-12 screenshot) and ChipFlow render card text in San Francisco, so the system font stack is not a macOS divergence.
- KanbanFlow's two near-duplicate 09-30 captures differ in 328 px, all at the cursor. Its renderer is deterministic, so any non-cursor diff in this batch is ChipFlow's.

DONE: pixel 2 PASS 41 FAIL 0 BLOCKED

