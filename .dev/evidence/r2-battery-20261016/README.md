# R2 readiness battery, 2026-10-16

Results of HANDOFF item R2, run against origin/main `62d573d` (release build,
not rebuilt) on throwaway local instances (ports 3240, 3250, 3260, 3270).
Production was not touched. The inventory is `readiness-inventory-20261016.md`
(230 rows, R2-001 to R2-230), and each batch's full report is
`report-<batch>.md`. Every evidence path below is relative to this directory.
The board and admin reports open with a note from their subagent about a
refused write, kept verbatim.

## Totals

Counted from each report's verdict table.

| Batch | Rows | PASS | FAIL | BLOCKED | N/A |
|---|---|---|---|---|---|
| pixel | 43 | 2 | 41 | 0 | 0 |
| board | 54 | 29 | 23 | 0 | 2 |
| modal | 68 | 47 | 17 | 2 | 2 |
| admin | 65 | 40 | 20 | 1 | 4 |
| all | 230 | 118 | 101 | 3 | 8 |

The modal report's own summary line says 46 PASS and 18 FAIL, but its table
holds 17 FAIL rows (R2-091 carries three findings). The table is used here.

N/A rows: R2-003, R2-012, R2-134, R2-166, R2-176, R2-197, R2-198, R2-223.

## Tracker changes

`.dev/kf-parity-defects.md` gained 43 new sections (KF-358 to KF-400) and an
OPEN status line on 32 existing sections:

- reopened from RELEASED: KF-004, KF-233, KF-239, KF-241, KF-259, KF-278,
  KF-279, KF-289, KF-302, KF-304, KF-306, KF-311, KF-316, KF-325, KF-333,
  KF-334, KF-349, KF-350, KF-354
- reopened from FIXED: KF-215, KF-223, KF-245
- first status line on a section that had none: KF-001 and KF-006 (marked
  VERIFIED in the header), KF-010, KF-015, KF-067, KF-082, KF-103, KF-111
- evidence added to an already OPEN section: KF-356, KF-357

`tracker_count.lua` went from OPEN 2 / RELEASED 110 / FIXED 56 / no status 183
to OPEN 75 / RELEASED 91 / FIXED 53 / no status 175 (400 sections).

Five reopens record that the release was built against a reference the
recovered captures contradict: KF-233 (lost GM-001 top bar note), KF-311 (lost
GM-031 header height), KF-316 (count read off the adjacent collapsed strip),
KF-354 (legend hexes sampled from a colour-managed macOS screenshot) and
KF-239 (pill taken from v1-era video). KF-223 and KF-279 are the same kind of
case.

## FAIL to KF mapping

| Row | KF | Kind |
|---|---|---|
| R2-001 | KF-233 | reopened |
| R2-002 | KF-239 | reopened |
| R2-005 | KF-379 | new |
| R2-006 | KF-380 | new |
| R2-010 | KF-361 (clock icon under KF-239) | new |
| R2-011 | KF-378 | new |
| R2-013 | KF-358 | new |
| R2-015 | KF-359 | new |
| R2-017 | KF-369 | new |
| R2-021 | KF-215 | reopened |
| R2-023 | KF-370 | new |
| R2-024 | KF-381 | new |
| R2-028 | KF-372 | new |
| R2-029 | KF-382 | new |
| R2-033 | KF-383 | new |
| R2-036 | KF-311, count cell under KF-316 | reopened |
| R2-037 | KF-311 | reopened |
| R2-039 | KF-362 | new |
| R2-040 | KF-362 | new |
| R2-041 | KF-223 | reopened |
| R2-042 | KF-223 | reopened |
| R2-043 | KF-241 | reopened |
| R2-044 | KF-223 | reopened |
| R2-046 | not filed (see below) | |
| R2-047 | KF-223 | reopened |
| R2-048 | KF-384 | new |
| R2-049 | KF-306 | reopened |
| R2-051 | not filed (see below) | |
| R2-052 | KF-316 | reopened |
| R2-053 | KF-385 | new |
| R2-054 | KF-365 | new |
| R2-055 | KF-365 | new |
| R2-056 | KF-365 | new |
| R2-057 | KF-387 | new |
| R2-058 | KF-371 | new |
| R2-060 | KF-363 | new |
| R2-061 | KF-245 | reopened |
| R2-062 | KF-376 | new |
| R2-063 | KF-377 | new |
| R2-064 | KF-363 | new |
| R2-065 | KF-259 | reopened |
| R2-066 | KF-259 | reopened |
| R2-067 | KF-259 | reopened |
| R2-068 | KF-364 | new |
| R2-069 | KF-363 | new |
| R2-070 | KF-363 | new |
| R2-073 | KF-103 | first status line |
| R2-074 | KF-325 | reopened |
| R2-076 | KF-333 | reopened |
| R2-077 | KF-334 | reopened |
| R2-079 | KF-386 | new |
| R2-091 | KF-360 (writes every task), KF-259 (wording, "(Done)") | new, reopened |
| R2-093 | KF-278 | reopened |
| R2-094 | KF-067 | first status line |
| R2-103 | KF-015 | first status line |
| R2-107 | KF-388 | new |
| R2-108 | KF-302 | reopened |
| R2-109 | KF-302 | reopened |
| R2-110 | KF-001 | first status line |
| R2-112 | KF-366 | new |
| R2-113 | KF-366 | new |
| R2-114 | KF-366 | new |
| R2-115 | KF-279 | reopened |
| R2-116 | KF-010 | first status line |
| R2-117 | KF-006 | first status line |
| R2-126 | KF-375 | new |
| R2-127 | KF-239 | reopened |
| R2-135 | KF-389 | new |
| R2-137 | KF-380 | new |
| R2-138 | KF-004 | reopened |
| R2-139 | KF-390 | new |
| R2-141 | KF-289 | reopened |
| R2-151 | KF-391 | new |
| R2-152 | KF-392 | new |
| R2-155 | KF-393 | new |
| R2-158 | KF-350 | reopened |
| R2-172 | KF-367 | new |
| R2-173 | KF-367 | new |
| R2-174 | KF-367 | new |
| R2-179 | KF-354 | reopened |
| R2-183 | KF-357 | evidence added |
| R2-184 | KF-357 | evidence added |
| R2-185 | KF-111 | first status line |
| R2-186 | KF-356 | evidence added |
| R2-188 | KF-394 | new |
| R2-190 | KF-395 | new |
| R2-191 | KF-394 | new |
| R2-192 | KF-082 | first status line |
| R2-193 | KF-396 | new |
| R2-199 | KF-368 | new |
| R2-200 | KF-304 | reopened |
| R2-204 | KF-394 | new |
| R2-208 | KF-380 | new |
| R2-210 | KF-397 | new |
| R2-211 | KF-398, KF-374 (attachment cap) | new |
| R2-212 | KF-399 | new |
| R2-215 | KF-400 | new |
| R2-219 | KF-373 | new |
| R2-220 | KF-373 | new |
| R2-221 | KF-373 | new |
| R2-222 | KF-349 | reopened |

