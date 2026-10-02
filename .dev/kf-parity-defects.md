# KanbanFlow → ChipFlow parity defects (merged audit)

- Audited: 2026-09-28/29 against deployed build `199af93c` (source at `/tmp/chipflow-audit-src`).
- Method: static source comparison (templates, `static/app.js`, `static/style.css`, `src/routes.rs`, `src/models.rs`) against Chip's walkthrough videos (frame catalogs `kf-video-catalog/v1,v3,v4,v7,v8,v9`), the 961-state frame review (`kf-frame-review-report.md`), the hi-res board screenshot, and the color-settings screenshot. **No live browser was available to this audit** — runtime behavior is inferred from code, not observed; behavioral confirmation is pending.
- Two load-bearing timer defects (KF-001, KF-002) were independently re-verified by the merge coordinator via direct source reads — marked **[VERIFIED]**.
- Per-surface detail files: `.dev/kf-defects-parts/{timer,header,columns,cards,task-modal,settings,reports}.md`.
- Reference corrections applied during merge: v7 catalog's Reports-menu "Start" → **"Print"** (v7-e020); v3 catalog's "Repeat it" → **"Reports"** (v3-e003/v7-e020).
- Contradiction resolved: `fidelity-status.md:132` ("No right-click menu on task cards") is **superseded** — v1-e003 shows the card context menu open and the frame review independently documents it ("Card context-menu Timer submenu"). The menu exists; right-click trigger per v1-S2.

## Totals

- **149 actionable defects** (150 headers KF-001–KF-150; KF-143 is reserved — description missing from browser handoff): high 41 · medium 53 · low 54 · critical 1
- By category: broken 35 · divergent 70 · missing 44
- By surface: timer 28 · header 14 · columns 21 · cards 7 · task modal 21 · settings 20 · reports 6 · cross-surface 11 · board 10 · colors 5 · legend 2 · swimlanes 1 · filter 1 · menu 1 · api 1
- Status as of 2026-09-29 ~18:25 CDT: **all 149 actionable defects FIXED** (KF-143 reserved, not actionable). The original 2026-09-28/29 audit covered the first 103; KF-104+ were filed by later browser passes.
- **2026-10-01 exhaustive golden-master audit**: 171 KanbanFlow UI elements inventoried (checklist `.dev/golden-master-coverage.md`), every element verified against a golden master in five independent waves plus adversarial spot-checks. **86 new defects filed KF-233–KF-318, all OPEN**: severity high 5 · medium 38 · low 33. Evidence: `.dev/evidence/golden-masters/`, `.dev/evidence/kf-drafts/`. 18 elements remain unobservable from this environment (live timer states, hover/mid-drag, transient toasts — no KanbanFlow access); they are known gaps, not guesses.

## Timer (KF-001–KF-021)

### KF-001 — Reconcile TimerUI with the real /api/timer/status shape — popup, pill, and card indicators never reflect live state **[VERIFIED]**
- Surface: timer · Category: broken · Severity: high
- KanbanFlow: idle popup shows big "25:00" + green ▶ Start; running panel counts down (v1 S4, v3 S2); header pill shows "▶ 25:00 ▾" idle / live countdown while running (v3-e022, v9-e002); task row shows the current task; TODAY lists entries (v1 S4).
- ChipFlow: `updateFromStatus` reads `status.phase / remaining_seconds / total_seconds / session_id / pomodoro_count / task_url` (static/app.js:887-901), but the server returns `{active, mode, mode_title, task_id, task_name, started_at, duration_secs}` (src/routes.rs:2997-3005) — none of the expected fields exist. Consequences: popup clock stuck at "--:--", red ■ Stop shows while idle and green ▶ Start stays hidden, task row always "No task", `#timer-today-list` never populated (no fetch of /api/timer/today), pill always renders "Pomodoro", and `updateCardIndicators` toggles `.card-timer-indicator`, which doesn't exist in templates/task_card.html.

### KF-002 — Every stop 400s, so no session is ever logged **[VERIFIED]**
- Surface: timer · Category: broken · Severity: high
- KanbanFlow: stopping logs the entry (v1 S5/S7 show stopped entries in the log).
- ChipFlow: `finishSession` posts `{"reason":"completed"}` and `stopAndLog` posts `{"reason":reason}` (static/app.js:1217-1276), but `TimerStopInput` requires a non-optional `completed: bool` with no `#[serde(default)]` (src/routes.rs:3129-3136) — both bodies fail deserialization ("missing field `completed`") and return 400. The client ignores the error, so the UI pretends the stop succeeded while the session is never logged. `whyTaskDone` additionally moves the task to Done after a failed stop.

### KF-003 — Rewire the "Add time manually" dialog end to end
- Surface: timer · Category: broken · Severity: high
- KanbanFlow: Task field, Date + calendar icon (Sun–Sat grid, selected day blue), From/To time fields, auto-computed Duration, "Add comment"/"Add labels" toggles, green Add (v3 S5, v7 S7); future-time error "You can not enter a time in the future" + OK with dialog state preserved (v3 S7).
- ChipFlow: the `mt-*` dialog (templates/board.html:78-109) has no working JS. `ManualTime` targets `manual-time-*`/`edit-entry-*` ids that exist nowhere (static/app.js:1358-1490); `submit()` posts to `/api/time-entries` (no such route — the server endpoint is `POST /api/time/manual`, src/routes.rs:1594) with the wrong body shape; `ManualTime.open` crashes on null; the task datalist is never populated; From/To→Duration never computed; calendar button/popup and comment toggle unwired; the future-time overlay is never shown (error path targets a nonexistent `#future-time-overlay`).

### KF-004 — Rewire the Edit time-entry dialog end to end
- Surface: timer · Category: broken · Severity: high
- KanbanFlow: "Edit Pomodoro entry" dialog — Date with green ✓ validation, Task autocomplete, From/To with seconds, Duration, "+ Add comment"/"+ Add labels", green Update → "Updating…" state; date edits re-bucket the entry (v3 S8; frame review).
- ChipFlow: the `ee-*` dialog (templates/board.html:111-141) has no working JS. `EditEntry` targets `edit-entry-*` ids that exist nowhere (static/app.js:1495-1610); `submit()` PATCHes `/api/time-entries/{id}` (no such route or method — the server is `PUT /api/time/entries/{id}`, src/routes.rs:1672) and `remove()` DELETEs the same nonexistent path. The delegation listens for `[data-edit-entry]` (app.js:1616-1629) but time_entries.html renders `data-entry-id`, so clicking Edit never reaches the opener.

### KF-005 — Populate the "Why did you stop?" menu
- Surface: timer · Category: broken · Severity: high
- KanbanFlow: dark menu, header "Why did you stop?" + 17 items verbatim in order: Boss interrupted, Colleague interrupted, Context switch, Dog, Email, Family, Finished with no new task, Food Delivery, Meeting, Other, Phone call, Restroom, Sleep, Web browsing, Workchat, "Add new reason…", "Task done" (v4 S2, zoom-verified).
- ChipFlow: nothing ever populates `#why-stop-reasons` (zero references in static/app.js), so the menu opens empty. `beginWhy` then throws on `document.getElementById('why-task-name')` / `#why-elapsed`, neither of which exists in templates/board.html:65-75.

### KF-006 — Restore the break flow **[VERIFIED 2026-09-29]** — real-Chromium run vs fresh DB: pomodoro forced to 00:00 shows single green Take break (rgb(22,163,74)); clicking it starts a short-break session ('SHORT BREAK 4:59 … Stop Switch task'); dead Pause/Resume branch removed — running session now shows only red Stop (rgb(220,38,38)) + Switch task, matching KanbanFlow (no /api/timer/pause|resume exist).
- Surface: timer · Category: broken · Severity: high
- KanbanFlow: at 00:00 the panel shows "00:00" with a single green "Take break" button next to the clock (v8 S1; v8-e001 verified).
- ChipFlow: `#timer-popup-breaks` ("Take a break:" + Short/Long break buttons, templates/board.html:50-54) is never unhidden by any JS. `finishSession` posts the invalid stop body (see KF-002) and plays the chime regardless, so a pomodoro that runs to zero is never logged as completed. No "long break every 4th break" logic exists client- or server-side.

### KF-007 — Show the "Session discarded" toast for stops under 20s
- Surface: timer · Category: broken · Severity: high
- KanbanFlow: stopping before 20s shows a dark toast verbatim "Session discarded" / "Session lasted less than 20 seconds"; the entry is removed and no reason menu appears (v9 S3; v9-e003 verified).
- ChipFlow: the server discards <20s sessions and returns `TimerStopResult{discarded, minutes, completed}` (src/routes.rs:3183-3197), but the client ignores the `discarded` flag — `stopClicked` always opens the why menu regardless of elapsed time, and nothing renders the toast text (no "discard" reference in static/app.js).

### KF-008 — Stopwatch panel: "Stopwatch" title, "Session time" label, count up from 00:00
- Surface: timer · Category: divergent · Severity: medium
- KanbanFlow: stopwatch panel header is "Stopwatch", the label reads "Session time", the clock counts up from "00:00", red ■ Stop, no progress fill (v9 S3; v9-e003 verified).
- ChipFlow: `#timer-popup-title` and `#timer-popup-label` are never updated — the panel always reads "Pomodoro" / "Time until break" (templates/board.html:36-39), and the clock never counts up.

### KF-009 — Rename the footer first tab to the other mode **[VERIFIED 2026-09-29]** — real-Chromium run: `#timer-foot-mode` span reads 'Stopwatch' in pomodoro mode and 'Pomodoro' in stopwatch mode (setModeTab renames it to the other mode, static/app.js:1250-1258).
- Surface: timer · Category: divergent · Severity: medium
- KanbanFlow: the bottom-nav first tab names the other mode: "Pomodoro" in stopwatch mode, "Stopwatch" in pomodoro mode (v4-00002, v4-00037; frame review).
- ChipFlow: `#timer-foot-mode` always shows "Stopwatch" (templates/board.html:58); `switchModeTab` only toggles nonexistent `.timer-modes button`s (static/app.js:1143-1150), so clicking changes nothing.

### KF-010 — Wire the popup "Change task" button; "Select open task" state
- Surface: timer · Category: broken · Severity: medium
- KanbanFlow: task-row link reads "Change task" normally, "Select open task" when a different task's modal is open (v1-02073, zoom-verified; frame review).
- ChipFlow: `#timer-popup-task-btn` has no click handler (templates/board.html:48); the only `changeTask()` posts to `/api/timer/change-task`, which has no route (only `/api/timer/retarget`, src/routes.rs:3250) — a 404 — and uses a native `window.prompt` with an invented "Are you sure you want to switch tasks mid-Pomodoro?" confirm (static/app.js:1291-1325).

### KF-011 — Render the why-stop items verbatim; "Task done" must log a reason, not complete the task
- Surface: timer · Category: divergent · Severity: medium
- KanbanFlow: the 15 defaults, then "Add new reason…" (ellipsis), then "Task done" as the final menu item — all plain clickable items (v4 S2).
- ChipFlow: "Task done" is a separate full-width button below the menu (templates/board.html:74) rather than the last item; the add-reason row is an input + Add button instead of the "Add new reason…" menu item. Worse, `whyTaskDone()` stops with reason "completed" and then moves the task to the Done column (static/app.js:1281-1289) — KanbanFlow's "Task done" is just a stop reason.

### KF-012 — Render stopped sessions as red "Stopped Pomodoro with reason 'X'"
- Surface: timer · Category: divergent · Severity: medium
- KanbanFlow: stopped entries show a red status line "Stopped Pomodoro with reason 'Other'" (v1 S5/S7, v4 S4; frame review).
- ChipFlow: stopped entries show "⚠ interrupted" with the reason only as a `title` tooltip (templates/time_entries.html:9).

### KF-013 — Add the task-modal Time-spent hover menu ("Add time entry" / "Open time log")
- Surface: timer · Category: missing · Severity: medium
- KanbanFlow: hovering "Time spent" in the task modal shows a hover menu with "Add time entry" and "Open time log" (v3-01449; frame review).
- ChipFlow: the modal stats row is static text with no hover menu (templates/modal.html:69-74); the Timer submenu's "Time log" item merely scrolls to the modal's log section (static/app.js:663-665).

### KF-014 — Match the task-modal counters and drop the extra Interruptions stat
- Surface: timer · Category: divergent · Severity: medium
- KanbanFlow: the modal shows "🍅 1 Pomodoro" and "Time spent: 17m" (v3 S3, v8-e004; frame review) — no interruptions counter.
- ChipFlow: the modal shows "🍅 N Pomodori", "⏱ Nm spent", and an extra "⚠ N interruptions" stat with no KanbanFlow counterpart (templates/modal.html:69-74).

### KF-015 — Use the "Pomodoro" / "Stopped with reason 'X'" tooltip on P badges
- Surface: timer · Category: divergent · Severity: low
- KanbanFlow: per-task P-badge tooltips read "Pomodoro", or "Stopped with reason 'Other'" for stopped sessions (v3-01594; frame review).
- ChipFlow: every P badge shows the tooltip "Pomodori" regardless of stopped state (src/routes.rs:669-672, 1401-1404).

### KF-016 — Use the "Manually added" tooltip on M badges
- Surface: timer · Category: divergent · Severity: low
- KanbanFlow: the per-task M badge tooltip reads "Manually added" (v3-01594; frame review).
- ChipFlow: the badge tooltip is "Manually added time" (src/routes.rs:669-672).

### KF-017 — Title the edit dialog "Edit Pomodoro entry"
- Surface: timer · Category: divergent · Severity: low
- KanbanFlow: the dialog title is "Edit Pomodoro entry" (v3 S8).
- ChipFlow: the dialog title is "Edit time entry" (templates/board.html:113).

### KF-018 — Add the × close button to the "Add time manually" dialog
- Surface: timer · Category: missing · Severity: low
- KanbanFlow: the dialog title has an × close (v3 S5, v7 S7).
- ChipFlow: no close button — only Cancel/Add in the footer (templates/board.html:78-109).

### KF-019 — Keep second-level precision for sub-minute durations ("31s", not a 1m floor)
- Surface: timer · Category: divergent · Severity: low
- KanbanFlow: a 31s session renders "31s" in the time log (v3 S4, v1 S5).
- ChipFlow: `log_timer_session` stores `minutes = (elapsed / 60).max(1)` (src/routes.rs:3202-3204), so a 31s session is logged and displayed as 1m — precision lost at write time.

### KF-020 — Update the tab title with the live timer
- Surface: timer · Category: missing · Severity: low
- KanbanFlow: the browser tab title counts down ("00:00 General - Kanba…", v8-e001) / counts up for stopwatch (frame review).
- ChipFlow: `document.title` is never updated by TimerUI.

### KF-021 — Fix the Add-reason row: "Add new reason…" placeholder and a working Add button
- Surface: timer · Category: divergent · Severity: low
- KanbanFlow: the menu's 16th item is "Add new reason…" (with ellipsis, v4 S2).
- ChipFlow: the input placeholder is "Add new reason" (no ellipsis, templates/board.html:70), and the Add button calls `addWhyReason()` with no argument, so it posts `reason: undefined` instead of reading the input value (static/app.js:1277-1279).

## Header (KF-022–KF-033)

### KF-022 — Add the KanbanFlow brand logo to the header
- Surface: header · Category: missing · Severity: medium
- KanbanFlow: KanbanFlow logo centered in the dark top bar (v1-e001; board screenshot).
- ChipFlow: no brand mark anywhere in the board header; the only "ChipFlow" text is on the settings page (templates/settings.html:5).

### KF-023 — Add board tabs with active-board tab and add-board "+" to the top bar
- Surface: header · Category: missing · Severity: medium
- KanbanFlow: dark bar shows "Boards" tab plus the active board tab ("General" with CS chip and "+" add-board icon) left of the centered logo (v1-e001).
- ChipFlow: a single "☰ Boards" link and no board tabs or add-board affordance in the header (templates/board.html:7-10).

### KF-024 — Move the timer pill to the board-bar right, before the filter icon
- Surface: header · Category: divergent · Severity: medium
- KanbanFlow: pill sits in the light board bar at the right, immediately left of the filter funnel icon (v1-e001; v9-e002; board screenshot).
- ChipFlow: pill sits in the dark top bar, left of the username text (templates/board.html:14-25).

### KF-025 — Add the ▾ dropdown chevron to the timer pill
- Surface: header · Category: divergent · Severity: low
- KanbanFlow: pill always shows a ▾ chevron after the time in every state (v1-e001 idle; v9-e002 running stopwatch; v1-e023 running pomodoro).
- ChipFlow: `#timer-pill` markup contains only the status dot and time spans — no chevron element (templates/board.html:14-17).

### KF-026 — Show a green play triangle on the idle pill, not a red stop square
- Surface: header · Category: divergent · Severity: medium
- KanbanFlow: idle pill icon is a green ▶ play triangle (v1-e001; v3-e022).
- ChipFlow: `.timer-pill-dot` is a static `#e57373` red rounded square that never changes color or shape (static/style.css:1028-1033) — it reads as a stop icon even when idle.

### KF-027 — Give the running pill a visible running state **[VERIFIED 2026-09-29]** — real-Chromium run: idle pill '▶ 25:00 ▾' on rgb(31,41,55); running pill '■ 24:55 ▾' on rgb(63,29,29) with `running` class — visibly distinct.
- Surface: header · Category: broken · Severity: high
- KanbanFlow: running pomodoro pill shows a red ■ stop icon plus countdown digits on the dark pill (v1-e023; frame review §(e) notes red digits and a likely subtle red fill); running stopwatch shows red ■ + counting-up "00:02" + ▾ (v9-e002).
- ChipFlow: `renderPill()` toggles a `running` class (static/app.js:953) but no `.timer-pill.running` CSS rule exists — the pill looks identical whether idle or running.

### KF-028 — Render the running pill as icon + countdown, not "Stop (24:56)"
- Surface: header · Category: divergent · Severity: medium
- KanbanFlow: running pill shows the stop icon and the time only (v1-e023; v9-e002).
- ChipFlow: running label is the literal string "Stop (24:56)" / paused "Resume (24:56)" (static/app.js:940-946).

