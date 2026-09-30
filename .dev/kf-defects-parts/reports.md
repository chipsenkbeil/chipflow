# Parity defects — reports, filters, legend, swimlanes, misc (audited 2026-09-28, build 199af93c)

Surface keys: reports | filters | legend | swimlanes | misc.
Refs: catalog v1 = kf-video-catalog/v1.md (S1–S16), v3, v4, v7, v8, v9; FR = kf-frame-review-report.md; FS = fidelity-status.md.

## Defects

- **Add the top-bar Reports menu with all 15 items** — Surface: reports
  - KanbanFlow: top-bar reports/chart icon opens a dropdown with verbatim items in order: Pomodoro statistics, Time spent, Print, Board history, Burndown, Calendar, Cumulative flow, Cycle & lead time, Due date performance, Monte Carlo forecasting, Task count, Throughput, Time estimate, Time in column, Export (catalog v1 S15, e023).
  - ChipFlow: no reports menu in the top bar at all; topbar has only Boards, board name, timer pill, user, settings, logout (templates/board.html:6-22). Reports are reachable only via dead full-page routes and the task-modal Reports menu.
  - Category: missing — Severity: high

- **Implement the Timer log page — it currently renders empty** — Surface: reports
  - KanbanFlow: "Timer log" with All boards / Period ("This + Last week", "This week", "Last week", "This month", "Last month", "Custom (absolute)", "Custom (relative)") / Entry type filters; print/export/reload icons; day groups "Friday, 10 July — 3h 20m — 8 Pomodoros"; green "Successful Pomodoro" vs red "Stopped Pomodoro with reason 'X'"; orange dots for stopped; blue "Add time entry" link per day group; empty state "No entries exist for the given filter" (catalog v1 S5/S7; FR).
  - ChipFlow: /timer/log (templates/timer_log.html) has only a Task dropdown filter — no board filter, no period filter, no entry-type filter, no print/export/reload icons, no day grouping, no green/red entries. Worse, the page is broken: `#timer-log-list` is never populated — no code fetches /api/timer/log, and the template calls `TimerLogPage.init()` (timer_log.html:38) which is undefined; only `initTimerLogPage` exists (static/app.js:1718), which no-ops on a nonexistent `timer-log-period` button (app.js:1721). Backend API exists (src/routes.rs:1357 api_timer_log) but is never called.
  - Category: broken — Severity: high

- **Implement the Time spent report UI** — Surface: reports
  - KanbanFlow: full-page "Time spent" with Filter/Print/Export buttons; filter pane (Period: Last 30 days, User: Chip Senkbeil, Color: Show all, Label: "Add labels…", Reload); Group by: Date (descending); View: Summary | Detailed; day-grouped per-task entries with durations; Bookmarks (catalog v1 S10; FR).
  - ChipFlow: only a "Time spent" tab inside /timer/log with From/To date inputs + Apply + an empty bar chart (templates/timer_log.html:23-31); never populated — nothing fetches /api/timer/time-spent (static/app.js). No User/Color/Label filters, no Summary/Detailed views, no Bookmarks, no per-task detail, no Print/Export.
  - Category: missing — Severity: high

- **Implement the Pomodoro statistics page UI** — Surface: reports
  - KanbanFlow: full-page "Pomodoro Statistics"; tabs Pomodoros / Interruptions / Break activities / Highscores; Period presets (Last 7/14/30 days, This week, Last week, This month, Last month, Custom (absolute), Custom (relative)); Group by Day; Board: All boards; Export; Reload; bar chart with weekday tooltips; "No data to display" / "Loading chart..." states; custom-absolute dual-calendar dialog (catalog v1 S16; FR).
  - ChipFlow: /timer/statistics (templates/timer_statistics.html) has a stats-summary div, a "Pomodoros per day" chart div, and an "Interruptions by reason" div — none ever populated. The template calls `TimerStatsPage.init()` (timer_statistics.html:14) which is undefined; only `initTimerStatsPage` exists (static/app.js:1735), and it returns early because the chart div carries no data. No tabs, no Period presets, no Board selector, no Export/Reload, no empty states.
  - Category: broken — Severity: high

- **Add the board Filter panel** — Surface: reports
  - KanbanFlow: right-side Filter panel — User "Show all", Color dropdown ("Show all" (checked), "1 Pomodoro", "2 Pomodoros", "3 Pomodoros", ">3 Pomodoros"), Date "Show all", Labels "Add labels…", "Remember filter" toggle (off), "Bookmarks (0)" (catalog v1 S14, e021); plus a "Timer users" filter — cards with an active timer (FR).
  - ChipFlow: no filter panel of any kind — no User/Color/Date/Labels/Timer-users filters, no Remember filter, no Bookmarks (templates/board.html; static/app.js — no filter code exists).
  - Category: missing — Severity: high

