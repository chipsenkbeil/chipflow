# ChipFlow readiness inventory (R2 step 1), re-derived 2026-10-16

## 1. Header

- Date: 2026-10-16. Repo `main` at `62d573d`; production build `6c8d773`.
  Derived read-only; nothing in the repo was changed.
- Sources:
  - `.dev/HANDOFF.md` ("What is lost", R2, R3, R4) and `AGENTS.md`.
  - `.dev/kf-parity-defects.md`: 357 `### KF-NNN` sections, parsed with the AGENTS.md contract (first section per id wins, first `- Status:` line is the state).
  - `.dev/golden-master-2026-09-30.md` and `.dev/golden-master-board.json` (the fixture).
  - The 13 captures in `.dev/evidence/golden-masters/` and `.dev/evidence/kf331-kanbanflow-settings-colors-recovered-20261016.png`. I opened every image and measured its size.
  - `.dev/fidelity-status.md` ("Discovered behaviors"), `.dev/kf-frame-review-report.md`, `.dev/kf-video-catalog/v1-v9.md`, and `.dev/evidence/final-battery-20261001.md`. I used these for surfaces only, not for verdicts.
  - `src/routes.rs` (every `.route(` call), `templates/*.html` (every dialog, menu, overlay and page), `static/app.js` (board menu, reports and timer handlers), and `agent-guide.md` together with the `#[utoipa::path]` and input structs in `src/routes.rs` for what the API can seed.
  - Old `GM-NNN` ids still cited in the tracker, CSS, JS, templates and evidence. Ninety-nine of the 171 ids can still be recovered (Appendix A).
- How rows were derived: I made one row per distinct SURFACE x STATE. Surfaces come from three places: the routes and templates (so every page and dialog ChipFlow ships is listed, even ones no defect touched), the KanbanFlow and ChipFlow lines of every tracker section, and the surface catalogs. Duplicates were merged. When several defects describe the same state, one row cites them all.
- Check types:
  - `pixel`: a recovered capture exists and the state can be reproduced on a seeded local mirror. Clock text, the notification badge and the mouse cursor are masked. Framing must match the capture (viewport, DPR, scroll position).
  - `text`: the reference is a description in the tracker, the fixture docs or the catalogs, or it is a capture that cannot anchor a pixel diff (see section 4).
  - `functional`: behavior with no visual reference.
  - `none`: should be checked, but no reference exists and the state cannot be observed in a meaningful way. The row gives the reason.
  - `n/a`: KanbanFlow parity does not apply, because ChipFlow leaves the feature out on purpose (single admin, free, no recycle bin). The row gives the reason.
- KF state codes come from the first `Status:` line: `REL` RELEASED, `FIX` FIXED, `OPEN`, `INV` INVALID, `CLO` CLOSED, `CORR` CORRECTED, `RETR` RETRACTED, and `nostat` (no `Status:` line at all). KF-001 to KF-188 are mostly `nostat`: they predate the status-line contract, and `.dev/kf-fix-progress.md` records them as fixed. I did not re-verify them.
- Fixture profiles used in the "fixture needs" column:
  - **GM**: the full `.dev/golden-master-board.json` board, seeded with these choices:
    - 5 columns, In Progress WIP 3.
    - 8 colors enabled (Yellow, Green, Blue, Red, Orange, Purple, Magenta, Cyan); Brown and White disabled; Yellow is the default.
    - The admin username renders as "CS".
    - Legend off, all columns expanded, a single Default swimlane.
    - Task #11 has 2.5h logged.
    - Tasks are numbered in JSON order: #1 "Set up CI/CD pipeline for staging" through #22.
  - **GM@0930**: GM with the server and browser clocks pinned to 2026-09-30, about 15:22 America/Chicago, and the three Done tasks completed that day.
  - **GM@1012**: GM viewed on a date after 2026-10-09, so due dates render as absolute dates, To Do collapsed, and Done tasks completed 2026-09-30. R3 must be resolved, or the #11 time text masked.
  - **POMO**: a board created from the built-in Pomodoro template, with 4 enabled colors named "1 Pomodoro" (Default), "2 Pomodori", "3 Pomodori", ">3 Pomodori" and 6 disabled colors.
  - **MULTI**: three or more boards, one of them a favorite, one archived, and one custom template.
  - **LOG**: time entries on several days: a completed pomodoro, a pomodoro stopped with a reason, a manual entry, a 31-second session, and an entry with labels and a comment.
  - **TIMER**: GM with a short work time, so a pomodoro can reach "Take break", and the timer started on #11.
  - **FRESH**: an empty database before `/setup`.

## 2-3. Inventory by area