### KF-029 — Render the user identity as an initials avatar circle, not plain username text
- Surface: header · Category: divergent · Severity: low
- KanbanFlow: identity is a "CS" initials avatar circle in the dark top bar (and board bar); no username text is shown (v1-e001 header crops).
- ChipFlow: `<span class="user">{{ username }}</span>` renders the raw username as inert text (templates/board.html:21); no avatar circle, no account menu. (KanbanFlow's avatar/account menu contents were never captured open — needs a live capture.)

### KF-030 — Add the notifications bell icon to the top bar
- Surface: header · Category: missing · Severity: low
- KanbanFlow: bell icon at the right of the dark top bar, left of the help icon (v1-e001 header crop).
- ChipFlow: no notifications bell anywhere in the header (templates/board.html:5-28). (Bell panel contents unverified — needs a live capture.)

### KF-031 — Add the help "?" icon to the top bar
- Surface: header · Category: missing · Severity: low
- KanbanFlow: "?" in a circle at the right of the dark top bar, between bell and avatar (v1-e001 header crop).
- ChipFlow: no help icon in the header (templates/board.html:5-28). (Help destination unverified — needs a live capture.)

### KF-032 — Add the edit-layout pencil icon to the board bar
- Surface: header · Category: missing · Severity: medium
- KanbanFlow: pencil icon in the board bar right, between filter and Menu; opens the layout-edit view ("Layout: General" with Add column / Add swimlane buttons — a view in which the timer pill is absent, frame review §(e)) (board screenshot; v1-e001 crop; v1 S10).
- ChipFlow: no layout-edit view and no pencil control; column/swimlane adding lives in a separate board toolbar (templates/board.html:30-40).

### KF-033 — Remove the settings gear and Log out button from the board header
- Surface: header · Category: divergent · Severity: low
- KanbanFlow: the board header shows no settings gear and no logout button; settings/logout live behind other surfaces (v1-e001; board screenshot).
- ChipFlow: exposes a ⚙ gear link to /settings and a "Log out" submit button directly in the board header (templates/board.html:22-26).

## Columns (KF-034–KF-049)

### KF-034 — Show WIP-limit violation in darkred with a red warning line, not amber
- Surface: columns · Category: divergent · Severity: medium
- KanbanFlow: exceeded limit shows "3 / 2" in darkred + red warning line; tokens `--board-headerWarningTextColor:darkred`, `--board-warningLineColor:#ff8080`; header background unchanged (fidelity-status Discovered behaviors).
- ChipFlow: amber theme instead — header bg #fffbeb, border #f59e0b, count #b45309, line #ef4444 (style.css:171-184); the two tokens are absent from style.css entirely.

### KF-035 — Warn on WIP limit only when the limit is exceeded, not when reached
- Surface: columns · Category: divergent · Severity: medium
- KanbanFlow: warning is a *violation* — observed on "3 / 2", count div titled "*LIMIT EXCEEDED*" (fidelity-status); "exceeded" implies count > limit. (Caveat: no video frame captured a count == limit state, so the at-limit rendering is inferred, not frame-verified.)
- ChipFlow: warns when count >= limit — "2 / 2" already shows the warning state (routes.rs:733-736 `count as i64 >= limit`; app.js:117 `count >= wip`).

### KF-036 — Render the header count inline with the name, not on a second line
- Surface: columns · Category: divergent · Severity: medium
- KanbanFlow: name + gray count on one line, e.g. "In progress 0 / 3" (hi-res board screenshot header crops).
- ChipFlow: `.columnHeader-count{display:block}` on its own line below the name (style.css:142-147; board.html:170-172).

### KF-037 — Make the column add-task button big and green
- Surface: columns · Category: divergent · Severity: medium
- KanbanFlow: large bright-green "+" button in each column header (hi-res screenshot; v3-S1 "green '+' add button in the column header").
- ChipFlow: small muted-gray "+" (style.css:150-165, `color:var(--muted)`; board.html:171).

### KF-038 — Hide the ⋮ button on board column headers; open the menu on right-click
- Surface: columns · Category: divergent · Severity: low
- Status: trigger FIXED (ChipFlow's ⋮ is display:none; menu opens on right-click like KanbanFlow — confirmed 2026-09-30 by visual verifier). CORRECTION 2026-09-30: the old note "(Items/order already match; this is about the trigger)" and "the 6-item menu" were wrong — the LIVE golden-master board's column menu has 3 items (Edit / Collapse / Show details), not 6. Menu contents split into KF-223.
- KanbanFlow: board column headers show no ⋮ affordance — the menu opens on right-click (v1-S12; live board 2026-09-30).
- ChipFlow (before fix): an always-visible ⋮ button on every header, at the far right where KanbanFlow puts the count (board.html:173; style.css:151,165).

### KF-039 — Match the collapsed column strip: count color, vertical name, expand behavior
- Surface: columns · Category: divergent · Severity: medium
- CORRECTION 2026-09-30 (verified live on the golden-master board): the count badge is RED only when the WIP limit is exceeded ("5", tooltip "Task count: 5 / Click to expand"); otherwise it is black/dark (To Do shows "5" in dark). The old "always red" premise was wrong.
- KanbanFlow: collapsed column is a narrow white vertical strip (~20–24px), full task-area height; count badge at top; column name vertical, rotated 90° CCW (reads bottom-to-top), UPPERCASE via CSS ("IN PROGRESS" from source "In Progress"); click anywhere on the strip (badge, name, header) expands it. Collapsed strips also carry an overdue indicator: To Do shows "1 overdue task" + warning line beneath, tooltip "Overdue tasks: 1 / Total tasks: 5 / Click to expand".
- ChipFlow: no column collapse exists at all (no reachable fold UI/API/JS).
- Evidence: column-behavior browser check 2026-09-30 (collapsed-strip screenshots).

### KF-040 — Uppercase the collapsed column name
- Surface: columns · Category: divergent · Severity: low
- KanbanFlow: collapsed strip shows the name vertical (90° CCW, reads bottom-to-top) and UPPERCASE via CSS ("IN PROGRESS"; "PERSONAL TO-DO", "BACKLOG") (live golden-master board, 2026-09-30).
- ChipFlow: no column collapse exists at all. (Subsumed by the missing collapse feature — see KF-039/KF-223.)

### KF-041 — Insert "Add to left/right" adjacent to the column, not at the board ends
- Surface: columns · Category: broken · Severity: medium
- KanbanFlow: Add to left / Add to right insert a column immediately left/right of the column whose menu was opened (menu semantics, v1-S12).
- ChipFlow: both actions just preselect the dialog's Position as 'beginning'/'end' — the new column lands at the board's beginning or end regardless of which column's menu was used (app.js:298-299; the Position select only offers "At the end"/"At the beginning", board.html:296-301).

### KF-042 — Load the Edit dialog with the column's saved values instead of defaults
- Surface: columns · Category: broken · Severity: medium
- KanbanFlow: the dialog reflects the column's actual saved settings.
- ChipFlow: openEditColumnDialog hard-resets sorting to 'none' and unchecks column-sum/group-by-date while checking all show/hide boxes (app.js:422-431, with a comment admitting it "edits them from defaults"); saving then silently discards the stored config_json values — editing a column's name wipes its grouping/sorting/display settings.

### KF-043 — Rebuild "Task properties to display on board" as per-property dropdowns
- Surface: columns · Category: divergent · Severity: high
- KanbanFlow: six per-property dropdowns — Description, Labels, Subtasks, Due dates ("Show active due in 7 days"), Created date, Added to column (v1-e019 enlarged frame).
- ChipFlow: a "Show / Hide" fieldset with three CHECKBOXES — Description, Task count, WIP limit (board.html:339-346). Different control type, five of six KanbanFlow properties missing, two non-KanbanFlow toggles added.

### KF-044 — Restore the "Archiving" checkbox with its end-state note
- Surface: columns · Category: divergent · Severity: medium
- KanbanFlow: "Archiving" checkbox labeled "Group tasks by the date they were added to the column" plus a blue info note: "Archiving is ONLY recommended for columns that represent an end state for tasks, like the Done column. Loads the 20 most recent tasks from the start." (v1-e019).
- ChipFlow: checkbox relabeled "Group tasks by date"; the note is missing entirely (board.html:336-338).

### KF-045 — Make Column sum a dropdown, not a checkbox
- Surface: columns · Category: divergent · Severity: low
- KanbanFlow: Column sum is a dropdown showing "None" (v1-e019). (Its option list wasn't captured; only the control type is verified.)
- ChipFlow: a checkbox (board.html:337).

### KF-046 — Label the Edit dialog button "Update column" in green
- Surface: columns · Category: divergent · Severity: low
- KanbanFlow: Cancel + green "Update column" (v1-e019).
- ChipFlow: Cancel + blue "Save" (board.html:349-352).

### KF-047 — Replace window.confirm() with a styled delete-confirmation dialog
- Surface: columns · Category: divergent · Severity: low
- KanbanFlow: deleting a column goes through an in-page confirmation dialog (fidelity-status: "Delete via confirmation dialog").
- ChipFlow: native `window.confirm()` (app.js:376-386).

### KF-048 — Render the dragged column blank/empty mid-drag
- Surface: columns · Category: divergent · Severity: low
- KanbanFlow: the column being dragged renders blank/empty while the cursor holds it, with a gap at the drop position (v1-e017).
- ChipFlow: no dragClass wired — SortableJS drags a full-opacity clone of the column; the defined `.drag-ghost{opacity:.4}` rule (style.css:274) is never applied (app.js:133-161).

### KF-049 — Use a flat light-gray column header, not a white card
- Surface: columns · Category: divergent · Severity: low
- KanbanFlow: headers are flat light-gray table cells (hi-res screenshot).
- ChipFlow: white rounded cards with border (`background:#fff; border:1px solid #e2e5ea; border-radius:8px`, style.css:120-130).

## Task cards (KF-050–KF-055)

### KF-050 — Add the card context menu (right-click; 8 items verbatim)
- Surface: cards · Category: missing · Severity: high
- KanbanFlow: right-clicking a card opens a menu with verbatim items in order: **Timer** ▶ (submenu: **Start timer**, **Select in timer**), **Move** ▶, **Edit grouping date**, **Color** ▶, **Assign members**, **Copy here**, **Task URL**, **Delete** (v1-S2, v1-e003; Timer submenu re-verified in frame-review §(e)7). (Move ▶ and Color ▶ submenu contents were never captured — only their presence is verified.)
- ChipFlow: no card context menu exists at all. Left-click opens the task modal directly (app.js:1784-1791); right-click on a card yields only the browser default menu. "Task URL"/"Copy" exist only inside the task modal's More menu (modal.html:40-41), not on the board surface.

### KF-051 — Use a uniform colored border on cards, not a 4px left accent bar
- Surface: cards · Category: divergent · Severity: medium
- KanbanFlow: cards carry a uniform ~2px border in the task's border color all around, rounded corners (verified on zoomed crops of the hi-res board screenshot).
- ChipFlow: `.task-card { border: 1px solid; border-left-width: 4px; }` (static/style.css:329-336) — a 1px border plus a thick left accent bar, a visibly different card anatomy.

### KF-052 — Show a dashed outline on the timer-selected card
- Surface: cards · Category: missing · Severity: medium
- KanbanFlow: the task selected in the timer renders with a dashed outline on the board, even while idle (v1-e001: dashed-outline "Retro categorization" with timer at 25:00; v8 persistent context).
- ChipFlow: no visual marker at all. `.card-timer-running` (style.css:359-362) and `.card-live` (style.css:355-357) are dead styles — never applied by any JS or template; `<span class="card-live" hidden>` (task_card.html:15) is always hidden; the `.card-timer-indicator` loop (app.js:915) queries elements that don't exist in the markup.

### KF-053 — Add card metadata/footer (dates, labels, subtasks, members)
- Surface: cards · Category: missing · Severity: medium
- KanbanFlow: the Edit column dialog's "Task properties to display on board" exposes Description, Labels, Subtasks, Due dates, Created date, Added to column (v1-S13); cards also carry member assignments ("Assign members" in the card context menu, v1-S2).
- ChipFlow: task_card.html renders title + time/pomodori/done indicators only. No description, labels, subtask counts, due dates, created/added dates, and no member concept anywhere in the data model (models.rs, routes.rs have no assignee/member fields).

### KF-054 — Use a clock-outline icon for the card time badge, not ⏱ emoji
- Surface: cards · Category: divergent · Severity: low
- KanbanFlow: badge row shows a clock-outline SVG icon + gray time text ("🕐 14m", "🕐 1h 9m" in the hi-res screenshot zoom).
- ChipFlow: `&#9201;` stopwatch emoji before the time (task_card.html:8).

### KF-055 — Match the tomato badge format ("🍅 1m")
- Surface: cards · Category: divergent · Severity: low
- KanbanFlow: tomato badge reads "🍅 1m" — space + "m" suffix (frame review, v1-00102 / v1-00391 / v1-02173; low-res frames, count-vs-minutes semantics unverified).
- ChipFlow: renders "🍅{N}" with no space and no "m" suffix, e.g. "🍅1", titled "N pomodori completed" (task_card.html:11-13).

## Task modal (KF-056–KF-073)

### KF-056 — Restore the vertical right-edge action icon column
- Surface: task modal · Category: missing · Severity: high
- KanbanFlow: vertical column pinned to the modal's right edge — circular icon buttons with label pills, top→bottom: Add (+), Move (→), Timer (stopwatch), Reports (stacked-layers), More (•••), Delete (trash). Order/glyphs verified in zoomed frames of v7-e003, v7-e020, v3-e003 (v7 S1–S2).
- ChipFlow: a horizontal row of text buttons ("Add Move Timer Reports More Delete") under the header instead of the icon column (templates/modal.html:14-50; .task-modal-actions in static/style.css:556).

### KF-057 — Add the "Work To-do · Created: Jun 23" subtitle under the modal title
- Surface: task modal · Category: missing · Severity: medium
- KanbanFlow: bold title + subtitle "Work To-do · Created: Jun 23" + × close (v7 S1; v7-e015, v8-e003).
- ChipFlow: header is just an inline-editable name input + ×; no subtitle row anywhere (templates/modal.html:4-8).

### KF-058 — Implement the Subtasks section with the "Add subtask..." row
- Surface: task modal · Category: missing · Severity: high
- KanbanFlow: "Subtasks" section header below the Color/Time-spent rows with an "Add subtask..." input row (v7 S1/S8; v7-e015; typing visible in v7-e010).
- ChipFlow: no subtasks UI anywhere; zero subtask code in templates/modal.html or static/app.js.

### KF-059 — Add the 8 missing Add-menu items
- Surface: task modal · Category: missing · Severity: high
- KanbanFlow: 10 items verbatim in order, each with a small icon: Description, Member, Label, Subtask, Due date, Time estimate, Manual time entry, Comment, Attachment, Relation (v7 S2; v7-e003 verified in zoom).
- ChipFlow: only "Manual time entry" and "Add time estimate", no item icons (templates/modal.html:17-22).

### KF-060 — Build the Members sub-dialog (Search + gear + member rows)
- Surface: task modal · Category: missing · Severity: high
- KanbanFlow: "Members" title, gear icon at top right, "Search..." input, rows like "● Chip Senkbeil" (v7 S4; v7-e005; Members row also appears in the modal body, v8-e002).
- ChipFlow: no members feature anywhere in the modal or app.js; no Members row in the body. (Members-row show/hide rules unverified — likely hidden when no members assigned.)

### KF-061 — Build the Labels sub-dialog ("Add labels...", "No recently used labels exist", Save)
- Surface: task modal · Category: missing · Severity: medium
- KanbanFlow: "Labels" title, "Add labels..." input, body text "No recently used labels exist", green "Save" button (v7 S5; v7-e006).
- ChipFlow: no labels feature anywhere in the modal or app.js.

### KF-062 — Build the Add due date dialog (Date/Time/Repeat fields, calendar, column list, green Add)
- Surface: task modal · Category: missing · Severity: medium
- KanbanFlow: "Add due date" title; Date "2026-09-28", Time "05:00 PM"; calendar grid ("September 2028", Sun–Sat headers); column list (Personal To-do / Do today / In progress / Done / Backlog, Backlog highlighted); "Repeat (e.g. every week)" field; green "Add" (v7 S6; v7-e007).
- ChipFlow: no due-date feature anywhere in the modal or app.js.

### KF-063 — Make Description an Add-menu item instead of an always-visible textarea
- Surface: task modal · Category: divergent · Severity: medium
- KanbanFlow: Description is an Add-menu item; the default modal body shows only Color / Time-spent rows + Subtasks (v7 S1/S2).
- ChipFlow: Description is a permanently visible labeled textarea in the modal body (templates/modal.html:60-62), and there is no Add → Description item.

### KF-064 — Add Comment and Attachment Add-menu items
- Surface: task modal · Category: missing · Severity: medium
- KanbanFlow: Comment and Attachment are Add-menu items (v7 S2).
- ChipFlow: neither exists; a comment field only appears inside the manual-time dialog. (Sub-dialog contents uncaptured — only absence is filed.)

### KF-065 — Add the Relation Add-menu item
- Surface: task modal · Category: missing · Severity: low
- KanbanFlow: Relation is the last Add-menu item (v7 S2).
- ChipFlow: no relation feature anywhere.

### KF-066 — Match "Time estimate" wording and Add-menu order
- Surface: task modal · Category: divergent · Severity: low
- KanbanFlow: item reads "Time estimate" and precedes "Manual time entry" (v7-e003).
- ChipFlow: item reads "Add time estimate" and follows "Manual time entry" (templates/modal.html:17-22).

### KF-067 — Match the Time spent row format ("● 2 Pomodori" / "2m")
- Surface: task modal · Category: divergent · Severity: medium
- KanbanFlow: labeled rows "Color" and "Time spent"; Time spent reads "● 2 Pomodori" and "2m" (v7 S1; v7-e002, v7-e015, v7-e021).
- ChipFlow: an icon stats block — "🍅 N Pomodori", "⏱ X spent", "⚠ N interruptions", "Created ..." (templates/modal.html:64-76); no "Time spent" labeled row in KanbanFlow's format.

### KF-068 — Show the "You are selecting this task" hover tooltip on Time spent
- Surface: task modal · Category: missing · Severity: low
- KanbanFlow: hovering the "Time spent" label/row shows a small tooltip reading "You are selecting this task" (v7 S13; v7-e021).
- ChipFlow: stat spans carry native title attributes ("Pomodori completed", "Time spent", "Interruptions") instead (templates/modal.html:65-68).

### KF-069 — Make the Move task dialog's Move button green and add the × close
- Surface: task modal · Category: divergent · Severity: low
- KanbanFlow: "Move task" title + × close; Board dropdown ("General", blue selected); "Column" label + dropdown ("Work To-do"); green "Move" button (v7 S9; v7-e012).
- ChipFlow: no × (Cancel button instead); Move is blue btn-primary, not green (templates/board.html:356-380).

### KF-070 — Add the Board dropdown to the Move task dialog
- Surface: task modal · Category: missing · Severity: low
- KanbanFlow: Move task dialog has a Board dropdown with "General" selected (v7 S9; v7-e012).
- ChipFlow: dialog lists only the current board's columns; no board selection (static/app.js:717-729).

### KF-071 — Make "Watch" actually do something instead of a "not supported" toast
- Surface: task modal · Category: broken · Severity: medium
- KanbanFlow: More menu = Watch, Task URL, Copy, Keyboard shortcuts — Watch is a working feature (fidelity-status Discovered behaviors; v7 More-menu surfaces).
- ChipFlow: the item exists but modalAction("watch") only shows toast('Task watching is not supported yet.') (static/app.js:676-677).

### KF-072 — Fix Add time estimate dialog nits (× close, clock placeholder, green Add)
- Surface: task modal · Category: divergent · Severity: low
- KanbanFlow: "Add time estimate" title with ×; text input with clock placeholder; green "Add" (v7 S3; v7-e004).
- ChipFlow: Cancel button instead of ×, plain placeholder "0h", blue btn-primary Add instead of green (templates/board.html:382-396).

### KF-073 — Match the Color row presentation (single color dot under the label)
- Surface: task modal · Category: divergent · Severity: low
- KanbanFlow: "Color" label with a single color dot under it in the body (v7-e015).
- ChipFlow: an always-visible row of selectable color dots (templates/modal.html:55-56; buildModalColorPicker in static/app.js:599-618).

## Settings / colors / templates / board list (KF-074–KF-089)

### KF-074 — Ship the Timer settings modal with four tabs instead of linking to /settings
- Surface: settings · Category: missing · Severity: high
- KanbanFlow: the Pomodoro panel's Settings icon opens a "Timer settings" modal with a left tab list — General (selected), Interruptions, Break activities, Sounds — per v7-e023.
- ChipFlow: the timer popup footer Settings button and the topbar gear both link to the full-page `/settings` form; there is no timer-settings modal anywhere (templates/board.html:61 links the footer Settings to `/settings`).

### KF-075 — Rebuild the General tab with labeled dropdowns and a Picture-in-Picture toggle
- Surface: settings · Category: divergent · Severity: medium
- KanbanFlow: General tab shows "Pomodoro durations" with dropdowns: "Work time" 25 minutes, "Short break time" 5 minutes, "Long break interval" "Every 4th break", "Long break time", plus a "Picture-in-Picture" toggle switched ON (v7-e023).
- ChipFlow: /settings has bare number inputs ("Pomodoro length (minutes)", "Short break (minutes)", "Long break (minutes)", "Long break every N pomodori") and no PiP control (templates/settings.html:14–27).

### KF-076 — Rebuild the Interruptions tab with drag-reorder, inline rename, and a green Add reason
- Surface: settings · Category: divergent · Severity: medium
- KanbanFlow: Interruptions tab lists the 15 default reasons with drag-reorder handles, inline rename via click-to-yellow-highlighted input, and a green "Add reason" button (frame-review §a; reason order verified verbatim, v4 "Why did you stop?" menu).
- ChipFlow: /settings has a plain one-reason-per-line `<textarea>`; reorder is cut/paste, no inline rename, no add button (templates/settings.html:44–46).

### KF-077 — Drop "Task done" from the seeded interruption reasons
- Surface: settings · Category: divergent · Severity: low
- KanbanFlow: exactly 15 default reasons, Boss interrupted … Workchat; "Task done" was Chip's own custom-added reason seen live in logs, not a default (frame-review §a; v4 menu shows "Task done" after "Add new reason…").
- ChipFlow: `Settings::default()` seeds 16 reasons including "Task done" (src/models.rs:260–277).

### KF-078 — Add the Break activities tab and its Add-break-activity dialog
- Surface: settings · Category: missing · Severity: medium
- KanbanFlow: "Break activities" tab with info text "Configure activities for your Pomodoro breaks to keep body and mind fresh.", a light-blue Examples box (Meditate, Take a short walk, Switch sitting/standing at your desk), a green "Add activity" button, and an "Add break activity" dialog with Name, Description, Daily goal = 1, Daily limit = "No limit" (v7-e024–e026).
- ChipFlow: no break-activities UI, model, or routes exist anywhere.

### KF-079 — Add the Sounds tab: ticking mode, 9 alarm sounds, volumes, and toggles
- Surface: settings · Category: missing · Severity: high
- KanbanFlow: Sounds tab has "Ticking mode" dropdown (Always / Timer start / Never), 9 alarm sounds (Bell ✓, Chime, Beeps, Blip, Glass, Microwave, Egg timer, Grandpa clock, Melodic), "Alarm volume" 70%, "Points volume" 70%, Sounds ON, PiP ON (frame-review §113–115).
- ChipFlow: no sounds UI exists; /settings only offers an invented "Play a ding when a timer ends" checkbox with no sound choice, ticking mode, or volume sliders (templates/settings.html:31–40).

### KF-080 — Restore the Board Settings shell with left nav around the Colors page
- Surface: settings · Category: missing · Severity: medium
- KanbanFlow: "Board settings: General" page with a left nav (General, Layout, Colors, Task settings, Advanced, Custom fields, Custom roles, API & Webhooks, Add task from email) and a "← View board" back link (a8 color screenshot).
- ChipFlow: no board-settings page or nav exists; the colors admin is a standalone page with only a "Back to board" link (templates/board_colors.html:8–12). (Sub-page contents uncaptured — only the shell/nav gap is filed.)

### KF-081 — Split the color list into Enabled colors and Disabled colors sections
- Surface: settings · Category: divergent · Severity: low
- KanbanFlow: two tables — "Enabled colors" (with Edit/Disable per row) and "Disabled colors" (with Enable per row) (a8 screenshot).
- ChipFlow: one combined list; enable/disable is a per-row "Enabled in picker" checkbox (templates/board_colors.html:14–61).

### KF-082 — Reorder colors by drag handles, not ▲▼ buttons
- Surface: settings · Category: divergent · Severity: medium
- KanbanFlow: ⋮⋮ drag handles on each enabled-color row reorder the palette (a8 screenshot).
- ChipFlow: per-row ▲/▼ buttons that move the color one slot and reload the page (templates/board_colors.html:52–56).

### KF-083 — Add "Set default color" and "Copy from board" actions
- Surface: settings · Category: missing · Severity: medium
- KanbanFlow: "Set default color" and "Copy from board" buttons sit above the Enabled colors table, next to the green "Add custom color" button (a8 screenshot).
- ChipFlow: neither action exists; the default is set with a per-row radio button and there is no palette copy between boards (templates/board_colors.html; no matching routes in src/routes.rs).

### KF-084 — Rename colors through an Edit dialog, not always-visible inline inputs
- Surface: settings · Category: divergent · Severity: low
- KanbanFlow: per-row "Edit" button opens an edit dialog that only renames (≤50 chars) and sets the description/legend tooltip; hex is fixed (fidelity-status Discovered behaviors; a8 screenshot).
- ChipFlow: Label and Description are always-visible inline text inputs on every row (templates/board_colors.html:37–45).

### KF-085 — Add the premium upsell box and green "Add custom color" button
- Surface: settings · Category: missing · Severity: low
- KanbanFlow: green "Add custom color" button above the table and a bottom box: "Looking for more or different colors for your tasks? … Upgrade to the premium version to get access to this feature." (a8 screenshot).
- ChipFlow: neither the button nor the upsell text is present (templates/board_colors.html).

### KF-086 — Match the Colors admin intro copy
- Surface: settings · Category: divergent · Severity: low
- KanbanFlow: "The colors are commonly used to indicate the type or priority of a task. Rename them to better represent their meaning. Show more…" (a8 screenshot).
- ChipFlow: "Labels appear in the legend and tooltips; disabled colors are hidden from the task color picker. The default color is pre-selected on new tasks." (templates/board_colors.html:13–14).

### KF-087 — Show the standard color name inside the swatch
- Surface: settings · Category: divergent · Severity: low
- KanbanFlow: each row's swatch shows the standard name text ("Yellow", "Green", "Blue", "Red"…) inside the colored box (a8 screenshot).
- ChipFlow: the swatch is an empty colored span with the value only in a title attribute (templates/board_colors.html:35).

### KF-088 — Add a persistent Boards sidebar with sections, search, and favorites
- Surface: settings · Category: missing · Severity: medium
- KanbanFlow: an always-visible left sidebar — "Boards" header, search box, "Drag to add to Favorites" hint, boards grouped in sections (Personal: Home, Organization & Work, Chats & Forums, Finances, Health, Shopping, Travel, Games & Misc, Education & Learning, News & Articles, Pull Requests) (/tmp/kf-survey/v4/e001.jpg).
- ChipFlow: no sidebar; switching boards requires leaving the board for the flat "Open a board" list on /boards/new.

### KF-089 — Add the Settings → Delete board flow
- Surface: settings · Category: missing · Severity: high
- KanbanFlow: Settings → Delete board → red confirmation page → final "cannot be undone" dialog (fidelity-status Discovered behaviors).
- ChipFlow: board deletion is entirely absent — no DELETE board route or handler, no delete button in any template.

## Reports (KF-090–KF-093)

### KF-090 — Implement the Timer log page — it currently renders empty
- Surface: reports · Category: broken · Severity: high
- KanbanFlow: "Timer log" with All boards / Period ("This + Last week", "This week", "Last week", "This month", "Last month", "Custom (absolute)", "Custom (relative)") / Entry type filters; print/export/reload icons; day groups "Friday, 10 July — 3h 20m — 8 Pomodoros"; green "Successful Pomodoro" vs red "Stopped Pomodoro with reason 'X'"; orange dots for stopped; blue "Add time entry" link per day group; empty state "No entries exist for the given filter" (v1 S5/S7; frame review).
- ChipFlow: /timer/log (templates/timer_log.html) has only a Task dropdown filter — no board/period/entry-type filters, no print/export/reload icons, no day grouping, no green/red entries. Worse, the page is broken: `#timer-log-list` is never populated — no code fetches /api/timer/log (backend exists, src/routes.rs:1357 api_timer_log), and the template calls `TimerLogPage.init()` (timer_log.html:38) which is undefined; only `initTimerLogPage` exists (static/app.js:1718), which no-ops on a nonexistent `timer-log-period` button.

### KF-091 — Implement the Time spent report UI
- Surface: reports · Category: missing · Severity: high
- KanbanFlow: full-page "Time spent" with Filter/Print/Export buttons; filter pane (Period: Last 30 days, User: Chip Senkbeil, Color: Show all, Label: "Add labels…", Reload); Group by: Date (descending); View: Summary | Detailed; day-grouped per-task entries with durations; Bookmarks (v1 S10; frame review).
- ChipFlow: only a "Time spent" tab inside /timer/log with From/To date inputs + Apply + an empty bar chart (templates/timer_log.html:23-31); never populated — nothing fetches /api/timer/time-spent. No User/Color/Label filters, no Summary/Detailed views, no Bookmarks, no per-task detail, no Print/Export.

### KF-092 — Implement the Pomodoro statistics page UI
- Surface: reports · Category: broken · Severity: high
- KanbanFlow: full-page "Pomodoro Statistics"; tabs Pomodoros / Interruptions / Break activities / Highscores; Period presets (Last 7/14/30 days, This week, Last week, This month, Last month, Custom (absolute), Custom (relative)); Group by Day; Board: All boards; Export; Reload; bar chart with weekday tooltips; "No data to display" / "Loading chart..." states; custom-absolute dual-calendar dialog (v1 S16; frame review).
- ChipFlow: /timer/statistics (templates/timer_statistics.html) has a stats-summary div, a "Pomodoros per day" chart div, and an "Interruptions by reason" div — none ever populated. The template calls `TimerStatsPage.init()` (timer_statistics.html:14) which is undefined; only `initTimerStatsPage` exists (static/app.js:1735), and it returns early because the chart div carries no data. No tabs, no Period presets, no Board selector, no Export/Reload, no empty states.

### KF-093 — Add Excel/CSV export to the timer log
- Surface: reports · Category: missing · Severity: medium
- KanbanFlow: export icon opens a menu with "Download Excel file" and "Download CSV file" (v1 S9).
- ChipFlow: no Excel/CSV export exists anywhere — no csv/excel/xlsx references in src/routes.rs, static/app.js, or templates/.

## Cross-surface (KF-094–KF-103)

### KF-094 — Fix T / P / Y keyboard shortcuts and make the dialog text agree with the code **[VERIFIED 2026-09-29]** — real-Chromium run vs fresh DB: T toggles popup; P opens Reports menu (Timer log + Pomodoro statistics); Y opens Add time manually; E opens estimate dialog in task modal; V opens Move dialog; `.` opens More menu; Esc closes dialogs incl. with focus in a field (fixed 2026-09-29: Escape was swallowed by the inField early-return, static/app.js); Ctrl+Enter saves+closes modal; Delete deletes (after confirm).
- Surface: cross-surface (header, timer, reports, task modal) · Category: broken · Severity: high
- KanbanFlow: T = timer menu, P = reports menu, Y = manual time entry, E = time estimate (v7-e018; frame review).
- ChipFlow: T immediately STARTS a pomodoro on the open task instead of opening the timer menu (static/app.js:1679-1682); **P calls `TimerUI.stopClicked()` — it stops a running timer** — instead of opening the Reports menu (static/app.js:1683-1684); Y smooth-scrolls to the first task card instead of opening manual time entry (static/app.js:1676-1678). The shortcuts dialog additionally disagrees with the code on all three (says Y = "Add time entry manually", T = "Open / close the timer popup", P = "Open pomodoro statistics", templates/board.html:402-407); no Enter handler exists though the dialog lists one; "?" opens the dialog but isn't listed (static/app.js:1670-1700).

### KF-095 — Rewrite the shortcuts dialog with KanbanFlow's 9 rows **[VERIFIED 2026-09-29]** — real-Chromium run: `#shortcuts-dialog` contains exactly the nine KanbanFlow rows (V/T/P/./↑↓/Cmd+↑↓/Esc/Cmd+Enter/Delete) with no extras. NOTE: the dialog is compiled into the Rust binary (Askama) — template edits require `cargo build` before browser verification.
- Surface: cross-surface (task modal, reports) · Category: divergent · Severity: high
- KanbanFlow: 9 rows verbatim in order: Open Move dialog — V; Open Timer menu — T; Open Reports menu — P; Open More menu — `.`; Navigate subtask list — ↑ ↓; Move subtask in list — Cmd + ↑ ↓; Close window / discard changes — Esc; Save changes — Cmd + Enter; Delete task — Delete (v7-e018).
- ChipFlow: 6 unrelated rows (Y/T/P/E/Enter/Esc) with different meanings, omitting V, `.`, subtask navigation, Cmd+Enter, Delete entirely (templates/board.html:398-412).

### KF-096 — Add the board Filter: funnel icon + Filter panel
- Surface: cross-surface (header, reports) · Category: missing · Severity: high
- KanbanFlow: filter funnel icon in the board bar right, between the timer pill and the pencil icon; opens a right-side Filter panel — User "Show all", Color dropdown ("Show all" (checked), "1 Pomodoro", "2 Pomodoros", "3 Pomodoros", ">3 Pomodoros"), Date "Show all", Labels "Add labels…", "Remember filter" toggle (off), "Bookmarks (0)", plus a "Timer users" filter for cards with an active timer (v1-e001; v1 S14, v1-e021; frame review).
- ChipFlow: no filter control and no filter panel anywhere — no User/Color/Date/Labels/Timer-users filters, no Remember filter, no Bookmarks (templates/board.html; static/app.js has no filter code).

### KF-097 — Hide the timer pill while the timer popup panel is open
- Surface: cross-surface (header, timer) · Category: divergent · Severity: medium
- KanbanFlow: the pill is absent while the timer popup panel is open/docked and in layout-edit view (frame review §(e); v1-02504–02516; v9-e001).
- ChipFlow: `renderPill` forces `pill.hidden = false` with the comment "The pill is the always-visible timer dropdown control; never leave it hidden" (static/app.js:966-970) — the pill stays visible under the open popup.

### KF-098 — Show the configured duration with green ▶ and ▾ on the idle pill (red ■ + 00:00 in stopwatch mode) **[VERIFIED 2026-09-29]** — real-Chromium run: pomodoro idle pill '▶ 25:00 ▾'; stopwatch idle pill '■ 00:00 ▾' (red square).
- Surface: cross-surface (header, timer) · Category: divergent · Severity: high
- KanbanFlow: idle pill is dark with a green ▶, "25:00", and a ⌄ dropdown (v1-e001; v3-e022); stopwatch idle pill shows a red ■, "00:00", and ⌄ (v9-e003; frame review §(d)).
- ChipFlow: the pill label is always the literal word "Pomodoro" (`pillLabel`, static/app.js:937-939); the dot is a pink rounded square (`border-radius: 3px`, `#e57373`, static/style.css:1028-1033) — no green play triangle, no red square, no dropdown arrow in any state; no chevron element in the markup (templates/board.html:14-17).

### KF-099 — Add the board-bar Menu button with the 15-item Reports menu
- Surface: cross-surface (header, reports) · Category: missing · Severity: high
- KanbanFlow: "≡ Menu" (hamburger + "Menu" text) at the board-bar far right; opens the Reports menu with items verbatim in order: Pomodoro statistics, Time spent, Print, Board history, Burndown, Calendar, Cumulative flow, Cycle & lead time, Due date performance, Monte Carlo forecasting, Task count, Throughput, Time estimate, Time in column, Export (v1-e023; v1 S15).
- ChipFlow: no Menu button and no Reports menu anywhere in the header (templates/board.html:5-28); reports are reachable only via dead full-page routes and the task-modal Reports menu.

### KF-100 — Add History (and Time in column) to the task-modal Reports menu
- Surface: cross-surface (task modal, reports) · Category: missing · Severity: medium
- KanbanFlow: Time log, Print, History, Time in column (v7-e020, corrected — the catalog's "Start" was a mistranscription of "Print").
- ChipFlow: Reports menu has only Time log and Print (templates/modal.html:33-38). Note: the frame review marks Print and Time in column as premium-gated in KanbanFlow, so History is the required gap for free-tier parity; Time in column only if premium parity is in scope.

### KF-101 — Rebuild the per-task Time log as a dedicated in-modal sub-view
- Surface: cross-surface (timer, task modal) · Category: divergent · Severity: medium
- KanbanFlow: Reports → Time log switches the modal to a "Time log" view: "← Retro categorization" back-arrow header, "Time log" title, "+ ADD ENTRY" top right; day-grouped rows ("Today 2m", "Yesterday 40m"); each entry shows avatar + member name + duration + time range with a red trash icon (v7 S10; v7-e014; v3 S3–S4).
- ChipFlow: the action scrolls to an inline `<h3>Time log</h3>` section in the same body; "+ Add entry" sits below the list; entries are a flat newest-first list of minutes/badge/note/date + Edit button with no avatar, member name, day grouping, or trash icon (templates/modal.html:75-77, 84-86; templates/time_entries.html; static/app.js:672-673).

### KF-102 — Add label support to the manual-time and edit-entry dialogs
- Surface: cross-surface (timer, task modal) · Category: missing · Severity: medium
- KanbanFlow: both dialogs have "Add comment" and "Add labels" toggles/links; the Labels field suggests existing labels ("test", "another test", "Add labels…") (v3 S5/S8, v7 S7/e009).
- ChipFlow: only a "+ Add comment" toggle exists in each dialog (templates/board.html:100, 132); no labels affordance anywhere.

### KF-103 — Live-increment the card badge during a run ("42m + 1m") and revert on discard
- Surface: cross-surface (timer, cards) · Category: missing · Severity: medium
- KanbanFlow: the card's time badge live-increments during the run ("42m + 1m") and reverts if the session is discarded (v4-00001–00002; frame review §P1-7).
- ChipFlow: a `<span class="card-live" hidden>` exists in templates/task_card.html with CSS classes for it (static/style.css:355-357), but no JS ever unhides or updates it — the badge never changes during a run.

## Breakdown

- By severity: high 25 · medium 44 · low 34
- By category: broken 15 · divergent 51 · missing 37
- By surface: timer 21 · task modal 18 · settings 16 · columns 16 · header 12 · cross-surface 10 · cards 6 · reports 4

## Top 20 highest-impact

1. KF-001 — TimerUI reads status fields the server never sends (whole timer UI dead) [VERIFIED]
2. KF-002 — Every stop 400s; no session is ever logged [VERIFIED]
3. KF-003 — "Add time manually" dialog entirely unwired
4. KF-004 — Edit time-entry dialog entirely unwired
5. KF-005 — "Why did you stop?" menu never populates
6. KF-006 — Break flow dead
7. KF-090 — Timer log page is a dead shell
8. KF-092 — Pomodoro statistics page is a dead shell
9. KF-094 — T/P/Y shortcuts wrong; P stops a running timer
10. KF-096 — Board Filter (funnel icon + panel) entirely missing
11. KF-099 — Board-bar Menu + 15-item Reports menu entirely missing
12. KF-027 — Running pill has no visible running state
13. KF-098 — Idle pill shows the word "Pomodoro" instead of ▶ + duration
14. KF-056 — Task modal missing the vertical right-edge action icon column
15. KF-058 — Subtasks entirely missing from the task modal
16. KF-059 — 8 of 10 Add-menu items missing from the task modal
17. KF-060 — Members feature entirely missing
18. KF-074 — Timer settings modal (4 tabs) entirely missing
19. KF-079 — Sounds settings entirely missing
20. KF-089 — Board deletion flow entirely absent

## What this audit could NOT cover (needs a live signed-in KanbanFlow browser session)

- KanbanFlow's avatar/account menu contents (never captured open).
- Bell and help icon destinations.
- Break settings/countdown visuals beyond the pre-break "Take break" state.
- Destinations of the 13 Reports-menu items beyond Time spent / Pomodoro statistics.
- Board Settings sub-pages (General, Layout, Task settings, Advanced, Custom fields, Custom roles, API & Webhooks, Add task from email) — only nav labels captured.
- Swimlanes (premium-only in KanbanFlow; no reference exists).
- Pixel-level visual fidelity of any surface (no rendered comparison was possible).
- Actual runtime behavior — every "broken" finding above is inferred from static code, not observed in a browser.

### KF-104 — Color legend uses small dots, not full-width color segments [MEDIUM | divergent | Legend]
- KanbanFlow reference: Board footer is a full-width strip of solid color segments, each segment the task color with its label centered inside ("1 Pomodoro", "2 Pomodori", "3 Pomodori", ">3 Pomodori"). (Board screenshot, footer row.)
- ChipFlow behavior: Footer is a wrapping flex row of small 0.9rem dots next to labels (`templates/board.html:208-212`, `static/style.css:426-447` `.color-legend` / `.color-legend-dot`).
- Evidence: Runtime HTML from `/b/{id}` shows `<footer class="color-legend">` with `<span class="color-legend-dot">` per color; visually a row of dots, not KanbanFlow's segmented strip.
- Not a duplicate: No existing KF covers the legend's presentation.

### KF-105 — "Show a browser notification when a timer ends" checkbox has no KanbanFlow counterpart [LOW | divergent | Settings]
- KanbanFlow reference: Timer settings modal's four tabs (General / Interruptions / Break activities / Sounds) have a "Sounds ON" toggle; no browser-notification control exists in the verified settings UI (frame review v2-03302–03913).
- ChipFlow behavior: Extra "Show a browser notification when a timer ends" checkbox plus permission hint in the Alerts section (`templates/settings.html:36–40`).
- Not a duplicate: KF-079 covers the missing Sounds tab and the invented "ding" checkbox; this is a second invented control.

### KF-106 — Settings claims custom reasons "are added automatically"; KanbanFlow's verified path is the Interruptions tab's "Add reason" button [LOW | divergent | Settings]
- KanbanFlow reference: Reasons are added via the Interruptions tab's green "Add reason" button (frame review §a, v2-03411–03484); automatic addition on typing was never verified in any frame.
- ChipFlow behavior: `templates/settings.html:45` asserts "Custom reasons you type there are added automatically", and the server auto-inserts the reason on stop (`src/routes.rs:3183–3184`).
- Not a duplicate: KF-076 covers the Interruptions tab's missing drag-reorder/inline-rename; this is a separate unverified behavior/copy claim.

### KF-107 — Colors admin page carries a "Log out" button; KanbanFlow's Board settings pages have none [LOW | divergent | Colors]
- KanbanFlow reference: Board settings page (a8 screenshot) shows no logout control; identity is the avatar in the top bar.
- ChipFlow behavior: `templates/board_colors.html:8` — `<a href="/logout" class="btn">Log out</a>` in the page header next to "Back to board". The Boards page header has one too (`templates/new_board.html:8`).
- Not a duplicate: KF-033 covers gear + logout in the board header; this is a separate surface.

### KF-108 — Colors intro is missing the "Show more…" expander [LOW | missing | Colors]
- KanbanFlow reference: The intro copy in the light-blue info box ends with a "Show more…" link that expands additional text (a8 screenshot).
- ChipFlow behavior: `templates/board_colors.html:13–14` — static muted paragraph, no expander link.
- Not a duplicate: KF-086 covers the intro copy wording; the expander control is distinct.

### KF-109 — Missing "?" help icon on the "Disabled colors" heading [LOW | missing | Colors]
- KanbanFlow reference: The "Disabled colors" heading carries a circled "?" help icon (a8 screenshot).
- ChipFlow behavior: No help affordance anywhere on the color admin; no disabled-colors section at all (`templates/board_colors.html`).
- Not a duplicate: KF-081 covers the missing Enabled/Disabled table split; the help icon is distinct.

### KF-110 — Colors page H1 is "Task colors — {board}"; KanbanFlow keeps "Board settings: General" on every tab [LOW | divergent | Colors]
- KanbanFlow reference: a8 screenshot shows H1 "Board settings: General" while the Colors nav item is selected — the title does not change per tab.
- ChipFlow behavior: `templates/board_colors.html:6` — H1 is "Task colors — {{ board_name }}".
- Not a duplicate: KF-080 covers the missing settings shell/nav; the per-tab title behavior is a detail it doesn't capture.

### KF-111 — Premium-gated settings nav items (Custom fields, Custom roles) render dimmed in KanbanFlow [LOW | divergent | Settings]
- KanbanFlow reference: a8 screenshot's left nav shows "Custom fields" and "Custom roles" grayed out (premium-gated); other nav items render at full weight.
- ChipFlow behavior: No Board settings nav exists at all.
- Not a duplicate: KF-080 covers the missing shell/nav; the dimmed premium state of two specific items is a distinct visual detail.

### KF-112 — Template deletion on the Boards page uses native window.confirm() [LOW | divergent | Board list]
- KanbanFlow reference: Deletions go through styled in-page confirmation dialogs (fidelity-status, cited in KF-047 for column deletion).
- ChipFlow behavior: `templates/new_board.html:90` — `if (!window.confirm('Delete this template?')) return;` in the delete-template handler.
- Not a duplicate: KF-047 covers column deletion on the board surface; this is template deletion on the Boards page.

### KF-113 — Timer log day groups have a blue "Add time entry" link per group [LOW | missing | Reports]
- KanbanFlow reference: Each day group in the Timer log carries a blue "Add time entry" link (frame review, "Other verified findings").
- ChipFlow behavior: `/timer/log` has no day groups at all (`templates/timer_log.html:17–31`).
- Not a duplicate: KF-090 covers the missing day grouping/filters/icons as rendering gaps; the per-day-group "Add time entry" action is a distinct manual-entry entry point.

### KF-114 — Duration renders "1h 9m" (spaced); KanbanFlow renders "1h6m" [LOW | divergent | Cards]
- KanbanFlow reference: Card badges show "34m", "1h6m", "42m" — no space between hour and minute components (frame review, "Other verified findings").
- ChipFlow behavior: `templates/task_card.html:10` renders `{{ total_minutes / 60 }}h {{ total_minutes % 60 }}m` (space); identical format duplicated in the modal's Time-spent stat (`templates/modal.html:66`).
- Not a duplicate: KF-054 (clock icon), KF-055 (tomato badge), KF-067 (modal stat labels) don't cover hour/minute spacing.

### KF-115 — Time-log entries show start time only; KanbanFlow shows the From–To range in 12h [LOW | divergent | Task modal]
- KanbanFlow reference: Per-task Time log entries show "duration + time range", e.g. "1:25 PM - 1:55 PM" (frame review P1-5, v7 S10).
- ChipFlow behavior: `templates/time_entries.html:9` renders a single start timestamp via `src/routes.rs:679-684` as `"%b %d, %Y %H:%M"` → "Sep 28, 2026 13:25". No end time, no range, 24h clock instead of 12h. The end time is derivable server-side but never rendered.
- Not a duplicate: KF-101 (avatar/grouping/trash), KF-012 (interrupted text), KF-015/016 (tooltips) don't cover the range or 12h/24h.

### KF-116 — `refreshModalTimeLog()` fetches a nonexistent route into a nonexistent element [MEDIUM | broken | Task modal]
- KanbanFlow reference: Editing a time entry ends with "Updating…" and the modal's log reflects the change in place (frame review v3 S8).
- ChipFlow behavior: `static/app.js:1348-1351` calls `fetchHtmlInto('/api/tasks/{id}/time-entries', '#modal-time-log')`. (a) No such route exists — the route is `/api/tasks/{id}/time` (`src/routes.rs:1214`) → 404. (b) The modal renders `<div id="time-entries">` (`templates/modal.html:76`), not `#modal-time-log` → the `if (el)` guard silently drops even a successful fetch. Invoked on every mutate success path (ManualTime.submit `app.js:1484`, EditEntry.submit `app.js:1589`, EditEntry.remove `app.js:1609`): the dialog closes but the Time log shows stale data until the modal is reopened.
- Not a duplicate: KF-003/KF-004 cover the dead dialogs; this is the distinct post-mutate refresh wiring (double mismatch: wrong route AND wrong selector).

### KF-117 — "Task URL" copies a deep link that nothing consumes [MEDIUM | broken | Task modal]
- KanbanFlow reference: Task URL is a working feature (card context menu v1-S2 via KF-050; modal More menu v7 via KF-071).
- ChipFlow behavior: `static/app.js:684` copies `origin + '/b/' + boardId + '#task-' + id` (toast confirms). But repo-wide grep for `location.hash` / `hashchange` / `#task` finds only the construction — nothing reads the fragment, scrolls to the card, or opens the modal. A pasted Task URL lands on the board with no indication of the task.
- Not a duplicate: No KF covers Task URL behavior (KF-050: missing card menu; KF-071: broken Watch item).

### KF-118 — Add-task form swaps the new card into itself, then deletes itself — the card never appears [HIGH | broken | Columns]
- KanbanFlow reference: Adding a task via the column "+" shows the new card immediately in the column.
- ChipFlow behavior: `templates/board.html:220-223` — `#add-task-form-template` declares `hx-post="/api/tasks" hx-swap="afterbegin"` with NO `hx-target`, so htmx inserts the returned card HTML as the first child of the form itself. Then `hx-on::after-request="this.remove()"` (plus the document-level `initAddTask` listener) removes the form — card included. Sequence: card inserted inside form → form removed → column count bumped with no visible card. The task exists server-side and appears only after reload. Verified against the shipped htmx 1.9.12: swap runs before `htmx:afterRequest` fires.
- Not a duplicate: No KF covers the add-task form submission (KF-037: button appearance only).

### KF-119 — `[data-open-manual-time]` delegation matches zero rendered elements [LOW | broken | Cross-surface]
- ChipFlow behavior: `initEntryEdit` (`static/app.js:1661-1666`) attaches handlers to `[data-open-manual-time]` ("Add time buttons elsewhere (time log page header)"), but grep across all templates confirms no element carries that attribute. The block silently does nothing.
- Not a duplicate: Not mentioned in any KF defect.

### KF-120 — `db.move_task` position collision can persist a dropped card one slot off [MEDIUM | broken | Columns]
- ChipFlow behavior: `db.move_task` sets the moved task's position to the client-sent index, then `renumber_cell` (`src/db.rs`) sorts by position with ties broken by task id. Dropping at a non-end position collides with the displaced task's existing value, so id-order tie-breaking can leave the card one slot off from the drop point. The DOM looks correct until reload.
- Not a duplicate: Server-side ordering defect; not in KF-001–KF-103. (Client sends the right value; found during wiring audit.)

### KF-121 — Legend footer shows task colors; KanbanFlow's footer is a fixed pomodoro-count band [MEDIUM | divergent | Legend]
- Status: CLOSED 2026-09-30 — false premise. KanbanFlow's footer is NOT a fixed pomodoro-count band: the legend-trigger investigation (2026-09-30) proved the footer is a COLOR LEGEND showing one segment per board-ENABLED color with custom names, controlled by the board Menu's "Color legend" toggle. Chip's General board shows pomodoro labels only because its colors were renamed (Yellow→"1 Pomodoro" etc.). KF-222 implements exactly this mechanism (verified 2026-09-30); KF-104 (presentation: dots vs segments) remains the open styling question.
- KanbanFlow reference (original, based on Chip's customized board): The board footer is a full-width band of four equal pastel segments — "1 Pomodoro" (pale yellow), "2 Pomodori" (pale green), "3 Pomodori" (pale blue), ">3 Pomodori" (pale pink). It is a pomodoro-count legend, not a color key (hi-res board screenshot, bottom strip).
- ChipFlow behavior (before KF-222): `footer.color-legend` (`templates/board.html:208-214`) renders one segment per enabled board color. On any non-Pomodoro board it shows task colors, which KanbanFlow never shows in the footer.
- Not a duplicate: KF-104 covers the legend's PRESENTATION (dots vs full-width segments); this is the CONTENT (task-color key vs fixed pomodoro-count band).

### KF-122 — Invented board-toolbar row occupies the board-bar slot [LOW | divergent | Header]
- KanbanFlow reference: The row beneath the dark top bar is the board bar — board title + member chip + add-board "+" left; timer pill, filter funnel, edit-layout pencil, "≡ Menu" right (board screenshot).
- ChipFlow behavior: `.board-toolbar` (`templates/board.html:22-31`) renders "+ Add column", "+ Add swimlane" left and "Save as template", "Colors", "Boards" right. KanbanFlow has no such row; ChipFlow has no board bar at all.
- Not a duplicate: KF-032 mentions the toolbar only as context; KF-023/024/096/099 file the missing KanbanFlow controls individually; the invented row itself was never filed.

### KF-123 — Board name sits in the dark topbar instead of the light board bar [LOW | divergent | Header]
- KanbanFlow reference: Two-row header; the board title ("General") is in the light board bar beside the member chip and add-board "+". The dark top bar carries no board name (board screenshot).
- ChipFlow behavior: `<h1>{{ board_name }}</h1>` inside the dark `.topbar` (`templates/board.html:7`); the light board bar doesn't exist.
- Not a duplicate: KF-023 covers the missing board tabs/add-board affordance, not the misplaced title.

### KF-124 — Done-column date group labels are uppercase/letterspaced with no divider rule [LOW | divergent | Columns]
- KanbanFlow reference: Done-column group headers read "Today", "Friday, 10 July" in normal-case gray, each with a horizontal rule beneath spanning the column (board screenshot).
- ChipFlow behavior: Label text matches ("Today"/"Friday, 10 July" via `done_group_label`, `src/routes.rs:464-472`), but `.done-group-label` (`static/style.css:256-263`) forces `text-transform: uppercase; letter-spacing: 0.05em; font-weight: 700` with no rule (`templates/board.html:194`).
- Not a duplicate: KF-044 covers only the "Group tasks by date" checkbox; group-label presentation was never filed.

### KF-125 — Collapsed column strip keeps the white card style instead of KanbanFlow's gray strip [LOW | divergent | Columns]
- KanbanFlow reference: The collapsed BACKLOG strip is a light-gray band with gray text (board screenshot).
- ChipFlow behavior: `.columnHeader--collapsed` (`static/style.css:186`) only narrows to 48px; background stays the white header card (`#fff`) with default ink text.
- Not a duplicate: KF-039 (missing red count) and KF-040 (missing uppercase) don't cover the strip background.

### KF-126 — Board body is floating rounded cards with 8px gaps instead of a flush grid [LOW | divergent | Columns]
- KanbanFlow reference: The board is a continuous table — columns separated by thin gray lines, cells flush with subtle borders, no gaps (board screenshot).
- ChipFlow behavior: `.board-table { border-spacing: 8px }` (`static/style.css:102`) with `.board-cell` as rounded (`border-radius: 8px`) cards floating over the page background (`static/style.css:238-245`).
- Not a duplicate: KF-049 covers only the column header cards vs. flat gray; body-cell treatment was never filed.

### KF-127 — `GET /api/timer/settings` returns 404; timer settings never load [HIGH | broken | Timer]
- ChipFlow behavior: The frontend TimerUI (`static/app.js:854`) fetches `/api/timer/settings` on board initialization, but `src/routes.rs` defines no such route. Every request returns 404.
- Evidence: Browser network monitor on authenticated board load captured 3× `HTTP 404 GET /api/timer/settings`. Timer settings UI cannot initialize.
- Not a duplicate: KF-001 covers the timer status schema mismatch; this is a missing endpoint, not a schema issue.

### KF-128 — `GET /api/timer/status` returns 401 for authenticated browser session [HIGH | broken | Timer]
- ChipFlow behavior: The frontend polls `/api/timer/status` (`static/app.js:880`) to render the timer pill. For a session-cookie-authenticated browser, the request returns 401 Unauthorized. The timer pill shows "--:--" and never updates with real status.
- Evidence: Browser network monitor captured `HTTP 401 GET /api/timer/status` on authenticated board load. Route defined at `src/routes.rs:52` requires `Extension<AuthUser>`.
- Not a duplicate: KF-001 covers the status schema mismatch; this is an auth failure preventing any status from loading at all.

### KF-129 — Unhandled JSON parse error when timer API returns HTML error pages [MEDIUM | broken | Timer]
- ChipFlow behavior: Timer `fetch()` calls invoke `.json()` without checking response status. When the server returns an HTML error page (404/401), this throws `SyntaxError: Unexpected token '<', "<!DOCTYPE "... is not valid JSON` as an uncaught (in promise) rejection.
- Evidence: Browser pageerror captured the SyntaxError during authenticated board load.
- Not a duplicate: Distinct from the 404/401 themselves (KF-127/KF-128); this is missing error handling in the timer frontend.

### KF-130 — Timer UI writes to non-existent DOM elements (null textContent/innerHTML) [MEDIUM | broken | Timer]
- ChipFlow behavior: TimerUI code assigns `textContent` and `innerHTML` on query results that are null — the referenced element IDs do not exist in the board DOM.
- Evidence: Browser pageerror captured `TypeError: Cannot set properties of null (setting 'textContent')` and `TypeError: Cannot set properties of null (setting 'innerHTML')` during authenticated board load.
- Not a duplicate: Distinct from API failures; these are DOM selector mismatches in the timer frontend code.

### KF-131 — Swimlane labels horizontal on left; KanbanFlow has them vertical on right [MEDIUM | divergent | Swimlanes]
- KanbanFlow reference: Swimlane labels ("PERSONAL TO-DO", "BACKLOG") render as vertical (rotated 90°) text on the RIGHT side of the board (KanbanFlow reference board screenshot).
- ChipFlow behavior: Swimlane labels render as horizontal text on the LEFT side of the board (1440px screenshot `010-board-fresh.png`).
- Evidence: Side-by-side visual comparison of 1440×900 screenshots.
- Not a duplicate: No existing defect covers swimlane label position/orientation. (This also corrects the limitations note claiming "no reference exists" for swimlanes — the reference board screenshot does show them.)

---

## Browser confirmation (2026-09-29)

The following pre-existing defects were confirmed via real Chromium browser testing against a fresh local database (build 199af93c, 1440×900 screenshots, network monitoring). Screenshots: `~/workspace/target/pw-capture/shots/`.

- **KF-104 (footer band):** CONFIRMED. Screenshot `010-board-fresh.png` shows a dot legend ("🟨 1 Pomodoro 🟩 2 Pomodori..."); KanbanFlow reference shows a full-width 4-segment colored footer band.
- **KF-122 (invented toolbar):** CONFIRMED. Screenshot shows "+ Add column | + Add swimlane" left and "Save as template | Colors | Boards" right in a light toolbar row; KanbanFlow has no such row.
- **KF-123 (board name placement):** CONFIRMED. Screenshot shows "General" in the dark top bar next to "☰ Boards"; KanbanFlow shows the board name in the light board bar below the dark top bar.
- **KF-036 (count on second line):** CONFIRMED. Screenshot shows "0", "0 / 3" rendered below the column names; KanbanFlow shows them inline to the right of the names.
- **KF-037 (green + button):** CONFIRMED. Screenshot shows small muted "+" buttons in column headers; KanbanFlow shows large bright-green "+" buttons.
- **KF-001 (timer schema):** CONFIRMED as browser-visible breakage via KF-127 through KF-130 (missing endpoint, 401, unhandled errors, null DOM writes). The timer pill is non-functional in the browser.

New defects KF-127 through KF-131 were discovered during this browser pass and are listed above.

### KF-132 — Timer pill in dark top bar; KanbanFlow has timer clock icon in the light board bar [HIGH | divergent | Timer]
- KanbanFlow reference: The timer is a clock-icon button in the BOARD BAR (the light bar containing the board name), NOT in the dark top header. (Live browser handoff §A: "Timer is in the BOARD BAR (clock icon), not the top header." Timer panel idle shows Stopwatch/Session time/00:00/green Start/No task selected, with bottom tabs Pomodoro, Add time, Log, Settings.)
- ChipFlow behavior: A "Pomodoro" pill sits in the dark top bar next to the username/admin/gear/Log out (screenshot `010-board-fresh.png`). The board bar has no timer control.
- Evidence: Live KanbanFlow handoff + 1440px ChipFlow screenshot.
- Not a duplicate: KF-122/KF-123 cover the invented toolbar and board-name placement; this is specifically about timer location.

### KF-133 — Board bar lacks KanbanFlow's standard buttons (Invite, Timer, Filter, Edit layout, Menu) [HIGH | missing | Board]
- KanbanFlow reference: Board bar contains: board heading, owner avatar, Invite member (+), Timer (clock icon), Filter (funnel), Edit board layout (pencil), Menu (hamburger + "Menu"). (Live browser handoff §B.)
- ChipFlow behavior: The light toolbar row contains invented controls "+ Add column", "+ Add swimlane" (left) and "Save as template", "Colors", "Boards" (right). None of KanbanFlow's five board-bar controls exist.
- Evidence: Live KanbanFlow handoff + screenshot `010-board-fresh.png`.
- Not a duplicate: KF-122 covers the invented toolbar's existence; this enumerates the specific missing KanbanFlow controls.

### KF-134 — Board Filter panel missing [HIGH | missing | Filter]
- KanbanFlow reference: Filter panel with sections: User (Show all, Timer users, Unassigned, Chip Senkbeil); Color (Show all, Yellow, Green, Blue, Red, Orange, Purple, Magenta, Cyan); Date (Show all, No due date, Overdue, Due today, Due tomorrow, Due in 7 days, Due in 14 days, Due in 30 days, Due in 90 days, Due in September, Due in October, Due in 2026, Due in 2027); Labels ("Add labels..."); "Remember filter" toggle; "Bookmarks (0)". (Live browser handoff §B.)
- ChipFlow behavior: No filter functionality exists anywhere.
- Evidence: Live KanbanFlow handoff.
- Not a duplicate: No existing defect covers the Filter panel.

### KF-135 — Board Menu missing [HIGH | missing | Menu]
- KanbanFlow reference: Board Menu contains: Filter, Reports, Board layout, Settings, Members, Recycle bin, Dark mode, Color legend, Large task names, Help & feedback, Get Premium. (Live browser handoff §B.)
- ChipFlow behavior: No board menu exists.
- Evidence: Live KanbanFlow handoff.
- Not a duplicate: No existing defect covers the Board Menu.

### KF-136 — Reports submenu missing (15 items) [HIGH | missing | Reports]
- KanbanFlow reference: Reports submenu lists 15 items: Pomodoro statistics, Time spent, Print, Board history, Burndown, Calendar, Cumulative flow, Cycle & lead time, Due date performance, Monte Carlo forecasting, Task count, Throughput, Time estimate, Time in column, Export. (Live browser handoff §B.)
- ChipFlow behavior: No reports menu exists.
- Evidence: Live KanbanFlow handoff.
- Not a duplicate: No existing defect enumerates the Reports submenu.

### KF-137 — Double init binds every handler twice: manual-time submit creates duplicate entries [HIGH | broken | Timer]
- KanbanFlow reference: Adding time manually creates exactly one entry; the calendar button toggles the date picker.
- ChipFlow behavior: `static/app.js` is loaded with `defer`, so at execution time `document.readyState` is already `'interactive'` — both the immediate init block AND the `DOMContentLoaded` listener fire, running `initEntryEdit()` (and `initBoard()`, `initTimerLogPage()`, `initTimerStatsPage()`) twice. Every handler is double-bound: the "Add time manually" submit double-POSTs to `/api/time/manual` (browser-verified: 3 tasks each got exactly 2 identical entries), and the calendar button's toggle opens-then-immediately-closes the popup (verified: `openCalendar` called twice per click). `TimerUI.init` already has an `initialized` guard; the others do not.
- Evidence: Playwright adversarial pass 2026-09-29 (dbg4.js: calls=2; dbg5.js: entries per task = 2,2,2).
- Not a duplicate: no existing defect covers init double-binding. (initEntryEdit guarded 2026-09-29; initBoard/initTimerLogPage/initTimerStatsPage still unguarded.)

### KF-138 — Timer popup DOM skeleton missing: running session view, idle start UI, and Today list never render [HIGH | broken | Timer] **[VERIFIED 2026-09-29]** — real-Chromium run vs fresh DB: popup renders mode tabs (Pomodoro/Stopwatch), idle start UI (task select + duration + Start), running view ticks down (24:59→24:55) with task name + red Stop; Today list populated from /api/timer/today ('Timer test task B 10:26 AM · 1m').
- KanbanFlow reference: the timer panel renders the live session (countdown/count-up, task, Pause/Stop), the idle start UI (task + duration pickers, Start button), and the Today entries list.
- ChipFlow behavior: `renderPopup()` writes the running session into `#timer-popup-body` and the idle start UI into `#timer-mode-tab` under `.timer-modes`, but none of those elements exist in `templates/board.html` (only `#timer-foot-mode` exists) — every one of those writes is a null-guarded no-op, so the popup is stuck on its static skeleton (frozen "--:--", always-visible Stop button). The `#timer-today-list` is never populated: no code fetches the existing `GET /api/timer/today`. Additionally `tick()` requires `phase === 'running'`, but the server reports the mode string as phase ('pomodoro'/'stopwatch'/…), so the 1s tick never fires — no live countdown, and `finishSession()` can never trigger client-side (KF-006's break flow is unreachable).
- Evidence: source read 2026-09-29 (static/app.js:971-973, 1030, 1087; templates/board.html popup markup).
- Not a duplicate: KF-001 covered the status field mismatch (fixed); this is the missing DOM + dead tick.

### KF-139 — Deleted-board color recreation guarded [LOW | divergent | Colors] **[VERIFIED 2026-09-29]**
- Discovery: If a board is deleted and `list_colors` is called on the deleted board ID, would `ensure_board_colors` recreate the default palette?
- ChipFlow behavior: `ensure_board_colors` (src/db.rs:1008) explicitly guards against this: "Don't backfill colors for a board that doesn't exist (e.g. after deletion); list_colors on a deleted board must stay empty." The function returns early if `get_board(board_id)` is None.
- Evidence: Source verification 2026-09-29. The guard is present and the intent is documented in the code comment.
- Not a duplicate: No existing defect covers deleted-board color backfilling.

### KF-140 — Card context menu has 8 entries incl. "Timer" and "Edit grouping date"; expected 7 starting with "Start timer" [MEDIUM | broken | Board]
- KanbanFlow reference: task context menu has exactly 7 entries: Start timer, Move, Color, Assign members, Copy here, Task URL, Delete.
- ChipFlow behavior: right-clicking a card shows 8 entries: Timer, Move, Edit grouping date, Color, Assign members, Copy here, Task URL, Delete.
- Evidence: Real-Chromium adversarial pass 2026-09-29 vs fresh DB (build 1b618c3).
- Not a duplicate: no existing defect covers the context-menu entry list.

### KF-141 — Context-menu Move does nothing: client PATCHes /api/tasks/{id}/move, API defines POST [HIGH | broken | Board]
- KanbanFlow reference: Move relocates the task to the chosen column.
- ChipFlow behavior: right-click card → Move → pick column does nothing; the client issues PATCH /api/tasks/{id}/move while the API (per /api/v1/openapi.json and /agents.md) defines POST for that route.
- Evidence: Real-Chromium adversarial pass 2026-09-29 vs fresh DB (build 1b618c3).
- Not a duplicate: no existing defect covers the Move verb mismatch.

### KF-142 — Color change via context menu leaves stale accessible title [LOW | broken | Board]
- KanbanFlow reference: card title/label updates to reflect the new color.
- ChipFlow behavior: right-click a green card → Color → Red changes the color but the accessible title still reads the old value (e.g. "2 Pomodori").
- Evidence: Real-Chromium adversarial pass 2026-09-29 vs fresh DB (build 1b618c3).
- Not a duplicate: no existing defect covers stale accessible titles after recolor.

### KF-143 — [RESERVED — description missing from browser handoff]
- The 2026-09-29 browser handoff claimed 9 defects filed but described only 8 (KF-140–142, KF-144–148). This number is reserved pending reconciliation; do not reuse.

### KF-144 — Task modal → Add → Label closes the menu and opens no dialog [HIGH | broken | Board]
- KanbanFlow reference: Add → Label opens the label editor dialog.
- ChipFlow behavior: Task modal → Add → Label closes the Add menu and opens no dialog. Confirmed twice in the same session.
- Evidence: Real-Chromium adversarial pass 2026-09-29 vs fresh DB (build 1b618c3).
- Not a duplicate: no existing defect covers the Label dialog.

### KF-145 — Task modal → Add → Due date closes the menu and opens no dialog [HIGH | broken | Board]
- KanbanFlow reference: Add → Due date opens the due-date picker dialog.
- ChipFlow behavior: Task modal → Add → Due date closes the Add menu and opens no dialog. Confirmed twice in the same session.
- Evidence: Real-Chromium adversarial pass 2026-09-29 vs fresh DB (build 1b618c3).
- Not a duplicate: no existing defect covers the Due date dialog.

### KF-146 — Settings → API tokens → Revoke does nothing; token persists after reload [HIGH | broken | API]
- KanbanFlow reference: n/a (ChipFlow-native feature). Expected: revoking a token invalidates it.
- ChipFlow behavior: Settings → API tokens → Revoke appears to do nothing; the token is still present after page reload. Security-relevant: revocation must actually invalidate the token.
- Evidence: Real-Chromium adversarial pass 2026-09-29 vs fresh DB (build 1b618c3). Bearer-header negative test not possible in that environment — re-verify server-side that a revoked token returns 401.
- Not a duplicate: no existing defect covers token revocation.

### KF-147 — Cards in the first swimlane row are obscured by the sticky column header and unclickable [HIGH | broken | Board]
- KanbanFlow reference: all cards are clickable regardless of swimlane row.
- ChipFlow behavior: cards in the first swimlane row render underneath the sticky column header ("obscured" actionability error); new tasks default to the first swimlane, so newly created cards cannot be opened at all.
- Evidence: Real-Chromium adversarial pass 2026-09-29 vs fresh DB (build 1b618c3).
- Not a duplicate: no existing defect covers sticky-header occlusion.

### KF-148 — Task deletion completely broken: modal Delete and context-menu Delete fail silently [CRITICAL | broken | Board]
- KanbanFlow reference: deleting a task removes it.
- ChipFlow behavior: both the task-modal Delete button and the card context-menu Delete entry fail silently; the card persists after reload.
- Evidence: Real-Chromium adversarial pass 2026-09-29 vs fresh DB (build 1b618c3).
- Not a duplicate: no existing defect covers task deletion.

### Observation (not filed) — unexplained "Foo" entry in global Interruption reasons list
- The 2026-09-29 browser pass noticed a "Foo" entry in Settings → Interruption reasons that the tester does not recall adding via the UI. The seeded default list in src/models.rs contains no such entry. Likely accidental creation during the settings pass; no repro. Not filed as a defect. If it reappears on a fresh DB without user input, file it then.

### KF-149 — Board created without a template has no swimlanes; column "+" add-task buttons silently do nothing [HIGH | broken | Board]
- KanbanFlow reference: a new board's column add-task buttons work immediately.
- ChipFlow behavior: POST /api/boards without template_id seeds colors + one "To-do" column but zero swimlanes. The board renders an empty tbody; the add-task click handler finds no `.task-list[data-column-id]` cell and returns early with no feedback, so every "+" button appears broken until the user discovers the layout view's "+ Add swimlane".
- Evidence: managed-browser verification pass 2026-09-29 vs fresh DB (build 1f4fb378) — new "KF Test Board" (3 columns, 0 swimlanes): column "+" buttons silently did nothing; confirmed in source (`initAddTask` early return in static/app.js; `create_board` in src/routes.rs seeds no lane).
- Not a duplicate: no existing defect covers the zero-swimlane board state.
- Fix (2026-09-29): new `Db::ensure_default_swimlane` backfills a "Default" lane when a board has none (idempotent, skips missing boards); `create_board` calls it for template-less boards; `DELETE /api/swimlanes/{id}` now returns 400 on the board's last lane (mirrors the existing last-board rule); agents.md documents both. Regression test `tests/board_default_swimlane.rs` (3 tests) pins the contract.

### KF-150 — New API token doesn't appear in Active tokens until manual reload (15s delayed full-page reload) [LOW | divergent | Settings]
- KanbanFlow reference: n/a (API tokens are a ChipFlow addition); reasonable UX is the new token appears in the list immediately.
- ChipFlow behavior: creating a token shows the one-time secret but schedules `setTimeout(function () { location.reload(); }, 15000)` (templates/settings.html inline script) — the Active tokens table stays stale for 15 seconds, inviting duplicate creation.
- Evidence: managed-browser verification pass 2026-09-29 (build 1f4fb378); confirmed in source.
- Not a duplicate: no existing defect covers the token list refresh.
- Fix (2026-09-29): after creation, the client re-fetches the settings page and swaps `#token-list` in place (server-rendered, so date formatting stays consistent), re-binding the revoke buttons; no reload, so the one-time secret stays visible.

### KF-151 — Only 4 task colors enabled by default (KanbanFlow enables its full 10-color standard palette) [severity unknown | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca ("Fix KF-151..KF-159 from final verification battery") after a `git reset --hard` destroyed the uncommitted .dev/ files. Code comment in the fix: "KF-151: KanbanFlow enables its standard palette by default, so all [10 colors enabled]". Fix: default enabled colors are now yellow, green, blue, red, orange, purple, magenta, cyan, brown, white (all 10). Regression test updated: colors_seed_with_pomodoro_defaults asserts all 10 enabled.
- Fix commit: 4acaaca. Verified by the fix-verification battery.

### KF-152 — Lone "Default" swimlane header shown (KanbanFlow's free tier renders no swimlane row) [severity unknown | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca. Board page rendered the swimlane label row even for a board with only the structural "Default" lane (needed by KF-149 so column "+" buttons work). Fix: new `hide_swimlane_header` template flag (true when exactly one lane named "Default"); board.html skips the corner cell and the label cell when set. DB row kept.
- Fix commit: 4acaaca. Verified by the fix-verification battery; KF-162's later fix preserves this behavior.

### KF-153 — Board settings H1 hardcoded "Board settings: General" instead of naming the board [severity unknown | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca. templates/board_settings.html rendered "Board settings: General" on every tab. Fix: H1 now renders "Board settings: {{ board_name }}".
- Fix commit: 4acaaca. Verified by the fix-verification battery.

### KF-154 — Board menu Settings opened account /settings instead of this board's settings [severity unknown | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca. The board menu's Settings entry navigated to the account-level /settings page. Fix (app.js menuAction): now navigates to '/b/' + boardId + '/settings'.
- Fix commit: 4acaaca. Verified by the fix-verification battery.

### KF-155 — "Kanban basics" built-in template missing / unprotected [severity unknown | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca. A second built-in template "Kanban basics" (10 standard colors, 3 columns To-do/In progress/Done with one Done column, 1 swimlane "Default") is seeded and protected from deletion (delete → RefusedBuiltIn). Fix also corrected the template seed lookup to match by name (BUILTIN_TEMPLATE_NAME) since there are now 2 built-ins. Regression test kanban_basics_builtin_template_shape added.
- Fix commit: 4acaaca. Verified by the fix-verification battery.

### KF-156 — Add/rename swimlane used window.prompt instead of an in-page dialog [HIGH | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca. addSwimlane() and renameSwimlane() used window.prompt, which automation environments auto-dismiss, so the add-swimlane flow appeared dead. Fix: real in-page dialog (#swimlane-dialog) with Add/Rename modes, Enter-to-save, name-required validation; POST /api/swimlanes / PATCH /api/swimlanes/:id, reload on success.
- Fix commit: 4acaaca. Verified by the fix-verification battery (22:40 entry).

### KF-157 — Label save misreported success as failure (PATCH returns HTML fragment, parsed as JSON) [HIGH | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca. The labels dialog's save() parsed PATCH /api/tasks/:id as JSON, but the endpoint returns the refreshed task card as an HTML fragment — so a successful save was misreported as a failure. Fix: parse as text (res.text()).
- Fix commit: 4acaaca. The fix-verification battery verified the label save path.

### KF-158 — Timer popup footer Settings button dead (TimerSettings not reachable from inline onclick) [severity unknown | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca. The timer popup's footer Settings button calls TimerSettings.open() from an inline onclick handler, but TimerSettings was not exposed globally, so the button did nothing. Fix: window.TimerSettings = TimerSettings.
- Fix commit: 4acaaca. Verified by the fix-verification battery.

### KF-159 — Date filters were no-ops (due dates not evaluated; undated tasks not handled per KanbanFlow) [HIGH | filed 2026-09-29 ~22:15 CDT by final verification battery (build e2cc0197)]
- RECONSTRUCTED 2026-09-29 23:30 CDT from fix commit 4acaaca. applyFilter treated date filters as no-ops ("Tasks carry no due dates" comment). Fix: task cards carry data-due-at (RFC3339, rendered from TaskView.due_at); new dateMatches() implements KanbanFlow parity — undated tasks hidden for every specific range, shown only by "Show all" or "No due date"; supports overdue/today/tomorrow/month:/year:/"due in N days".
- Fix commit: 4acaaca. KF-159 regression verified PASS on the test instance (22:40 entry).

### KF-160 — Swimlane Delete does nothing [HIGH | filed 2026-09-29 ~22:30 CDT by fix-verification battery (build 4acaaca)]
- NOTE (2026-09-29 23:15 CDT reconstruction): full entry lost in the 23:10 `git reset --hard`. Known: swimlane menu Delete had no effect. Fix (63af405 "Fix KF-160..KF-162 from fix-verification battery"): styled in-page confirmation dialog ("Delete swimlane", Cancel/Delete, dimmed backdrop; replaced native confirm()); confirming deletes the lane and it stays gone after reload; deleting the last swimlane is refused with a "Could not delete swimlane." toast. VERIFIED by the 22:33 watchdog's browser battery on build 63af405 (KF-160 PASS). Restore repro details from the filing agent's context.

### KF-161 — Column counts ignore filters [LOW | filed 2026-09-29 ~22:30 CDT by fix-verification battery (build 4acaaca)]
- NOTE (2026-09-29 23:15 CDT reconstruction): full entry lost in the 23:10 `git reset --hard`. Known: column header counts showed unfiltered totals when a color filter hid cards. Fix in 63af405. PARTIALLY VERIFIED by the 22:33 watchdog's browser battery: filtering by color reduced the column header count (2→0 when both cards hidden); clearing the filter restored the true total (2). The partial-hide case (some cards visible) was not tested — the test instance was swapped mid-battery. Restore repro details from the filing agent's context.

### KF-162 — Swimlane labels on right edge instead of left [LOW | filed 2026-09-29 ~22:30 CDT by fix-verification battery (build 4acaaca)]
- NOTE (2026-09-29 23:15 CDT reconstruction): full entry lost in the 23:10 `git reset --hard`. Known: swimlane labels rendered on the right edge of their rows; KanbanFlow renders them vertically on the LEFT edge. Fix in 63af405. VERIFIED by the 22:33 watchdog's browser battery (KF-162 PASS — visual: labels render vertically on the left edge). Restore repro details from the filing agent's context.

### KF-163 — No UI to save a board as a custom template [MEDIUM | missing | Templates]
- KanbanFlow reference: board Menu → Save board as template.
- ChipFlow behavior: neither the board Menu (Filter, Reports, Board layout, Settings, Members, Recycle bin, Dark mode, Color legend, Large task names, Help & feedback, Get Premium) nor board Settings (General/Layout/Colors/Task settings/Advanced/API & Webhooks/Add task from email) exposes save-as-template. The capability exists only as the documented API endpoint POST /api/boards/:id/save-as-template (per /agents.md).
- Evidence: final-build battery 2026-09-29 (build 63af405) on test instance. The gate #8 "Additional features" criterion requires creating a board from a template AND saving a board as a template in a real browser.
- Not a duplicate: no existing defect covers save-as-template UI.

**KF-163 fix notes (2026-09-29):** the in-page dialog markup (#save-template-dialog) and the POST wiring (doSaveTemplate → POST /api/boards/:id/save-as-template) already existed from the earlier save-template work, but KF-122's removal of the invented toolbar button left them orphaned with no menu entry. Fix: added "Save board as template" (data-bm="save-template") to the board menu after "Color legend"; menuAction case opens the existing dialog via a new openSaveTemplateDialog() (also reused by the dead toolbar-button wiring). Verified: cargo fmt --check PASS, cargo build PASS (Askama recompile validates board.html), cargo clippy 0 warnings, cargo test all suites ok (0 failed), node --check app.js PASS. Live curl against a local throwaway DB: POST save-as-template → 200 {id}, GET /api/templates lists "KF163 template" (built_in: false) alongside the 2 built-ins, empty name → 400; board page HTML renders both the menu item and the dialog.

### KF-164 — Save board as template silently drops tasks (template instantiation has 0 tasks) [MEDIUM | missing | Templates]
- Save-as-template dialog says it saves the board's "columns, tasks, and settings"; built-in templates (e.g. Pomodoro board with "Pomodoro 1") ship with tasks. But a board created from a user-saved template contains the columns and 0 tasks.
- ChipFlow behavior: board "KF163-FromTmpl" created from template "KF163-Tmpl" (saved from "KF163-Source" containing column "MyCol" and task "MyTask" in To-do) has "MyCol" but To-do count 0 — "MyTask" not carried over.
- Evidence: final-build battery 2026-09-29 (build 3be56b3) on test instance. KF-163 UI (menu entry → dialog → toast → picker listing) all VERIFIED; this is the remaining gap in the save-template flow.
- Repro: 1) New board from Blank → "KF163-Source". 2) Add task "MyTask" to To-do; add column "MyCol" via Edit board layout. 3) Board ☰ menu → "Save board as template" → name "KF163-Tmpl" → Save (toast confirms). 4) New board → select "KF163-Tmpl" → create. 5) Observe MyCol present, MyTask missing.
- Not a duplicate: no existing defect covers task-carry-over in saved templates.

**KF-164 fix notes (2026-09-29):** root cause: `Db::save_board_as_template` captured colors, columns, and swimlanes but not tasks; template application had no task restoration. Fix (commit d902459, pushed 5eea8a6, deployed to production 23:03 CDT): save-as-template now snapshots each task's name, description, size, color value, column/swimlane names, position, due date/repeat, subtasks, and labels; `apply_template` restores them with fresh per-instance state (no history/comments/attachments/assignments/completion/timer stats). Verified: cargo fmt --check clean, cargo build OK, cargo clippy --all-targets 0 warnings, cargo test 54/54 pass, node --check static/app.js clean; local E2E (template creation → source-board deletion → instantiation) confirmed a task with color, description, label, due date/repeat, and subtask survives. Final-build browser verification in progress (KF164-Source → KF164-Tmpl → KF164-Dest round trip).

### KF-165 — Label filtering unavailable in Filter panel (labels input permanently disabled; clicking a card label chip opens the task dialog instead of filtering) [MEDIUM | missing | Board]
- KanbanFlow reference: the board can be filtered by labels — clicking a label filters the board; the Filters panel has a Labels selector.
- ChipFlow behavior: the Filter panel's Labels textbox is disabled with tooltip "Labels are not available yet" even after creating labels on tasks and reloading. Clicking a label chip on a card opens the task dialog instead of filtering.
- Evidence: final verification battery 2026-09-29 23:13 CDT (build 5eea8a6) on test instance. Gate 2 check (h): created label "gate-label" on task "Reorder-B" (KF-157 label save/persistence verified PASS after reload), then Filter → Labels showed the disabled input. All other Gate 2 checks (a–g, i) passed: reorder, column rename/add/delete with confirm, WIP limit warning (3/2), swimlane add/rename/delete with confirm, timer start/stop + settings change (no pause control by design — KanbanFlow parity), date filter "Due today" hiding dateless tasks with counts counting only visible cards.
- Repro: 1) Task dialog → Add → Label → type "gate-label" → Enter → Save. 2) Open Filter (funnel icon) → Labels section shows disabled input "Labels are not available yet". 3) Edit column → Labels: Show, then click the label chip on the card → task dialog opens, no filter applied.
- Not a duplicate: no existing defect covers label filtering.
- Fix (2026-09-29 ~23:55 CDT, commit 5fac2c1 "Add label filtering to board filter panel (KF-165)", pushed as origin/main df6f2dd, deployed to production — verified live df6f2dd ~23:20 CDT): TaskView gains `labels_json`, rendered as the card's `data-labels` JSON attribute (templates/task_card.html) so filtering works even when a column hides label chips. Filter panel Labels section now renders checkboxes populated from GET /api/boards/:id/labels (templates/board.html, BoardChrome.loadFilterLabels). applyFilter hides cards missing any checked label (AND semantics, KanbanFlow parity); isFiltering, remember-filter save/restore (async restore via _pendingLabelFilter), and the filter change listener all include labels. Clicking a card's label chip now toggles its label filter (opens the filter panel with the label checked) instead of opening the task dialog; chips get cursor:pointer. Project checks green: cargo fmt --check, cargo build, cargo clippy --all-targets (0 warnings), cargo test (54 passed), node --check static/app.js. Browser verification of the fix in progress on the resumed final battery.

