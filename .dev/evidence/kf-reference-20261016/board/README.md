# KanbanFlow board, menu, column, filter and settings references (live, 2026-10-16)

Owner's logged-in Chrome, macOS, viewport 1568x600, screenshots at 0.7 scale
(1098x420). Golden Master board JYv2f3u (read-only use) and test board
"ChipFlow Visual Test" FQhhSs1. Structure, wording, options and states are
authoritative; text metrics are macOS (use the Linux golden captures for pixels).

| File | Facts |
|---|---|
| 01-board-menu-panel.jpg | "Menu" opens a FULL-HEIGHT RIGHT PANEL (light grey, title "Menu", × top-right) overlaying the right of the board. Groups separated by rules: [Filter, Reports] / [Board layout, Settings, Members] / [Recycle bin] / [Dark mode (toggle), Color legend (orange toggle ON), Large task names (toggle)] / [Help & feedback]; then a green full-width "Get Premium" button. Each item has a grey icon. KanbanFlow HAS a Dark mode toggle (account-wide; not toggled during capture). |
| 02-column-context-menu.jpg | Right-click a column header -> menu with exactly 3 items, each with an icon: Edit, Collapse, Show details. No Delete item. |
| 03-edit-column-dialog.jpg | "Edit column" dialog (×): Name; Description ⓘ [Add description]; WIP limit ⓘ [Add limit]; Column sum [None / Task count / Estimated hours]; Task sorting [None / Due date / Color / Name / Created date / Added to column — every option except None marked "[premium feature]"]; Archiving [ ] "Group tasks by the date they were added to the column"; "Task properties to display on board": Description [Hide/Show], Labels [Hide/Show], Subtasks [Hide / Show next unfinished / Show next 2..5 unfinished / Show all unfinished / Show all], Due dates [Show overdue / Show active due in 7 days / Show all], Created date [Hide/Show], Added to column [Hide/Show]; buttons Cancel and green "Update column". No Delete in this dialog either. |
| 04-new-task-modal.jpg | Column "+" opens a CENTERED modal tinted with the default color (yellow): "Task name..." input (underlined), color row "● Yellow ▾", buttons Members / Labels / Description / More; three buttons "Save & add more" (Ctrl + Shift + Enter), "Save & open details" (Shift + Enter), "Save & close" (Ctrl + Enter); × top-right. |
| 05-filter-panel.jpg | Filter uses the same right panel (← back to Menu, title "Filter", ×). User [Show all / Timer users / Unassigned / <member names>], Color [Show all / the 8 enabled colors], Date [Show all / No due date / Overdue / Due today / Due tomorrow / Due in 7 days / Due in 14 days / Due in 30 days / Due in 90 days / Due in <current month> / Due in <next month> / Due in <current year> / Due in <next year>], Labels input "Add labels..." with a list button, "Remember filter" toggle; bottom "Bookmarks (0)" collapsible section. |
| 06-settings-general.jpg | Board settings (URL /administration/board/<id>) inside the same chrome as KF-331 PLUS the dark global top bar and a dark footer "© 2011-2026 CodeKick AB. All rights reserved. \| Terms of Service \| Privacy Policy \| hello@kanbanflow.com". General: "Board name" [Edit] value; "Board owner" [Edit] value; rule; "Actions": white "Archive board", red "Delete board". |
| 07-settings-layout.jpg | Layout: light-blue info box "The board layout editor enables you to visually change the layout of your board. On that page, you can:" bullets (Add and remove columns / Rename columns / Re-order columns / Configure WIP limits / Configure how task properties are displayed on the board view / Add and remove swimlanes (premium only)); blue "Edit board layout" button. |
| 08-settings-task-settings.jpg | Task settings: "Task defaults": Color [Edit] "Yellow" (shown as a yellow swatch box), Member [Edit] "None", Due date time [Edit] "5:00 PM", Position of new tasks [Edit] "Bottom of column"; "Additional settings": Estimation unit [Switch to points] "Time", Task numbering [Activate]. |
| 09-settings-advanced.jpg | Advanced: [x] "Standard members can invite members to board", [ ] "Allow board owner to permanently delete tasks from the recycle bin". |

Gaps these expose in ChipFlow (observed by the orchestrator; verify before filing):
- New tasks: KanbanFlow default position is Bottom of column; ChipFlow inserts at the top.
- Date-only due dates: KanbanFlow default due time 5:00 PM; ChipFlow stores 23:59 ("11:59 PM").
- Column menu: ChipFlow adds a 4th "Delete" item; KanbanFlow has 3 (column delete is in the board layout editor).
- Task sorting options are premium-gated in KanbanFlow (free plan); ChipFlow offers them as working options (KF-371).
