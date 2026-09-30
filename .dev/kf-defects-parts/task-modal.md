# Task detail modal — parity defects (audit 2026-09-28, build 199af93c)

Reference corrections found during this audit (for the coordinator, not implementation defects):
- v7 catalog surface 12 says the Reports menu's 2nd item is "Start"; frame v7-e020 clearly shows **"Print"** with a printer icon. Correct order: Time log, Print, History, Time in column.
- v3 catalog's task-modal entry says the 4th action button is "Repeat it"; zoomed frames v3-e003 / v7-e020 show its label is **"Reports"** with a stacked-layers icon.

## Defects

- **Restore the vertical right-edge action icon column** — Surface: task modal
  - KanbanFlow: vertical column pinned to the modal's right edge — circular icon buttons with label pills, top→bottom: Add (+), Move (→), Timer (stopwatch), Reports (stacked-layers), More (•••), Delete (trash). Order/glyphs verified in zoomed frames of v7-e003, v7-e020, v3-e003 (v7 catalog surfaces 1–2).
  - ChipFlow: a horizontal row of text buttons ("Add Move Timer Reports More Delete") under the header instead of the icon column (templates/modal.html:14-50; .task-modal-actions in static/style.css:556).
  - Category: missing — Severity: high

- **Add the "Work To-do · Created: Jun 23" subtitle under the modal title** — Surface: task modal
  - KanbanFlow: bold title + subtitle "Work To-do · Created: Jun 23" + × close (v7 catalog surface 1; frames v7-e015, v8-e003).
  - ChipFlow: header is just an inline-editable name input + ×; no subtitle row anywhere (templates/modal.html:4-8).
  - Category: missing — Severity: medium

- **Implement the Subtasks section with the "Add subtask..." row** — Surface: task modal
  - KanbanFlow: "Subtasks" section header below the Color/Time-spent rows with an "Add subtask..." input row (v7 catalog surfaces 1, 8; frame v7-e015; typing visible in v7-e010).
  - ChipFlow: no subtasks UI anywhere; zero subtask code in templates/modal.html or static/app.js.
  - Category: missing — Severity: high

- **Add the 8 missing Add-menu items** — Surface: task modal
  - KanbanFlow: 10 items verbatim in order, each with a small icon: Description, Member, Label, Subtask, Due date, Time estimate, Manual time entry, Comment, Attachment, Relation (v7 catalog surface 2; frame v7-e003 verified in zoom).
  - ChipFlow: only "Manual time entry" and "Add time estimate", no item icons (templates/modal.html:17-22).
  - Category: missing — Severity: high

- **Build the Members sub-dialog (Search + gear + member rows)** — Surface: task modal
  - KanbanFlow: "Members" title, gear icon at top right, "Search..." input, rows like "● Chip Senkbeil" (v7 catalog surface 4; frame v7-e005; Members row also appears in the modal body, v8-e002).
  - ChipFlow: no members feature anywhere in the modal or app.js; no Members row in the body.
  - Category: missing — Severity: high

- **Build the Labels sub-dialog ("Add labels...", "No recently used labels exist", Save)** — Surface: task modal
  - KanbanFlow: "Labels" title, "Add labels..." input, body text "No recently used labels exist", green "Save" button (v7 catalog surface 5; frame v7-e006).
  - ChipFlow: no labels feature anywhere in the modal or app.js.
  - Category: missing — Severity: medium

- **Build the Add due date dialog (Date/Time/Repeat fields, calendar, column list, green Add)** — Surface: task modal
  - KanbanFlow: "Add due date" title; Date "2026-09-28", Time "05:00 PM"; calendar grid ("September 2028", Sun–Sat headers); column list (Personal To-do / Do today / In progress / Done / Backlog, Backlog highlighted); "Repeat (e.g. every week)" field; green "Add" (v7 catalog surface 6; frame v7-e007).
  - ChipFlow: no due-date feature anywhere in the modal or app.js.
  - Category: missing — Severity: medium

- **Make Description an Add-menu item instead of an always-visible textarea** — Surface: task modal
  - KanbanFlow: Description is an Add-menu item; the default modal body shows only Color / Time-spent rows + Subtasks (v7 catalog surfaces 1, 2).
  - ChipFlow: Description is a permanently visible labeled textarea in the modal body (templates/modal.html:60-62), and there is no Add → Description item.
  - Category: divergent — Severity: medium