Duplicates merged across batches: due wording (pixel-10, modal-2, modal-3 into
KF-259), new-task modal (pixel-14, modal-4 into KF-302), board menu (pixel-15,
modal-17 into KF-367), Done "Today" (pixel-12, board-6 into KF-365), console
errors (board-11, admin-9 and modal FAIL 15 into KF-380), timer pill (board-8,
pixel-18 into KF-239), and dead dialog buttons (admin-5, admin-8 into KF-394).
Where the merged severities differed (KF-239 MEDIUM and LOW, KF-223 HIGH and
LOW) the higher one was kept.

## FAILs not filed

- R2-046: the column menu has a 4th "Delete" item. KF-223's live-verified menu
  has 3 items and KF-230 asserts a delete item without a capture, so the
  references conflict and the board batch proposed no defect. Needs a fresh
  capture of KanbanFlow's column right-click menu.
- R2-051: mid-drag the column body stays visible with no gap at the drop
  position (`board/shots/s8d-03-col-middrag.png`). The only reference is
  KF-048's v1-era video frame, and `dragend` never fired under Playwright.
  Needs a fresh mid-drag capture.

`modal/shots/s15-menu-02-menu-pomo-legend-on.png` is kept as the legend-on
view of KF-367's menu.

## BLOCKED rows

- R2-132 (timer audio): needs reference recordings of KanbanFlow's alarm,
  ticking and volume behaviour from Chip's account.
- R2-133 (Picture-in-Picture window): needs a capture of KanbanFlow's PiP
  window from Chip's account. ChipFlow's toggle persists.
- R2-230 (touch drag on a phone): needs a CDP `Input.dispatchTouchEvent`
  start/move/end harness that SortableJS accepts, or a real phone or emulator
  pointed at a non-production instance.

Fresh captures from Chip's KanbanFlow account would also settle: the column
right-click menu (R2-046), a column mid-drag frame (R2-051), the card colour
submenu rows (R2-076, KF-333), "Edit grouping date" on a Done-column card
(R2-057, KF-387), a freshly created Pomodoro board's palette (R2-028, KF-372),
what the "(Done)" parenthetical names (KF-259), and whether expanded column
headers carry a count (KF-316).

## Observations not filed

No KanbanFlow reference and no documented ChipFlow contract, or outside the
battery's rows.

Pixel:

- The KF-331 reference H1 "Board settings: General" names the board, and
  ChipFlow matches it with 0 differing pixels. The inventory note saying the H1
  does not name the board is wrong.
- The 10-12 macOS captures left-align column names and the overdue strip text,
  the 09-30 Linux captures center them. ChipFlow centers.
- ChipFlow's column default hides inline subtasks, the GM capture shows them.
  KanbanFlow's default is unverified.
- Timer log `day_key` is the UTC date (an entry at 22:26 CDT on 10-05 is keyed
  `2026-10-06`).
- Both apps render card text in San Francisco on macOS.
- KanbanFlow's two near-duplicate 09-30 captures differ in 328 px, all at the
  cursor.

Board:

