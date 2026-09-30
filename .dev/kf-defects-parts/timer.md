# Timer system — parity defects (audit 2026-09-28, build 199af93c)

Reference checks: frame-review report (961 unique states, P1/P2 checklist), catalogs
v1 (S4), v3 (S2–S9), v4 (stop menu + panel), v7 (S7 add-time, S10 time log view,
S11 shortcuts), v8 (break at 00:00), v9 (stopwatch + discard toast); survey frames
/tmp/kf-survey/v9/e002.jpg, e003.jpg and /tmp/kf-survey/v8/e001.jpg viewed directly.
No duplicates of the two Chip-filed Known defects (both are outside this surface).

## Defects

- **Reconcile TimerUI with the real /api/timer/status shape — popup, pill, and card indicators never reflect live state** — Surface: timer
  - KanbanFlow: idle popup shows big "25:00" + green ▶ Start; running panel counts down (v1 S4, v3 S2); header pill shows "▶ 25:00 ▾" idle / live countdown while running (v3-e022, v9-e002); task row shows the current task; TODAY lists entries (v1 S4).
  - ChipFlow: app.js `updateFromStatus` reads `status.phase / remaining_seconds / total_seconds / session_id / pomodoro_count / task_url` (static/app.js:887-901), but the server returns `{active, mode, mode_title, task_id, task_name, started_at, duration_secs}` (src/routes.rs:2997-3005) — none of the expected fields exist. Consequences: popup clock is stuck at "--:--" (nothing ever writes #timer-popup-time), the red ■ Stop button shows while idle and green ▶ Start stays hidden, task row is always "No task", `#timer-today-list` is never populated (no JS fetches /api/timer/today), the pill always renders the label "Pomodoro" (pillLabel, app.js:949-961), and `updateCardIndicators` toggles `.card-timer-indicator`, which doesn't exist in templates/task_card.html.
  - Category: broken — Severity: high

- **Stop requests never reach the server — every stop 400s, so no session is ever logged** — Surface: timer
  - KanbanFlow: stopping logs the entry (v1 S5/S7 show stopped entries in the log).
  - ChipFlow: `finishSession` posts `{"reason":"completed"}` and `stopAndLog` posts `{"reason":reason}` (static/app.js:1217-1276), but `TimerStopInput` requires a non-optional `completed: bool` with no `#[serde(default)]` (src/routes.rs:3129-3136) — both bodies fail deserialization ("missing field `completed`") and return 400. The client ignores the error (the `.then` doesn't check `res.ok`), so the UI pretends the stop succeeded while the session is never logged. `whyTaskDone` additionally moves the task to Done via `moveTaskToDone` after a failed stop.
  - Category: broken — Severity: high

- **Rewire the "Add time manually" dialog end to end — markup exists but every wire is dead** — Surface: timer
  - KanbanFlow: Task field, Date + calendar icon (Sun–Sat grid, selected day blue), From/To time fields, auto-computed Duration ("0h" for zero), "Add comment"/"Add labels" toggles, green Add (v3 S5, v7 S7); future-time error "You can not enter a time in the future" + OK with dialog state preserved (v3 S7).
  - ChipFlow: the `mt-*` dialog in templates/board.html:78-109 has no working JS. `ManualTime` targets `manual-time-*`/`edit-entry-*` ids that exist nowhere (static/app.js:1358-1490); `submit()` posts to `/api/time-entries` (no such route — the server endpoint is `POST /api/time/manual` with `{task_id, date, from, to, note}`, src/routes.rs:1594) with `{duration_minutes, comment, source}` (server expects From/To and computes duration itself). `ManualTime.open` crashes on `document.getElementById('manual-time-task').textContent` (null). The task datalist `#mt-task-list` is never populated, From/To→Duration is never computed, the calendar button/popup (`mt-cal-btn`, `mt-cal-popup`) are unwired, the comment toggle is unwired, and the future-time overlay `#mt-error-overlay` (correct exact text, board.html:146) is never shown — the error path instead targets a nonexistent `#future-time-overlay`, and its OK button calls `ManualTime.closeError()`, which throws on the missing `#manual-time-error`.
  - Category: broken — Severity: high

- **Rewire the Edit time-entry dialog end to end — Edit buttons are dead and the dialog targets wrong endpoints** — Surface: timer
  - KanbanFlow: "Edit Pomodoro entry" dialog — Date with green ✓ validation, Task autocomplete (reassignment possible), From/To with seconds, Duration, "+ Add comment"/"+ Add labels", green Update → "Updating…" state; date edits re-bucket the entry (v3 S8; frame review).
  - ChipFlow: the `ee-*` dialog in templates/board.html:111-141 has no working JS. `EditEntry` targets `edit-entry-*` ids that exist nowhere (static/app.js:1495-1610); `submit()` PATCHes `/api/time-entries/{id}` (no such route or method — the server is `PUT /api/time/entries/{id}` with `{task_id, date, from, to, note}`, src/routes.rs:1672) and `remove()` DELETEs the same nonexistent path. The delegation listens for `[data-edit-entry]` (app.js:1616-1629) but time_entries.html renders `data-entry-id`, so clicking Edit never even reaches the (broken) opener. The green ✓ validation, task autocomplete, and "Updating…" state have no JS behind them.
  - Category: broken — Severity: high

- **Populate the "Why did you stop?" menu — reasons never render and the opener crashes** — Surface: timer
  - KanbanFlow: dark menu, header "Why did you stop?" + 17 items verbatim in order: Boss interrupted, Colleague interrupted, Context switch, Dog, Email, Family, Finished with no new task, Food Delivery, Meeting, Other, Phone call, Restroom, Sleep, Web browsing, Workchat, "Add new reason…", "Task done" (v4 S2, zoom-verified).
  - ChipFlow: nothing ever populates `#why-stop-reasons` (zero references in static/app.js), so the menu opens empty. `beginWhy` then throws on `document.getElementById('why-task-name').textContent` and `#why-elapsed`, neither of which exists in templates/board.html:65-75. The defaults exist server-side (src/models.rs:260-276) but are never fetched into the menu.
  - Category: broken — Severity: high

- **Restore the break flow — the "Take a break:" row is permanently hidden and natural completion 400s** — Surface: timer
  - KanbanFlow: at 00:00 the panel shows "00:00" with a single green "Take break" button next to the clock (v8 S1; frame v8-e001 verified).
  - ChipFlow: `#timer-popup-breaks` ("Take a break:" + "Short break"/"Long break" buttons, templates/board.html:50-54) is never unhidden by any JS. `finishSession` (app.js:1215-1230) posts the invalid `{reason:'completed'}` body (see the stop-400 defect above) and plays the chime regardless of the failed request, so a pomodoro that runs to zero is never logged as completed. There is also no "long break every 4th break" logic anywhere client- or server-side.
  - Category: broken — Severity: high

- **Show the "Session discarded" toast for stops under 20s instead of the why menu** — Surface: timer
  - KanbanFlow: stopping before 20s shows a dark toast verbatim "Session discarded" / "Session lasted less than 20 seconds", the entry is removed, and no reason menu appears (v9 S3; frame v9-e003 verified).
  - ChipFlow: the server discards <20s sessions and returns `TimerStopResult{discarded, minutes, completed}` (src/routes.rs:3183-3197), but the client ignores the `discarded` flag — `stopClicked` always opens the why menu regardless of elapsed time, and nothing ever renders the toast text (no "discard" reference anywhere in static/app.js). Card badges can't revert on discard either since the live-increment is missing.
  - Category: broken — Severity: high

- **Show the configured duration with green play and chevron on the idle pill; red ■ + 00:00 + ⌄ for idle stopwatch** — Surface: timer
  - KanbanFlow: idle pill is dark with a green ▶, "25:00", and a ⌄ dropdown (v1 S1, v3-e022); stopwatch idle pill shows a red ■, "00:00", and ⌄ (v9-e003, frame review).
  - ChipFlow: the pill label is always the text "Pomodoro" (pillLabel, static/app.js:949-961), there is no chevron element in templates/board.html:10-13, and the dot is a pink rounded square (`border-radius: 3px`, `#e57373`, static/style.css:1028-1033) — no green play triangle, no red square, no dropdown arrow in any state.
  - Category: divergent — Severity: medium

- **Switch the panel to Stopwatch title, "Session time" label, and count UP from 00:00 in stopwatch mode** — Surface: timer
  - KanbanFlow: stopwatch panel header is "Stopwatch", the label reads "Session time", the clock counts up from "00:00", red ■ Stop, no progress fill (v9 S3; frame v9-e003 verified).
  - ChipFlow: `#timer-popup-title` and `#timer-popup-label` are never updated — the panel always reads "Pomodoro" / "Time until break" (templates/board.html:36-39; no JS references either id), and the clock never counts up (nothing writes #timer-popup-time).
  - Category: divergent — Severity: medium

- **Rename the footer's first tab to the other mode ("Pomodoro" while in stopwatch mode)** — Surface: timer
  - KanbanFlow: the bottom-nav first tab names the other mode: "Pomodoro" in stopwatch mode, "Stopwatch" in pomodoro mode (v4-00002, v4-00037, frame review).
  - ChipFlow: `#timer-foot-mode` always shows "Stopwatch" with a fixed `title="Stopwatch"` (templates/board.html:58); no JS references `timer-foot-mode`, and `switchModeTab` only toggles nonexistent `.timer-modes button`s (static/app.js:1143-1150), so clicking it changes nothing.
  - Category: divergent — Severity: medium

- **Hide the header pill while the timer popup is open (and in layout-edit view)** — Surface: timer
  - KanbanFlow: the pill is absent while the timer panel is open/docked and in layout-edit view (frame review P1.9: v1-02504–02516, v2-01808).
  - ChipFlow: `renderPill` forces `pill.hidden = false` with the comment "The pill is the always-visible timer dropdown control; never leave it hidden" (static/app.js:966-970) — the pill stays visible under the open popup.
  - Category: divergent — Severity: medium

- **Wire the popup "Change task" button and show "Select open task" when another task's modal is open** — Surface: timer
  - KanbanFlow: task-row link reads "Change task" normally, "Select open task" when a different task's modal is open (v1-02073, zoom-verified, frame review).
  - ChipFlow: `#timer-popup-task-btn` has no click handler (templates/board.html:48); the only `changeTask()` posts to `/api/timer/change-task`, which has no route (only `/api/timer/retarget`, src/routes.rs:3250) — a 404 — and uses a native `window.prompt` with an invented "Are you sure you want to switch tasks mid-Pomodoro?" confirm (static/app.js:1291-1325).
  - Category: broken — Severity: medium

- **Render the why-stop items as 17 verbatim menu items ("Add new reason…" then "Task done" last); "Task done" must log a reason, not complete the task** — Surface: timer
  - KanbanFlow: the 15 defaults, then "Add new reason…" (ellipsis), then "Task done" as the final menu item — all plain clickable items (v4 S2).
  - ChipFlow: "Task done" is a separate full-width button below the menu (templates/board.html:74) rather than the last item; the add-reason row is an input + Add button instead of the "Add new reason…" menu item. Worse, `whyTaskDone()` stops with reason "completed" and then moves the task to the Done column (static/app.js:1281-1289) — KanbanFlow's "Task done" is just a stop reason, it doesn't complete the task.
  - Category: divergent — Severity: medium

- **Rebuild the per-task Time log view with day groups, trash icons, and KanbanFlow's row layout** — Surface: timer
  - KanbanFlow: header is back arrow + task name, title "Time log", "+ ADD ENTRY" top-right; day-grouped sections ("Today 2m", "Yesterday 40m"); each entry is "Chip Senkbeil — 31s — 11:19 AM – 11:19 AM" with a red trash icon (v3 S4, v7 S10).
  - ChipFlow: templates/time_entries.html renders a flat newest-first list of "Nm [badge] ⚠ interrupted note date" rows with a text "Edit" button; the modal section header is a bare `<h3>Time log</h3>` with a "+ Add entry" button below (templates/modal.html:75-77) — no back-arrow sub-view, no day grouping, no user name per row, no trash icons.
  - Category: divergent — Severity: medium

- **Render stopped sessions as red "Stopped Pomodoro with reason 'X'" instead of "⚠ interrupted"** — Surface: timer
  - KanbanFlow: stopped entries show a red status line "Stopped Pomodoro with reason 'Other'" (v1 S5/S7, v4 S4; frame review).
  - ChipFlow: stopped entries show "⚠ interrupted" with the reason only as a `title` tooltip (templates/time_entries.html:9); nothing renders the red "Stopped Pomodoro with reason 'X'" line anywhere.
  - Category: divergent — Severity: medium

- **Live-increment the card badge during a run ("42m + 1m") and revert on discard** — Surface: timer
  - KanbanFlow: the card's time badge live-increments during the run ("42m + 1m") and reverts if the session is discarded (v4-00001–00002, frame review).
  - ChipFlow: a `<span class="card-live" hidden>` exists in templates/task_card.html with CSS classes for it (static/style.css:355-357), but no JS ever unhides or updates it — the badge never changes during a run.
  - Category: missing — Severity: medium

- **Fix keyboard shortcuts: T opens the timer menu, P opens the Reports menu, Y opens "Add time manually"** — Surface: timer
  - KanbanFlow: T = timer menu, P = reports menu, Y = manual time entry, E = time estimate (v2-02558, v2-02705; frame review).
  - ChipFlow: T immediately starts a pomodoro on the modal task instead of opening the timer menu; P calls `TimerUI.stopClicked()` (it stops the running timer!) instead of opening the Reports menu; Y scrolls the first card into view instead of opening the manual-time dialog (static/app.js:1656-1679). Only E matches.
  - Category: divergent — Severity: medium

- **Add the task-modal Time-spent hover menu ("Add time entry" / "Open time log")** — Surface: timer
  - KanbanFlow: hovering "Time spent" in the task modal shows a hover menu with "Add time entry" and "Open time log" (v3-01449, frame review).
  - ChipFlow: the modal stats row is static text with no hover menu (templates/modal.html:69-74); the Timer submenu's "Time log" item merely scrolls to the modal's log section (static/app.js:663-665) instead of opening a log view.
  - Category: missing — Severity: medium

- **Match the task-modal counters and drop the extra Interruptions stat** — Surface: timer
  - KanbanFlow: the modal shows "🍅 1 Pomodoro" and "Time spent: 17m" (v3 S3, v8-e004; frame review) — no interruptions counter.
  - ChipFlow: the modal shows "🍅 N Pomodori", "⏱ Nm spent", and an extra "⚠ N interruptions" stat with no KanbanFlow counterpart (templates/modal.html:69-74).
  - Category: divergent — Severity: medium

- **Use the "Pomodoro / Stopped with reason 'X'" tooltip on P badges** — Surface: timer
  - KanbanFlow: per-task P-badge tooltips read "Pomodoro", or "Stopped with reason 'Other'" for stopped sessions (v3-01594; frame review).
  - ChipFlow: every P badge shows the tooltip "Pomodori" regardless of stopped state (src/routes.rs:669-672, 1401-1404).
  - Category: divergent — Severity: low

- **Use the "Manually added" tooltip on per-task M badges** — Surface: timer
  - KanbanFlow: the per-task M badge tooltip reads "Manually added" (v3-01594; frame review). ("Manually added time" is the board-log sub-line wording, v3-02891.)
  - ChipFlow: the badge tooltip is "Manually added time" (src/routes.rs:669-672).
  - Category: divergent — Severity: low

- **Title the edit dialog "Edit Pomodoro entry"** — Surface: timer
  - KanbanFlow: the dialog title is "Edit Pomodoro entry" (v3 S8).
  - ChipFlow: the dialog title is "Edit time entry" (templates/board.html:113).
  - Category: divergent — Severity: low

- **Add the × close button to the "Add time manually" dialog** — Surface: timer
  - KanbanFlow: the dialog title has an × close (v3 S5, v7 S7).
  - ChipFlow: the dialog has no close button — only Cancel/Add in the footer (templates/board.html:78-109).
  - Category: missing — Severity: low

- **Add the "Add labels" toggle to the manual and edit dialogs** — Surface: timer
  - KanbanFlow: both dialogs have "Add comment" and "Add labels" toggles/links (v3 S5/S8, v7 S7).
  - ChipFlow: only a "+ Add comment" toggle exists in each dialog (templates/board.html:100, 132); no labels affordance anywhere.
  - Category: missing — Severity: low

- **Keep second-level precision for sub-minute durations ("31s", not a 1m floor)** — Surface: timer
  - KanbanFlow: a 31s session renders "31s" in the time log (v3 S4, v1 S5).
  - ChipFlow: `log_timer_session` stores `minutes = (elapsed / 60).max(1)` (src/routes.rs:3202-3204), so a 31s session is logged and displayed as 1m — second-level precision is lost at write time.
  - Category: divergent — Severity: low

- **Update the tab title with the live timer (countdown/count-up)** — Surface: timer
  - KanbanFlow: the browser tab title counts down ("00:00 General - Kanba…", v8-e001) / counts up for stopwatch (frame review).
  - ChipFlow: `document.title` is never updated by TimerUI (no reference in static/app.js).
  - Category: missing — Severity: low

- **Fix the Add-reason row: "Add new reason…" placeholder and a working Add button** — Surface: timer
  - KanbanFlow: the menu's 16th item is "Add new reason…" (with ellipsis, v4 S2).
  - ChipFlow: the input placeholder is "Add new reason" (no ellipsis, templates/board.html:70), and the Add button calls `addWhyReason()` with no argument, so it posts `reason: undefined` instead of reading the input value (static/app.js:1277-1279).
  - Category: divergent — Severity: low

- **Match the per-task log header ("+ ADD ENTRY") and add the dedicated back-arrow Time log sub-view** — Surface: timer
  - KanbanFlow: the per-task Time log is a modal sub-view with back arrow "← Retro categorization", title "Time log", and "+ ADD ENTRY" at top-right (v7 S10).
  - ChipFlow: the modal has an inline `<h3>Time log</h3>` section and a "+ Add entry" button below it (templates/modal.html:75-77) — no back-arrow sub-view, and the button wording differs.
  - Category: divergent — Severity: low

## Surfaces that could NOT be assessed (and why)

- **Break-countdown visuals and break settings**: the "Break activities" tab (v2-03512), break-duration settings (Work 25 / Short 5 / Long 15 / Every 4th), and the actual break countdown panel never appear in frames; only the pre-break "Take break" state is captured (v8-e001). ChipFlow's break modes, durations, and long-break-every-4th logic cannot be verified against KanbanFlow.
- **"Why did you stop?" pick behavior**: v4 captures the open menu and hover on "Meeting" but no selection; post-pick behavior (toast, log entry rendering) is described from the v1 log frames, not from an observed pick.
- **Stopwatch count-up beyond 00:02**: only "00:02" is framed (v9-e002); longer count-up formatting (e.g. past 1h) is unverified.
- **Live toast styling/position**: the discard toast's dark style and bottom-right position are verified (v9-e003), but no other toast types (e.g. completion) are captured in the timer frames.
- **Task-row "Select open task" click target**: zoom-verified as a label in v1-02073 (frame review), but no frame shows the click landing state; ChipFlow has no handler to compare against.
- **Manual-time Task-field prefill**: v3 S5 shows it prefilled with "Retro categorization"; v7 S7 shows no Task field at all. Prefill rules are inconsistent across references, so only the presence of the field (and its broken wiring) is filed.
- **Live behavior**: this audit is source-only — no browser was available, so actual runtime symptoms (e.g. exact crash timing in beginWhy, the 400 responses) are inferred from code, not observed.
- **Full-page Timer log / Time spent report / Statistics**: out of this surface's scope (reports auditor's area); only the per-task log view and the manual/edit dialogs are covered here.