### KF-166 — Task cards use 2px borders + drop shadow; KanbanFlow cards are 1px flat with no shadow [HIGH | visual | Board]
- KanbanFlow reference (fresh capture 2026-09-30): task cards are "full pastel-colored rectangles with thin darker-colored borders and slightly rounded corners. No shadows."
- ChipFlow behavior: `.task-card { border: 2px solid; border-radius: 6px; box-shadow: 0 1px 2px rgba(0,0,0,0.08); }` plus a stronger hover shadow. Cards look heavy/chunky vs KanbanFlow's flat utilitarian cards.
- Evidence: static/style.css lines 476-484 vs live KanbanFlow demo board (kanbanflow.com/board/UTTNB5) captured 2026-09-30.
- Not a duplicate: no existing defect covers card border weight/shadow.

### KF-167 — Card titles semibold 14.4px; KanbanFlow card text is regular ~12px [HIGH | visual | Board]
- KanbanFlow reference (fresh capture 2026-09-30): "Text is small (~12px) dark sans-serif (Arial/Trebuchet-like)... bold only on column headers and board name."
- ChipFlow behavior: `.card-title { font-weight: 600; font-size: 0.9rem; }` (~14.4px semibold). Every card shouts vs KanbanFlow's quiet dense text.
- Evidence: static/style.css line 487 vs live KanbanFlow demo board captured 2026-09-30.
- Not a duplicate: no existing defect covers card title weight/size.

