# KanbanFlow timer references (live, 2026-10-16)

Captured in the owner's logged-in Chrome on the KanbanFlow test board
"ChipFlow Visual Test" (https://kanbanflow.com/board/FQhhSs1), window viewport
1568x600 (screenshots scaled to 0.7, so 1098x420; multiply coordinates by
1/0.7 for the viewport). macOS Chrome, so fonts are San Francisco/Helvetica,
not the Linux captures' Liberation Sans: use these for structure, wording,
colors and states, not for pixel-exact text metrics. One stopped pomodoro (30s,
reason "Email") was logged on "Task Beta" on that test board; the timer was left
idle with no task selected.

| File | State and facts |
|---|---|
| 01-idle-popup-no-task.jpg | Board-header clock icon (left of the funnel) opens a dark panel anchored under it: title "Pomodoro" + ^ collapse; "Time until break"; large "25:00"; green "▶ Start" button; grey bar "No task selected"; underlined link "Select task"; footer row of 4 icon+label buttons: Stopwatch, Add time, Log, Settings. No timer pill anywhere in the dark top bar. |
| 02-select-task-mode.jpg | "Select task" closes the panel and shows a centered white pill "Select task ×" in the board header; clicking a card selects it. |
| 03-idle-task-selected-card-outline.jpg | Task selected, not running: panel shows a row "● Task Beta ×" (dot in the TASK's color) and link "Change task"; the selected card already has the dashed orange outline (no solid border underneath visible); tab title "25:00 <board> - KanbanFlow". |
| 04-running-header-chip-avatar-badges.jpg | Running, panel closed: the board-header clock icon is replaced by a dark chip "■ 24:55 ⌄" (red square stop glyph, white digits, chevron), still in the BOARD HEADER row next to the funnel (not in the dark top bar). The running card shows the dashed outline and the member avatar "CS" gains a small play badge; the board-header member disc "CS" also gains a play badge. Tab title counts down ("24:55 <board> - KanbanFlow"). |
| 05-running-popup-today-pending.jpg | Running panel: "Time until break", "24:47", red "■ Stop" button, task row "● Task Beta" (no × while running), "Change task"; section "TODAY" lists "● Task Beta" / italic "5:43 PM – pending" with a ⋮ menu per entry. No day total shown while nothing has completed. |
| 06-why-did-you-stop.jpg | Stop after >20s replaces the panel with "‹ Why did you stop?" and a dark scrollable list: Boss interrupted, Colleague interrupted, Context switch, Dog, Email, Family, Finished with no new task, Food Delivery, Meeting, Other, ... |
| 07-after-stop-today-entry.jpg | After picking a reason: back to idle WITH the task still selected (row "● Task Beta ×", "Change task", green Start, "25:00"); TODAY lists "● Task Beta" / "5:43 PM – 5:43 PM" with ⋮; the dot is the task's color (yellow here). |
| 08-settings-general.jpg | "Timer settings" modal, left tabs General / Interruptions / Break activities / Sounds. General: "Pomodoro durations": Work time 25 minutes, Short break time 5 minutes, Long break interval "Every 4th break", Long break time 15 minutes (defaults); "Picture-in-Picture ⓘ" orange toggle ON. |
| 09-settings-interruptions.jpg | "Interruption reasons": list rows with a ⋮ handle each (Boss interrupted ... Meeting, Other ...). |
| 10-settings-break-activities.jpg | "Break activities": light-blue info box "Configure activities for your Pomodoro breaks to keep body and mind fresh." + "Examples:" - Stretch / - Drink water / - Take a short walk / - Switch sitting/standing at your desk; green "Add activity" button. |
| 11-settings-sounds.jpg | "Sounds" with an orange master toggle ON; Ticking mode "Never"; Alarm sound "Bell"; Alarm volume 70 %; Points volume 70 %. |
| 12-timer-log.jpg | "Timer log" modal: filters "All boards", "Period: This + Last week", "Entry type: All", icons print / download / (time) / chart. Day block "Today 30s"; entry "● Task Beta  5:43 PM - 5:43 PM  30s ⋮" with a red italic sub-line `Stopped Pomodoro with reason "Email"`; blue "Add time entry" button. |
| 13-timer-log-export-menu.png | Download icon menu: "Download Excel file", then "Download CSV file". |

Also from the KanbanFlow golden board (read earlier today): time log entry types
are letters P (pomodoro: red when stopped, green when completed), S (stopwatch),
M (manual entry).