### Board (chrome, header row, boards navigation, swimlanes, layout editor, filter, board pages)

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-001 | Top bar | idle, signed in | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930; mask bell badge and cursor | KF-233 REL KF-169 nostat KF-213 FIX KF-022 nostat KF-030 nostat KF-031 nostat KF-029 nostat KF-234 REL KF-235 FIX | Top bar #333333, 36px (GM-001 values preserved in style.css). KF-235 bell glyph is FIX, not REL |
| R2-002 | Top bar | idle timer: is the pill visible? | `kanbanflow-golden-master-board-20260930.png` `kanbanflow-board-todo-collapsed-20260930.png` `kanbanflow-board-menu-open-20261012.png` `kanbanflow-board-20261012.png` | text | GM | KF-098 nostat KF-239 REL KF-024 nostat KF-025 nostat KF-026 nostat KF-132 nostat | All 11 idle board captures show NO pill in KanbanFlow's top bar. ChipFlow always renders the idle "▶ 25:00 ▾" pill (renderPill in static/app.js). Videos v1/v3 do show an idle pill. Settle the rule before pixel-diffing the top-right corner |
| R2-003 | Top bar | unread notification badge on bell | `kanbanflow-golden-master-board-20260930.png` `kanbanflow-board-menu-open-20261012.png` | n/a | — | KF-030 nostat KF-310 FIX | KanbanFlow account state (badge 1 / 4). ChipFlow has no notification source. Mask in every pixel diff |
| R2-004 | Top bar | notifications panel open | none | functional | GM | KF-310 FIX | ChipFlow renders a static "No notifications" panel |
| R2-005 | Top bar | help "?" clicked | none | functional | GM | KF-031 nostat | ChipFlow opens the shortcuts dialog. The KanbanFlow destination was never captured |
| R2-006 | Top bar | account avatar menu open ("Signed in as", Sign out) | none | functional | GM | KF-195 FIX KF-029 nostat KF-234 REL | Sign out POSTs /logout |
| R2-007 | Top bar | reports icon menu open | none | text | GM | KF-236 FIX | GM-128 text: the reports list. KF-236 is FIX, not REL |
| R2-008 | Boards dropdown | open: search, Favorites, All boards, + New board | none | text | MULTI | KF-330 REL KF-240 REL KF-088 nostat KF-172 nostat KF-174 nostat KF-023 nostat KF-196 FIX | GM-021/022/026 were text-source masters (no screenshot) |
| R2-009 | Boards dropdown | search filters the list; drag a board into Favorites | none | functional | MULTI | KF-088 nostat KF-240 REL | "Favorites" appears in no evidence file |
| R2-010 | Board header row | board name, member avatar, invite +, clock, funnel, pencil, Menu | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930; username renders "CS" | KF-123 nostat KF-133 nostat KF-173 nostat KF-178 nostat KF-237 FIX KF-238 REL KF-309 REL KF-032 nostat KF-204 FIX KF-122 nostat KF-033 nostat KF-187 nostat | KF-237 is FIX, not REL |
| R2-011 | Board header row | member avatar while the user is timing (small timer badge on "CS") | `kanbanflow-pomodoro-panel-20261005.png` | pixel | TIMER | — | Visible in the 10-05 capture. No defect or evidence mentions it |
| R2-012 | Board header row | invite + clicked | none | n/a | — | KF-237 FIX | Single-admin model. ChipFlow shows an "Invite to board / single-user" dialog |
| R2-013 | Board body | flush grid, fluid columns, white column bodies at 1919x998 | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930 | KF-126 nostat KF-211 FIX KF-355 REL KF-170 nostat KF-171 nostat | KF-355 white bodies, measured #ffffff |
| R2-014 | Board body | single Default swimlane, no swimlane header row | `kanbanflow-golden-master-board-20260930.png` | pixel | GM | KF-152 nostat KF-149 nostat | KanbanFlow's free plan has no swimlanes. ChipFlow keeps one lane |
| R2-015 | Swimlanes | two or more swimlanes with labels | none | functional | GM plus 2 extra lanes | KF-131 nostat KF-162 nostat | No KanbanFlow reference exists (swimlanes are premium there, per golden-master-2026-09-30.md) |
| R2-016 | Swimlanes | folded swimlane strip | none | text | GM plus 2 lanes, one folded | KF-201 FIX KF-202 FIX KF-314 REL KF-248 INV | GM-037 values survive in style.css (regular weight, #111111) |
| R2-017 | Swimlanes | add/rename dialog, swimlane menu, delete | none | functional | GM plus 2 lanes | KF-156 nostat KF-160 nostat | The `#swimlane-menu` element appears in no defect |
| R2-018 | Swimlanes | first lane's cards clickable under the sticky header; "+" works with all lanes collapsed | none | functional | GM plus 2 lanes | KF-147 nostat KF-206 FIX | |
| R2-019 | Layout editor | open via pencil: column/swimlane lists, green Add column at right end, pill hidden | none | text | GM | KF-180 nostat KF-246 REL KF-247 INV | GM-043 text-source. KF-247 INV. The frame review saw the pill absent in this view |
| R2-020 | Layout editor | drag-reorder columns and swimlanes | none | functional | GM | KF-180 nostat KF-048 nostat | |
| R2-021 | Filter panel | open, nothing selected | none | text | GM | KF-096 nostat KF-134 nostat KF-175 nostat KF-215 FIX | v1 S14 catalog |
| R2-022 | Filter panel | color filter applied; column counts follow the filter | none | text | GM | KF-217 FIX KF-296 REL KF-161 nostat KF-297 REL | GM-125/GM-127 text |
| R2-023 | Filter panel | user filter, including "Timer users" | none | functional | GM, timer running | KF-096 nostat | Frame review v1-02947 |
| R2-024 | Filter panel | due-date filter, including undated tasks | none | functional | GM@0930 | KF-159 nostat | |
| R2-025 | Filter panel | label filter; clicking a card label chip filters | none | functional | GM, labels shown | KF-165 nostat | |
| R2-026 | Filter panel | Remember filter across reload; Bookmarks | none | functional | GM | KF-176 nostat | |
| R2-027 | Boards page `/boards/new` | board list and template picker | none | text | MULTI | KF-190 FIX KF-155 nostat KF-163 nostat | fidelity-status DoD item 8 |
| R2-028 | Boards page `/boards/new` | create from built-in Pomodoro template | none | text | FRESH then create | KF-307 REL KF-308 REL KF-190 FIX | GM-141 text: To-do / Do today / In progress / Done with starter tasks |
| R2-029 | Boards page `/boards/new` | delete a custom template (styled confirm) | none | functional | MULTI | KF-112 nostat | |
| R2-030 | Save as template dialog | save, then instantiate: tasks carried over | none | functional | GM | KF-163 nostat KF-164 nostat | |
| R2-031 | Archived boards `/boards/archived` | empty state; list; Unarchive | none | functional | MULTI | KF-305 REL | No defect or evidence mentions Unarchive |
| R2-032 | Keyboard shortcuts dialog | open (9 rows) | none | text | GM | KF-095 nostat KF-186 nostat KF-276 REL | GM-093 text |
| R2-033 | Keyboard shortcuts | T / P / Y / E / V / . / Delete / Esc / Ctrl+Enter | none | functional | GM, #11 modal open | KF-094 nostat KF-298 REL KF-299 REL KF-273 FIX | |
| R2-034 | Floating popups | dragging floating popups; toast placement | none | functional | GM | KF-209 FIX | |
| R2-035 | Whole app | persistence across a server restart | none | functional | GM plus LOG | — | fidelity-status DoD item 7; last run 2026-10-01 |

### Columns

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-036 | Column header | normal: centered bold name, green +, separators, count cell | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930 | KF-036 nostat KF-037 nostat KF-049 nostat KF-168 nostat KF-182 nostat KF-192 FIX KF-203 FIX KF-311 REL KF-313 REL | GM-031/032 values |
| R2-037 | Column header | WIP exceeded, expanded: "5 / 3" darkred, #ff8080 line | `kanbanflow-golden-master-board-20260930.png` | pixel | GM (In Progress 5 tasks, WIP 3) | KF-034 nostat KF-035 nostat KF-312 REL | GM-033 value: rgb(91,91,91) for the non-violated count |
| R2-038 | Column header | WIP set but not exceeded ("0 / 3") | `kanbanflow-general-board-done-groups-20260930.png` | text | POMO (In progress WIP 3) | KF-035 nostat | Seen only on Chip's private board |
| R2-039 | Column header | overdue indicator "1 overdue task" (To Do) | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930 (only #7 overdue) | KF-218 FIX KF-219 FIX | On any later date the count becomes 2 (#6 and #7). The 10-01 battery saw "2 overdue tasks". Pin the clock |
| R2-040 | Column header | overdue indicator on In Progress (#11 past due) | `kanbanflow-board-20261012.png` | pixel | GM@1012; viewport matched to 1380x1030 | KF-218 FIX | `kanbanflow-board-menu-open-20261012.png` and `kanbanflow-pomodoro-panel-20261005.png` show the same state |
| R2-041 | Collapsed column strip | no WIP limit (To Do) | `kanbanflow-board-todo-collapsed-20260930.png` | pixel | GM@0930, To Do collapsed | KF-039 nostat KF-040 nostat KF-125 nostat KF-223 FIX | `kanbanflow-board-todo-collapsed-2-20260930.png` is a near-duplicate. Diffing `kanbanflow-board-todo-collapsed-20260930.png` against `kanbanflow-board-todo-collapsed-2-20260930.png` gives the capture noise floor |
| R2-042 | Collapsed column strip | holds overdue tasks | `kanbanflow-board-todo-collapsed-20260930.png` | pixel | GM@0930, To Do collapsed | KF-223 FIX | `kanbanflow-board-todo-collapsed-20260930.png` shows only the count and a red line. ChipFlow prints "N overdue task(s)" text in the strip (templates/board.html:544). KF-223's "verified live" spec says the text shows. Settle with the capture |
| R2-043 | Collapsed column strip | WIP exceeded: red count, #ff8080 line, nothing in the body | `kanbanflow-board-columns-collapsed-20260930.png` | pixel | GM@0930, In Progress collapsed | KF-241 REL | GM-040 |
| R2-044 | Collapsed column strip | two adjacent strips | `kanbanflow-board-columns-collapsed-20260930.png` | pixel | GM@0930, To Do and In Progress collapsed | KF-039 nostat KF-241 REL | |
| R2-045 | Collapsed column strip | collapsed column at the right edge, after Done | `kanbanflow-general-board-done-groups-20260930.png` | text | POMO plus an extra collapsed last column | KF-039 nostat | Private board: structure only |
| R2-046 | Column context menu | right-click menu | none | text | GM | KF-038 nostat KF-223 FIX KF-230 REL | KanbanFlow's verified menu has 3 items (Edit / Collapse / Show details). KF-230 added Delete: check the item list against KF-223 |
| R2-047 | Column details popover | "Column: Backlog / Task count: 5" | `kanbanflow-column-info-popover-20260930.png` | pixel | GM@0930, To Do collapsed | KF-223 FIX | |
| R2-048 | Edit column dialog | opened on a configured column (saved values loaded) | none | text | GM (In Progress) | KF-042 nostat KF-043 nostat KF-044 nostat KF-045 nostat KF-046 nostat KF-315 REL | GM-045 text |
| R2-049 | Delete column | empty column; non-empty column (requires a move target) | none | text | GM plus one empty column | KF-047 nostat KF-230 REL KF-306 REL | GM-164 text |
| R2-050 | Add column dialog | name and Position ("At the beginning", next to a column) | none | text | GM | KF-041 nostat KF-181 nostat | fidelity-status describes the dialog |
| R2-051 | Column drag | mid-drag (blank column, placeholder) | none | text | GM | KF-048 nostat | Hover and mid-drag states were among the old audit's 18 unobservable elements |
| R2-052 | Column body | empty column | `kanbanflow-general-board-done-groups-20260930.png` | text | POMO ("Do today" empty) | KF-316 REL | KF-316 (GM-156) covers only the empty column's header; the empty body has no defect |
| R2-053 | Column body | overflow: column scrollbar | `kanbanflow-general-board-done-groups-20260930.png` | text | Done column with 15+ tasks | — | No defect covers it |
| R2-054 | Done column | date groups: single "Today" | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930 (Done tasks moved on 09-30) | KF-124 nostat KF-326 REL | GM-059 |
| R2-055 | Done column | empty "Today" header above "Wednesday, 30 September" | `kanbanflow-board-20261012.png` | pixel | GM@1012 | KF-326 REL | `kanbanflow-board-menu-open-20261012.png` shows the same. ChipFlow builds groups only from tasks, labeled from `completed_at` (src/routes.rs about line 1604), so an empty "Today" header probably never renders. Likely divergence |
| R2-056 | Done column | several day groups ("Friday, 10 July") | `kanbanflow-general-board-done-groups-20260930.png` | text | Done tasks completed on several days | KF-326 REL | Private board |
| R2-057 | Done column | "Edit grouping date" | none | functional | GM | KF-140 nostat | `grouping_date` is stored, but the group label comes from `completed_at` |
| R2-058 | Column config effects | Column sum, task sorting, group-by-date, display properties | none | functional | GM | KF-043 nostat KF-045 nostat | |
| R2-059 | Column counts | count after a context-menu delete; drop position persists | none | functional | GM | KF-193 FIX KF-120 nostat | |

### Cards

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-060 | Card | base: tinted face, border shade matched to the face, flat, 12px text | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930 | KF-051 nostat KF-166 nostat KF-167 nostat KF-170 nostat KF-171 nostat KF-179 CORR KF-191 FIX KF-245 FIX | GM-051/052/053. KF-245 is FIX, not REL |
| R2-061 | Card | all 8 color variants | `kanbanflow-golden-master-board-20260930.png` | pixel | GM | KF-051 nostat KF-191 FIX | All 8 colors appear on the fixture board |
| R2-062 | Card | icon row (description, due, subtasks, comments, clock); missing features omitted | `kanbanflow-golden-master-board-20260930.png` | pixel | GM | KF-224 FIX KF-242 REL KF-054 nostat | GM-069 |
| R2-063 | Card | assignee avatar at top right | `kanbanflow-golden-master-board-20260930.png` | pixel | GM; username "Chip Senkbeil" | KF-243 REL KF-029 nostat | |
| R2-064 | Card | time badge "0h / 4h", "2h 30m / 8h" | `kanbanflow-golden-master-board-20260930.png` | pixel | GM; #11 at 2.5h | KF-054 nostat KF-114 nostat | `kanbanflow-pomodoro-panel-20261005.png`, `kanbanflow-board-menu-open-20261012.png` and `kanbanflow-board-20261012.png` show "2h 46m" (R3) |
| R2-065 | Card | due line, relative ("Today 5:00 PM (Done)", "Friday 5:00 PM") | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930 | KF-244 REL KF-053 nostat | |
| R2-066 | Card | due line, overdue (red "!", red date) | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930 (#7) | KF-219 FIX KF-244 REL | `kanbanflow-board-20261012.png` shows #11 |
| R2-067 | Card | due line, absolute date ("2 October") | `kanbanflow-board-20261012.png` | pixel | GM@1012 | KF-244 REL | |
| R2-068 | Card | inline subtasks expanded; "2 hidden (2 done)" | `kanbanflow-golden-master-board-20260930.png` | pixel | GM@0930; per-card section state set in localStorage | KF-224 FIX | In `kanbanflow-golden-master-board-20260930.png`, #1 and #6 are expanded. In `kanbanflow-board-todo-collapsed-20260930.png`, #1 is collapsed. Section state is per browser |
| R2-069 | Card | sections collapsed (icon row only) | `kanbanflow-board-todo-collapsed-20260930.png` | pixel | GM@0930 | KF-224 FIX | |
| R2-070 | Card | no attributes (one line) | `kanbanflow-golden-master-board-20260930.png` | pixel | GM (#8, #13) | KF-170 nostat | |
| R2-071 | Card | labels shown when the column displays them | none | functional | GM, labels enabled in the column | KF-043 nostat | KanbanFlow captures show no labels on cards |
| R2-072 | Card | pomodoro count badge ("🍅 1m") | none | text | LOG | KF-055 nostat KF-200 FIX | Frame review v1 |
| R2-073 | Card | live badge while timing ("2h 46m + 42m") | `kanbanflow-pomodoro-panel-20261005.png` | pixel | TIMER; mask the numbers | KF-103 nostat | Reverting on discard: see the Timer rows |
| R2-074 | Card | timer-selected: orange dashed outline | `kanbanflow-pomodoro-panel-20261005.png` | pixel | TIMER | KF-052 nostat KF-325 REL KF-327 REL | GM-055/GM-103 |
| R2-075 | Card context menu | open (per-item icons, black Delete) | none | text | GM | KF-050 nostat KF-140 nostat KF-329 REL | The GM-065 image is lost |
| R2-076 | Card context menu | Color submenu (8 names, tinted rows) | none | text | GM | KF-333 REL KF-142 nostat | GM-066 |
| R2-077 | Card context menu | Move submenu (Move right / Move to bottom / Move advanced; last column omits the first two) | none | text | GM | KF-334 REL KF-141 nostat | GM-067 |
| R2-078 | Card context menu | Start timer, without a "Select in timer" item | none | text | GM | KF-327 REL KF-050 nostat | Frame review: the card Timer submenu is Start timer / Select in timer |
| R2-079 | Card drag | mid-drag dashed placeholder | none | text | GM | KF-120 nostat | fidelity-status gives the CSS |
| R2-080 | Card | re-render after a modal edit, a move while timing, or a save | none | functional | GM | KF-185 nostat KF-225 FIX KF-199 FIX | |
| R2-081 | Card | delete via the context menu (styled confirm) | none | functional | GM | KF-148 nostat KF-184 nostat | |
| R2-082 | Card | Large task names on | none | functional | GM | KF-353 REL | Only the menu switch was ever checked; the effect on cards never was |
| R2-083 | Card | icon tooltips ("Has description. Click to expand or collapse.") | none | text | GM | KF-224 FIX | |

### Task modal

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-084 | Task modal | opened on #11: 2-column grid, 600px wide, top offset, tinted with the task color | none | text | GM | KF-250 REL KF-251 FIX KF-319 REL KF-323 REL KF-073 nostat | No task-editor capture survives (GM-070/073/076 are lost) |
| R2-085 | Task modal | subline: column, "Created: Today", 3 icons | none | text | GM | KF-057 nostat KF-255 FIX KF-256 REL | Task `created_at` cannot be seeded |
| R2-086 | Task modal | toolbar outside the right edge (Add, Move, Timer, Reports, More, Delete) | none | text | GM | KF-056 nostat KF-252 REL KF-266 REL KF-267 REL KF-320 REL KF-321 REL | |
| R2-087 | Task modal | description as static text; Add > Description editor | none | text | GM (#11) | KF-063 nostat KF-253 FIX KF-322 RETR | `#description-dialog` appears in no defect or evidence |
| R2-088 | Task modal | Color row "● Orange"; color picker open | none | text | GM | KF-073 nostat KF-257 FIX KF-214 CLO | |
| R2-089 | Task modal | Members row (avatar, ⊕); Members dialog open | none | text | GM | KF-060 nostat KF-261 REL KF-328 REL | GM-074/079 |
| R2-090 | Task modal | label chips; Labels dialog (empty state, changes apply at once) | none | text | GM | KF-061 nostat KF-157 nostat KF-260 REL KF-268 REL KF-269 FIX KF-317 REL KF-328 REL | GM-080/158 |
| R2-091 | Task modal | Due row, relative and "(Done)"; Add due date dialog | none | text | GM@0930 | KF-062 nostat KF-145 nostat KF-227 FIX KF-259 REL KF-270 FIX | GM-081 text-source |
| R2-092 | Task modal | Time estimate dialog | none | text | GM | KF-066 nostat KF-072 nostat KF-216 FIX | |
| R2-093 | Task modal | Time spent "2h 30m"; hover tooltip and hover menu | none | text | GM | KF-013 nostat KF-067 nostat KF-068 nostat KF-258 FIX KF-278 REL | GM-075 |
| R2-094 | Task modal | counters (pomodori, time spent) | none | text | GM plus LOG | KF-014 nostat | Manual time adds to Time spent, not to pomodori |
| R2-095 | Task modal | Subtasks: header "2 / 4" with ⋮ menu, drag handles, done not struck | none | text | GM (#11) | KF-058 nostat KF-262 REL KF-263 REL KF-264 REL KF-265 REL | GM-085 |
| R2-096 | Task modal | subtask keyboard navigation and reorder | none | functional | GM (#11) | KF-273 FIX | GM-086/149/150 |
| R2-097 | Task modal | comments (avatars, relative times, composer, ⋮ menu) | none | text | GM (#11, 2 comments) | KF-064 nostat KF-254 REL | Comment `created_at` cannot be seeded |
| R2-098 | Task modal | Attachments: Add menu entry greyed; upload; download | none | text | GM | KF-189 FIX KF-332 REL | GM-077: KanbanFlow greys Attachment out (premium). KF-189 is FIX |
| R2-099 | Task modal | Add menu (items and order) | none | text | GM | KF-059 nostat KF-064 nostat KF-065 nostat KF-066 nostat KF-144 nostat | |
| R2-100 | Move task dialog | open | none | text | GM | KF-069 nostat KF-070 nostat KF-274 FIX | GM-087 text-source |
| R2-101 | Task modal | Timer menu (Start Pomodoro / Start Stopwatch / Time log) | none | text | GM | KF-298 REL | Frame review §e.6 |
| R2-102 | Task modal | Reports menu (Time log, Print, History, Time in column) | none | text | GM | KF-100 nostat KF-275 REL | GM-089. Print and Time in column are premium in KanbanFlow |
| R2-103 | Task modal | Time log sub-view | none | text | GM plus LOG | KF-101 nostat KF-115 nostat KF-116 nostat KF-277 REL KF-015 nostat KF-016 nostat | GM-096 |
| R2-104 | Task modal | History sub-view | none | text | GM with edits | KF-100 nostat | R4: deletions are not logged |
| R2-105 | Task modal | More menu: Watch, Task URL, Copy, Keyboard shortcuts | none | text | GM | KF-071 nostat KF-117 nostat | Copy has no evidence anywhere |
| R2-106 | Task modal | Delete (styled confirm) | none | functional | GM | KF-148 nostat KF-184 nostat KF-188 nostat | |
| R2-107 | Task modal | Esc discards edits first, then closes; Ctrl+Enter saves | none | functional | GM | KF-299 REL KF-094 nostat | GM-151 |
| R2-108 | New-task modal | from a column "+", dropdown closed | none | text | GM | KF-118 nostat KF-302 REL KF-208 FIX | |
| R2-109 | New-task modal | color dropdown open (8 named rows, ⋮, default in bold) | `kanbanflow-task-modal-color-dropdown-20260930.png` | pixel | GM@0930, To Do collapsed, Yellow default | KF-302 REL | GM-133. README calls this a "task editor modal", but it is the empty new-task modal ("Task name...", Save) |

### Timer / pomodoro

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-110 | Timer popup | idle, pomodoro mode ("25:00", "Time until break", Start) | none | text | GM | KF-001 nostat KF-138 nostat KF-177 nostat | Frame review §e.3 |
| R2-111 | Timer popup | pomodoro running (countdown, task row with color dot) | none | text | TIMER | KF-001 nostat KF-280 REL KF-138 nostat | GM-101/103 |
| R2-112 | Timer popup | pomodoro elapsed, waiting for break ("Time until break", Take break) | `kanbanflow-pomodoro-panel-20261005.png` | pixel | TIMER; mask clock text | KF-006 nostat | |
| R2-113 | Timer popup | task row with Change task | `kanbanflow-pomodoro-panel-20261005.png` | pixel | TIMER | KF-010 nostat | |
| R2-114 | Timer popup | footer tabs (Stopwatch / Add time / Log / Settings) | `kanbanflow-pomodoro-panel-20261005.png` | pixel | TIMER | KF-009 nostat KF-158 nostat | |
| R2-115 | Timer popup | TODAY list (day total, time ranges, ⋮, "— pending", "...") | `kanbanflow-pomodoro-panel-20261005.png` | text | TIMER plus LOG | KF-279 REL | Entry times cannot be reproduced exactly. Compare structure and format |
| R2-116 | Timer popup | "Select open task" while another task's modal is open | none | text | TIMER, #6 modal open | KF-010 nostat | Frame review §e.4 |
| R2-117 | Timer popup | short break running; long break on every 4th | none | text | TIMER | KF-006 nostat KF-283 REL | |
| R2-118 | Timer popup | stopwatch idle (big "00:00", green Start) | none | text | GM | KF-281 REL | GM-105 |
| R2-119 | Timer popup | stopwatch running ("Session time", count-up, red Stop) | none | text | TIMER (stopwatch) | KF-008 nostat KF-281 REL | |
| R2-120 | Toast | "Session discarded" (stop under 20s) | none | text | TIMER | KF-007 nostat | |
| R2-121 | Why-stop menu | open (17 items, Add new reason, Task done) | none | text | TIMER, 20s or more elapsed | KF-005 REL KF-011 nostat KF-021 nostat KF-194 FIX KF-077 nostat | The frame review never saw this menu in KanbanFlow |
| R2-122 | Why-stop menu | stop with a reason, then the red log entry | none | functional | TIMER | KF-012 nostat KF-198 FIX KF-002 nostat | |
| R2-123 | Timer context menu | right-click on the timer area (Comment, Label, Edit, Delete) | none | text | TIMER | KF-282 REL | GM-109: catalog text only |
| R2-124 | Timer pill | idle: pomodoro "▶ 25:00 ▾" / stopwatch "■ 00:00 ▾" | none | text | GM | KF-098 nostat KF-026 nostat KF-025 nostat KF-229 FIX | See the pill-visibility row under Board |
| R2-125 | Timer pill | running: zero-padded live tick | none | text | TIMER | KF-027 nostat KF-028 nostat KF-197 FIX KF-231 REL | |
| R2-126 | Timer pill | after a stop (no stale state); tab title shows the live timer | none | functional | TIMER | KF-232 REL KF-020 nostat | |
| R2-127 | Timer pill | hidden while the popup is open | `kanbanflow-pomodoro-panel-20261005.png` | pixel | TIMER | KF-097 nostat | |
| R2-128 | Timer settings modal | General tab | none | text | GM | KF-074 nostat KF-075 nostat KF-283 REL KF-328 REL | v7 S14. GM-111/112 lost |
| R2-129 | Timer settings modal | Interruptions tab (drag, inline rename, Add reason) | none | text | GM | KF-076 nostat KF-106 nostat | GM-113 lost |
| R2-130 | Timer settings modal | Break activities tab; Add break activity dialog | none | text | GM | KF-078 nostat KF-284 REL KF-285 REL | GM-114/115 |
| R2-131 | Timer settings modal | Sounds tab | none | text | GM | KF-079 nostat | GM-116 lost |
| R2-132 | Timer audio | alarm, ticking and volume playback | none | none | — | KF-079 nostat | No reference; audio is not compared |
| R2-133 | Timer | Picture-in-Picture window | none | none | TIMER | KF-075 nostat | Only the toggle is covered. The PiP window was never observed in either app |
| R2-134 | Timer | browser notification when the timer ends | none | n/a | — | KF-105 nostat | KanbanFlow has no counterpart (KF-105) |
| R2-135 | Add time manually dialog | open; From/To give Duration; calendar; Add labels / Add comment checkboxes | none | text | GM | KF-003 REL KF-018 nostat KF-102 nostat KF-119 nostat KF-137 nostat KF-271 FIX | v3 S5, GM-082 |
| R2-136 | Date picker popup | "Sun Mon...", Today / yesterday / This month buttons | none | text | GM | KF-291 REL | GM-122/171 |
| R2-137 | Future-time error | "You can not enter a time in the future" + OK; dialog state kept | none | text | GM | KF-272 FIX | GM-083 |
| R2-138 | Edit Pomodoro entry dialog | open from an entry; Update | none | text | LOG | KF-004 REL KF-017 nostat KF-271 FIX KF-102 nostat | |
| R2-139 | Timer log modal | period stepper, Entry type, icons, day groups | none | text | LOG | KF-090 nostat KF-286 REL KF-288 REL KF-113 nostat KF-318 REL KF-019 nostat | GM-117/129 text-source |
| R2-140 | Timer log modal | empty state ("No entries exist for the given filter") | none | text | FRESH board | KF-090 nostat | |
| R2-141 | Timer log modal | export menu (Excel first) | none | text | LOG | KF-093 nostat KF-289 REL | GM-119 |
| R2-142 | Timer log modal | Custom (absolute) dialog (two calendars, Done) | none | text | LOG | KF-290 REL | GM-121 |
| R2-143 | Timer log modal | Custom (relative) period | none | functional | LOG | — | No defect or evidence |
| R2-144 | Card live badge | reverts after a discard (stop under 20s) | none | functional | TIMER | KF-103 nostat | |
| R2-145 | Timer API | status/stop/today/settings shapes | none | functional | TIMER | KF-001 nostat KF-002 nostat KF-127 nostat KF-128 nostat KF-129 nostat KF-130 nostat | |

### Reports

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-146 | Board Menu > Reports submenu | 15 items in order | none | text | GM | KF-099 nostat KF-136 nostat | v7-e020 |
| R2-147 | Pomodoro statistics | Pomodoros tab: presets, bar chart, tooltip | none | text | LOG | KF-092 nostat KF-293 REL | GM-124/130 conflict, recorded in deployed-recheck-20261016.md |
| R2-148 | Pomodoro statistics | Interruptions tab | none | functional | LOG with stop reasons | KF-092 nostat | The frame review says KanbanFlow's tab was never opened, so there is no reference |
| R2-149 | Pomodoro statistics | Break activities tab | none | functional | LOG with break activities | KF-347 REL | |
| R2-150 | Pomodoro statistics | Highscores tab | none | functional | LOG | KF-092 nostat | No evidence file mentions Highscores |
| R2-151 | Pomodoro statistics | no data ("No data to display") | none | text | FRESH | KF-092 nostat | |
| R2-152 | Time spent report `/timer/log` | default view (bars, %, breakdown, totals) | none | text | LOG | KF-091 nostat KF-287 REL KF-292 REL | GM-123 |
| R2-153 | Time spent report | Export menu (Excel Detailed / Summary, PDF) | none | text | LOG | KF-294 REL | GM-170 |
| R2-154 | Time spent report | Print sheet | none | text | LOG | KF-295 REL | GM-169 |
| R2-155 | Time spent report | Bookmarks; Summary vs Detailed; Group by | none | functional | LOG | KF-091 nostat | No evidence file mentions Bookmarks |
| R2-156 | Board history `/b/:id/board-history` | renders with history | none | functional | GM with edits | KF-336 REL | R4: deletions do not show |
| R2-157 | Burndown `/b/:id/burndown` | renders | none | functional | GM plus done history | KF-337 REL | |
| R2-158 | Cumulative flow `/b/:id/cumulative-flow` | renders; after a column rename | none | functional | GM with moves | KF-339 REL KF-350 REL | |
| R2-159 | Cycle & lead time `/b/:id/cycle-lead` | renders | none | functional | GM with done history | KF-340 REL | |
| R2-160 | Due date performance `/b/:id/due-date` | renders | none | functional | GM@0930 | KF-341 REL | |
| R2-161 | Monte Carlo `/b/:id/monte-carlo` | run forecast | none | functional | GM with throughput history | KF-342 REL | R4: unseeded, so results vary between runs; compare only shape and range |
| R2-162 | Task count `/b/:id/task-count` | renders | none | functional | GM | KF-343 REL | |
| R2-163 | Throughput `/b/:id/throughput` | renders | none | functional | GM with done history | KF-344 REL | |
| R2-164 | Time estimate `/b/:id/time-estimate` | renders | none | functional | GM | KF-345 REL | |
| R2-165 | Export `/b/:id/export` | Excel / CSV / XML / JSON downloads | none | functional | GM | KF-346 REL | |
| R2-166 | Reports submenu | "Time in column" (board level) | none | n/a | — | KF-275 REL | Premium in KanbanFlow (frame review). ChipFlow shows a "not available" toast |
| R2-167 | Reports submenu | Print | none | functional | GM | — | ChipFlow calls `window.print()`. KanbanFlow's print view was never captured |

### Calendar

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-168 | Calendar `/b/:id/calendar` | month grid with due tasks; Today navigation; filters | none | functional | GM (due dates 09-25 to 10-30) | KF-338 REL | |
| R2-169 | Calendar | repeating due dates expanded | none | functional | GM plus a task with `due_repeat` "every week" | KF-352 REL | |
| R2-170 | Calendar iCal panel | Subscribe, Copy link, Reset link | none | functional | GM | KF-351 REL | Reset link appears in one evidence file only |
| R2-171 | iCal feed `/api/calendar-feed/:token` | unauthenticated feed; token rotation invalidates the old link | none | functional | GM | KF-351 REL | |

### Board menu / legend

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-172 | Board menu | open, all toggles off | `kanbanflow-board-menu-open-20261012.png` | pixel | GM@1012; viewport matched to 1375x1038 | KF-353 REL KF-099 nostat KF-135 nostat KF-204 FIX | ChipFlow adds a "Save board as template" item that `kanbanflow-board-menu-open-20261012.png` does not have, so expect a structural diff |
| R2-173 | Board menu | open, Color legend on, legend visible | `kanbanflow-board-menu-open-legend-20260930.png` | pixel | GM@0930, To Do collapsed, legend on | KF-353 REL KF-183 nostat | |
| R2-174 | Board menu | open on the Pomodoro board, legend on | `kanbanflow-general-board-menu-open-20260930.png` | text | POMO | KF-353 REL | Private board |
| R2-175 | Board menu | Settings opens this board's settings | none | functional | GM | KF-154 nostat | |
| R2-176 | Board menu | Members / Get Premium / Recycle bin | none | n/a | — | — | Single admin, free, no recycle bin. ChipFlow shows an explanatory dialog or toast |
| R2-177 | Board menu | Help & feedback | none | functional | GM | — | Opens the shortcuts dialog. No evidence file mentions it |
| R2-178 | Color legend | off (new-board default) | `kanbanflow-golden-master-board-20260930.png` | pixel | GM | KF-183 nostat KF-222 FIX | |
| R2-179 | Color legend | on: 8 named segments, full width, flush to the bottom | `kanbanflow-board-20261012.png` | pixel | GM@1012, legend on | KF-354 REL KF-249 REL KF-210 FIX KF-212 FIX KF-205 FIX KF-104 nostat | `kanbanflow-board-menu-open-legend-20260930.png` shows the same at 1919 under the open menu |
| R2-180 | Color legend | on, Pomodoro board: 4 renamed segments | `kanbanflow-general-board-done-groups-20260930.png` | text | POMO, legend on | KF-335 REL KF-121 CLO | KF-121 is CLO |
| R2-181 | Color legend | segment tooltip (color description) | none | functional | GM with color descriptions | — | fidelity-status: the description is the legend tooltip. No defect covers it |
| R2-182 | Color legend | persisted per board across reload | none | functional | MULTI | KF-222 FIX | |

### Board settings

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-183 | Board settings shell | chrome: #d3e8f2 page, 900px container, header band, nav 240px | `kf331-kanbanflow-settings-colors-recovered-20261016.png` | pixel | POMO | KF-080 nostat KF-153 nostat KF-331 REL | Measurements in `.dev/evidence/kf331-verify-20261016/` |
| R2-184 | Board settings shell | global dark top bar | `kf331-kanbanflow-settings-colors-recovered-20261016.png` | pixel | POMO | KF-357 OPEN | OPEN |
| R2-185 | Board settings shell | Custom fields / Custom roles nav items dimmed | `kf331-kanbanflow-settings-colors-recovered-20261016.png` | pixel | POMO | KF-111 nostat | |
| R2-186 | Colors tab | Pomodoro palette: intro and Show more, Enabled table with header row, Disabled list alphabetical with ?, premium box | `kf331-kanbanflow-settings-colors-recovered-20261016.png` | pixel | POMO | KF-081 nostat KF-082 nostat KF-083 nostat KF-085 nostat KF-086 nostat KF-087 nostat KF-107 nostat KF-108 nostat KF-109 nostat KF-110 nostat KF-300 REL KF-301 REL KF-303 REL KF-356 OPEN | KF-356 is OPEN |
| R2-187 | Colors tab | Golden Master palette (8 enabled, Brown and White disabled) | none | text | GM | KF-303 REL KF-151 nostat | Derived from the fixture palette |
| R2-188 | Colors tab | Edit color dialog | none | text | GM | KF-084 nostat | |
| R2-189 | Colors tab | Set default color dialog | none | text | GM | KF-083 nostat | |
| R2-190 | Colors tab | Copy from board dialog (no other boards; with boards) | none | functional | MULTI | KF-083 nostat | No evidence file mentions it |
| R2-191 | Colors tab | Add custom color, then the premium overlay | none | text | GM | KF-085 nostat | |
| R2-192 | Colors tab | drag-reorder; Disable / Enable | none | functional | GM | KF-082 nostat | |
| R2-193 | General tab | board fields and Save | none | functional | GM | KF-080 nostat KF-153 nostat | |
| R2-194 | Layout tab | columns/swimlanes table (WIP, Done, Collapsed) | none | functional | GM, To Do collapsed | KF-080 nostat | Only the shell was ever checked |
| R2-195 | Task settings tab | default color, Done column | none | functional | GM | KF-080 nostat | |
| R2-196 | Advanced tab | Board ID, Board position, Danger zone | none | functional | MULTI | KF-080 nostat | Board ID and Board position appear in no defect |
| R2-197 | API & Webhooks tab | API pointers; "no webhooks" note | none | n/a | — | KF-080 nostat | ChipFlow has no outgoing webhooks |
| R2-198 | Add task from email tab | pointer to API tokens | none | n/a | — | KF-080 nostat | Not implemented on purpose |
| R2-199 | Delete board page `/b/:id/settings/delete` | confirmation (Cancel / Delete board / Archive board) | `kanbanflow-delete-board-dialog-20260929.png` | pixel | a board named "Dale fidelity test" | KF-089 nostat KF-304 REL KF-305 REL | KanbanFlow's footer bar is in the capture. Check ChipFlow's page against it |
| R2-200 | Delete board | Archive flow; delete the last board (refused) | none | functional | MULTI; single board | KF-305 REL KF-139 nostat | The last-board refusal is documented in agent-guide.md and never tested |

### User settings

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-201 | Settings `/settings` | renders: Timer, Alerts, Interruption reasons, API tokens | none | functional | GM | KF-074 nostat KF-105 nostat KF-106 nostat | ChipFlow-only page; KanbanFlow has no counterpart |
| R2-202 | Settings | Save, then "Settings saved." | none | functional | GM | — | |
| R2-203 | API tokens | create: shown once, listed immediately | none | functional | GM | KF-150 nostat | |
| R2-204 | API tokens | revoke (styled confirm); token rejected afterwards | none | functional | GM plus a token | KF-146 nostat | |
| R2-205 | API tokens | scopes and expiry (Never/30/90/365 days); Last used | none | functional | GM plus tokens | — | No evidence covers the Expires column |

### Auth / setup

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-206 | Setup `/setup` | first run: create the admin account | none | functional | FRESH | — | No defect touches /setup |
| R2-207 | Setup `/setup` | revisited after the admin exists | none | functional | GM | — | |
| R2-208 | Login `/login` | valid login; invalid credentials message | none | functional | GM | — | No evidence covers "Invalid username or password." |
| R2-209 | Session | logout; unauthenticated redirect; Secure cookie only over HTTPS | none | functional | GM | KF-195 FIX | |
| R2-210 | Headless setup | `CHIPFLOW_API_TOKEN` on first start | none | functional | FRESH | — | |

### API / agents

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-211 | Discovery | `/agents.md`, `/agents/skill.md`, `/.well-known/agents.json` public and accurate | none | functional | — | KF-220 FIX | |
| R2-212 | OpenAPI `/api/v1/openapi.json` | matches every `.route(` call | none | functional | — | — | |
| R2-213 | Version `/api/v1/version` | public build SHA | none | functional | — | — | |
| R2-214 | Token auth | 401 invalid/expired, 403 insufficient scope, a token cannot mint tokens | none | functional | GM plus tokens | KF-146 nostat | |
| R2-215 | Task API | create with due_at/due_repeat; bare local datetime | none | functional | GM | KF-226 FIX KF-348 REL KF-207 FIX | |
| R2-216 | Column API | PATCH `wip_limit: null`; collapsed; delete a non-empty column (400) | none | functional | GM | KF-228 FIX | |
| R2-217 | API ergonomics | comments/labels/members/time JSON endpoints | none | functional | GM | KF-221 FIX | |
| R2-218 | Fixture seeding | seed the whole fixture through the API, end to end | none | functional | GM | KF-221 FIX | No committed seeder exists. See section 7 |

### Dark mode

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-219 | Board | dark mode on | none | none | GM, dark on | KF-312 REL | KanbanFlow's dark mode was never captured. R4: column bodies stay white (accepted gap) |
| R2-220 | Task modal and dialogs | dark mode on | none | none | GM, dark on | — | No reference |
| R2-221 | Timer popup and settings modal | dark mode on | none | none | TIMER, dark on | — | No reference |
| R2-222 | Report pages | dark mode on | none | functional | LOG, dark on | KF-349 REL | Contrast check |
| R2-223 | Settings pages | dark mode on | none | none | GM, dark on | — | No reference |
| R2-224 | Dark mode toggle | persists across reload and boards | none | functional | MULTI | KF-353 REL | |

### Mobile / narrow widths

| id | surface | state | reference | check | fixture needs | covering KF ids | notes |
|---|---|---|---|---|---|---|---|
| R2-225 | Top bar | about 500px viewport: wordmark does not overlap the bell | none | functional | GM | KF-324 REL | |
| R2-226 | Timer log modal | 1100x900 | none | functional | LOG | KF-286 REL | Verified at 1440 and 1100 in October |
| R2-227 | Board | under 1024px: horizontal scroll, column min-width | none | functional | GM | — | No defect or evidence |
| R2-228 | Task modal | narrow viewport | none | functional | GM | — | No defect or evidence |
| R2-229 | Timer popup / quick-add popup | stays inside the viewport | none | functional | TIMER | KF-177 nostat KF-208 FIX | |
| R2-230 | Board | touch drag on a phone | none | none | GM | — | No reference and no touch harness |

## 4. Which captures can anchor a pixel comparison

I opened all 14 images. The README in `.dev/evidence/golden-masters/` says the captures are "1920x1000 unless noted". Only two of the 14 are: the delete dialog and the KF-331 image. Eleven are 1919x998, and Chip's two macOS screenshots are 1375x1038 and 1380x1030, which the README does not note.

| capture | size | anchors pixel? | why | rows anchored |
|---|---|---|---|---|
| `kanbanflow-golden-master-board-20260930.png` | 1919x998 | yes, with the clock pinned | Golden Master board, every column expanded. Shows #11 at 2h 30m, so it predates R3. Relative due text ("Today 5:00 PM", "Friday 5:00 PM"), "1 overdue task" and the Done "Today" group need the server and browser clocks pinned to 2026-09-30 about 15:22 CDT. Cards #1, #6, #7 and #11 have sections expanded (browser localStorage). Mask the cursor over "Backlog" and the bell badge | 18 |
| `kanbanflow-board-todo-collapsed-20260930.png` | 1919x998 | yes, with the clock pinned | Same board with To Do collapsed. Card #1 is collapsed here | 3 |
| `kanbanflow-board-todo-collapsed-2-20260930.png` | 1919x998 | as the noise floor | A near-duplicate of the capture above (only the cursor moved). Use the diff between the two as KanbanFlow's own capture-to-capture noise, not as a separate anchor | 0 |
| `kanbanflow-board-columns-collapsed-20260930.png` | 1919x998 | yes, with the clock pinned | To Do and In Progress collapsed; folded WIP violation | 2 |
| `kanbanflow-task-modal-color-dropdown-20260930.png` | 1919x998 | yes, modal region | This is the empty new-task modal from a column "+" (placeholder "Task name...", Save), not the task editor. Its color dropdown is open with Yellow as the default. The board behind it is dimmed | 1 |
| `kanbanflow-column-info-popover-20260930.png` | 1919x998 | yes | "Show details" popover on Backlog, To Do collapsed | 1 |
| `kanbanflow-board-menu-open-legend-20260930.png` | 1919x998 | yes, with the clock pinned | Menu open with Color legend on; the legend strip is visible under the menu | 1 |
| `kanbanflow-general-board-done-groups-20260930.png` | 1919x998 | no: text and structure only | Chip's real "General" board, with private task names and July dates. It cannot be mirrored without copying private data. Use it for Done date-group format, an empty "Today" header, a collapsed column at the right edge, the column scrollbar, "0 / 3" WIP and the 4-segment Pomodoro legend | 0 (6 text rows) |
| `kanbanflow-general-board-menu-open-20260930.png` | 1919x998 | no: text and structure only | Same private board with the Menu open (Color legend on) | 0 (1 text row) |
| `kanbanflow-pomodoro-panel-20261005.png` | 1919x998 | partly | Panel chrome, the timer-selected outline, footer tabs, the hidden pill and the timing badge on the avatar can be reproduced with a short work time. Wall-clock times, "58m", "+ 42m" and the TODAY rows cannot, so mask them. #11 reads "2h 46m" (R3) | 7 (plus 1 text row) |
| `kanbanflow-board-menu-open-20261012.png` | 1375x1038 | yes, if the viewport is matched | Chip's macOS window screenshot. Viewport and DPR must be inferred; there is a thin strip of window chrome at the top. Due dates are absolute, so no clock pin is needed, but the Done group needs a 2026-09-30 completion. #11 reads "2h 46m" (R3) and the bell badge reads 4 | 1 |
| `kanbanflow-board-20261012.png` | 1380x1030 | yes, if the viewport is matched | Same caveats. This is the only capture of the full 8-color legend with the menu closed (the KF-354 reference) | 4 |
| `kanbanflow-delete-board-dialog-20260929.png` | 1920x1000 | yes | Needs a board named "Dale fidelity test". KanbanFlow's site footer bar is part of the frame | 1 |
| `kf331-kanbanflow-settings-colors-recovered-20261016.png` | 1920x1000 | yes, on a Pomodoro-template board | It shows a Pomodoro palette (4 renamed colors, 6 disabled), not the Golden Master's 8-color palette, so it anchors against POMO and not against the GM mirror. The H1 does not name the board | 4 |

Pixel rows by anchor: 11 of the 14 images anchor at least one pixel row. The two "General" captures and the duplicate do not. Rows that cite more than one capture are counted under each.

## 5. Counts and comparison with the historical claim

| check type | this inventory | historical claim |
|---|---|---|
| pixel | 43 | 97 |
| text | 85 | 54 |
| functional | 88 | (no such category) |
| none | 7 | 15 |
| n/a | 7 | 5 |
| **total** | **230** | **171** |

By area (pixel / text / functional / none / n/a):

- Board: 5 / 10 / 18 / 0 / 2
- Columns: 11 / 10 / 3 / 0 / 0
- Cards: 13 / 7 / 4 / 0 / 0
- Task modal: 1 / 22 / 3 / 0 / 0
- Timer / pomodoro: 4 / 24 / 5 / 2 / 1
- Reports: 0 / 6 / 15 / 0 / 1
- Calendar: 0 / 0 / 4 / 0 / 0
- Board menu / legend: 4 / 2 / 4 / 0 / 1
- Board settings: 5 / 4 / 7 / 0 / 2
- User settings: 0 / 0 / 5 / 0 / 0
- Auth / setup: 0 / 0 / 5 / 0 / 0
- API / agents: 0 / 0 / 8 / 0 / 0
- Dark mode: 0 / 0 / 2 / 4 / 0
- Mobile / narrow widths: 0 / 0 / 5 / 1 / 0

Every tracker id from KF-001 to KF-357 appears in at least one row, except KF-143, which is reserved and has no description.

Where the numbers differ, and why:

1. **The unit of inventory changed.** The tracker header (2026-10-01 bullet) says the old document listed "171 KanbanFlow UI elements". Those were the `GM-001` to `GM-171` masters: things visible in KanbanFlow, one row each. This inventory lists ChipFlow surfaces x states. It covers every route and template, including ChipFlow-only pages (eleven report pages, user settings, setup and login, the API and discovery endpoints, the iCal feed). Those pages had no place in a KanbanFlow element list, and they account for most of the 88 `functional` rows.
2. **Pixel fell from 97 to 43** because the per-element PNG crops behind the old pixel rows are lost. Ninety-nine GM ids are still cited somewhere (Appendix A), but their images are gone. What survives is 11 usable full-page captures from 4 days. They cover the board, columns, cards, the board menu and legend, the new-task color dropdown, the pomodoro panel, the delete dialog and the board settings Colors tab. The task editor modal, the timer settings tabs, the dialogs, the timer log and the reports (old masters GM-070 to GM-130) all drop to `text`.
3. **Text rose from 54 to 85.** It absorbs the lost pixel masters, now backed by tracker KanbanFlow lines and the video catalogs. The old text-source masters (GM-021/022/026/043/081/082/083/086/087/089/117/129/149/150/158, called text-source in the evidence) are still text. Where a state already had a defect, I split it into more states, for example the collapsed-strip overdue case versus the WIP case.
4. **None fell from 15 to 7.** The old audit had "18 elements unobservable" (live timer states, hover and mid-drag, transient toasts). Most of those now have text references: the tracker and frame review describe "Session discarded", the why-stop menu and the drag placeholder, and the 10-05 capture shows the Take break state. What stays `none` is dark mode (KanbanFlow's dark mode was never captured), timer audio, the Picture-in-Picture window and touch drag.
5. **The old record contradicts itself.** The same tracker bullet says 18 elements were unobservable, but the claimed split has 15 none plus 5 n/a. I cannot reconcile 97/54/15/5 with "18 unobservable" from what survives. Treat the old numbers as a rough guide only.
6. **N/a rose from 5 to 7** because I list each deliberate omission separately: notification badge, invite, Members/Premium/Recycle, webhooks, add-from-email, board-level Time in column, and browser notifications.

## 6. States never tested per the tracker

These are surfaces in the routes or templates that no tracker section and no evidence file touches. I checked with keyword searches over `.dev/kf-parity-defects.md` and `.dev/evidence/`. The first 10 matter most for R2:

1. Empty "Today" Done-group header above older groups. KanbanFlow shows it in the 10-12 captures and on the General board. ChipFlow groups only by task `completed_at`, so it probably never renders the empty header.
2. The pill-visibility rule when the timer is idle. All 11 idle Golden Master captures show no top-bar pill, and ChipFlow always shows one. Never checked against these captures.
3. The collapsed-strip overdue text. KanbanFlow's collapsed To Do strip shows only the count and a red line; ChipFlow prints "N overdue task(s)". KF-223 asserted the text from a live check, and the recovered capture disagrees.
4. The timing badge on the board-header member avatar while a timer runs (10-05 capture).
5. `/setup` first run and revisit; the `/login` invalid-credentials message.
6. `/boards/archived`: empty state and Unarchive.
7. Board settings Layout, Task settings and Advanced tabs (Board ID, Board position, Danger zone). Only the nav shell was ever checked (KF-080).
8. The task-modal Description editor dialog (`#description-dialog`) and More > Copy.
9. Pomodoro statistics Highscores and Interruptions tabs (the frame review notes KanbanFlow's Interruptions tab was never opened), the Custom (relative) period, and Time spent Bookmarks.
10. Dark mode on the board, task modal, timer popup and settings pages. There is no reference, and only the report pages were checked (KF-349).

Also never tested:

- the swimlane menu
- the effect of Large task names on cards
- the color legend segment tooltip
- column overflow scrollbar
- the Copy from board dialog
- Disable / Enable color actions
- the API token Expires column and expiry enforcement
- iCal Reset link (one evidence mention)
- refusal to delete the last board
- the Picture-in-Picture window
- viewports narrower than 1024px other than the 500px header check (KF-324)
- touch drag

## 7. Fixture gaps

What `.dev/golden-master-board.json` asks for that the ChipFlow API (agent-guide.md, plus the `CreateTaskInput`, `UpdateTaskInput`, `UpdateColumnInput`, manual-time and comment structs in `src/routes.rs`) cannot seed, or seeds only approximately:

1. **The clock.**
   - *Problem:* The 09-30 captures depend on the date: "Today 5:00 PM", "Friday 5:00 PM", "1 overdue task" (task #6 becomes overdue at 17:00 that day) and the Done "Today" group. ChipFlow has no clock override; `src/*.rs` calls `now` in about 43 places. The 2026-10-01 battery already saw "2 overdue tasks".
   - *Needed:* a server-side time shim (for example libfaketime) plus a browser clock override, with TZ America/Chicago. Otherwise, mask every date-dependent region.
2. **Completion dates.**
   - *Problem:* Moving a task into Done stamps `completed_at` with the current time, and the Done group label comes from `completed_at` (`done_group_label`). `grouping_date` can be PATCHed but does not feed the label. So the 10-12 "Wednesday, 30 September" group can only be reproduced by moving the tasks while the clock reads 09-30.
3. **Creation timestamps.**
   - *Problem:* Task `created_at` (the modal's "Created: ..." subline) and comment `created_at` (relative comment times) are assigned by the server. They cannot be seeded.
4. **Assignee identity.**
   - *Problem:* "Chip Senkbeil" must render as "CS". Avatar initials come from the username (`user_initials`), and members are registered users with no separate display name. Seeding needs an admin whose username is literally "Chip Senkbeil", with a space.
   - *Context:* Comments accept an `author` string. The 2026-10-01 battery used user `battery` with comment author "Chip Senkbeil". KF-261 calls the "cs" vs "admin" difference fixture data.
5. **Palette and default.**
   - *Problem:* A blank board seeds 10 colors. The fixture lists 8, so Brown and White must be disabled by PATCH. The fixture does not say which color is the default; capture E shows Yellow.
6. **Logged time.**
   - *Problem:* "logged_hours: 2.5" can only be seeded as manual entries (`POST /api/time/manual`, with a past date and HH:MM times). That makes them "Manually added" entries, not pomodori. The fixture gives no date, time of day or entry type. The 10-01 battery used one 2.5h manual entry on 2026-09-29.
   - *Effect:* This shapes the Time spent report, the timer log, the modal time log and the pomodori counters.
7. **Card section expand state.**
   - *Problem:* Expanded description, due and subtask sections are per-browser localStorage (KF-224). Capture A has #1 and #6 expanded; B, F and G have #1 collapsed. The fixture does not record this, so the harness must set localStorage per capture.
8. **Column display properties.**
   - *Problem:* No KanbanFlow capture shows labels on cards, yet all 22 tasks have labels. The per-column display properties (Edit column, `config_json`) are not in the fixture.
9. **Date-only due date.**
   - *Problem:* Task #4 has `"due": "2026-10-30"`. ChipFlow stores a bare date as the end of that day, while KanbanFlow renders a due date that has no time. Check the rendering (related: KF-348).
10. **"(Done)" on due lines.**
    - *Problem:* Every due line in the captures ends in "(Done)". The fixture has no `due_done` flags. Confirm whether "(Done)" is KanbanFlow's mark-done link or a done state before seeding `due_done`.
11. **Swimlanes.**
    - *Problem:* ChipFlow keeps at least one swimlane and KanbanFlow's free plan has none. This depends on KF-152 hiding the lone header.
12. **Notification badge.**
    - *Problem:* The bell badge (1 or 4 in the captures) is KanbanFlow account state that cannot be seeded. Mask it.
13. **No committed seeder.**
    - *Problem:* Nothing in the repo seeds the fixture. `.dev/tools/visual-audit-setup.sh` only creates an "Audit Board". The production mirror was seeded with a token that is now orphaned (R6). R2 needs a scripted, repeatable seeder for both local mirrors and the DB-identity record.
14. **The fixture doc contradicts itself.**
    - *Problem:* `golden-master-2026-09-30.md` says "Enriched tasks (10)" but lists 7 ids, and "Basic tasks (12)" but lists 15. It also says basic tasks carry name, color, column, labels and assignees only, yet the JSON gives #2, #3 and #5 descriptions, and capture A shows their description icons.
    - *Resolution:* The JSON is the authority.
15. **Separate boards for two references.**
    - *Problem:* The KF-331 Colors reference needs a POMO board and the delete-dialog reference needs a board named "Dale fidelity test". Neither is part of the Golden Master fixture.
16. **The R3 discrepancy.**
    - *Fixture:* "Build payment webhook handler" (#11) has `logged_hours: 2.5`.
    - *KanbanFlow:* The 09-30 captures show "2h 30m / 8h". The 10-05 pomodoro capture shows "2h 46m + 42m": 2h 46m stored, and a running session with a TODAY total of 58m, which is 16m done plus 42m running. Both 10-12 captures show "2h 46m / 8h". So KanbanFlow holds 16 minutes more than the fixture.
    - *Conflict:* HANDOFF.md accounts for about 3 minutes accidental plus 10 minutes pomodoro, which is about 13 minutes, not the 16 the captures show. The 42-minute session in the 10-05 capture did not survive into the 10-12 captures.
    - *Action:* Before deciding R3, read the exact entries from KanbanFlow's task time log rather than from the handoff.
    - *What it blocks:* Until R3 is settled, the #11 time text must be masked in every 10-05 and 10-12 pixel diff, and the side-by-side board comparison cannot match.

## Appendix A. Old GM-NNN ids recoverable by reference

These ids are cited in the tracker, `static/style.css`, `static/app.js`, the templates, `src/routes.rs` or `.dev/evidence/`. I inferred the category ranges from the surviving master paths (`golden-masters/<category>/GM-NNN-<name>.png|.md`).

| range | category (from master paths) | ids still cited | what they showed |
|---|---|---|---|
| 001-016 | top-bar | 001, 002, 005, 007, 009, 010, 015, 016 | Boards button, board-tab row, user avatar, bell, timer pill, invite + |
| 017-020 | board-header-row | 017, 018, 019, 020 | clock, funnel, pencil, Menu button |
| 021-029 | boards-drawer | 021, 022, 026 | drawer overlay and selection (text-source) |
| 030-050 | columns | 030, 031, 032, 033, 037, 040, 043, 045 | collapsed strip, header row, separator, count color, folded swimlane, folded WIP, layout editor, edit column |
| 051-069 | task-cards | 051, 052, 053, 054, 055, 059, 065, 066, 067, 069 | borders, timer-selected outline, Done groups, context menu and submenus, attributes |
| 070-098 | task-detail-modal | 070, 071, 073, 074, 075, 076, 077, 079, 080, 081, 082, 083, 084, 085, 086, 087, 089, 092, 093, 096 | modal body, subline, grid, members, time-spent hover, toolbar, Add menu, sub-dialogs, subtasks, move, reports menu, delete confirm, shortcuts, time log |
| 099-110 | timer-pomodoro (popup) | 099, 101, 103, 104, 105, 109 | TODAY list, running task row, running countdown, take-break view, stopwatch, timer context menu |
| 111-116 | timer-pomodoro (settings) | 111, 112, 113, 114, 115, 116 | settings modal and its four tabs, add-activity dialog |
| 117-130 | timer log / reports / filter | 117, 118, 119, 121, 122, 123, 124, 125, 127, 128, 129, 130 | timer log, entries, export, custom range, date picker, time spent, statistics, filter colors and counts, top-bar reports |
| 131-138 | board-settings | 131, 132, 133, 134, 138 | color rows, (Default), quick-add color modal, settings H1 and chrome, delete board |
| 139-171 | legend, templates, keyboard, dialogs, misc | 139, 141, 144, 146, 149, 150, 151, 156, 157, 158, 163, 164, 165, 167, 169, 170, 171 | legend strip, Pomodoro template, disabled colors, T key, subtask keys, Esc, empty header, layout dialog, labels empty state, styled confirms, column delete, board delete, future-time inline variant, print sheet, export menu, date picker |

That is 99 of 171 ids. The other 72 left no trace in the repo.