### KF-168 — Column headers left-aligned; KanbanFlow centers bold column names [MEDIUM | visual | Board]
- KanbanFlow reference (fresh capture 2026-09-30): "light gray header bars with centered bold black column names."
- ChipFlow behavior: `.columnHeader { text-align: left; }`, `.columnHeader-name { font-weight: 600; font-size: 0.92rem; }`. Names sit left, not centered, and are 600-weight rather than bold black.
- Evidence: static/style.css lines 224, 243-250 vs live KanbanFlow demo board captured 2026-09-30.
- Not a duplicate: no existing defect covers header alignment.

### KF-169 — Top dark bar ~50px tall; KanbanFlow's is a slim ~30px bar [MEDIUM | visual | Board]
- KanbanFlow reference (fresh capture 2026-09-30): "dark charcoal/black bar (~30px)."
- ChipFlow behavior: `.topbar { padding: 0.6rem 1.2rem; }` with a 2rem (32px) avatar in the flow — total bar height ~50px+. Eats vertical space and feels heavier than KanbanFlow's slim strip.
- Evidence: static/style.css lines 38-48 vs live KanbanFlow demo board captured 2026-09-30.
- Not a duplicate: no existing defect covers topbar height.

### KF-170 — Cards too spacious (padding + 8px gaps); KanbanFlow stacks cards tightly [MEDIUM | visual | Board]
- KanbanFlow reference (fresh capture 2026-09-30): "Cards are compact and stack tightly edge-to-edge... dense, utilitarian... maximal information density."
- ChipFlow behavior: `.task-card { padding: 0.45rem 0.55rem; }` (~7px/9px), `.task-list { gap: 0.5rem; }` (8px between cards), `.board-cell { padding: 0.5rem; }`. The board feels airy where KanbanFlow is dense.
- Evidence: static/style.css lines 476-481, 388-393, 373-380 vs live KanbanFlow demo board captured 2026-09-30.
- Not a duplicate: no existing defect covers card density/spacing.

### KF-171 — Over-rounded UI chrome (6px cards, 8px buttons, pill toolbar); KanbanFlow is minimally rounded [LOW | visual | Board]
- KanbanFlow reference (fresh capture 2026-09-30): "No large display type, no rounded 'pill' UI... slightly rounded corners" on cards only.
- ChipFlow behavior: cards 6px radius, `.toolbar-btn { border-radius: 8px; }`, pill-style toolbar buttons. Reads as a "modern rounded web app" rather than KanbanFlow's flat utilitarian chrome.
- Evidence: static/style.css vs live KanbanFlow demo board captured 2026-09-30.
- Not a duplicate: no existing defect covers chrome roundness.

### KF-172 — Persistent left Boards sidebar; KanbanFlow has no sidebar (board fills width) [CRITICAL | visual | Board]
- KanbanFlow reference (video frames f001/f010/f030, 2026-09-30): The board fills the full viewport width. There is NO persistent left sidebar. The "Boards" button in the top bar opens a temporary overlay drawer, not a persistent sidebar.
- ChipFlow behavior: `<aside class="boards-sidebar">` (KF-088) is always visible, pushing the board right by ~190px. This is the single largest structural deviation from KanbanFlow.
- Evidence: /tmp/kf-f001.jpg, /tmp/kf-f010.jpg vs /home/hatch/workspace/cf-board2.png (2026-09-30).
- Not a duplicate: KF-088 ADDED the sidebar based on a misreading; this defect corrects it.

### KF-173 — Light board toolbar with Invite/Timer/Filter/Menu; KanbanFlow has minimal board header [HIGH | visual | Board]
- KanbanFlow reference (video frames f001/f010, 2026-09-30): Below the dark top bar is a minimal light bar with just the board name "General" and a small icon. There is NO toolbar with Invite, Timer, Filter, Edit, Menu buttons. Those actions live in the dark top bar.
- ChipFlow behavior: Light toolbar with "General" title, avatar, "+ Invite" button, timer pill, filter button, edit button, "Menu" button. This adds a whole extra chrome layer KanbanFlow doesn't have.
- Evidence: /tmp/kf-f001.jpg vs /home/hatch/workspace/cf-board2.png (2026-09-30).
- Not a duplicate: no existing defect covers the extra toolbar layer.

### KF-174 — Board tabs in topbar; KanbanFlow has just a "Boards" button [MEDIUM | visual | Board]
- KanbanFlow reference (video frames f001/f010, 2026-09-30): Dark topbar left side has a single "☰ Boards" button. No tabs.
- ChipFlow behavior: Board tabs "[☰ Boards] [General] [+]" — the "General" tab duplicates the board name in the light bar below, and the "+" is extra chrome.
- Evidence: /tmp/cf-v2.png (2026-09-30 00:32 CDT) vs /tmp/kf-f001.jpg.
- Not a duplicate: KF-023 added tabs as a feature; this defect corrects to KanbanFlow parity.

### KF-175 — Filter uses radio buttons; KanbanFlow uses dropdowns [MEDIUM | visual | Filter]
- KanbanFlow reference (video frame f105, 2026-09-30): Filter panel has User/Task/Date as DROPDOWNS ("Show all"). Compact.
- ChipFlow behavior: USER/COLOR/DATE sections use radio buttons (verbose, long scrolling list).
- Evidence: /tmp/kf-f105.jpg vs /tmp/cf-filter.png (2026-09-30).
- Not a duplicate: no existing defect covers filter control type.

### KF-176 — Filter missing "Remember filter" toggle and Bookmarks [MEDIUM | functional | Filter]
- KanbanFlow reference (video frame f105): Filter panel has "Remember filter" toggle and "Bookmarks (0)" at bottom.
- ChipFlow behavior: No remember-filter toggle visible; no bookmarks section.
- Evidence: /tmp/kf-f105.jpg vs /tmp/cf-filter.png.
- Not a duplicate: no existing defect covers these filter features.

### KF-177 — Timer popup cut off on right edge [MEDIUM | visual | Timer]
- KanbanFlow reference (video frame f030): Timer popup fully visible, positioned top-right with margin.
- ChipFlow behavior: Timer popup extends beyond viewport right edge, content cut off.
- Evidence: /tmp/cf-timer2.png (2026-09-30).
- Not a duplicate: no existing defect covers timer popup positioning.

### KF-176 — CORRECTION: Remember filter and Bookmarks exist [INVALID]
- Correction (2026-09-30): The "Remember filter" checkbox and "Bookmarks (0)" DO exist in the filter panel, at the bottom below the scroll viewport. The screenshot /tmp/cf-filter.png was scrolled to the top.
- Status: INVALID - not a defect.

### KF-178 — Board actions in dark topbar; KanbanFlow puts Invite/Timer/Filter/Edit/Menu in the gray board-header row [HIGH | visual | Board]
- KanbanFlow reference (live comparison 2026-09-30): two-row header — dark top bar, then a GRAY row with board name + member avatar + Invite (+) + Timer (clock icon) + Filter + Edit-board-layout (pencil) + Menu. The dark topbar does NOT carry these actions.
- ChipFlow behavior (build 943d79c3): the KF-173 fix removed the light toolbar and moved Invite/Timer pill/Filter/Edit/Menu into the dark topbar; the gray row carries only the board name.
- Evidence: battery Part C items 1–3 (2026-09-30). NOTE conflicting evidence: KF-173 was filed from video frames claiming actions live in the dark topbar; the 2026-09-29 live handoff (KF-132/KF-133: "Timer is a clock icon in the BOARD BAR", "Board bar: Invite (+), Timer (clock), Filter, Edit layout (pencil), Menu") agrees with this defect. Two live references beat the video-frame reading — KF-173's placement premise is superseded.
- Fix direction: restore the gray board-name row carrying board name + avatar + Invite + Timer pill + Filter + Edit layout + Menu; keep the dark topbar to Boards button / brand / user menu.

### KF-179 — Task cards full-width tinted; KanbanFlow cards are compact with color as a left-edge stripe [HIGH | visual | Cards]
- Status: CORRECTED 2026-09-30 — the "left-edge stripe" premise is CONTRADICTED by live element markup on the golden-master board: KanbanFlow cards are `task taskColor-orange taskBorderColor-orange` — full-card light tint + solid colored outer border (~1–2px, rounded corners), NOT a left-edge stripe. ChipFlow's tint+border presentation matches this; the stripe claim is dropped. Remaining open question: card compactness/width (the battery claimed "compact cards") — unverified; re-check against the golden master before filing anything further.
- KanbanFlow reference (live comparison 2026-09-30, SUPERSEDED for the stripe claim): compact cards, task color shown as a left-edge stripe, not a full-card tint.
- ChipFlow behavior: cards render full-width pale background tint + colored border (taskColor-* classes).
- Evidence: battery Part C item 5 (2026-09-30); card-detail capture 2026-09-30 (element markup).

### KF-180 — Board layout editor "Arrange columns and swimlanes here" area is empty; drag-reorder impossible [HIGH | functional | Columns]
- Repro: Menu → Board layout. The arrange area lists no existing columns/swimlanes, so drag-reorder cannot be performed. Only Move left/right in the column ⋮ context menu works.
- Evidence: battery Part D item D2 (2026-09-30).

### KF-181 — Add-column Position ignored ("At the beginning" appends last) [HIGH | functional | Columns]
- Repro: Menu → Board layout → + Add column → Name "X", Position "At the beginning" → Add. Result: the column appears LAST, not first.
- Evidence: battery Part D item D1 (2026-09-30).

### KF-182 — Column headers left-aligned name+count with "+" at right; KanbanFlow centers name, centered green "+", vertical separators [MEDIUM | visual | Columns]
- KanbanFlow reference (live comparison 2026-09-30): column name centered, a centered green "+" add button, vertical separators between columns.
- ChipFlow behavior: name+count left-aligned, green "+" at the header's right edge. (KF-168 centered header text but the "+" placement and separators still diverge.)
- Evidence: battery Part C item 4 (2026-09-30).

### KF-183 — Color legend bar always visible; KanbanFlow shows no legend by default (opt-in toggle) [MEDIUM | visual | Board]
- KanbanFlow reference (live comparison 2026-09-30): no color legend bar on the board by default; the legend is an opt-in menu toggle.
- ChipFlow behavior: the color legend bar always renders at the board bottom.
- Evidence: battery Part C item 6 (2026-09-30).

### KF-184 — Task delete uses native window.confirm() instead of the styled in-page confirmation dialog [MEDIUM | consistency | Cards]
- ChipFlow behavior: card context-menu Delete and task-modal Delete gate on native window.confirm() (static/app.js:501, :964). In automation environments the native dialog auto-dismisses so deletion silently does nothing (this is why the battery reported D3 "task deletion completely broken"); in real browsers the confirm appears and deletion works (server DELETE verified working, cf. KF-148).
- KanbanFlow parity / consistency: column delete (KF-047 pattern), swimlane delete (KF-160), and token revoke (KF-146) all use the styled in-page #confirm-dialog. Task delete is the only remaining native confirm.
- Fix direction: route task delete through showConfirmDialog like the others.

### KF-185 — Card does not re-render after its name is edited in the task modal until page reload [LOW | visual | Cards]
- Repro: open task modal, edit name, Ctrl+Enter (saves). The board card still shows the old name until reload.
- Evidence: battery Part D item D5 (2026-09-30).

### KF-186 — "Y" (manual time) and "E" (time estimate) shortcuts missing from the Keyboard shortcuts reference dialog [LOW | functional | Shortcuts]
- Both shortcuts work (Y opens "Add time manually", E opens "Add time estimate") but neither is documented in the shortcuts dialog.
- Evidence: battery Part D item D6 (2026-09-30).

### KF-187 — Icon glyphs render as tofu "☐" in the test browser (Boards button, Menu button, swimlane toggles, column ⋮) [LOW | visual | Board]
- Observation (2026-09-30 battery): ☰ (U+2630), ⋮ (U+22EE) and related glyphs rendered as empty boxes in the managed test browser.
- Likely a sandbox font-availability artifact, not a code defect: the glyphs are standard Unicode present in system fonts on real devices, and the page uses the normal system font stack. VERIFY ON A REAL DEVICE before treating as a code defect; if real, replace the entities with inline SVG icons.
- Evidence: battery Part C item 7 / D4 (2026-09-30).

### KF-188 — Comment/attachment/time-entry deletes still use native window.confirm() [LOW | consistency | Task modal]
- Same class as KF-184 (which covered task delete): modal sub-view deletes gate on native window.confirm(), which auto-dismisses under automation so deletion silently does nothing. KanbanFlow parity/consistency: route through the styled in-page #confirm-dialog like task/column/swimlane deletes.
- Call sites (static/app.js): deleteModalComment (comment delete), deleteModalAttachment (attachment delete), deleteTimeEntry (time-entry delete).
- Evidence: worker candidate report on the KF-184 fix batch (2026-09-30), verified by grep in the source tree.
- Fix direction: wrap each DELETE in showConfirmDialog like deleteCardTask/deleteModalTask.

### KF-189 — Cannot add attachments; "Add attachment" is inert [HIGH | broken | Task modal]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior (2026-09-30 battery, build 248ce1e, Part C): in the task modal, the "+ Add attachment" text is inert; Add → Attachment closes the menu and opens nothing.
- Consequence: no attachment can ever be added; attachment delete is untestable (battery Part A item 7).
- Evidence: verification battery completed 2026-09-30 07:09:44 CDT.
- Fix direction: wire the Add → Attachment menu entry to a working add-attachment dialog/flow; verify an attachment can be added and appears on the task.

### KF-190 — No board-creation UI; cannot create boards or instantiate templates [HIGH | missing | Board]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior (2026-09-30 battery, build 248ce1e, Part C/D): no board-creation UI anywhere — not in the Boards panel, not in any menu; /templates returns 404.
- Consequence: cannot create boards or instantiate templates (Pomodoro, Kanban basics); blocked the battery's Part D persistence setup (no Persist-Board could be created). "Save board as template" works ("Board saved as a template." toast), but there is no way to create a board FROM a template.
- Evidence: verification battery completed 2026-09-30 07:09:44 CDT.
- Fix direction: restore/add a board-creation entry point (Boards drawer/panel) offering the template picker including the built-in Pomodoro and Kanban basics templates; ensure the /templates route exists if the picker needs it.