- **Add Comment and Attachment Add-menu items** — Surface: task modal
  - KanbanFlow: Comment and Attachment are Add-menu items (v7 catalog surface 2).
  - ChipFlow: neither exists; a comment field only appears inside the manual-time dialog.
  - Category: missing — Severity: medium

- **Add the Relation Add-menu item** — Surface: task modal
  - KanbanFlow: Relation is the last Add-menu item (v7 catalog surface 2).
  - ChipFlow: no relation feature anywhere.
  - Category: missing — Severity: low

- **Match "Time estimate" wording and Add-menu order** — Surface: task modal
  - KanbanFlow: item reads "Time estimate" and precedes "Manual time entry" (frame v7-e003).
  - ChipFlow: item reads "Add time estimate" and follows "Manual time entry" (templates/modal.html:17-22).
  - Category: divergent — Severity: low

- **Add History and Time in column to the Reports menu** — Surface: task modal
  - KanbanFlow: Time log, Print, History, Time in column (v7 catalog surface 12; frame v7-e020 — note the v7 catalog's "Start" is a mistranscription of "Print").
  - ChipFlow: only Time log and Print (templates/modal.html:33-38).
  - Category: missing — Severity: medium

- **Open Time log as a separate in-modal view with back navigation** — Surface: task modal
  - KanbanFlow: Reports → Time log switches the modal to a "Time log" view: "← Retro categorization" back-arrow header, "Time log" title, "+ ADD ENTRY" top right, day-grouped rows ("Today 2m") with avatar + member name + duration + time range + red trash icon (v7 catalog surface 10; frame v7-e014; v3 catalog surfaces 3–4).
  - ChipFlow: the action scrolls to an inline "Time log" h3 in the same body; "+ Add entry" sits below the list; entries show minutes/badge/note/date + Edit button with no avatar, member name, or trash icon (templates/modal.html:84-86; templates/time_entries.html; static/app.js:672-673).
  - Category: divergent — Severity: medium

- **Match the Time spent row format ("● 2 Pomodori" / "2m")** — Surface: task modal
  - KanbanFlow: labeled rows "Color" and "Time spent"; Time spent reads "● 2 Pomodori" and "2m" (v7 catalog surface 1; frames v7-e002, v7-e015, v7-e021).
  - ChipFlow: an icon stats block — "🍅 N Pomodori", "⏱ X spent", "⚠ N interruptions", "Created ..." (templates/modal.html:64-76); no "Time spent" labeled row in KanbanFlow's format.
  - Category: divergent — Severity: medium

- **Show the "You are selecting this task" hover tooltip on Time spent** — Surface: task modal
  - KanbanFlow: hovering the "Time spent" label/row shows a small tooltip reading "You are selecting this task" (v7 catalog surface 13; frame v7-e021).
  - ChipFlow: stat spans carry native title attributes ("Pomodori completed", "Time spent", "Interruptions") instead (templates/modal.html:65-68).
  - Category: missing — Severity: low

- **Rewrite the keyboard shortcuts dialog with KanbanFlow's 9 rows** — Surface: task modal
  - KanbanFlow: 9 rows verbatim — Open Move dialog V; Open Timer menu T; Open Reports menu P; Open More menu .; Navigate subtask list ↑ ↓ ("Moves focus to the subtask above or below the focused subtask"); Move subtask in list Cmd + ↑ ↓ ("Moves the focused subtask up or down in the list"); Close window / discard changes Esc ("If you have unsaved changes, pressing ESC will discard those changes. Otherwise ESC will close the task details window."); Save changes Cmd + Enter ("Saves any changes and leaves editing. Works for popups and forms with a 'Save' button."); Delete task Delete (v7 catalog surface 11; frame v7-e018 verified verbatim).
  - ChipFlow: 6 unrelated rows — Y / T / P / E / Enter / Esc with different meanings (templates/board.html:397-413), none matching KanbanFlow's bindings.
  - Category: divergent — Severity: high

- **Fix shortcut keys that disagree with the shortcuts dialog text** — Surface: task modal
  - KanbanFlow: n/a — ChipFlow-internal inconsistency, but it makes the (already wrong) dialog actively misleading.
  - ChipFlow: dialog says Y = "Add time entry manually" but app.js scrolls to the first task card (static/app.js:1677-1679); T = "Open / close the timer popup" but the code starts a Pomodoro (1680-1682); P = "Open pomodoro statistics" but the code stops the timer (1683-1684); Enter = "Open the focused task card" but no Enter handler exists in the keydown listener; "?" opens the shortcuts dialog yet is not listed (static/app.js:1670-1700 vs templates/board.html:402-407).
  - Category: broken — Severity: medium

- **Make the Move task dialog's Move button green and add the × close** — Surface: task modal
  - KanbanFlow: "Move task" title + × close; Board dropdown ("General", blue selected); "Column" label + dropdown ("Work To-do"); green "Move" button (v7 catalog surface 9; zoomed frame v7-e012).
  - ChipFlow: no × (Cancel button instead); Move is blue btn-primary, not green (templates/board.html:356-380; .btn-primary is blue in static/style.css:862; KanbanFlow's green matches .btn-success, :871).
  - Category: divergent — Severity: low

- **Add the Board dropdown to the Move task dialog** — Surface: task modal
  - KanbanFlow: Move task dialog has a Board dropdown with "General" selected (v7 catalog surface 9; frame v7-e012).
  - ChipFlow: dialog lists only the current board's columns; no board selection (static/app.js:717-729).
  - Category: missing — Severity: low

- **Make "Watch" actually do something instead of a "not supported" toast** — Surface: task modal
  - KanbanFlow: More menu = Watch, Task URL, Copy, Keyboard shortcuts — Watch is a working feature (status-file Discovered behaviors; v7 catalog surfaces for the More menu).
  - ChipFlow: the item exists but modalAction("watch") only shows toast('Task watching is not supported yet.') (static/app.js:676-677).
  - Category: broken — Severity: medium

- **Add label support to the manual time entry dialog** — Surface: task modal
  - KanbanFlow: "Add time manually" has "Add comment" and "Add labels" checkboxes, a Comment text field, and a Labels field with suggestions ("test", "another test", "Add labels…"), green Add (v3 catalog surface on the dialog; v3-e001–e008; v7 catalog surface 7, frame v7-e009).
  - ChipFlow: only a "+ Add comment" toggle with comment textarea; no labels support anywhere (templates/board.html:77-109).
  - Category: missing — Severity: medium

- **Fix Add time estimate dialog nits (× close, clock placeholder, green Add)** — Surface: task modal
  - KanbanFlow: "Add time estimate" title with ×; text input with clock placeholder; green "Add" (v7 catalog surface 3; frame v7-e004).
  - ChipFlow: Cancel button instead of ×, plain placeholder "0h", blue btn-primary Add instead of green (templates/board.html:382-396).
  - Category: divergent — Severity: low

- **Match the Color row presentation (single color dot under the label)** — Surface: task modal
  - KanbanFlow: "Color" label with a single color dot under it in the body (frame v7-e015).
  - ChipFlow: an always-visible row of selectable color dots (templates/modal.html:55-56; buildModalColorPicker in static/app.js:599-618).
  - Category: divergent — Severity: low

### Matched (no defect)
- Timer submenu items match verbatim: Start Pomodoro / Start Stopwatch / Time log (templates/modal.html:28-32; status-file Discovered behaviors).
- More menu items match verbatim: Watch, Task URL, Copy, Keyboard shortcuts (templates/modal.html:42-47).
- Modal tints with the task's color (style="background-color: var(--taskColorLight)" in templates/modal.html:4-5; taskColorVars-* blocks in static/style.css:279-324; live retint in setModalColor, static/app.js:621-646).
- × close button exists in the modal header (templates/modal.html:7).

## Surfaces that could NOT be assessed (and why)
- **Delete confirmation flow**: no KanbanFlow delete-confirmation reference exists in v1/v3/v4/v7/v8/v9 catalogs or the status file (only the trash icon and the "Delete task — Delete" shortcut row are documented). ChipFlow uses a native window.confirm('Delete this task and its time entries?') (static/app.js:588-596); cannot verify whether KanbanFlow shows a styled confirmation dialog or deletes immediately.
- **Timer-menu live behavior** (what happens after Start Pomodoro / Start Stopwatch from the modal): no frame shows the task-modal Timer menu open in KanbanFlow; only the menu item list is documented in the status file.
- **Add-menu sub-dialogs for Description / Comment / Attachment / Relation**: beyond their menu-item labels, the catalogs capture no frames of these editors/dialogs in KanbanFlow, so only their absence can be filed, not their content.
- **Members row presence rules**: the Members row appears in the modal body in v8-e002/e003 but not in v7 frames; the catalogs do not document when it is hidden vs shown (likely hidden when no members are assigned — unverified).
- **Modal opening/closing transitions and overlay-click behavior**: no frame captures these; ChipFlow closes on overlay click (static/app.js:798-801), unverified against KanbanFlow.