- `static/app.js` keeps dead favorites code aimed at a missing
  `#bs-favorites`. The v4 video "sidebar" behind KF-088 lists browser tabs, so
  it is not KanbanFlow UI.
- KF-236's chart icon comes from v1-era frames, and no current capture shows it.
- Template-created Done tasks with a null `completed_at` group under a flat
  "Completed" header.
- Board lists are ordered by board UUID, so a new board lands at a random
  position.
- An archived board opens normally from its URL with no archived indication.
- Remember filter is one global localStorage key and leaks across boards.
- The overdue indicator ignores the active filter.
- The layout view hint still mentions removed "⋮ menus", and Esc does not
  close the view.
- Opening a card submenu hides the main context menu.
- Quick-add inserts new cards at the top of the column.
- Large task names is a global setting, not per board.
- Requests whose body is never read, or that race a reload, report
  `net::ERR_ABORTED` and would trip a strict zero-failed-requests gate.

Modal:

- `POST /api/tasks/{id}/time {"minutes":30}` at 16:17 logged an entry ending
  in the future.
- Date-only dues are stored as 23:59:59 server-local time, not UTC.
- More > Copy copies only name, colour and column.
- History headings are raw event codes ("SUBTASK_RENAMED").
- The modal footer repeats "Created Oct 16, 2026" below the subline's
  "Created: Today".
- The estimate dialog opens empty, and "0h" is rejected, so an estimate cannot
  be cleared from the UI.
- Timer settings: reasons and activities save instantly, durations and sounds
  need Save, Close discards silently, and an API value outside the dropdown (1
  minute) displays as "15 minutes" and would be overwritten on Save.
- "Add new reason…" ignores Enter.
- Stopping a stopwatch after 20 s opens "Why did you stop?".
- Stopwatch mode is forgotten on reload.
- The server never completes a pomodoro by itself. With no browser open the
  status stays active at 0 remaining and an API stop logs it as interrupted.
- Replacing a running timer logs "Switched task" even for the same task.
- Deleting a task whose timer runs discards the elapsed time silently.
- Breaks have no task and are never logged, so Break activities statistics
  stay at 0 (R2-149, KF-347).
- Timer log day totals sum truncated per-entry minutes, and "9 Pomodoros"
  counts stopped sessions.
- Durations of an hour or more show "150m" in the time logs and API while cards
  use "2h 30m". The task sub-view uses a hyphen in time ranges, other views an
  en dash.
- Calendar is month view only, chip clicks do nothing, chip subtitles read "On
  11. Oct".
- iCal "Reset link" uses native `window.confirm`.
- Running pill digits are white. Frame review §e.2 reports red from
  low-resolution frames.
- KF-283 (GM-112) and frame review §b disagree on KanbanFlow's default long
  break (10 vs 15 minutes).
- The Members dialog adds a blue "Done" button that KF-060 does not list.

Admin:

- Sessions never expire on the server (`created_at` is never checked,
  `src/db.rs` ~603).
- A session ended mid-use fails silently on card click and shows "Could not add
  task: unauthorized" on add. Nothing redirects to `/login`.
- Every `/settings` save sets `ding_enabled` and `notifications_enabled` to
  false (`src/routes.rs` ~6306). Nothing reads them today.
- `pomodoro_minutes=0` saved through the form silently becomes 25.
- Horizontal overflow at 400 px on `/settings`, board settings tabs, reports
  and the calendar (scroll widths in `report-admin.md`).
- The board history type filter mixes labels with raw ids.
- Monte Carlo counts 5 completions where Cycle & lead and Throughput count 4.
- Break activities show `daily_goal` as minutes while the docs define a count.
- htmx and SortableJS load from `cdn.jsdelivr.net`, so a self-hosted install
  needs internet for drag and drop and add-task forms.
- Creating a column without `board_id` puts it on the first board, and the last
  column on a board can be deleted.
- The only board can be archived, after which `/api/boards` returns `[]`.
- A lowercase `bearer` scheme is not recognized.
- `expires_in_days` of 0 or less creates a never-expiring token.
- OpenAPI documents no 403 or 413 responses.
- The printer glyph (U+1F5B6) renders as an empty box in this Chromium.
- The guide titles `GET /api/tasks` "List the board's tasks" but it lists all
  boards, and never mentions the "Kanban basics" template.
- Statistics counts stopped pomodori in "Pomodori" (4) while the task's
  `pomodori_completed` (2) does not.
- Dark mode is a per-browser localStorage setting, not an account setting.
- `seed-golden-master.sh` was untracked. It is now at
  `.dev/tools/seed-golden-master.sh`. It deletes Brown and White where the
  inventory's GM profile expects them disabled.

## R3 finding

The 10-05 KanbanFlow capture shows "2h 46m + 42m / 8h" on "Build payment
webhook handler" (#11), where `.dev/golden-master-board.json` gives 2.5 logged
hours ("2h 30m"). The 16 minute gap is larger than the ~3 minute accidental
entry plus the 10 minute pomodoro HANDOFF R3 accounts for. The battery did not
compare the "+ 42m" live part. Evidence: `pixel/crops/R2-073-074-card11.png`.