### KF-191 — Task-card color style diverges: white + left stripe vs KanbanFlow full-card tint [HIGH | divergent | Cards]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior (2026-09-30 battery, build 248ce1e, Part A item 2 / Part B): cards render white with a 4px left-edge stripe (verified Red, Blue, Orange). Live KanbanFlow renders FULL-CARD color tints (verified: "Reference task" card entirely yellow #ffffe0).
- Contradiction note: the KF-179 "fix" (white + stripe) rested on the 2026-09-30 03:10 battery's one-line reading. Two independent sources agree full-card tint is correct: the 2026-09-28 live-account review of KanbanFlow's own CSS (taskColor-{value} sets the card background; taskBorderColor-{value} the border — see fidelity-status.md Discovered behaviors), and this battery's live observation. KF-179 was a mis-fix; this defect reverts it.
- Evidence: verification battery completed 2026-09-30 07:09:44 CDT.
- Fix direction: cards render the task color as the full card background (taskColor-{value}) with the colored border (taskBorderColor-{value}); leave color-picker dots and modal tint unchanged.

### KF-192 — Column-header "+" centered vs KanbanFlow right-edge [MEDIUM | divergent | Columns]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior (2026-09-30 battery, build 248ce1e, Part A item 5 / Part B): headers have centered name, centered green "+", vertical separators. Live KanbanFlow places the green "+" at the RIGHT EDGE of the header, not centered. The KF-182 fix got the "+" position wrong.
- Evidence: verification battery completed 2026-09-30 07:09:44 CDT.
- Fix direction: move the column-header add-task "+" to the right edge of the header (keep centered name + separators).

### KF-193 — Column task count stale after card context-menu delete [MEDIUM | broken | Columns]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior (2026-09-30 battery, build 248ce1e): after deleting a card via the card context menu, the column header still showed "Do today 2" with 1 card remaining; the count corrected only after dialog close/re-render.
- Evidence: verification battery completed 2026-09-30 07:09:44 CDT.
- Fix direction: re-render/update the column header count when a card is deleted via the context menu, matching the modal-delete path.

### KF-194 — Stopwatch stop unreliable: closing the "Why did you stop?" dialog leaves the timer running [HIGH | broken | Timer]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior (2026-09-30 battery, build 248ce1e, Part C): after stopping the stopwatch, the "Why did you stop?" interruption dialog appeared; closing it without choosing a reason left the timer running (header pill showed 0:42 while the popup said session 0:00); a reason had to be selected to fully stop.
- Evidence: verification battery completed 2026-09-30 07:09:44 CDT.
- Fix direction: make the stop deterministic — dismissing the why-stop dialog without a reason must still finalize the stop (discard or log consistently), and the header pill and popup must agree on timer state.

### KF-195 — No logout control; "Signed in as admin" is static text [LOW | missing | Header]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior (2026-09-30 battery, build 248ce1e, Part C): no logout UI found — "Signed in as admin" is not clickable; GET /logout → 405 (POST only, so the server does support logout).
- Design tension: logout UI was intentionally removed under KF-033; this battery files its absence as a parity gap (KanbanFlow has an account menu with sign-out).
- Evidence: verification battery completed 2026-09-30 07:09:44 CDT.
- Fix direction: make the account control open a menu (or add a control) with a working sign-out that POSTs /logout.

### KF-196 — Hidden Boards-panel DOM nodes linger, causing spurious "obscured" actionability errors [LOW | broken | Board]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior (2026-09-30 battery, build 248ce1e): hidden Boards-panel DOM nodes remain in the accessibility tree and intercept hits, producing spurious "obscured" actionability errors (e.g., blocked Boards button, Help button).
- Evidence: verification battery completed 2026-09-30 07:09:44 CDT.
- Fix direction: when the drawer/panel is hidden, remove it from hit-testing (display:none / visibility:hidden + aria-hidden, or detach from DOM) so it cannot obscure other controls.

### KF-197 — Timer pill doesn't tick live; only updates when clicked [HIGH | broken | Timer]
- Status: FIXED 2026-09-30 (W-timer)
- ChipFlow behavior: pomodoro/stopwatch timer pill in header shows static time (e.g. 24:19) and doesn't count down until clicked.
- Evidence: Chip's screenshots 2026-09-30 11:15 CDT — timer showed 24:19 frozen, updated only on click.
- Fix direction: timer pill must tick/update every second while running, matching KanbanFlow.

### KF-198 — Selected stop reason not shown in time log [HIGH | broken | Timer]
- Status: FIXED 2026-09-30 (W-timer)
- ChipFlow behavior: after stopping timer and selecting "Other" in the "Why did you stop?" dialog, the time log entry shows "senkwich 1m" with no reason displayed.
- Evidence: Chip's screenshots 2026-09-30 11:15 CDT — screenshot 3 shows "Other" selected, screenshot 4 shows the log entry without it.
- Fix direction: display the selected interruption reason in the time log entry, matching KanbanFlow.

### KF-199 — Card doesn't refresh after move while timer running [MEDIUM | broken | Board]
- Status: FIXED 2026-09-30 (W-board, merged 527ae68)
- ChipFlow behavior: moving a task to another column while its timer runs doesn't update the card position until the timer pill is clicked, then "the outline follows."
- Evidence: Chip's report 2026-09-30 11:15 CDT.
- Fix direction: card move must re-render immediately regardless of timer state.

### KF-200 — Canceled pomodoro shows tomato indicator [MEDIUM | divergent | Timer]
- Status: FIXED 2026-09-30 (W-timer)
- ChipFlow behavior: card shows "🍅 1m" after a pomodoro was canceled early. Chip: "since I canceled early there shouldn't be a tomato."
- Evidence: Chip's screenshot 5, 2026-09-30 11:15 CDT.
- Fix direction: only show the pomodoro tomato for completed pomodoros; canceled/interrupted sessions log time without the tomato, matching KanbanFlow.

### KF-201 — Collapsed swimlanes render as grid instead of hidden [HIGH | divergent | Board]
- Status: FIXED 2026-09-30 (W-board, merged 527ae68)
- ChipFlow behavior: empty/collapsed swimlanes ("Personal To-do", "Backlog") render as a vertical grid sidebar with labels rotated, separate from the main column area. In KanbanFlow these were folded/collapsed and the board was just the columns.
- Evidence: Chip's screenshots 2026-09-30 11:15 CDT; report "those sections were just empty and folded to be collapsed. You seem to have made this weird grid thing."
- Fix direction: empty swimlanes must collapse to thin bars (or hide) so the board shows only the columns, matching KanbanFlow's folded-swimlane behavior.

### KF-202 — Folded swimlanes must be vertical strips with rotated labels, not horizontal bars [HIGH | divergent | Board]
- Status: FIXED 2026-09-30 (watchdog code-triage vs origin/main 8cc826a: folded lanes render as `.folded-strip` side strips with `.swimlane-name { writing-mode: vertical-rl }`; templates/board.html + static/style.css)
- ChipFlow behavior: KF-201 fix renders empty/folded swimlanes as thin horizontal bars across the board.
- KanbanFlow behavior: folded swimlanes render as thin VERTICAL strips with rotated (vertical) text labels — "PERSONAL TO-DO" as a vertical strip between columns, "BACKLOG" as a vertical strip on the far right edge.
- Evidence: Chip's KanbanFlow screenshot 2026-09-30 12:16 CDT vs ChipFlow screenshot 12:15 CDT.
- Fix direction: change `.swimlane-row--collapsed` from horizontal bars to vertical strips with `writing-mode: vertical-rl` labels, positioned as side strips like KanbanFlow.

### KF-203 — Column headers lack gray background bar and right-edge count badges [MED | divergent | Board]
- Status: FIXED 2026-09-30 (watchdog code-triage vs origin/main 8cc826a: `.columnHeader { background: #eef0f2 }` + `.columnHeader-countBadge` absolute right; templates/board.html renders badge; app.js updateColumnCount refreshes it)
- KanbanFlow: column headers sit on a light gray background bar spanning the board width; each column has a small count badge ("0") on the right edge of its header cell.
- ChipFlow: column headers have no gray bar background; counts render inline with the name ("Work To-do 0").
- Evidence: Chip's KanbanFlow screenshot 2026-09-30 12:16 CDT vs ChipFlow screenshot 12:15 CDT.