- **Fix P shortcut — it stops the running timer** — Surface: misc
  - KanbanFlow: P = "Open Reports menu" (catalog v7-e018).
  - ChipFlow: P calls `TimerUI.stopClicked()` (static/app.js:1683-1684) — it stops the running timer, while the shortcuts dialog claims "P — Open pomodoro statistics" (templates/board.html:404). Destructive on a live session and wrong on both counts.
  - Category: broken — Severity: high

- **Add "Start" and "History" to the task-modal Reports menu** — Surface: reports
  - KanbanFlow: task-modal Reports menu items verbatim: Time log, Start, History, Time in column (catalog v7 surface 12; v7-e020 frame-verified).
  - ChipFlow: Reports menu has only Time log and Print (templates/modal.html:37-42).
  - Category: missing — Severity: medium (note: Print and Time in column are premium-gated in KanbanFlow per FR, so parity there is not required; Start and History are the genuine gap)

- **Fix Y shortcut — it scrolls instead of opening manual time entry** — Surface: misc
  - KanbanFlow: Y opens the "Add time manually" dialog (FR: entry point "Add menu → Manual time entry (shortcut Y)").
  - ChipFlow: Y scrolls the first task card into view (static/app.js:1676-1678), while the shortcuts dialog claims "Y — Add time entry manually" (templates/board.html:402).
  - Category: divergent — Severity: medium

- **Fix T shortcut — it starts a pomodoro instead of opening the timer menu** — Surface: misc
  - KanbanFlow: T = "Open Timer menu" — opens the menu, starts nothing (catalog v7-e018).
  - ChipFlow: T immediately starts a pomodoro on the open (or no) task via `TimerUI.start('pomodoro', id)` (static/app.js:1679-1682), while the shortcuts dialog claims "T — Open / close the timer popup" (templates/board.html:403).
  - Category: divergent — Severity: medium

- **Fill out the shortcuts dialog to match KanbanFlow's list** — Surface: misc
  - KanbanFlow: shortcuts modal rows in order: Open Move dialog — V; Open Timer menu — T; Open Reports menu — P; Open More menu — `.`; Navigate subtask list — ↑ ↓; Move subtask in list — Cmd + ↑ ↓; Close window / discard changes — Esc; Save changes — Cmd + Enter; Delete task — Delete (catalog v7-e018).
  - ChipFlow: dialog lists only Y/T/P/E/Enter/Esc, mislabels Y/T/P semantics (see above), and omits V, `.`, subtask navigation, Cmd+Enter, Delete entirely (templates/board.html:398-412).
  - Category: divergent — Severity: medium

- **Add Excel/CSV export to the timer log** — Surface: reports
  - KanbanFlow: export icon opens a menu with "Download Excel file" and "Download CSV file" (catalog v1 S9).
  - ChipFlow: no Excel/CSV export exists anywhere — no csv/excel/xlsx references in src/routes.rs, static/app.js, or templates/.
  - Category: missing — Severity: medium

## Surfaces at parity (no defect filed)

- **Color legend bar** (Surface: legend): ChipFlow renders a bottom `footer.color-legend` (templates/board.html, `.color-legend` styles at static/style.css:426-442) listing enabled colors only (src/routes.rs filters `color.enabled` before passing to the template) with `taskColor-{value}` segments and `taskBorderColorBg-{value}` dots plus label and description tooltip — matches FS ("Legend items reuse the same classes") and the spirit of KanbanFlow's bottom pomodoro strip ("1 Pomodoro | 2 Pomodori | 3 Pomodori | >3 Pomodori", catalog v1 S1). Pixel-level styling not verifiable read-only.
- **E shortcut** (Surface: misc): E opens the time-estimate dialog when a task modal is open (static/app.js:1685-1690) — matches KanbanFlow "Time estimate" (catalog v7-e018).
- **Swimlane UI internal consistency** (Surface: swimlanes, reference-unavailable): swimlane header is a `th.swimlane-label` with name + ⋮ menu (templates/board.html); menu offers Rename / Move up / Move down / Delete (templates/board.html, "Swimlane ⋮ menu") — sensible and consistent with the column menu's operations. No defect, but there is no KanbanFlow reference (swimlanes are premium-only per FS).

## Surfaces NOT assessable

- **Swimlane UI vs KanbanFlow reference** — swimlanes are premium-only in KanbanFlow (per fidelity-status.md Discovered behaviors: "Swimlanes are premium-only in KanbanFlow (blocked there)"), so no video or frame reference exists. ChipFlow's swimlane UI was assessed for internal consistency only; visual/behavioral parity against KanbanFlow cannot be established.
- **Reports menu destinations beyond Time spent/Statistics** (Board history, Burndown, Calendar, Cumulative flow, Cycle & lead time, Due date performance, Monte Carlo forecasting, Task count, Throughput, Time estimate, Time in column, Export, Print) — no video coverage of these reports exists in the supplied catalogs/frame-review; their presence/absence in ChipFlow was not separately audited item by item (all are absent, since the top-bar menu does not exist at all).
- **Pixel-level visual fidelity of any surface** — audit was read-only code inspection; no live browser rendering was available, so exact styling/layout comparisons (legend band colors, dialog typography, chart appearance) could not be made.