### KF-204 — Menu button lacks "Menu" text label [LOW | divergent | Board]
- Status: FIXED 2026-09-30 (watchdog code-triage vs origin/main 8cc826a: templates/board.html renders `☰ Menu` text in #board-menu-btn)
- KanbanFlow: board header right side shows "☰ Menu" (hamburger icon + "Menu" text).
- ChipFlow: shows only a hamburger icon with no text.
- Evidence: Chip's KanbanFlow screenshot 2026-09-30 12:16 CDT vs ChipFlow screenshot 12:15 CDT.

### KF-205 — Missing bottom Pomodoro color legend bar [MED | divergent | Board]
- Status: FIXED 2026-09-30 (deployed in 4fa15398; legend visible in Chip's screenshot 2026-09-30 14:09 CDT showing all four Pomodoro segments). SUPERSEDED 2026-09-30 by KF-222: the legend is now KanbanFlow's actual mechanism — a per-board "Color legend" toggle rendering enabled colors with custom names (a board with pomodoro-renamed colors + toggle ON shows exactly the four Pomodoro segments).
- KanbanFlow: bottom of board shows a color legend bar ("1 Pomodoro" yellow, "2 Pomodori" green, "3 Pomodori" blue, ">3 Pomodori" pink/red).
- ChipFlow: no legend bar visible.
- Evidence: Chip's KanbanFlow screenshot 2026-09-30 12:16 CDT vs ChipFlow screenshot 12:15 CDT.

### KF-207 — PATCH task with empty color_id does not clear the color assignment [MED | divergent | Board]
- Status: FIXED 2026-09-30 (watchdog code-triage vs origin/main 8cc826a: routes.rs update_task maps empty string to color_id=Some(None); db.rs update_task sets slot to null on Some(None))
- ChipFlow behavior: `PATCH /api/tasks/{id}` with `{"color_id":""}` leaves the task's color unchanged (card keeps `taskColor-green taskBorderColor-green`, `data-color-value="green"` in both the PATCH response fragment and the board HTML).
- Expected behavior: per the OpenAPI schema for UpdateTaskInput, "empty string clears the assignment (back to the legacy size-based coloring)."
- Evidence: gate-8 verification 2026-09-30 12:2x CDT on test instance; task dc2b3cb3-4a73-465f-ac6d-3652d9749297 on Pomodoro-template board. Set yellow on create (card showed taskColor-yellow), PATCHed to green (card showed taskColor-green), PATCHed with empty string (card STILL showed taskColor-green in fragment and board HTML).
- Fix direction: treat empty-string color_id in the PATCH handler as a clear (set color slot to null) instead of ignoring it.

### KF-206 — Green "+" buttons do nothing when all swimlanes are collapsed [HIGH | functional | Board]
- Status: FIXED 2026-09-30 (watchdog code-triage vs origin/main 8cc826a: initAddTask has no early return; KF-208 floating popup submits via fetch POST /api/tasks and unhides/inserts into the column task-list; folded strips render hidden .task-list divs as fallback)
- ChipFlow behavior: clicking the green "+" in a column header does nothing when all swimlanes are collapsed/empty.
- Root cause: `initAddTask()` in `static/app.js` does `document.querySelector('.task-list[data-column-id="..."]')` and returns early if null. Collapsed swimlane rows (KF-201) render as `<tr><th colspan>` with NO `<td>` cells and NO `.task-list` divs. When all swimlanes are collapsed, the page has zero `.task-list` elements, so the handler exits silently.
- Fix direction: when no `.task-list` exists for the column, fall back to creating the task via the API (fetch POST) using the first available swimlane, or ensure collapsed rows still render hidden task-list containers.
- Evidence: Chip 2026-09-30 12:17 CDT "green plus buttons do nothing now".

### KF-208 — Column "+" quick-add popup opens out of view [HIGH | fixed | Board]
- Status: FIXED 2026-09-30, deployed to production as build 8cc826a (verified live 14:45 CDT)
- ChipFlow behavior: clicking the green "+" in a column header clones the add-task form into the first `.task-list` in DOM order for that column. With folded swimlane strips (KF-202), that task-list lives inside the narrow left folded strip, so the form renders squeezed and half-obscured out of view — the "Task name" popup is cut off at the left edge of the board.
- KanbanFlow behavior: quick-add opens as a floating popup anchored near the clicked "+", fully within the viewport.
- Evidence: Chip's screenshot 2026-09-30 14:09 CDT ("green plus makes the popup show somewhere out of view").
- Fix: the add-task form now renders as a floating `position: fixed` popup anchored under the clicked "+" button, clamped to the viewport; submits via fetch POST /api/tasks (urlencoded — parse_body rejects multipart) and inserts the returned card HTML into the column's task-list; updates the count (also fixed `updateColumnCount` to refresh `.columnHeader-countBadge` on non-WIP columns); closes on Add/Cancel/Escape/outside click; toggle on re-click.
- Verification 2026-09-30 ~14:35 CDT (Playwright, local :3100): popup at (412,132) 272x106 fully in view at 1600x900; submit created the card in the column, header badge went 3->4, popup closed. Screenshots: /tmp/kf208-popup.png, /tmp/kf210-full.png.

### KF-209 — Floating popups are not draggable [MED | fixed | Board]
- Status: FIXED 2026-09-30, deployed to production as build 8cc826a (verified live 14:45 CDT)
- ChipFlow behavior: the quick-add popup cannot be moved; it stays where it opened.
- Expected behavior: Chip — "it would be great to be able to drag the popups" (KanbanFlow parity).
- Fix: reusable `makeDraggable` helper (drag by any non-interactive popup area, clamped to viewport) applied to the quick-add popup. Also fixed a real drag bug found in testing: releasing a drag/text-selection outside the popup fired a click on the common ancestor (body), which the outside-click dismiss treated as "click away" and removed the popup mid-drag — the dismiss handler now ignores clicks whose mousedown was >6px away (i.e. drags).
- Verification 2026-09-30 ~14:35 CDT: dragged popup from (412,132) to (632,292); popup survived and stayed in view. Screenshot: /tmp/kf209-dragged.png.

### KF-210 — Pomodoro legend floats above the bottom instead of anchored flush [MED | fixed | Board]
- Status: FIXED 2026-09-30 (verified in local browser battery, not yet deployed)
- ChipFlow behavior: the color legend had `margin-top: auto` but sat above a band of empty gray — `.board-wrap` kept `padding-bottom: 2rem` and the legend itself had bottom padding, so the colored segments never sat flush at the board's bottom edge; the legend also scrolled away with content instead of staying pinned.
- KanbanFlow behavior: the legend bar is anchored flush to the bottom of the board and stays visible.
- Evidence: Chip's screenshot 2026-09-30 14:09 CDT ("the colors should be anchored to the bottom like with the kanbanflow tool").
- Fix: `.board-wrap` padding-bottom 0, legend padding-bottom 0, legend `position: sticky; bottom: 0` with opaque background and z-index 6. Additionally gave `.board-wrap` `max-height: calc(100vh - 8rem)` so tall boards scroll *inside* the wrap (previously the page scrolled and the legend rode the content down) — header stays pinned at top, legend pinned at bottom while cards scroll, matching KanbanFlow.
- Verification 2026-09-30 ~14:40 CDT: short board legend bottom == wrap bottom (gap 0px); tall board (25 cards) scrolled mid-way: header row pinned at wrap top, legend pinned at wrap bottom (809-855 == wrap bottom 855); document does not scroll. Screenshots: /tmp/kf210-full.png, /tmp/kf210-mid.png, /tmp/kf210-scrolled2.png.

### KF-211 — Board columns are fixed 280px; KanbanFlow columns are fluid and fill the viewport [MED-HIGH | fixed | Board]
- Status: FIXED 2026-09-30 (verified in 3 browser rounds; final round: 1100px viewport → table computed min-width 1270px, all columns exactly 280px, wrap scrolls horizontally; 1600px → columns 347px equal share, zero dead space edge to edge. Report: .dev/evidence/verifier-2026-09-30-kf211-213c.md)
- ChipFlow behavior (DOM-measured): `.columnHeader` th width = 280px exactly; `.board-table` = 1270px wide on a 1600px viewport. The BACKLOG strip ends at ~x=1289; the remaining ~290px on the right is dead gray space. Same at 1366x768 (table 1264px — still fixed, still not filling).
- KanbanFlow behavior: 4 columns + PERSONAL TO-DO/BACKLOG strips fill the window edge to edge (~780px/column at 2880 wide; ~435px/column scaled to 1600) with zero dead space and no horizontal scrollbar.
- Evidence: `.dev/evidence/visual-2026-09-30.md`, screenshots `cf-overview-1600.png`, `cf-overview-1366.png`. Reference: dispositive screenshot (live KanbanFlow unreachable from VM, ERR_EMPTY_RESPONSE).
- Fix direction: make board columns fluid (flex grow / table-layout auto with min-widths) so the table fills the viewport width edge to edge; keep folded strips narrow.

### KF-212 — Pomodoro legend is not full-bleed [MED | fixed | Board]
- Status: FIXED 2026-09-30 (verified: legend x=0 to viewport edge at 1600 and 1366, 0px gaps; KF-210 bottom pin intact. Report: .dev/evidence/verifier-2026-09-30-kf211-213.md)
- ChipFlow behavior (DOM-measured): `FOOTER.color-legend` x=19, w=1334 on a 1600 viewport — ends at x=1353, leaving a ~247px gray gap on the right (19px inset left). KF-210 fixed the vertical anchoring (legend is flush at the bottom); this is the horizontal extent only.
- KanbanFlow behavior: the legend spans the full window width, edge to edge.
- Evidence: `.dev/evidence/visual-2026-09-30.md`, screenshot `cf-overview-1600.png`.
- Fix direction: legend spans the full board width (0 inset), edge to edge.

### KF-213 — Top-bar controls lack button wells [LOW | fixed | Board]
- Status: FIXED 2026-09-30 (verified: .boards-btn and bell/?/avatar wells compute #1f2937 with 6px radius; duplicate .boards-btn rules merged after KF-174's background:none was found overriding. Report: .dev/evidence/verifier-2026-09-30-kf211-213b.md)
- ChipFlow behavior: "☰ Boards" is bare text; bell / "?" / avatar are bare icons with no container.
- KanbanFlow behavior: "☰ Boards" sits inside a dark rounded button; bell / "?" / avatar sit in rounded-square button wells.
- Evidence: `.dev/evidence/visual-2026-09-30.md`, screenshots `cmp-kf-topbar.png` vs `cmp-cf-topbar.png`. (KF-023/030/031 cover control presence; this is styling only.)
- Fix direction: wrap top-bar controls in dark rounded button wells matching KanbanFlow.

### KF-214 — No UI affordance to clear a task's color [MED | closed | Board]
- Status: CLOSED 2026-09-30 — parity investigation found KanbanFlow ALSO offers no color-clear UI: its color picker (task editor → color button) shows exactly the 8 palette swatches with no "No color"/"Clear"/"None" option, and right-clicking a card opens no color submenu at all. ChipFlow's API-level clear support exceeds KanbanFlow; adding a "No color" entry would be a deviation, not parity. No change needed.
- ChipFlow behavior: API `PATCH /api/tasks/:id` with `{"color_id": ""}` clears correctly (HTTP 200, `color_id: null` on subsequent GET; `{"color_id": null}` also clears). The card context-menu Color submenu lists 10 colors with no clear entry, and the task edit modal exposes no color controls.
- KanbanFlow behavior: CONFIRMED 2026-09-30 on the golden-master board — no clear-color UI exists (8-swatch picker only).
- Evidence: `.dev/evidence/functional-2026-09-30.md`, screenshot `func-color-submenu.png`; golden-master color-picker screenshot.
- OPEN QUESTION (needs a dedicated check): the KanbanFlow worker reported right-clicking a card opens NOTHING — if true, ChipFlow's whole card right-click context menu (Color/Move/etc.) may be a deviation. Verify against the video catalog and live board before acting.

### KF-215 — Filter panel covers its own toggle button [LOW | FIXED | Board]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior: open `#filter-panel` (300px wide, x=1300+) overlays `#filter-btn` (x~1426), so the button can't toggle it closed while open — closing requires x or Escape.
- KanbanFlow behavior: CONFIRMED 2026-09-30 on the golden-master board — the filter is a RIGHT-DOCKED sidebar (~245px wide, full viewport height starting just below the top black header bar, y≈36) and the Filter funnel toggle stays fully visible and clickable in the top toolbar (~x=1514, y≈47); the panel never covers it. ChipFlow's floating 300px panel overlaying the toggle is a real deviation.
- Evidence: `.dev/evidence/functional-2026-09-30.md`, screenshot `func-filter-open.png`; golden-master filter-panel screenshot.
- Fix direction: dock the filter panel to the right edge below the top bar (like KanbanFlow) instead of floating it over the toggle.

### KF-216 — Hour-based time estimates not expressible [MED-HIGH | FIXED | Task modal]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior: the estimate field is pomodori size 1–4 only; there is no field or endpoint for hour-based estimates, so the fixture's 4h/6h/3h/8h/8h estimates could not be mirrored (left at default, not faked).
- KanbanFlow behavior: task time estimates are entered in hours (golden master: tasks 1, 6, 9, 10, 11).
- Evidence: `.dev/evidence/golden-master-mirror-2026-09-30.md` (P1).
- Fix direction: generic parity requires hour-based estimates; reconcile with Chip's Pomodoro sizing afterward.

### KF-217 — Filter Color dropdown shows hardcoded pomodoro options, not the board's palette [MED | FIXED | Filter]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior: Filter → Color offers 4 hardcoded pomodoro options; purple/orange/cyan/magenta cards are unfilterable even when the board palette contains them.
- KanbanFlow behavior: filter Color section lists the board's palette verbatim (golden-master reference: Show all, Yellow, Green, Blue, Red, Orange, Purple, Magenta, Cyan — see tracker line 756).
- Evidence: `.dev/evidence/golden-master-mirror-2026-09-30.md` (P3).

### KF-218 — No column-level "N overdue task(s)" indicator [LOW-MED | FIXED | Board]
- Status: FIXED 2026-09-30 — independent verifier passed 9/9 functional checks (singular/plural text, exact tooltips "Overdue tasks: {n} / Total tasks: {t}", no indicator on clean/empty columns, red bold centered block styling, no new overdue logic — reuses col.overdue_count). Evidence: .dev/evidence/verifier-2026-09-30-kf218-225.md.
- ChipFlow behavior: the To Do column header shows only the count badge ("5"); no overdue indicator.
- KanbanFlow behavior: To Do shows a "1 overdue task" indicator under/beside the header (golden-master board, task 7 due Sep 25).
- Evidence: `.dev/evidence/golden-master-mirror-2026-09-30.md` (O1).

### KF-219 — Card flagged overdue before its due time [MED | FIXED | Cards]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- ChipFlow behavior: the Sep-30 5:00 PM due card ("Implement dark mode toggle") already carried `card-due-overdue` at ~15:45 CDT, hours before its due time.
- KanbanFlow behavior: the golden-master board shows exactly 1 overdue task (the Sep-25 one); the Sep-30 card is not flagged overdue.
- Evidence: `.dev/evidence/golden-master-mirror-2026-09-30.md` (O2).
- Fix direction: overdue flagging should compare against the full due timestamp, not the due date.

### KF-220 — agents.md documents wrong column-move schema [LOW | FIXED | Docs]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- agents.md says column move takes `{"to_index":2}`; the real schema requires `{"position":N}` (the generated OpenAPI annotations are correct).
- Evidence: `.dev/evidence/golden-master-mirror-2026-09-30.md` (D1).
- Fix direction: correct agents.md (and audit agents/skill.md + agents.json for the same error).

### KF-221 — REST API ergonomics gaps found during golden-master mirroring [LOW | FIXED | API]
- Status: FIXED 2026-09-30 (W1/W2/W3 repair batch, merged to master)
- Gaps: (a) `POST /api/tasks` returns an HTML card fragment — the id must be parsed from `data-task-id`; (b) no JSON read endpoint for comments (must scrape `/api/tasks/{id}/modal`); (c) columns list returns only `{id,name}` — no `wip_limit`/`is_done`; (d) login Set-Cookie carries `Secure`, so cookie auth over plain http silently drops it; (e) comments created via API are attributed to `api-token` with no way to attribute to a user.
- Evidence: `.dev/evidence/golden-master-mirror-2026-09-30.md` (D2–D5, P4).
- Note: the mirror worker's P2 (legend hardcoded to 4 pomodoro segments) is NOT filed separately — it is entangled with KF-205/KF-121 (both divergent, based on Chip's customized board). The visual verifier must first establish what a *generic* KanbanFlow board's footer actually shows on the golden-master board, then decide.

### KF-222 — Pomodoro legend footer renders on generic boards; KanbanFlow shows no footer [HIGH | FIXED | Board]
- Status: FIXED 2026-09-30 — implementation passed independent verification (diff review, full check suite green, functional tests on :3106: new board → no footer, toggle ON → labeled segments, rename/disable reflected, OFF → zero markup, survives restart, migration backfill exactly-once). Deployed in `64d017ed754f2da1d3322d5742c6923efde8278f`; `/api/v1/version` confirms the new SHA locally on green-box and via the public URL. (Deploy report `.dev/evidence/deploy-2026-09-30-kf222.md` uncommitted at release time; will fold into the next commit.)
- ChipFlow behavior: every board renders the full-width Pomodoro legend strip ("1 Pomodoro | 2 Pomodori | 3 Pomodori | >3 Pomodori") pinned at the bottom.
- KanbanFlow behavior: TRIGGER FOUND 2026-09-30. The footer is controlled by a "Color legend" toggle in the board's Menu (top-right toolbar of the board view). When ON, a full-width footer strip renders one labeled segment per board-ENABLED color, using each color's custom name; when OFF, `#board-footer` stays empty (`colorLegendVisible=false`). Chip's General board (UTTNB5) has it ON with renamed colors — Yellow→"1 Pomodoro" (also board default), Green→"2 Pomodori", Blue→"3 Pomodori", Red→">3 Pomodori" (Brown/Cyan/Magenta/Orange/Purple/White disabled) — which is exactly where his "1 Pomodoro / 2 Pomodori / 3 Pomodori / >3 Pomodori" footer comes from. Toggling it ON on the golden-master test board immediately produced an 8-segment footer with default color names; toggle alone is sufficient, no custom colors needed. (Board Settings > General/Advanced have no legend options; footer colors are the board palette entries, not column colors.)
- Evidence: `.dev/evidence/visual-compare-golden-2026-09-30.md`, `vcompare-board-1600.png` vs KanbanFlow footer close-up; legend-trigger browser investigation 2026-09-30 (screenshots of both boards' footers + Menu toggle).
- Fix direction: add a per-board `color_legend` boolean (default OFF for new boards); put a "Color legend" toggle in the board Menu (≡ Menu, board-bar far right); when ON render the footer as one labeled segment per enabled board color in palette order using custom names (keep KF-210 bottom-anchored + KF-212 full-bleed); when OFF render nothing. Migrate existing boards to ON (preserves their current rendering). Related: KF-205, KF-121 (both divergent, based on Chip's customized board — re-evaluate after this lands); KF-083 (default color), KF-084 (color rename dialog).

### KF-223 — Column menu contents differ from KanbanFlow [MED | FIXED | Columns]
- Status: FIXED 2026-09-30 — independent verifier passed (diff review, node --check, cargo fmt --check, cargo build, clippy 0 warnings, cargo test 56/56; functional tests on :3108 with fresh DB: 34/34 — menu exactly Edit/Collapse/Show details, popover position/content/dismiss, collapse → 22px white strip, red badge only when WIP exceeded, red vertical overdue indicator with exact tooltips, uppercase vertical name, click-anywhere-to-expand, collapse label swaps to "Expand", persistence across reload AND full server restart; regression: ≡ board Menu, KF-222 legend toggle, Edit dialog all OK). Deployed in `7a9a465eb8af69ef61501f6761c3d31ab1720857`; `systemctl is-active chipflow.service` → active and green-box-local `/api/v1/version` reports the new SHA. (Deploy report `.dev/evidence/deploy-2026-09-30-kf223.md`.)
- ChipFlow behavior: column menu has 6 items — Edit / Move left / Move right / Add to left / Add to right / Delete.
- KanbanFlow behavior: column menu has 3 items — Edit / Collapse / Show details (live golden-master board, right-click menu screenshot).
- Evidence: `.dev/evidence/visual-compare-golden-2026-09-30.md`, `vcompare-colmenu.png`; live behaviors verified 2026-09-30 on the golden-master board (details-popover + collapsed-strip screenshots).
- "Show details" spec (verified live): small white popover directly below the column header, overlapping the top of the first card; title row bold "Column: {name}" with a Close (×) at top-right; body a single line "Task count: {n}"; dismissed via ×.
- "Collapse" spec (verified live): column becomes a narrow white vertical strip (~20–24px), full task-area height; count badge at top (RED when WIP exceeded, tooltip "Task count: N / Click to expand"; black/dark otherwise); column name vertical, rotated 90° CCW (reads bottom-to-top), UPPERCASE via CSS; collapsed strips also show the overdue indicator when applicable ("1 overdue task" + warning line, tooltip "Overdue tasks: 1 / Total tasks: 5 / Click to expand"); click anywhere on the strip expands it. (No column collapse exists in ChipFlow today — see also corrected KF-039/KF-040.)
- Fix direction: match the 3-item menu; "Collapse" needs a working column-collapse implementation (no reachable fold UI/API/JS exists today — treat as unimplemented); determine what "Show details" does on the live board before implementing.

### KF-224 — Cards lack KanbanFlow's icon row and inline subtasks [LOW-MED | FIXED | Cards]
- Status: FIXED 2026-09-30 in release `3116db5a` — independent verifier passed 17/17 behaviors (icon row order/tooltips/omission, due-line markup, due_done round-trip, inline subtasks undone-first + `2 hidden (2 done)`, section toggles + localStorage persistence, label clicks don't open modal, rendered screenshot visually identical to golden-master reference); static checks green (node --check, fmt --check, build, clippy 0 warnings, 77/77 tests); deployed to green-box 18:25 CDT, `/api/v1/version` confirms build_sha `3116db5abebb9d5be750f2481458c184589c247f`. Deploy report: `.dev/evidence/deploy-2026-09-30-kf224.md`.
- ChipFlow behavior: cards show only a due chip + assignee avatar + logged time.
- KanbanFlow behavior: cards show an icon row (description / due date / subtasks / comments / time), a "Due: Friday 5:00 PM (Done)" line, and inline subtask checkboxes with a "2 hidden (2 done)" summary.
- FULL SPEC (verified live 2026-09-30 on the golden-master board, "Build payment webhook handler" card — authoritative):
  1. Icon row (top): 5 dark-gray glyphs + time text, avatar right-aligned. Description icon (document w/ lines, tooltip "Has description. Click to expand or collapse."), due-dates icon (calendar grid, "Has due dates. Click to expand or collapse."), subtasks icon (checklist, "Has subtasks. Click to expand or collapse."), comments icon (speech bubble, "Has comments. Click to view comments."), clock icon + "2h 30m / 8h" (title "Time spent / Time estimate"). Each icon toggles expand/collapse of its section. Icons for features the card LACKS are absent entirely, never greyed out. Avatar: circle, thin orange ring, #aa5d00 fill, white "CS" initials.
  2. Due-date line: "Due: Friday 5:00 PM (Done)" — "Due:" regular weight, date bold, "(Done)" in `<span class="task-dueDateColumn">`; whole line bold dark orange-brown; dotted orange divider above. PARENTHETICAL RULE: "(Done)" means the due-date item itself was marked done, independent of card column (NOT overdue, NOT card-done). Near-term dates render as weekday ("Friday 5:00 PM"); farther dates absolute ("30 October 5:00 PM"). Only cards with the due-date section expanded show the line.
  3. Inline subtasks: `<ul class="subTasks">` — visible rows are empty square checkboxes (custom, hidden native input) + name; below, smaller gray summary "2 hidden (2 done)" (title "Click to expand subtasks"). Dotted orange divider between due dates and subtasks.
  4. Color presentation: FULL-CARD light tint + solid colored outer border (~1–2px, rounded corners); dotted section dividers tinted with the card color. (NOT a left-edge stripe — see corrected KF-179.)
- Evidence: `.dev/evidence/visual-compare-golden-2026-09-30.md`; card-detail capture 2026-09-30 (screenshot + element markup).

### KF-225 — Task modal dialog save leaves stale card markup on the board [LOW-MED | FIXED | Task modal]
- Status: FIXED 2026-09-30 — independent verifier passed 7/7 functional checks: race test (aborted post-save modal refresh fetch, saved due date, closed modal → automatic closeModal() reload re-rendered card "Due: 9 October 3:00 PM"), normal save and Clear paths both auto-reload with fresh card markup. `modalDirty = true` set synchronously in DueDateDialog.applyToSelected's success handler. Evidence: .dev/evidence/verifier-2026-09-30-kf218-225.md.
- ChipFlow behavior: the due-date dialog's `applyToSelected` success handler never sets `modalDirty`, so after any dialog save (due date edits, the new "Mark as done" checkbox, etc.) the background board card keeps stale markup until a full page reload.
- KanbanFlow behavior: (parity expectation) the board card reflects dialog edits immediately after save.
- Evidence: `.dev/evidence/verifier-2026-09-30-kf224.md` — PATCH succeeds and DB is correct; only the rendered card is stale.
- Fix direction: set `modalDirty = true` in `applyToSelected`'s success handler (one-line, suggested by the verifier).

### KF-226 — POST /api/tasks silently drops due_at/due_repeat [LOW-MED | FIXED | API]
- Status: FIXED 2026-09-30 — independent verifier passed: POST with "2026-10-05 17:00" + due_repeat stored/round-tripped correctly; null/null and blank/"" → no due date; bare date → end-of-day. `CreateTaskInput` gained optional `due_at`/`due_repeat` applied via `db.set_task_due`. agents.md + agents/skill.md document the new fields; openapi.json regenerates with nullable types. Evidence: .dev/evidence/verifier-2026-09-30-kf226-227.md.
- ChipFlow behavior: `POST /api/tasks` accepts `{"due_at": ..., "due_repeat": ...}` without error (200) but creates the task with no due date; the caller must issue a separate PATCH afterward.
- Expected behavior: either accept the fields on create (parity with the update path) or reject with 400 listing the unsupported fields — never a silent 200 drop.
- Fix direction: add optional `due_at`/`due_repeat` to CreateTaskInput and apply them at creation, or add `#[serde(deny_unknown_fields)]` and document the PATCH-after-create requirement.

### KF-227 — Due-date dialog Clear is a server-side no-op [LOW-MED | FIXED | Task modal]
- Status: FIXED 2026-09-30 — independent verifier passed: PATCH {"due_at":null,"due_repeat":null} clears both (verified via GET detail); absent fields unchanged; "" still clears (legacy); {"due_at":null} keeps due_repeat; clearing resets due_done true→false. End-to-end browser: task modal → due dialog → Clear → card lost its due-date line, API confirms nulls. Server now uses a `de_opt_opt` serde helper (missing→None, null→Some(None), value→Some(Some(v))) on both fields. Evidence: .dev/evidence/verifier-2026-09-30-kf226-227.md.
- ChipFlow behavior: the due-date dialog's Clear button sends `PATCH {"due_at": null, "due_repeat": null}`; `UpdateTaskInput.due_at: Option<String>` treats JSON null as absent, so the clear is a silent no-op (200, due date unchanged). The KF-225 auto-reload fires, but the re-rendered card still shows the old due date.
- Expected behavior: Clear actually clears the due date (KanbanFlow parity: removing a due date removes it).
- Fix direction: distinguish "absent" from "explicit null" on the server (e.g. `Option<Option<String>>` for due_at/due_repeat in UpdateTaskInput: null → clear, absent → unchanged) or have the client send "" (the verifier confirmed "" clears today). Related: KF-226 (accept due fields on POST /api/tasks).

### KF-228 — PATCH {"wip_limit": null} does not clear the column WIP limit [LOW-MED | FIXED | Columns]
- Status: FIXED 2026-09-30 — independent verifier passed 6/6 functional checks: PATCH {"wip_limit":5} → 5; PATCH {"wip_limit":null} → cleared (the fix); absent → unchanged; {"wip_limit":7} → 7; {"wip_limit":0} → 400 "WIP limit must be at least 1"; KF-227 regression (PATCH {"due_at":null} clears). Server reuses the KF-227 `de_opt_opt` serde helper (now generic) on `UpdateColumnInput.wip_limit`; doc comment corrected. Evidence: .dev/evidence/verifier-2026-09-30-kf228.md.
- ChipFlow behavior: `UpdateColumnInput.wip_limit: Option<Option<i64>>` lacks a `deserialize_with`, so stock serde collapses JSON null to absent (unchanged); the field's doc comment and the OpenAPI description claim "JSON null clears the limit", which is false.
- Expected behavior: either null actually clears the limit (same `de_opt_opt` pattern as KF-227) or the docs/schema stop claiming it does.
- Fix direction: apply the KF-227 `de_opt_opt` deserializer to `UpdateColumnInput.wip_limit` and add the clear arm in the column PATCH handler; keep the doc/OpenAPI text accurate.

### KF-229 — Timer pill never shows countdown or play/stop glyph [MEDIUM | FIXED | Timer]
- Status: FIXED 2026-10-01 02:31 UTC — released 1e8441e2b8a660271d7709c2f88e48e7f05c9463, deployed to green-box, /api/v1/version verified.
- Status: OPEN — filed 2026-10-01 from the final golden-master battery (browser-verified on the deployed tree, build c3307b7).
- ChipFlow behavior: `TimerUI.updatePill` (static/app.js ~2389) writes the countdown text and play/stop glyph into `#timer-pill-icon` / `#timer-pill-time` child elements, but KF-209 replaced the pill's contents with a bare clock SVG (KanbanFlow parity) — those child elements no longer exist in templates/board.html. The writes are null-guarded so they silently do nothing: the pill never shows a countdown and never swaps play/stop while a session runs.
- KanbanFlow behavior: the header pill shows the live countdown (v3-e022, v9-e002).
- Not a total loss: the running card gets its outline + live badge and the tab title counts down, so timer state is still visible elsewhere — the board-bar pill alone is dead.
- Fix direction: rewrite `updatePill` to target the SVG (e.g. swap the icon path for play/stop states and/or render the remaining time as text next to the clock icon, KanbanFlow-parity), or restore the `#timer-pill-icon`/`#timer-pill-time` spans inside the pill.
- Evidence: `.dev/evidence/final-battery-20261001.md` (timer section).

### KF-230 — Column delete is unreachable in the UI; deleteColumn() is dead code [MEDIUM | FIXED | Columns]
- Status: OPEN — filed 2026-10-01 from the final golden-master battery (browser-verified on the deployed tree, build c3307b7).
- ChipFlow behavior: the column menu (right-click, KF-038/KF-223) offers exactly three items — Edit, Collapse, Show details. There is no Delete item; the layout-edit view offers only Add column / Add swimlane / drag-reorder; and `deleteColumn()` (static/app.js:814, with the KF-047 styled confirm dialog and the "server will refuse until tasks are moved" message) is never called from anywhere — grep over static/app.js and templates/ finds zero callers. The server-side `DELETE /api/columns/:id` endpoint exists and works, but no UI entry point reaches it, so a column created in the UI can never be deleted in the UI.
- KanbanFlow behavior: the column menu includes "Delete column" (with confirmation when the column holds tasks).
- Fix direction: add a Delete item to the column menu (wiring the existing `deleteColumn`, which already handles the non-empty-column confirm copy) or add a delete affordance to the layout-edit view's column list. Also decide what happens to the vestigial hidden ⋮ button (`button.columnHeader-menu`, display:none) — KF-038 removed it but the markup remains.
- Evidence: `.dev/evidence/final-battery-20261001.md` (column-delete section); screenshot `.dev/evidence/final-battery-column-menu.png`.
- Status: FIXED 2026-10-01T03:13:00Z — deployed 91230ec8e450c61bd9a74d60cd7f728000d94fdf, /api/v1/version build_sha verified on green-box port 3000. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261001-kf-230.md

### KF-231 — Timer chip minutes not zero-padded: "0:03" vs KanbanFlow's "00:02" [LOW | FIXED | Timer]
- Status: OPEN — filed 2026-10-01 by the KF-229 independent verifier (browser-verified on the KF-229 fix tree, commit 667b66d).
- ChipFlow behavior: a running stopwatch shows the chip counting up as "0:03", "0:04" — `TimerUI.fmt` (static/app.js:2362) zero-pads the minutes only when hours > 0 (`var mm = (h > 0 && m < 10 ? '0' : '') + m;`), so any sub-10-minute value renders as "M:SS".
- KanbanFlow behavior: the running stopwatch chip shows "00:02" (video evidence v9-e002: "the session appears as a running \"00:02\" chip in the top bar"; `.dev/kf-video-catalog/v9.md` line 19). Minutes are zero-padded to two digits.
- Scope note: pre-existing behavior, not introduced by KF-209 or the KF-229 fix; correctly out of KF-229's scope (KF-229 was the pill showing nothing at all). `fmt()` is shared by the pill, timer popup, tab title, and card live badge — changing it aligns all of them, but direct video evidence exists only for the chip; the builder should confirm the other surfaces against KanbanFlow before or while changing the shared formatter.
- Fix direction: in `TimerUI.fmt`, always zero-pad minutes (`(m < 10 ? '0' : '') + m`), yielding "00:03", "24:57", "1:05:03" — matching KanbanFlow's chip format for the observed cases.
- Evidence: `.dev/evidence/verifier-KF-229-20261001T0219Z.md` (stopwatch count-up check: chip text "0:03"); screenshot `.dev/evidence/verifier-KF-229-stopwatch.png`.
- Status: FIXED 2026-10-01T02:53:00Z — deployed 91230ec8e450c61bd9a74d60cd7f728000d94fdf, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261001-kf-231.md

### KF-232 — Timer pill keeps stale running state after UI stop (frozen "24:59", red stop icon, stale tab title) [LOW | FIXED | Timer]
- Status: OPEN — filed 2026-10-01 by the final golden-master battery (browser-verified on deployed build 91230ec8e450c61bd9a74d60cd7f728000d94fdf, local tree identical: `git diff 91230ec8 HEAD -- src/ static/ templates/` empty).
- ChipFlow behavior: start a pomodoro via the UI (task modal → Timer → Start Pomodoro), click the timer pill to open the popup, click Stop, then close the popup (click outside). The pill keeps class `running`, the red stop-square icon, the frozen text "24:59", and the tab title stays "24:59 ChipFlow Golden Master — ChipFlow". The underlying stop works — server `/api/timer/status` phase and `TimerUI.state.phase` are both `idle`, tick cleared.
- KanbanFlow behavior: after stopping a session the timer control returns to the idle state (green play icon, full "25:00" duration, normal tab title).
- Root cause: `TimerUI.renderPill` (static/app.js ~2377) early-returns while the timer popup is open (KF-097 hides the pill then), so the stop's `refresh()` → `updateFromStatus()` → `renderPill()` updates state but never repaints the label/icon. `closePopup()` unhides the pill without re-rendering it, and with the session idle `syncTick()` clears the 1s interval — so nothing ever repaints the pill until the next timer state change. Verified by tracing renderPill: last call after stop ran with phase=idle yet label stayed "24:59".
- Fix direction: call `renderPill()` in `closePopup()` after unhiding the pill (or move the KF-097 early-return to only skip the countdown text update, not the icon/label repaint).
- Severity: LOW (visual only; timer itself stops correctly). Repro: any UI stop via the pill popup.
- Evidence: `.dev/evidence/final-battery-v4d-stale-pill.png` (pill showing "24:59" + red stop icon while idle); trace log in the battery report `.dev/evidence/final-battery-20261001b.md`.
- Status: FIXED 2026-10-01T04:25:09Z — deployed 66f84da064dcd8a5c1948f8c6d5e965cfb8346bc, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261001-kf-232.md

### KF-233 — Boards button and top-bar colors differ (#111827/#1f2943 vs #333333/#515151; 31px vs ~47px tall) [MEDIUM | OPEN | Top bar]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: dark top bar bg `#111827`, 40px tall; "☰ Boards" button bg `#1f2943`, rect ≈ (16,4,99,31) CSS px at 1440w — a 31px button vertically centered with ~4px margins.
- KanbanFlow behavior: dark top bar bg `#333333`, 38px tall; "☰ Boards" button bg `#515151`, rect ≈ (11,5,113,47) CSS px — ~47px tall, filling the bar nearly edge-to-edge vertically (GM-001).
- Fix direction: restyle the bar bg to `#333333`, the button bg to `#515151`, and make the button fill the bar's height.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-001.md`; `.dev/evidence/kf-drafts/v1-evidence/cmp-GM-001.png` (zoomed side-by-side), `cf-1600.png`; golden `.dev/evidence/golden-masters/top-bar/GM-001-boards-button.png`.
- Status: FIXED 2026-10-02T02:48:30Z — deployed 8e0c5db1ac1f12803c4cfec3dc5048d9182d1f1a, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-233.md

### KF-234 — Member avatar is dark circle/white initials, not white circle + brown-initials disc [LOW | OPEN | Top bar]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: avatar is the initials "VE" in white directly on the dark button (no white circle); button bg dark navy.
- KanbanFlow behavior: avatar is a white circle containing a smaller brown circle with "CS" initials (white outer ring, brown inner disc) (GM-005).
- Fix direction: render the white-circle + brown-initials disc avatar instead of bare initials on the dark button.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-005.md`; `.dev/evidence/kf-drafts/v1-evidence/cmp-GM-005.png`, `cf-1600.png`; golden `.dev/evidence/golden-masters/top-bar/GM-005-member-avatar.png`.

### KF-235 — Bell icon is yellow emoji glyph, not white outline bell [LOW | OPEN | Top bar]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: notifications button shows a yellow 🔔 emoji glyph on the dark button (32×32 CSS px at (1302,4)).
- KanbanFlow behavior: notifications button shows a white outline bell icon on a `#505050` button (GM-007).
- Fix direction: replace the emoji with a white outline bell icon matching the master.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-007.md`; `.dev/evidence/kf-drafts/v1-evidence/cmp-GM-007.png`, `cf-1600.png`; golden `.dev/evidence/golden-masters/top-bar/GM-007-bell-icon.png`.

### KF-236 — Top bar has no chart/reports icon (15 destinations only via ☰ Menu → Reports) [MEDIUM | OPEN | Top bar]
- Status: FIXED 2026-10-02T03:03:46Z — deployed b1127ece6cb8c180cef21023295cd9e0b5c57968, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-236.md
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the top bar has no chart/reports icon at all (top bar = Boards / brand / bell / ? / account). The 15 destinations are reachable only via the board ☰ Menu → Reports submenu — and that submenu's order was verified verbatim-correct.
- KanbanFlow behavior: the top bar carries a chart icon whose dropdown lists the 15 report destinations: Pomodoro statistics, Time spent, Print, Board history, Burndown, Calendar, Cumulative flow, Cycle & lead time, Due date performance, Monte Carlo forecasting, Task count, Throughput, Time estimate, Time in column, Export (GM-128).
- Fix direction: add the chart icon to the top bar opening the reports dropdown; not a duplicate of KF-030/KF-031 (bell/help icons, which are present).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-128.md`.

### KF-237 — Board-tab row styling: gray `#e8eaed` vs `#f2f2f2`; avatar styling; invite "+" bare glyph vs white circle button [MEDIUM | OPEN | Board header row]
- Status: FIXED 2026-10-02T03:45:00Z — deployed f8a02bdda6509b94c5c712d799e5d86ef01f720d, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-237.md
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge of V1:KF-237 (GM-002/GM-015) + V1:KF-238 (GM-016).
- ChipFlow behavior: gray row bg `#e8eaed`; board name "General" bold black; member avatar is a dark-navy circle with white "VE" (no white ring); invite "+" (`button#invite-btn` at (134,46,27,29)) is a bare dark text glyph with no button chrome; dark bar above is `#111827`.
- KanbanFlow behavior: gray row bg `#f2f2f2`; member avatar is a white circle with brown "CS"; invite "+" is a white circle button containing a dark "+"; dark bar above is `#333333` (GM-002, GM-015, GM-016 — the latter a zoomed crop of the same invite control).
- Fix direction: restyle the gray row bg, avatar, and invite button chrome per the masters.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-002.md`, `.dev/evidence/kf-drafts/kf-draft-v1-GM-016.md`; `.dev/evidence/kf-drafts/v1-evidence/vs-GM-002.png`, `vs-GM-015.png`, `vs-GM-016.png`; candidate `cf-2880.png`.

### KF-238 — Menu hamburger is thin outline bars, not thick solid bars [LOW | OPEN | Board header row]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: `button#board-menu-btn` at (1348,46,72,29) shows three thin outline-style bars + "Menu" text.
- KanbanFlow behavior: three thick solid dark bars + bold "Menu" text (GM-020; true box (2688,56,2878,136)).
- Fix direction: redraw the hamburger as thick solid bars.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-020.md`; `.dev/evidence/kf-drafts/v1-evidence/vs-GM-020.png`; candidate `cf-2880.png`.

### KF-239 — Timer pill lives in gray board row as light pill, not in dark top bar as dark chip [MEDIUM | OPEN | Board header row]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: `button#timer-pill` at (1168,46,89,28) in the GRAY board-header row; light pill bg `#e8eaed`; pomodoro idle shows green ▶, dark "25:00", gray ▾; stopwatch idle shows red ■, "00:00", ▾.
- KanbanFlow behavior: the idle pill sits in the DARK TOP BAR at top-right — a dark chip with green ▶, "25:00", ⌄ (pomodoro) or red ■, "00:00", ⌄ (stopwatch) (GM-009, GM-010; KF-098 verified 2026-09-29).
- Fix direction: move the timer pill into the dark top bar and restyle it as a dark chip.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-009.md`; `.dev/evidence/kf-drafts/v1-evidence/cf-timerpill-3x.png`, `cf-swpill-3x.png`.

- Status: FIXED 2026-10-02T04:06:48Z — deployed 31da0a63ecb90c2089933210bcde06699ef63be6, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-239.md

### KF-240 — Boards drawer is white panel; selection highlight blue `#e0e7ff`, not green indicator [LOW | OPEN | Boards drawer]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: drawer is a WHITE panel (~440 CSS px wide) with header "Boards" + green "+ New board" button, search box ("Search boards" ✓), hint text "Drag to add to Favorites" ✓, "FAVORITES" and "ALL BOARDS" sections; the active board row is highlighted light INDIGO/BLUE `#e0e7ff`.
- KanbanFlow behavior: v8.md persistent context calls the drawer a "dark sidebar"; v3.md S1 records the "General" row selected "with a green indicator" (GM-022, GM-026).
- Fix direction: drawer styling and a green selection indicator; not a duplicate of KF-172/KF-174 (drawer existence/behavior, which passed).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-022.md`; `.dev/evidence/kf-drafts/v1-evidence/cf-drawer-left.png`.

### KF-241 — Folded-column WIP-violation styling wrong: black count, missing `#ff8080` line, spurious red "5" in strip body [HIGH | OPEN | Columns]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V2 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior (reproduced: "To Do" folded, `collapsed:true`, `wip_limit:3`, 5 tasks): (1) the violated strip-header count "5" renders in near-black/gray (107,114,128) — the unfolded header's "5 / 3" correctly renders darkred `#8B0000` (139,0,0), so the strip uses a different code path; (2) the red warning line under the strip header (`#ff8080` (255,128,128) spanning the full strip width in KF) is entirely absent; (3) a red "5" renders inside the strip body next to the vertical "TO DO" text — KanbanFlow has no such element.
- KanbanFlow behavior: violated strip count is DARKRED (same `#8B0000` family as the unfolded header), a `#ff8080` red line spans the strip width under the header, and the strip body carries only the vertical column name (GM-040).
- Fix direction: align the folded-strip WIP styling with the unfolded header's darkred treatment, add the full-width `#ff8080` line, and remove the spurious red "5" from the strip body.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v2-GM-040.md`; `.dev/evidence/kf-drafts/v2-evidence/g5zoom.png` (KanbanFlow) vs `c5zoom.png` (ChipFlow); `v2-evidence/boardb-final2.png`.
- Status: FIXED 2026-10-01T20:31:00Z — deployed 6089eab39ea298d2337fb7af99ea1aa9294d2c6b, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261001-kf-241.md

### KF-242 — Card icon row missing description and subtask icons (3 of 5) [MEDIUM | OPEN | Task cards]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V2 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior (task with description, due date, 2 subtasks, comment, 2.5h logged, one assignee): the attribute icon row shows only 3 icons — calendar, comment, clock.
- KanbanFlow behavior: the icon row shows 5 icons in order — description (document), calendar (due), subtask list, comment, clock (time). The description icon is missing even though the task has a description; the subtask icon is missing even though the task has 2 subtasks (the "2 hidden (2 done)" subtask section on the card still renders — only the icon is absent) (GM-069).
- Fix direction: render the description and subtask icons whenever the task has a description/subtasks, matching the 5-icon order.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v2-GM-069-icons.md`; `.dev/evidence/kf-drafts/v2-evidence/boardb-final2.png`; golden `.dev/evidence/golden-masters/task-cards/GM-069-card-due-subtasks-time-attributes.png`.
- Verify 2026-10-02 (independent blind verifier): FAILED — presence/order/conditional logic all pass (5 icons, GM-069 order, pitch 23.59px, partial-combo sweep clean, no regressions), but L2 pixel gate fails: icon boxes 14px (clock svg 13px) vs golden ~16px boxes / ~17px clock circle; time text also smaller (aligned-crop pixdiff: 1635 px, ratio 0.47, max delta 221). Resizing SVGs is in-scope size parity; remaining artwork-style gap (feather vs KF icon set, e.g. speech bubble missing interior dots) is out-of-scope glyph re-drawing — planner may need to rule on the L2 gate vs scope. Evidence: `.dev/evidence/verifier-20261002-kf-242.md`.
- Repair 2026-10-02 (commit 4874209): icon SVGs 14->16px, clock 13->20px (16.7px circle), row gap 0.475rem (pitch 23.59px preserved), time chip -3.6px margin (clock center 22.0px after comments icon = golden), time text 0.71rem (digit height 8.0px = golden). DOM-verified at 1440x900 + 1100x900 DPR1; aligned-crop pixdiff 1506px, remainder is out-of-scope glyph artwork only. All permutations pass; L0/L1/L4 green. Evidence appended to `.dev/evidence/builder-20261002-kf-242.md`; screenshots in `.dev/evidence/repair-20261002-kf242/`.
- Verify 2026-10-02 round 2 (independent blind verifier): FAILED — presence/order/conditional logic all PASS (5 icons GM-069 order, pitch 23.6px vs 23–24 golden, single + non-adjacent combos clean, noattr → no row, no gaps/shift, time text 8px = golden, 1100px identical, hover fine, L0 green, L1 zero console errors/failed requests with CDN pinned). One in-scope size deviation: clock circle 18px (svg 20x20) vs golden 14px — the repair's "16.7px = golden" rested on round-1's erroneous "~17px" golden measurement; direct golden column-profile = 14x14 at (617–630, 153–166). Fix: clock svg 16px (≈14px circle), no glyph redraw needed. Aligned-crop pixdiff 1737px/0.26 (remainder is out-of-scope feather-vs-KF glyph artwork; KF-320/321 are toolbar-only). Evidence: `.dev/evidence/verifier-20261002-kf-242-r2.md` + `.dev/evidence/verifier-20261002-kf242/`.
- Verify 2026-10-02 round 3 (independent blind verifier): FAILED — presence/order/conditional logic all PASS (5 icons GM-069 order at 1440+1100, pitch 23.6px, singles/noattr/desc_sub/overflow/hover clean, L0 green, L1 zero app errors/requests). One in-scope position deviation: clock/time chip sits ~2px too far left — comments-center → clock-center = 20.0px (DOM) vs golden 22.0px (bubble glyph center 601.5 → clock circle center 623.5, tail-excluded). `margin-left: -3.6px` on `.card-ico-time` (static/style.css:956) overshoots; `-1.6px` reproduces the golden 22px last-interval (natural pitch 23.6px). The CSS comment's "22px" claim measures 20.0px — wrong. Clock circle center-line ≈13.3px vs golden ≈13px (dark-extent delta is stroke-weight artwork, out of scope); time text 8px cap-height = golden (4px width delta is font-metrics). Fix: margin -3.6px → -1.6px, correct the comment. Evidence: `.dev/evidence/verifier-20261002-kf242-r3.md` + `.dev/evidence/verifier-20261002-kf242-r3/`.
 - Repair 2026-10-02 round 4 (commit ed7752b): `margin-left: -3.6px` → `-1.6px` on `.card-ico-time` (static/style.css:956) + comment corrected, per the r3 verifier's prescribed fix. DOM-verified (verifier's method): comments-center → clock-center = 22.0px = golden at 1440+1100; pitch 23.59px unchanged; 5 icons GM-069 order; singles/noattr/hover permutations clean; L0 green (fmt/build/clippy 0 warnings/79 tests pass); L1 zero console errors/failed requests. Aligned-crop pixdiff 1782px/0.269 (residual = documented out-of-scope feather-vs-KF glyph artwork, no shift signature). Evidence: `.dev/evidence/repair-20261002-kf242-r4.md` + `.dev/evidence/repair-20261002-kf242-r4/`.
- Status: FIXED 2026-10-02T05:52:18Z — deployed 5c747a88bbcc8bde9db98998213285debdce5859, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-242.md

### KF-243 — Assignee avatar sits at end of card icon row instead of top-right of title row [LOW | OPEN | Task cards]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V2 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the assignee avatar ("A") renders at the right end of the attribute icon row, below the title.
- KanbanFlow behavior: the assignee avatar sits at the top-right of the card's title row, aligned with the task name (GM-069).
- Fix direction: move the assignee avatar to the top-right of the card title row.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v2-GM-069-avatar.md`; `.dev/evidence/kf-drafts/v2-evidence/boardb-final2.png`; golden `.dev/evidence/golden-masters/task-cards/GM-069-card-due-subtasks-time-attributes.png`.

### KF-244 — Card due line renders orange-brown instead of dark text [LOW | OPEN | Task cards]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V2 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior (task due 2026-10-02 17:00, `due_done:true`): the card's "Due: Friday 5:00 PM (Done)" line renders in an orange-brown hue.
- KanbanFlow behavior: the equivalent due line renders in dark/near-black text (GM-069).
- Fix direction: render the card due line in dark text.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v2-GM-069-dueline.md`; `.dev/evidence/kf-drafts/v2-evidence/boardb-final2.png`; golden `.dev/evidence/golden-masters/task-cards/GM-069-card-due-subtasks-time-attributes.png`.

### KF-245 — Card border is near-black instead of a color-matched shade of the card face [MEDIUM | FIXED | Task cards]
- Status: FIXED 2026-10-02T06:25:51Z — deployed 01e733d5b5dc16ca9bc22523a6487a51f3a6c5ad, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-245.md
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V2 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior (green "Retro categorization" task; also orange "webhook" task): every card renders a 1px border in near-black (31,41,55) regardless of card color.
- KanbanFlow behavior: the border is a shade matched to the card's color — green card → (124,211,64) border; orange card → (250,162,0) border. Face colors already match (green (225,254,199) vs (219,255,194), Δ≤6) (GM-051, GM-052, GM-053).
- Fix direction: derive each card's border color from its face color instead of the fixed dark value.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v2-GM-051-border.md`; `.dev/evidence/kf-drafts/v2-evidence/g-corner.png` (KanbanFlow) vs `c-corner.png` (ChipFlow); goldens `.dev/evidence/golden-masters/task-cards/GM-05{1,2,3}-*.png`.
### KF-246 — Layout editor missing the green "Add column" button at the right end of the column list [LOW | OPEN | Columns]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V2 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior (opened via the board-bar "Edit board layout" pencil): header "Layout: <board>" ✓; blue "+ Add column" and "+ Add swimlane" buttons in the header ✓; columns listed in board order ✓ — but there is NO green "Add column" button at the right end of the column list (only "← Back to board" below the lists), and the header's Add-column button is blue, not green.
- KanbanFlow behavior (GM-043 verbatim catalog): header "Layout: General" with buttons Add column, Add swimlane; column list in board order; green Add column button at right.
- Fix direction: add the green "Add column" button at the right end of the column list per GM-043.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v2-GM-043.md`; `.dev/evidence/kf-drafts/v2-evidence/c-layoutview3.png`; golden `.dev/evidence/golden-masters/columns/GM-043-layout-editor.md`.

### KF-247 — Layout editor is a full-page in-board view, not a modal dialog over the dimmed board [MEDIUM | OPEN | Columns]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the layout editor is a full-page in-board view (`#layout-view`) — none of the modal chrome (Collapse all/Expand all, gear icons, green settings buttons, OK/Cancel) exists. (Settings access exists elsewhere: the column ⋮ menu offers Edit/Collapse/Show details/Delete/Rename/Move up/Move down — verified via right-click.) Distinct from KF-246 (structure vs one button).
- KanbanFlow behavior: the layout editor is a modal dialog over the dimmed board with "Collapse all"/"Expand all", per-item gear icons, green "Column settings"/"Swimlane settings" buttons, and OK/Cancel (GM-157).
- Fix direction: rebuild the layout editor as a modal dialog per GM-157; live-browser check still needed for drag-reorder (SortableJS and htmx were both `undefined` in the sandbox, jsdelivr unreachable — not filed as a defect).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-157.md`; `/tmp/gm-verify-v5/cand-layout-*.png` (scratch; not archived).

### KF-248 — Collapsed swimlane strip at board edge, title-case, ~30px vs interleaved/UPPERCASE/70px; extra "…" control [MEDIUM | OPEN | Swimlanes]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the collapsed "Personal to-do" strip renders at the RIGHT edge of the board (and "Default" at the left edge), ~30 CSS px wide, bg `#f3f4f6`; vertical label is title-case "Personal to-do" (rotated, reads bottom-to-top); strip head has green "+", "0", and a "…" at the bottom.
- KanbanFlow behavior: the collapsed "PERSONAL TO-DO" strip is INTERLEAVED BETWEEN columns (at the swimlane's column position), 70 CSS px wide; vertical label is UPPERCASE dark gray; strip head has green "+" and "0" only (GM-030).
- Fix direction: interleaved placement at the swimlane's column position, 70px width, uppercase label, remove the extra "…" control. (Position also tracked under KF-131/KF-162.)
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-030.md`; `.dev/evidence/kf-drafts/v1-evidence/vs-GM-030.png` (left=KanbanFlow, right=ChipFlow), `rightedge.png`.

### KF-249 — Color legend strip 29px vs 38px; all 4 segment colors differ; not flush to viewport bottom [MEDIUM | OPEN | Color legend]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V1 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: legend strip is 29px tall; segment fills `#ffffe0 / #e4ffd1 / #d6e9ff / #ffe0e3`; labels "1 Pomodoro / 2 Pomodori / 3 Pomodori / >3 Pomodori" (text matches); the strip sits in-flow at y 824–852 with a 47px board-bg gap below it — NOT flush to the viewport bottom.
- KanbanFlow behavior: legend strip is 38px tall (34px of segments); segment fills `#ffffe2 / #e1fec7 / #d1e1fd / #f7cdd1`; same four labels; the strip is flush to the viewport bottom (GM-139).
- Fix direction: 38px strip height, the master's four segment colors, and pin the strip flush to the viewport bottom.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v1-GM-139.md`; `.dev/evidence/kf-drafts/v1-evidence/vs-GM-139.png` (top=KanbanFlow, bottom=ChipFlow), `cand-GM-139.png`.

### KF-250 — Task modal is taller than the viewport (content cut off at the bottom) [HIGH | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: with the same fixture data (4 subtasks, 2 comments, description), the modal's content overflows the 998px viewport; the bottom (Attachments section) is cut off. Modal bounding box (652,42,1268,996).
- KanbanFlow behavior: the modal fits entirely within the viewport (top y≈26, bottom y≈900) (GM-070).
- Fix direction: cap the modal height to the viewport and scroll the modal content internally.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-070.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`, `gm-v3-candidate.png`.
- Status: FIXED 2026-10-01T21:48:50Z — deployed 2a96fa0ec74645281db67094cf531573ede7f1bb, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261001-kf-250.md

### KF-251 — Modal body uses single-column stacked UPPERCASE-label layout instead of 2-column label-left grid [HIGH | OPEN | Task modal]
- Status: FIXED 2026-10-01T23:25:25Z — deployed 1186e216906967dcc253d219cbf2f786128ec847, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261001-kf-251.md
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: body sections are single-column stacked rows, each with a small UPPERCASE gray label above the value (COLOR / TIME SPENT / ESTIMATE / LABELS / DUE DATE / MEMBERS).
- KanbanFlow behavior: two-column grid — row 1: Color | Members | Labels | Time spent; row 2: Time estimate | Due date — with mixed-case small labels above each value (GM-073).
- Fix direction: rebuild the modal body as the 2-column label-left grid per the master.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-073.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-252 — Toolbar buttons sit inside the modal's right edge with labels below, instead of outside vs dimmed board w/ labels right [HIGH | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: 6 small circular buttons are pinned INSIDE/overlapping the modal's right edge, each with a tiny label pill BELOW the circle.
- KanbanFlow behavior: 6 larger light-gray circles sit OUTSIDE the modal against the dimmed board, each with a white text label to the RIGHT of the circle (GM-076).
- Fix direction: move the icon rail outside the modal against the dimmed board with labels to the right.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-076.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.
- Status: FIXED 2026-10-02T00:25:00Z — deployed c8fd5d9a27d4bbc588e17b5df12c9c4eb33056af, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-252.md

### KF-253 — Description renders as an editable textarea instead of static text [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the description is an editable textarea with a resize handle under a "DESCRIPTION" label.
- KanbanFlow behavior: the description is a static text block (no textarea chrome, no resize handle); "Description" is a small mixed-case label (GM-070).
- Fix direction: render the description as static text (click-to-edit), with the mixed-case label.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-070.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-254 — Comments section: no avatars, absolute timestamps, × delete, boxed composer + Add button, no count/menu in header [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: header "Comments" (no count, no ⋮ menu); comment boxes tinted (same as body) with no avatar circles; timestamps absolute ("Oct 01, 2026 2:09 PM"); each comment has a × delete button (no smiley/⋮ icons); composer is an "Add a comment…" textarea plus a blue "Add" button.
- KanbanFlow behavior: header "Comments 2" with ⋮ menu; each comment has a "cs" avatar circle; timestamps relative ("Today 3:03 PM"); white comment boxes; smiley + ⋮ icons per comment; single-line "Add comment…" row (no boxed textarea, no Add button) (GM-070).
- Fix direction: restructure the comments section per the master (count + menu in header, avatars, relative timestamps, per-comment smiley/⋮, single-line composer).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-070.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-255 — Modal subline shows absolute created date ("Oct 01") instead of relative "Today" [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: subline reads "In Progress · Created: Oct 01" (absolute "MMM DD" even for a task created today).
- KanbanFlow behavior: subline reads "In Progress · Created: Today" (relative date for same-day creation) (GM-071).
- Fix direction: use relative dates in the modal subline.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-071.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-256 — Modal subline missing the three trailing icons [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: nothing follows "Created: Oct 01" in the subline.
- KanbanFlow behavior: three small icons follow "Created: Today" (subtask-list icon, comment icon, eye icon) (GM-071).
- Fix direction: add the three trailing icons to the subline.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-071.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-golden-crop.png` (icons visible after subline) vs `gm-v3-modal-cand-crop.png`.

### KF-257 — Color row shows only the dot, missing the color name text [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: COLOR row shows the orange dot alone.
- KanbanFlow behavior: "● Orange" — dot followed by the color name (GM-073).
- Fix direction: render the color name after the dot.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-073.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-258 — Time spent shows pomodori count, emoji, and "Time log" link instead of plain "2h 30m" [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: TIME SPENT row shows "🍅 0 Pomodori · ⏱ 2h30m" plus a "Time log" link.
- KanbanFlow behavior: "2h 30m" — no pomodori count, no emoji, spaced format, no inline link (GM-073).
- Fix direction: render plain "2h 30m"; the hover menu for time actions belongs to KF-278.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-073.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-259 — Due date renders absolute "Oct 02, 2026 5:00 PM" and omits the "(Done)" marker [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: DUE DATE row shows "📅 Oct 02, 2026 5:00 PM" — no "(Done)" marker despite `due_done: true` (confirmed via `GET /api/tasks/{id}`).
- KanbanFlow behavior: "Friday 5:00 PM (Done)" — relative weekday format with "(Done)" suffix when the due date is marked done (GM-073).
- Fix direction: render the due date in the relative weekday format and show the "(Done)" marker when done.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-073.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-260 — Label chips are blue-tinted instead of neutral chips [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`). Severity note: keeps the draft's LOW (V3:KF-243 was filed LOW; the §4 table's MEDIUM tag was a typo).
- ChipFlow behavior: labels render as light-blue/blue-text pills ("feature", "backend").
- KanbanFlow behavior: labels render as neutral gray/white chips with dark text (GM-073).
- Fix direction: restyle label chips to neutral gray/white with dark text.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-073.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-261 — Members row shows "● admin" text chip instead of avatar circle + ⊕ add button [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: MEMBERS row shows a "● admin" pill (dot + username text); no add button.
- KanbanFlow behavior: Members row shows a circular "cs" avatar plus a ⊕ add-member button (GM-074). (The "cs" vs "admin" identity difference is fixture data; the presentation difference is structural.)
- Fix direction: render member avatar circles and add the ⊕ add-member button.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-074.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-262 — Subtasks header missing the "2 / 4" count and the ⋮ section menu [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: header reads "Subtasks" with no count and no ⋮ menu at the right.
- KanbanFlow behavior: header reads "Subtasks 2 / 4" with a ⋮ section menu at the right (GM-085).
- Fix direction: add the done/total count and the ⋮ section menu to the subtasks header.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-085.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.
### KF-263 — Completed subtasks are struck through [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: done subtasks render with strikethrough text.
- KanbanFlow behavior: done subtasks render in normal text (checkbox state is the only indicator) (GM-085).
- Fix direction: remove the strikethrough from completed subtasks.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-085.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-264 — Subtask rows use × remove instead of the ⋮ drag handle [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: each subtask row has a × remove button at the right.
- KanbanFlow behavior: each subtask row has a ⋮ drag handle at the right (no ×) (GM-085).
- Fix direction: replace the × remove button with the ⋮ drag handle per row.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-085.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-265 — "Add subtask…" is a boxed input instead of a plain text row [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: "Add subtask…" is a boxed text input with a border.
- KanbanFlow behavior: "Add subtask…" is a plain unstyled text row (GM-085).
- Fix direction: restyle as a plain unstyled text row.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-085.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-266 — Reports toolbar icon is a play triangle instead of the stacked-layers icon [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the Reports button shows a ▶ play triangle.
- KanbanFlow behavior: the Reports button shows a stacked-layers icon (GM-076 names it the "☰-stack" glyph).
- Fix direction: replace the play triangle with the stacked-layers icon.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-076.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-267 — Delete toolbar icon is red instead of gray [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the trash icon is red.
- KanbanFlow behavior: the trash icon is dark gray like the other icons (GM-076).
- Fix direction: render the trash icon dark gray.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-076.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-modal-cand-crop.png` vs `gm-v3-modal-golden-crop.png`.

### KF-268 — Labels dialog Save button is blue instead of green [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (waves V3 and V5 independent verifiers; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge of V3:KF-253 (GM-080) + V5:KF-246's Save-color part (GM-158).
- ChipFlow behavior: the Labels dialog's Save button is blue (Cancel is white); V5 confirms the blue `.btn-primary` accent (`#2563eb`) in the dialog markup.
- KanbanFlow behavior: the Save button is green (GM-080, GM-158).
- Fix direction: render the Labels dialog Save button green.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-080.md` (`.dev/evidence/kf-drafts/v3-evidence/gm-v3-add-label-empty.png`), `.dev/evidence/kf-drafts/kf-draft-v5-GM-158.md` (`/tmp/gm-verify-v5` scratch, not archived).

### KF-269 — Labels empty-state text differs ("No labels yet."/"No suggestions." vs "No recently used labels exist") [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (waves V3 and V5 independent verifiers; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge of V3:KF-254 (GM-080) + V5:KF-246's wording part (GM-158).
- ChipFlow behavior: body reads "No labels yet." (with "Add a label" input and suggestion chips for labels used elsewhere); the suggestions area separately shows "No suggestions.".
- KanbanFlow behavior: the dialog body reads "No recently used labels exist" (a single body text) (GM-080, GM-158).
- Fix direction: use the verbatim "No recently used labels exist" text.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-080.md` (`.dev/evidence/kf-drafts/v3-evidence/gm-v3-add-label-empty.png`), `.dev/evidence/kf-drafts/kf-draft-v5-GM-158.md` (`/tmp/gm-verify-v5` scratch, not archived).

### KF-270 — Due-date dialog titled "Due date" with no calendar grid, no column list, Clear/Cancel/Save instead of green Add [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior (Add → Due date): dialog titled "Due date"; Date text field + calendar-icon button (grid opens separately, not inline); Time field "05:00 PM"; NO calendar grid inline; NO column list (instead: "Mark as done" checkbox + "Apply to tasks in this column" + per-task checkbox); Repeat field "e.g. every week" present; buttons Clear (red) / Cancel / Save (blue) — no green "Add".
- KanbanFlow behavior: title "Add due date"; Date field; Time field; inline calendar grid; column list (Personal To-do, Do today, In progress, Done, Backlog); "Repeat (e.g. every week)" field; green "Add" button (GM-081).
- Fix direction: title "Add due date", inline calendar grid, column list with selection, green Add button.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-081.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-add-duedate.png`.

### KF-271 — "Add time manually"/"Edit Pomodoro entry" use "+ Add comment/labels" links instead of checkboxes [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: both dialogs show "+ Add comment" / "+ Add labels" text LINKS instead of checkboxes. All other described attributes match: title with ×, prefilled Task field, Date field with calendar icon, From/To 12h time fields, auto-computed Duration, green Add/Update, Cancel.
- KanbanFlow behavior: "Add comment" / "Add labels" checkboxes that expand a Comment input and a Labels field with suggestions (GM-082, GM-084).
- Fix direction: replace the text links with checkbox expanders.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-082.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-add-manual-time.png`, `gm-v3-edit-pomodoro-entry.png`.

### KF-272 — Future-time error: title/message wording differs from KanbanFlow's verbatim ("Cannot add time"/"Error" vs "Cannot add time in the future") [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (waves V3 and V5 independent verifiers; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge group G4: V3:KF-257 (GM-083) + V5:KF-249 (GM-167).
- ChipFlow behavior: the entry is rejected and an error overlay appears (dialog state preserved, OK dismisses), but the message reads "You can not enter a time in the future" under a "Cannot add time" title. Neither string matches verbatim.
- KanbanFlow behavior: per GM-083, an error modal titled "Error" with body "You can not enter a time in the future"; per GM-167, the red validation string "Cannot add time in the future".
- Fix direction: the two masters' expected wordings differ from each other (GM-083 expects title "Error" + body "You can not enter a time in the future"; GM-167 expects the single red string "Cannot add time in the future") — the builder must reconcile both master claims against live KanbanFlow.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-083.md` (`.dev/evidence/kf-drafts/v3-evidence/gm-v3-future-time-error.png`), `.dev/evidence/kf-drafts/kf-draft-v5-GM-167.md` (`/tmp/gm-verify-v5/cand-future-error.png` scratch, not archived).

### KF-273 — Subtask list has no ↑/↓ keyboard navigation and no Cmd+↑/↓ reorder (both documented in shortcuts dialog) [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (waves V3 and V4 independent verifiers; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge group G5: V3:KF-258 (GM-086) + V4:KF-290 (GM-149) + V4:KF-291 (GM-150).
- ChipFlow behavior: arrow keys do nothing on focused subtask rows (V3 pressed ArrowDown/ArrowUp/Meta+ArrowDown/Meta+ArrowUp via Playwright — focus never moved, order never changed); Ctrl+↓ on a focused subtask does not change the order (V4); there is no ArrowUp/ArrowDown or Ctrl/Cmd+Arrow handler for subtask rows anywhere in static/app.js. The shortcuts dialog documents "↑ ↓ — Navigate subtask list" and "Cmd + ↑ ↓ — Move subtask in list", so the gap is self-contradictory.
- KanbanFlow behavior: ↑/↓ moves focus between subtask rows; Cmd+↑/↓ reorders the focused subtask (GM-086, GM-149, GM-150).
- Fix direction: implement ↑/↓ focus navigation and Ctrl/Cmd+↑/↓ reorder for subtask rows.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-086.md` (`.dev/evidence/kf-drafts/v3-evidence/gm-v3-subtask-kbdnav.png`), `.dev/evidence/kf-drafts/kf-draft-v4-GM-149.md`, `.dev/evidence/kf-drafts/kf-draft-v4-GM-150.md` (`/tmp/gm-v4-shots/keys_subtasks.png` ephemeral; static/app.js keydown handler has no such bindings).

### KF-274 — Move dialog uses Column/Swimlane/Position dropdowns instead of static Column label [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: title "Move task" ✓; Board dropdown "General" ✓; Column is a DROPDOWN (currently "In Progress") instead of a static label; EXTRA Swimlane and Position dropdowns; green "Move" ✓.
- KanbanFlow behavior: Column shown as a static label ("Work To-do"); no Swimlane/Position controls in the dialog (GM-087).
- Fix direction: show Column as a static label; remove the Swimlane/Position controls from the dialog.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-087.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-move-dialog.png`.

### KF-275 — Reports menu missing "Time in column"; History/Print order differs [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: Reports menu shows "Time log", "History", "Print" — 3 items; "Time in column" missing; History and Print swapped relative to KF's order.
- KanbanFlow behavior: 4 items in order: Time log, Print, History, Time in column (GM-089).
- Fix direction: add "Time in column" and restore the verbatim order.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-089.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-reports-menu.png`.

### KF-276 — Keyboard shortcuts dialog has extra "Add time manually (Y)" / "Add time estimate (E)" rows [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: all KF rows present (wording close: "Close window / discard changes — Esc"), PLUS extra rows "Add time manually — Y" and "Add time estimate — E" that KF's dialog does not list.
- KanbanFlow behavior: the dialog lists only Open Move dialog V; Open Timer menu T; Open Reports menu P; Open More menu .; Navigate subtask list ↑↓; Move subtask in list Cmd+↑↓; Close/discard Esc; Save changes Cmd+Enter; Delete task Delete (GM-093).
- Fix direction: remove the Y/E rows from the shortcuts dialog.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-093.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-kbd-shortcuts.png`.

### KF-277 — Time log view has "Total:" line, explicit "Edit" link, and "M" badges not present in KanbanFlow [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: all described attributes present (back link, "Time log" title, "+ ADD ENTRY", day sections, red trash, 12h ranges, comment text), PLUS: (1) a "Total: 2h 30m" line under the title; (2) an explicit "Edit" link per entry (KF opens edit by clicking the entry, per GM-084); (3) an "M" badge on each manual entry.
- KanbanFlow behavior: no "Total:" line, no explicit Edit link, no "M" badge (GM-096).
- Fix direction: remove the Total line, the explicit Edit link, and the "M" badges.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-096.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-time-log-view.png`.

### KF-278 — Hovering Time spent shows no tooltip and no hover menu [MEDIUM | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V3 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: hovering Time spent produces nothing (label has no `title` attribute; no tooltip/hover-menu elements appear in the DOM); the row instead carries a static inline "Time log" link.
- KanbanFlow behavior: hovering "Time spent" shows tooltip "You are selecting this task" and a hover menu with "Add time entry" and "Open time log" (GM-075).
- Fix direction: add the hover tooltip and hover menu per the master.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v3-GM-075.md`; `.dev/evidence/kf-drafts/v3-evidence/gm-v3-hover-timespent.png`.
### KF-279 — Timer popup TODAY section differs: "Today" w/o day total, red dots, "2:14 PM · 0m" format, no per-entry trash, running session never listed as "— pending" [MEDIUM | OPEN | Timer popup]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge group G6: V4:KF-273 (GM-099) + V4:KF-274 dev 2 (GM-101) + V4:KF-275 dev 4 (GM-105).
- ChipFlow behavior: section heading "Today" with no day total; entries show a red dot (#f87171) for stopped sessions and green for completed; entry format "Task B / Task done / 2:14 PM · 0m" (single start time + minutes); no per-entry removal affordance; a running session is NOT listed (shows "No entries yet today." when only a running session exists).
- KanbanFlow behavior: heading "TODAY 2m" (day total); stopped-session entry "Timer test task B Task done 11:21 AM – 11:23 AM" with green dot and a per-entry trash/× removal; entries show a start–end time range; the running session appears as a "— pending" entry (GM-099, GM-101, GM-105).
- Fix direction: "TODAY 2m" day total; green dots for stopped sessions; "11:21 AM – 11:23 AM" time-range format; per-entry trash; list the running session as a "— pending" entry.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-099.md`, `.dev/evidence/kf-drafts/kf-draft-v4-GM-101.md`, `.dev/evidence/kf-drafts/kf-draft-v4-GM-105.md`; `/tmp/gm-v4-shots/gm104_expired2.png`, `gm105_sw_idle.png` (ephemeral); live DOM `#timer-popup .timer-today`.

### KF-280 — Running pomodoro popup: task row lacks the green color dot [LOW | OPEN | Timer popup]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`). V4:KF-274 (GM-101) minus the "— pending" deviation (moved to KF-279).
- ChipFlow behavior: the running popup's task row shows the task name with NO color dot. (Verified matching: header "Pomodoro" + ×, "Time until break", big "24:31" countdown, red Stop, "Change task" link, bottom toolbar.)
- KanbanFlow behavior: the task row is a green color dot + task name + "Change task" (GM-101).
- Fix direction: add the task's color dot to the running popup's task row.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-101.md`; `/tmp/gm-v4-shots/` running-popup captures (ephemeral); live DOM `#timer-popup`.

### KF-281 — Stopwatch popup: idle has no big "00:00" readout, blue "Start Stopwatch" vs green "Start", running footer missing Settings icon [MEDIUM | OPEN | Timer popup]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`). V4:KF-275 (GM-105) devs 1–3 (the "— pending" deviation moved to KF-279).
- ChipFlow behavior: idle tab shows Task dropdown + buttons only — NO big "00:00" readout; the idle button is blue "Start Stopwatch"; the running footer is Pomodoro / Add time / Log — Settings icon missing (the idle stopwatch footer HAS all four). (Verified matching: idle header "Stopwatch" + ×, "Session time"; running big "00:04" count-up, red Stop, "Change task", "Session time".)
- KanbanFlow behavior: idle shows a big "00:00" and a green "Start" button; running bottom row is Stopwatch, Add time, Log, Settings (GM-105).
- Fix direction: add the big "00:00" idle readout, a green "Start" button, and restore the Settings icon in the running footer.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-105.md`; `/tmp/gm-v4-shots/gm105_sw_idle.png`, `gm105_sw_running.png` (ephemeral); live DOM `#timer-popup`.

### KF-282 — No context menu on right-click over the timer area (thin evidence: catalog text only) [LOW | OPEN | Timer popup]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: right-clicking the timer pill and right-clicking inside the open timer popup opens NO context menu. (Right-clicking a task card opens ChipFlow's card menu: "Start timer ▸, Move ▸, Color ▸, Assign members, Copy here, Task URL, Delete" — different items, different target.)
- KanbanFlow behavior: right-clicking the timer area opens a context menu with "Comment, Label, Edit, Delete" (GM-109; thin evidence — catalog text only, no screenshot).
- Fix direction: implement the timer-area context menu; the exact right-click target should be re-verified against live KanbanFlow given the thin master.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-109.md`; `/tmp/gm-v4-shots/gm109_ctxmenu.png`, `gm109_ctxmenu2.png`, `gm109_cardctx.png` (ephemeral).

### KF-283 — Default long break is 15 minutes, KanbanFlow shows 10 [MEDIUM | OPEN | Timer settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: Settings → General → "Long break time" defaults to "15 minutes" (seeded `long_break_minutes: 15` in src/models.rs:449). All other General-tab points verified matching: Pomodoro durations heading, Work 25 minutes, Short break 5 minutes, Long interval "Every 4th break", Picture-in-Picture ON.
- KanbanFlow behavior: "Long break time" = "10 minutes" (GM-112).
- Fix direction: seed the default long break at 10 minutes.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-112.md`; `/tmp/gm-v4-shots/` settings captures (ephemeral); src/models.rs:449.

### KF-284 — Break-activities "Add activity" button is blue, KanbanFlow green [LOW | OPEN | Timer settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the "Add activity" button in Settings → Break activities is BLUE. (Verified matching: tab heading, descriptive text verbatim, light-blue "Examples:" box.)
- KanbanFlow behavior: the "Add activity" button is green (GM-114).
- Fix direction: render the button green.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-114.md`; `/tmp/gm-v4-shots/` settings captures (ephemeral).

### KF-285 — Add-break-activity dialog text and button differ (placeholder, "Daily limit (0=unlimited)" vs "No limit", blue button) [LOW | OPEN | Timer settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: Name placeholder "e.g. Coffee"; goal row labeled "Daily limit (pomodoros, 0 = unlimited)" defaulting to 0; "Description (optional)" label; BLUE "Add activity" button plus an extra Cancel button. (Verified matching: dialog title + ×, stacked modal, Daily goal = 1.)
- KanbanFlow behavior: Name placeholder "e.g. Meditate, Stretch, Drink water, Take a walk"; "No limit" (not a 0=unlimited numeric field); green "Add activity" button (GM-115).
- Fix direction: placeholder text, "No limit" wording, and green button per the master.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-115.md`; `/tmp/gm-v4-shots/` dialog captures (ephemeral).

### KF-286 — Timer log is a full page, KanbanFlow shows a modal; Period plain select w/o stepper; titled "Timer log" w/ tabs; no settings/close × [HIGH | OPEN | Timer log]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (waves V4 and V5 independent verifiers; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge group G7: V4:KF-280 (GM-117) + V5:KF-242 dev 1+3 (GM-129).
- ChipFlow behavior: the "Log" action navigates to a full page at /timer/log (H1 "Timer log", "← Board" link, "Statistics" link, Time log / Time spent tabs). Filter row: Board "All boards" dropdown; Period is a plain <select> ("This + Last week" selected; options This + Last week / This week / Last week / This month / Last month / Custom (absolute) / Custom (relative)); "Entry type: All types"; icon buttons print / export / reload only. The Time spent report page is titled "Timer log" with "Time log" / "Time spent" tabs instead of a standalone "Time spent" title; there is no close ×.
- KanbanFlow behavior: a wide white centered MODAL over the dimmed board, title "Timer log" with × close. Filter row: "All boards" dropdown; Period control showing e.g. "Period: This < Last week >" WITH prev/next arrows; "Entry type: All" dropdown; icons print, export, reload, settings, close X (GM-117, GM-129).
- Fix direction: rebuild the timer log as a centered modal per GM-117 (with the Period stepper, settings icon, and close ×); retitle the Time spent report as standalone "Time spent".
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-117.md`, `.dev/evidence/kf-drafts/kf-draft-v5-GM-129.md`; `/tmp/gm-v4-shots/gm117_timerlog.png` (ephemeral), `templates/timer_log.html`; `/tmp/gm-verify-v5/cand-time-spent-tab.png` (scratch, not archived).
- Status: FIXED 2026-10-02T01:04:30Z — deployed 2232a8173019ef41c35737e391bd5c8d600898cd, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-286.md (modal shell + filter row + Time spent page + entry rows + GM-117/GM-129 verbatim records + fixture flows + scope + regression) at 1440x900 and 1100x900, zero console errors / zero failed requests (clean re-run); break-attempt: 1100px narrow viewport, empty-state board filter, nested settings modal, rapid open/close stress, hover on close ×. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/verifier-20261002-kf-286.md. SPOTCHECK 2026-10-02T01:00:49Z — independent pass 2 passed (own server/DB/fixture, 1440+1100+800 viewports; added 800x600 long-title overflow, functional entry-type filter, icon-row close ×; ghost-Total dismissal confirmed pre-existing in KF-292 scope; one break-attempt correction: Escape from nested settings closes both modals to board — no master covers it, no new defect). Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/spotcheck-20261002-kf-286.md.

### KF-287 — Time spent report Reload button is blue; KanbanFlow's is green [LOW | OPEN | Reports]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`). V5:KF-242 dev 2 (GM-129).
- ChipFlow behavior: the Reload button is blue.
- KanbanFlow behavior: the Reload button is green (GM-129).
- Fix direction: render the Reload button green.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-129.md`; `/tmp/gm-verify-v5/cand-time-spent-tab.png` (scratch, not archived).

### KF-288 — Timer log entries: sub-minute durations show "0m" vs "31s"; saved labels not rendered [MEDIUM | OPEN | Timer log]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: a 37-second session renders "0m" (the page uses fmtDuration(e.minutes) — whole minutes only; the API's `duration_display` "37s" is ignored). A manual entry saved WITH labels ["test", "another test"] (confirmed in GET /api/timer/log) renders NO label chips. Stop reasons render with curly quotes ('Task done') vs the claim's straight quotes. (Verified matching: day group header "Thursday, October 1 — 2m — 4 Pomodoros"; green "Successful Pomodoro" / red "Stopped Pomodoro with reason '…'" rows; "2:19 PM – 2:20 PM" + "1m"; blue "Add time entry" per day; manual entry comment "testing this out".)
- KanbanFlow behavior: sub-minute sessions show "31s" / "48s"; entries show comment and labels 'test' and 'another test' (GM-118).
- Fix direction: honor the API's `duration_display` in `fmtDuration` (static/app.js); render saved label chips on entries.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-118.md`; `/tmp/gm-v4-shots/gm118_entries.png`, `gm118_manual.png` (ephemeral); GET /api/timer/log JSON.

### KF-289 — Timer log export menu order: CSV before Excel, KanbanFlow lists Excel first [LOW | OPEN | Timer log]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the export dropdown under the log's export icon lists "Download CSV file" first, then "Download Excel file".
- KanbanFlow behavior: "Download Excel file" first, then "Download CSV file" (GM-119).
- Fix direction: swap the menu item order (see also KF-294 for the Time spent export items themselves).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-119.md`; `/tmp/gm-v4-shots/gm119_export.png` (ephemeral); templates/timer_log.html #log-export-menu.

### KF-290 — Custom (absolute) range is inline From/To inputs, not a dialog with two calendars + Done [MEDIUM | OPEN | Timer log]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: selecting "Custom (absolute)" in the Period dropdown reveals inline "From [date] To [date]" inputs in the filter row — no dialog, no "Custom (absolute)" title, no side-by-side month calendars, no Done button.
- KanbanFlow behavior: nested "Custom (absolute)" dialog with Start/End fields, two month calendars side by side with selected dates highlighted, green "Done" button (GM-121).
- Fix direction: replace the inline inputs with the nested dialog per the master.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-121.md`; `/tmp/gm-v4-shots/gm121_custom.png` (ephemeral).

### KF-291 — Date picker: weekday row uses single letters vs "Sun Mon…"; missing Today/yesterday/This month quick buttons [LOW | OPEN | Timer]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (waves V4 and V5 independent verifiers; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge group G8: V4:KF-284 (GM-122) + V5:KF-250 (GM-171).
- ChipFlow behavior: the shared `renderCalPopup` renders the weekday row as "S M T W T F S" (single initials); the popup shows the month grid with month navigation only — the "Today", "yesterday", "This month" quick buttons are absent. (Verified matching: popup header "October 2026" with < > arrows; selected date (today) highlighted blue.)
- KanbanFlow behavior: weekday row "Sun Mon Tue Wed Thu Fri Sat"; the date-field calendar popup shows quick buttons "Today", "yesterday", "This month" (GM-122, GM-171).
- Fix direction: three-letter day names in `renderCalPopup` (static/app.js) and add the three quick buttons.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-122.md`, `.dev/evidence/kf-drafts/kf-draft-v5-GM-171.md`; `/tmp/gm-v4-shots/gm122_datepicker.png` (ephemeral), `/tmp/gm-verify-v5/cand-date-picker.png` (scratch, not archived); static/app.js `renderCalPopup`.

### KF-292 — Time spent report lacks bar/percent columns, tasks/breaks/duration% breakdown, and totals footer rows [MEDIUM | OPEN | Timer reports]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: rows show date + duration only ("Thursday, 1 October 2026 | 2m"). No per-row bar column with "100%", no tasks/breaks breakdown columns, no duration-% column, no "8h 4m / 100% / Total" footer rows. The filter is a single "Filter" button opening a dropdown (Period/User/Color/Label) rather than the claimed Board/Period/Entry-type dropdown row. (Verified matching: title "Time spent"; print/export buttons; "Group by" control; Summary/Detailed links; per-date rows; "Total: 2m".)
- KanbanFlow behavior: rows per date with a bar column showing "100%"; columns date/tasks/breaks/duration%; footer rows "0m", "0%", "Total" with totals like "8h 4m", "100%" (GM-123).
- Fix direction: add the bar/percent column, the tasks/breaks/duration% breakdown, the totals footer rows, and the three-dropdown filter row.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-123.md`; `/tmp/gm-v4-shots/gm123_timespent.png`, `gm123_timespent.html` (ephemeral).

### KF-293 — Pomodoro statistics: Period presets don't match (GM-124 and GM-130 expected lists differ — reconcile vs live KF); no bar count labels; tooltip format differs [MEDIUM | OPEN | Timer reports]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (waves V4 and V5 independent verifiers; see `.dev/evidence/kf-drafts/TRIAGE.md`). Merge group G9: V4:KF-286 (GM-124) + V5:KF-243 (GM-130).
- ChipFlow behavior: Period dropdown options are "This + Last week / This week / Last week / This month / Last month / Custom (absolute) / Custom (relative)". Bars have NO count labels atop them. Bar tooltip (SVG <title>) reads "Thursday, Oct 01: 4 pomodoros" — no year, no duration, lowercase "pomodoros", native title tooltip rather than a styled tooltip. (Verified matching: title "Pomodoro Statistics"; tabs Pomodoros/Interruptions/Break activities/Highscores; summary numbers; "Pomodoros per day" bar chart with day labels; "No data to display" empty state verbatim.)
- KanbanFlow behavior: per GM-124, Period lists exactly '1 day', '2 days', '3 days', '7 days', '14 days', 'This week', 'Last week', 'This month'; numbers rendered atop bars; hovering a bar shows tooltip "Tuesday 1 Sep 2026: 2 Pomodori (30m)".
- Fix direction: the two masters' expected option lists differ from each other (GM-124: '1 day','2 days','3 days','7 days','14 days','This week','Last week','This month'; GM-130: "Last 7 days","Last 14 days","Last 30 days" (default),"This week","Last week","This month","Last month","Custom (absolute)","Custom (relative)") — the builder must reconcile both master claims against live KanbanFlow. Also add bar count labels and the tooltip format "…: 2 Pomodori (30m)".
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-124.md` (`/tmp/gm-v4-shots/gm124_stats.png` ephemeral), `.dev/evidence/kf-drafts/kf-draft-v5-GM-130.md` (`/tmp/gm-verify-v5/cand-pom-stats.png` scratch, not archived).
### KF-294 — Time spent Export menu items don't match verbatim list ("Excel (Detailed)", "Excel (Summary)", "PDF (Summary)") [MEDIUM | OPEN | Other dialogs]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the Export dropdown offers "Download CSV file" and "Download Excel file" — none of the master's items, no PDF option, no Detailed/Summary split.
- KanbanFlow behavior: verbatim items in order: "Excel (Detailed)", "Excel (Summary)", "PDF (Summary)" (GM-170).
- Fix direction: replace the export menu with the verbatim item list (including the Detailed/Summary split and PDF). Not a duplicate of KF-093 (export works — this is the residual menu-item fidelity) or KF-289 (different surface: the timer-log export order).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-170.md`; `/tmp/gm-verify-v5/cand-export-menu.png` (scratch, not archived).

### KF-295 — Time spent Print invokes the browser's native print dialog instead of KanbanFlow's "Print" sheet [MEDIUM | OPEN | Other dialogs]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the Print button fires `window.print()` — the browser's native print dialog — no "Print" sheet exists (code-verified: static/app.js:4517 `spent-print` click handler is a bare `window.print()`).
- KanbanFlow behavior: the Time spent report's Print button opens a sheet titled "Print" showing the task table and Filter/Print buttons (GM-169).
- Fix direction: implement the "Print" sheet with the task table and Filter/Print buttons.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-169.md`.

### KF-296 — Color filter says "2 Pomodori", KanbanFlow says "2 Pomodoros" [LOW | OPEN | Filter panel]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: Filter panel → Color dropdown (and the Time-spent report Filter → Color) lists "1 Pomodoro", "2 Pomodori", "3 Pomodori", ">3 Pomodori".
- KanbanFlow behavior: "2 Pomodoros", "3 Pomodoros", ">3 Pomodoros" (GM-125; the master's own note flags the "2 Pomodori" rendering as an unresolved interpretation question at capture time).
- Fix direction: use "Pomodoros" plural for the 2/3/>3 pomodoro-count colors.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-125.md`; `/tmp/gm-v4-shots/gm125_filter.png`, `gm126_after_selection.png` (ephemeral).

### KF-297 — Column header count badges ignore the active filter (KF-161 regression) [MEDIUM | OPEN | Filter panel]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: with Color = "1 Pomodoro" selected, non-matching cards hide correctly (visible: "Timer test task A", "Done task"), but the column header count badges still show unfiltered totals (To-do:2, Doing:2, Done:1 instead of 1/0/1). Root cause: static/app.js `applyFilter` recounts visible cards but writes only `.columnHeader-count` (the WIP "count / limit" element) and toggles warning classes; the badge element keeps the server-rendered `data-task-count`.
- KanbanFlow behavior: column counts drop to the filtered visible-card counts (GM-127).
- Fix direction: update `.columnHeader-countBadge` inside `applyFilter` (static/app.js ~L5015–5035). Regression: tracker KF-161 ("Column counts ignore filters") was previously fixed and regression-verified PASS; this reproduces the same symptom on the current build via the badge element.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-127.md`; `/tmp/gm-v4-shots/gm126_after_selection.png` (ephemeral); static/app.js ~L5015–5035.

### KF-298 — T opens the global timer popup, not the task modal's Timer menu [MEDIUM | OPEN | Keyboard shortcuts]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: with the task modal open, pressing T calls TimerUI.togglePopup() — the GLOBAL timer popup opens/closes at the top-right; the task modal's own Timer menu (stopwatch icon in the modal's right rail) does not open (static/app.js keydown handler, `key === 't'` branch).
- KanbanFlow behavior: T opens the task modal's timer menu (GM-146).
- Fix direction: when the task modal is open, T should open the modal-context timer menu instead of toggling the global popup.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-146.md`; `/tmp/gm-v4-shots/keys_t.png` (ephemeral); static/app.js ~L3613.

### KF-299 — Esc closes the modal outright; no discard-edits-first step [LOW | OPEN | Keyboard shortcuts]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V4 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: handleEscape() (static/app.js ~L4100) closes menus/dialogs, then closes the task modal directly — there is no "discard in-progress edits" stage. Verified: typing an unsent comment draft in #modal-comment-input and pressing Esc once closes the modal (draft lost). (The name field commits on change, so it has no in-progress state.)
- KanbanFlow behavior: first Esc discards in-progress edits while keeping the modal open; second Esc closes the modal (GM-151).
- Fix direction: implement two-stage Esc (discard edits first, close on second Esc).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v4-GM-151.md`; static/app.js handleEscape (~L4100).

### KF-300 — Stray horizontal rules render under each color-row cell on the colors admin page [LOW | OPEN | Board settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: a stray horizontal rule renders under each color-row cell (drag handle, chip, name, actions) — confirmed via zoomed crop.
- KanbanFlow behavior: each color row is a clean row of drag-handle, color chip, name, and actions with no rules under individual cells (GM-131).
- Fix direction: remove the stray horizontal rules under color-row cells.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-131.md`; `/tmp/gm-verify-v5/cand-*-colors.png`, `/tmp/gm-verify-v5/zoom-row.png` (scratch; not archived).

### KF-301 — Default color marker reads "(default)" instead of "(Default)" [LOW | OPEN | Board settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the marker reads "(default)" (lowercase d).
- KanbanFlow behavior: the default color's label is suffixed "(Default)" (capital D) (GM-132).
- Fix direction: capitalize to "(Default)".
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-132.md`.

### KF-302 — Quick-add color picker is a dot-only popover, not KanbanFlow's centered named modal [MEDIUM | OPEN | Board settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the quick-add "+" opens a dot-only popover (bare color dots, no names, no ⋮ menus, no bold selected row).
- KanbanFlow behavior: a centered modal whose color selector is a dropdown of named rows — dot + color name + ⋮ menu per row, the selected row bold (GM-133).
- Fix direction: rebuild the quick-add color picker as the centered named modal.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-133.md`; `/tmp/gm-verify-v5/cand-quickadd.png` (scratch; not archived).

### KF-303 — Disabled colors listed in palette order instead of alphabetically [LOW | OPEN | Board settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: disabled colors follow palette sort order: Orange, Purple, Magenta, Cyan, Brown, White.
- KanbanFlow behavior: the disabled-colors section lists them alphabetically: Brown, Cyan, Magenta, Orange, Purple, White (GM-131, GM-144).
- Fix direction: sort the disabled-colors section alphabetically.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-144.md`.

### KF-304 — Board delete uses native window.confirm instead of a styled in-page dialog [MEDIUM | OPEN | Board settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the delete-board page's final step fires a native `window.confirm` ("Delete the board \"V5 Delete Me\"? This cannot be undone.", dialog type `confirm`) — verified end-to-end on a throwaway board (since deleted).
- KanbanFlow behavior: destructive actions use a styled in-page confirmation dialog (GM-165; GM-163's styled-confirm standard).
- Fix direction: route board delete through the styled in-page confirm. Not a duplicate: KF-184 covers task/comment/time-entry/column/swimlane/API-token deletes (all styled ✓); KF-047/KF-188/KF-112 cover other surfaces. None covers the board delete flow.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-165.md`.

### KF-305 — No "Archive board" option on the delete-board page [MEDIUM | OPEN | Board settings]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: no board archiving exists anywhere — no API endpoint, no UI. The delete-board page shows only the red "Delete board" button.
- KanbanFlow behavior: the red delete-board page shows a blue "Archive board" button alongside the red "Delete board" button — archiving is a first-class alternative to deletion (GM-138; GM-165).
- Fix direction: add board archiving (API + UI) with the blue "Archive board" option on the delete-board page.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-138.md`; `/tmp/gm-verify-v5/cand-delete-board-1920.png` (scratch; not archived).

### KF-306 — Column delete confirm lacks the "Move the tasks to the column" option and verbatim text [MEDIUM | OPEN | Other dialogs]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: a styled in-page dialog (the styled-confirm mechanism itself is fine) titled "Delete column" with text `Delete column "<name>"?` and plain Cancel / Delete buttons — no "Move the tasks to the column" checkbox and not the verbatim "Are you sure you want to delete this column?" text.
- KanbanFlow behavior: the delete-column confirm reads "Are you sure you want to delete this column?" with a checkbox "Move the tasks to the column" — deletion of a non-empty column requires choosing a move target (no bare "Delete anyway") (GM-164).
- Fix direction: verbatim text plus the "Move the tasks to the column" move-target option. Not a duplicate: KF-047 covers the styled-confirm mechanism; this is the missing option and text.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-164.md`; `/tmp/gm-verify-v5/cand-col-delete.png` (scratch; not archived).

### KF-307 — Built-in "Pomodoro board" template ships no starter tasks [MEDIUM | OPEN | Templates]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: the built-in template snapshot (`pomodoro_template_snapshot` in src/db.rs:223) has no `tasks` key, and a board created from the template contains zero tasks (verified: 0 task cards on the new board). (Verified matching: picker lists "Pomodoro board" (Built-in); board created with the pomodoro color scheme and 4 columns.)
- KanbanFlow behavior: built-in templates (e.g. Pomodoro board with 'Pomodoro 1') ship with starter tasks (GM-141; tracker-documented expectation in KF-164).
- Fix direction: add the starter tasks to the built-in template snapshot. Not a duplicate of KF-164 (fixed): that fix covered save-board-as-template snapshots; this is the built-in snapshot — a distinct code path.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-141-KF-244.md`.

### KF-308 — Built-in Pomodoro template's first column is "Work To-do" instead of "To-do" [LOW | OPEN | Templates]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave V5 independent verifier; see `.dev/evidence/kf-drafts/TRIAGE.md`).
- ChipFlow behavior: src/db.rs:244 creates the first column as "Work To-do" (the board also gets a "PERSONAL TO-DO"/"BACKLOG" swimlane), so the template's column set doesn't match the documented set.
- KanbanFlow behavior: the pomodoro template has 4 columns "To-do / Do today / In progress / Done" (GM-141; KF-190 evidence).
- Fix direction: name the first template column "To-do". Not a duplicate of KF-190 (fixed): that fix covered the board-creation UI; this is the residual column name.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-v5-GM-141-KF-245.md`.

### KF-309 — Board-header funnel and pencil icons are thin outline glyphs, not KanbanFlow's solid glyphs [LOW | OPEN | Board header row]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (wave-V1 verifier; re-filed after the GM-017/018/019 master labels were corrected — previously dropped as master-error because the labels were shifted one icon right of their content).
- pixdiff (content-aligned): funnel-vs-funnel 80×75 → `diff_pixels 6000/6000 (100%), mean_delta 19.55` FAIL; pencil-vs-pencil 80×75 → `diff_pixels 6000/6000 (100%), mean_delta 18.30` FAIL.
- ChipFlow behavior: `button#filter-btn` shows a thin outline-style funnel (dark-navy stroke, hollow); `button#edit-layout-btn` shows a thin outline ✎ pencil.
- KanbanFlow behavior: solid dark funnel glyph; solid dark pencil glyph.
- Repro: open any board; compare the funnel and pencil icons against GM-017's image (funnel) and GM-018's image (pencil) — both masters relabeled 2026-10-01 to match their actual pixel content.
- Evidence: `.dev/evidence/kf-drafts/v1-evidence/vs-GM-017-3x.png` (funnel solid vs outline), `vs-GM-019-3x.png`, `vs-GM-018.png` (pencil solid vs outline); corrected masters `evidence/golden-masters/board-header-row/GM-017-timer-clock-button.md`, `GM-018-filter-button.md`, `GM-019-edit-layout-pencil.md`.

### KF-310 — Bell (notifications) button is dead: click opens nothing [HIGH | OPEN | Top bar]
- Status: FIXED 2026-10-02T01:42:05Z — deployed cfffe44bf48a5b86f6ce15561b2b73b14c81d20f, /api/v1/version build_sha verified. Evidence: /home/hatch/workspace/pomodoro-kanban/.dev/evidence/deploy-20261002-kf-310.md
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker A, adversarial re-verification of V1's hover-state pass; test server 127.0.0.1:3221, fresh redb, Playwright).
- ChipFlow behavior: `#notifications-btn` in the dark top bar shows a hover state (idle (31,41,55)→hover (55,65,81), cursor:pointer) but clicking opens NO panel, dropdown, or popover — no `[role=dialog]`, no new visible element, no DOM diff. (The bell's icon style is separately covered by KF-235; this is strictly the dead click.)
- KanbanFlow behavior: the bell opens the notifications panel (standard KanbanFlow top-bar behavior; the button exists precisely to surface notifications).
- Fix direction: wire the bell button to open the notifications panel/dropdown on click (re-click/Escape/scrim to dismiss, matching the boards-drawer dismissal behavior).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-spotA-bell.md`; `.dev/evidence/kf-drafts/SPOT-A.md` (hover measurements); scratch screenshot `/tmp/spot-evidence/p3-bell-clicked.png`.

### KF-311 — Column header row is shorter and its name smaller than KanbanFlow [MEDIUM | OPEN | Columns]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker A breaking V2's GM-031 pass; test server 127.0.0.1:3221).
- ChipFlow behavior: column header row measures 32.6px tall (`getBoundingClientRect()`); header name renders at 13.6px font-size, cap height ~9px. Name centering, bold weight, black color all match — this is strictly row height and font size.
- KanbanFlow behavior: golden GM-031 (`cmp-kf-header.png`) shows the header row y5–48 ≈ 41px tall, name ~17px with cap height ~11–12px.
- Fix direction: raise the header row to ~41px and the name font-size to ~17px.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-spotA-GM-031.md`; scratch `/tmp/spot-evidence/p1-board-1600.png`.

### KF-312 — Inline "n / 3" WIP count renders blue-gray instead of neutral gray [LOW | OPEN | Columns]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker A breaking V2's GM-033 pass; test server 127.0.0.1:3221).
- ChipFlow behavior: the inline WIP count (e.g. "5 / 3" in the In Progress header) renders in #6b7280 → rgb(107,114,128), a blue-gray. Placement is correct — strictly the color.
- KanbanFlow behavior: golden GM-033's master image shows the inline count in neutral gray rgb(91,91,91) (sampled from the master PNG). Master-text note: the GM-033 .md text says "black" but the image shows (91,91,91) — the master text is inaccurate; ChipFlow matches neither.
- Fix direction: change the inline count color from #6b7280 to neutral gray (91,91,91).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-spotA-GM-033.md`; scratch `/tmp/spot-evidence/p5-wip-violation.png`.

### KF-313 — Missing vertical separator between + button and standalone count cell [LOW | OPEN | Columns]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker A breaking V2's GM-032 pass; test server 127.0.0.1:3221).
- ChipFlow behavior: in the column header, the green "+" add-task button and the standalone count cell ("16", "0") sit adjacent with NO vertical separator between them. (Count color is separately covered by KF-316; this is strictly the missing separator.)
- KanbanFlow behavior: golden GM-032 shows a vertical separator line between the + button and the standalone count cell.
- Fix direction: add the vertical separator between the + button and the standalone count cell, matching the master's position/color.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-spotA-GM-032.md`; scratch `/tmp/spot-evidence/p1-board-1600.png`, `p7-collapsed.png`.

### KF-314 — Folded-swimlane strip label is bold; KanbanFlow uses regular weight [LOW | OPEN | Swimlanes]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker A breaking V2's GM-037 pass; test server 127.0.0.1:3221).
- ChipFlow behavior: the folded-swimlane strip at the far right renders its vertical "BACKLOG" label with `font-weight: 700` in rgb(75,85,99). Strip position is correct — strictly label weight/color. (Folded-strip WIP-violation styling is separately covered by KF-241.)
- KanbanFlow behavior: golden GM-037 shows the strip label in regular weight with a near-black core rgb(17,17,17) (stroke measured 4px @ DPR2 = 2 CSS px, consistent with regular-weight glyphs).
- Fix direction: change the strip label to regular font-weight and near-black (17,17,17).
- Evidence: `.dev/evidence/kf-drafts/kf-draft-spotA-GM-037.md`; scratch `/tmp/spot-evidence/p8c-strip-right.png`.

### KF-315 — Edit-column dialog: four verbatim-field deviations from KanbanFlow [MEDIUM | OPEN | Columns]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker A breaking V2's GM-045 pass; test server 127.0.0.1:3221; Edit dialog via right-click → Edit on the Done column). The info note, Cancel/Update-column buttons, and Column sum "None" all match — not part of this defect.
- ChipFlow behavior: (1) Description placeholder is empty (master: "Add description"); (2) WIP limit placeholder reads "No limit" (master: "Add limit"); (3) Task sorting defaults to "Manual order" with options Manual order/Name/Date created — no "None" option (master: "None"); (4) Archiving checkbox UNCHECKED for the Done column (master: checked for Done).
- KanbanFlow behavior: per the verbatim golden master GM-045 — Description "Add description", WIP limit "Add limit", Task sorting "None", Archiving checked for the Done column; info note verbatim "Archiving is ONLY recommended for columns that represent an end state for tasks, like the Done column. Loads the 20 most recent tasks from the start."
- Fix direction: Description placeholder "Add description"; WIP-limit placeholder "Add limit"; Task sorting default/option "None" per master; Archiving checked by default for done columns.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-spotA-GM-045.md`; scratch `/tmp/spot-evidence/p10-edit-column.png` + DOM dumps.

### KF-316 — "Do today" header: add-task control is a green button, not a bare green glyph; count is muted gray, not near-black [LOW | OPEN | Empty states]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker B breaking V5's GM-156 attribute pass; test server 127.0.0.1:3222, fresh redb; Firefox 1919×998 DPR 1). Breaks the 1px/1-shade bar on two axes.
- ChipFlow behavior: the add-task control is a green rounded-square button (background #22C55E = 34, 197, 94, Tailwind green-500) containing a white "+" glyph; the count "0" renders in muted gray #6b7280 (`var(--muted)`, `.columnHeader-countBadge`, static/style.css:415).
- KanbanFlow behavior: golden GM-156's master crop shows a bare green "+" glyph, core color #3DC54E (61, 197, 78), with no button background; the count "0" is near-black (#111111 core).
- Fix direction: render the header add control as a bare green glyph in #3DC54E (no button background); render the count in near-black.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-spotB-GM-156.md`; scratch `/tmp/spotb-master-hdr-zoom.png`, `/tmp/spotb-cf-hdr-zoom.png`.

### KF-317 — Labels dialog input label/placeholder does not match the master's "Add labels…" [LOW | OPEN | Task detail modal]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker B breaking V5's GM-158 partial match; test server 127.0.0.1:3222; verified via DOM). Not a duplicate of KF-268/KF-269 (Save color, suggestions empty text) — this covers the input field's own label/placeholder, which the V5 partial-match claim glossed over.
- ChipFlow behavior: the labels dialog's input is labeled by `<span>Add a label</span>` and carries the placeholder "Type a label and press Enter..." — neither matches the master's "Add labels…" text.
- KanbanFlow behavior: golden GM-158 records the input field as '"Add labels..." input field'.
- Fix direction: match the input's label/placeholder to the verbatim master text.
- Evidence: `.dev/evidence/kf-drafts/kf-draft-spotB-GM-158.md`; markup at templates/board.html.

### KF-318 — Board-filter option values on timer log/statistics render with literal surrounding quotes [MEDIUM | OPEN | Timer reports]
- Status: OPEN — filed 2026-10-01 by the exhaustive golden-master audit (spot-checker B incidental finding; test server 127.0.0.1:3222).
- ChipFlow behavior: the board-filter `<option>` values on `/timer/log` and `/timer/statistics` render with literal surrounding quotes (`value="&quot;…&quot;"`), which likely breaks selecting a specific board (default "All boards" unaffected).
- KanbanFlow behavior: board filter options carry clean board-id values and selecting a board filters the report.
- Fix direction: emit the option values without the extra quoting layer (fix the template escaping so `value` holds the raw board id).
- Evidence: spot-checker B report, `.dev/evidence/kf-drafts/SPOT-B.md` (incidental section).

### KF-319 — Task modal sits ~15px lower than KanbanFlow (4vh overlay padding vs fixed ~25px top offset) [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the KF-250 spot-checker (independent pass 2; the KF-250 verifier had noted this as a candidate separate defect in its §8 observations).
- ChipFlow behavior: `.modal-overlay` uses `padding: 4vh 1rem`, so the modal's top edge scales with viewport height (measured via DOM rect: 36px at 900px viewport, 39.9px at 998px, 24px at 600px, 20px at 500px).
- KanbanFlow behavior: GM-070 (1919×998) measures the modal at top=25px, bottom=901px (luminance edge detection, 1px resolution; see spotcheck evidence). ChipFlow at the same viewport: top=39.9px, bottom=915.1px — modal height matches within 1px (875.2 vs 876), so the whole modal is shifted down ~15px by the 4vh padding.
- Fix direction: match KanbanFlow's top offset (single data point suggests a fixed ~25px, not viewport-proportional; confirm with a second KanbanFlow viewport before implementing).
- Severity: LOW — cosmetic positioning only; nothing is cut off (KF-250's vertical-fit fix is intact).
- Evidence: `.dev/evidence/spotcheck-20261001-kf-250.md`; `.dev/evidence/spotcheck-kf250/measurements.json` (S2 1919×998); GM bbox measured from `.dev/evidence/golden-masters/task-detail-modal/GM-070-modal-open.png`.

### KF-320 — Move toolbar icon arrow shaft is heavier than KanbanFlow's (4px vs 2px) [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the KF-252 independent verifier (blind pass; zoomed glyph break-attempt).
- ChipFlow behavior: the Move (→) glyph's arrow shaft is 4px thick (measured on vertical scan through the shaft; total ink 26×19px, 184 ink px).
- KanbanFlow behavior: the Move arrow shaft is 2px thick (GM-076; total ink 20×18px, 86 ink px). Glyph color matches (#666 both).
- Fix direction: use a lighter arrow glyph matching the master's 2px shaft weight.
- Evidence: `.dev/evidence/verifier-20261001-kf-252.md`; `.dev/evidence/verifier-kf252-glyphs.png` (zoomed pair 2).

### KF-321 — Timer toolbar icon is a tick-mark stopwatch instead of KanbanFlow's plain stopwatch [LOW | OPEN | Task modal]
- Status: OPEN — filed 2026-10-01 by the KF-252 independent verifier (blind pass; zoomed glyph break-attempt).
- ChipFlow behavior: the Timer button shows a stopwatch with tick marks around the dial (ink 19×21px, 80 ink px).
- KanbanFlow behavior: the Timer button shows a plain stopwatch with a top button and no tick marks (GM-076; ink 20×24px, 150 ink px). Glyph color matches (#666 both).
- Fix direction: replace with the plain stopwatch glyph per GM-076.
- Evidence: `.dev/evidence/verifier-20261001-kf-252.md`; `.dev/evidence/verifier-kf252-glyphs.png` (zoomed pair 3).
- Repair 2026-10-02 round 3: clock svg 20->16px (one attribute, no glyph redraw) per r2 verifier prescription. Clock circle now 14x14 = golden 14x14 exact; aligned-crop pixdiff 1694px (was 1737), remainder is out-of-scope glyph artwork only. Presence/order/pitch-23.6px/permutations/hover/1440+1100 all PASS; L0/L1 (zero console errors w/ CDN pinned)/L4 green. Fixture note: POST /api/tasks has no description field — description set via PATCH (contract repro gap, not a product defect). Evidence: `.dev/evidence/repair-20261002-kf242-r3.md` + `.dev/evidence/repair-20261002-kf242-r3/`.
