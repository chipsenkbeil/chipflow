# Columns — parity defects (KanbanFlow vs ChipFlow)

Audited 2026-09-28 against deployed build 199af93c (/tmp/chipflow-audit-src).
References: kf-video-catalog v1/v3/v4/v7/v8/v9, board screenshot
(media_library/image/36/36656c… — hi-res, used for header-anatomy crops),
fidelity-status.md (Known defects + Discovered behaviors — no duplicates
filed here; the two Chip-filed defects are non-column).

Verified matches (no defect filed):
- ⋮ menu items and order: Edit, Move left, Move right, Add to left,
  Add to right, Delete (board.html:258-265 vs v1-S12 verbatim).
- Right-click menu items: Edit, Collapse, Show details
  (board.html:268-272 vs fidelity-status).
- Add column dialog fields: Name + Position combobox, in that order
  (board.html:289-308 vs fidelity-status).
- "Show details" popup rows: "Column:" / "Task count:"
  (board.html:283-287 vs fidelity-status).
- WIP count format "0 / 3" with spaces around the slash (board.html:172
  vs hi-res screenshot "In progress 0 / 3").
- Count div title="*LIMIT EXCEEDED*" on violation (board.html:172 vs
  fidelity-status DOM note).
- Column drag placeholder: `border:2px dashed rgba(0,0,0,.2)` exact
  (style.css:266-271; ghostClass wiring app.js:140) vs KanbanFlow's
  `.sortable-placeholder` token.
- Delete refuses while tasks remain: server 400 "Column still holds tasks"
  (routes.rs:1902-1910); the confirm text says so (app.js:376-386).
- Edit dialog has Name, Description, WIP limit, Task sorting fields in
  KanbanFlow's order (board.html:311-335 vs v1-e019).

## Defects

- **Show WIP-limit violation in darkred with a red warning line, not amber** — Surface: columns
  - KanbanFlow: exceeded limit shows "3 / 2" in darkred + red warning line;
    tokens --board-headerWarningTextColor:darkred,
    --board-warningLineColor:#ff8080; header background unchanged
    (fidelity-status Discovered behaviors).
  - ChipFlow: amber theme instead — header bg #fffbeb, border #f59e0b,
    count #b45309, line #ef4444 (style.css:171-184); the two tokens are
    absent from style.css entirely.
  - Category: divergent — Severity: medium

- **Warn on WIP limit only when the limit is exceeded, not when reached** — Surface: columns
  - KanbanFlow: warning is a *violation* — observed on "3 / 2", count div
    titled "*LIMIT EXCEEDED*" (fidelity-status); "exceeded" implies
    count > limit. (Caveat: no video frame captured a count == limit
    state, so the at-limit rendering is inferred, not frame-verified.)
  - ChipFlow: warns when count >= limit — "2 / 2" already shows the
    warning state (routes.rs:733-736 `count as i64 >= limit`;
    app.js:117 `count >= wip`).
  - Category: divergent — Severity: medium

- **Render the header count inline with the name, not on a second line** — Surface: columns
  - KanbanFlow: name + gray count on one line, e.g. "In progress 0 / 3";
    count-only "0" at the header's right edge when no WIP limit
    (hi-res screenshot header crops).
  - ChipFlow: `.columnHeader-count{display:block}` on its own line below
    the name (style.css:142-147; board.html:170-172).
  - Category: divergent — Severity: medium

- **Make the column add-task button big and green** — Surface: columns
  - KanbanFlow: large bright-green "+" button in each column header
    (hi-res screenshot; v3-S1 "green '+' add button in the column header").
  - ChipFlow: small muted-gray "+" (style.css:150-165,
    `color:var(--muted)`; board.html:171).
  - Category: divergent — Severity: medium

- **Hide the ⋮ button on board column headers; open the menu on right-click** — Surface: columns
  - KanbanFlow: board column headers show no ⋮ affordance — the 6-item
    menu opens on right-click (hi-res screenshot shows no ⋮; v1-S12).
    (Items/order already match; this is about the trigger.)
  - ChipFlow: an always-visible ⋮ button on every header, at the far
    right where KanbanFlow puts the count (board.html:173;
    style.css:151,165).
  - Category: divergent — Severity: low

- **Show the red task count on collapsed column strips** — Surface: columns
  - KanbanFlow: collapsed strip shows the task count in red, e.g. "3"
    (fidelity-status Discovered behaviors).
  - ChipFlow: `.columnHeader--collapsed .columnHeader-count{display:none}`
    (style.css:195-197) — the count is never rendered on collapsed
    columns. (Caveat: the hi-res screenshot's two collapsed strips show
    no count, but both columns appear empty there; the status-file
    behavior is the reference.)
  - Category: divergent — Severity: medium

- **Uppercase the collapsed column name** — Surface: columns
  - KanbanFlow: collapsed strip shows the name vertical and UPPERCASE in
    gray ("PERSONAL TO-DO", "BACKLOG") (hi-res screenshot).
  - ChipFlow: vertical name rendered as-is, no text-transform
    (style.css:188-193).
  - Category: divergent — Severity: low

- **Insert "Add to left/right" adjacent to the column, not at the board ends** — Surface: columns
  - KanbanFlow: Add to left / Add to right insert a column immediately
    left/right of the column whose menu was opened (menu semantics,
    v1-S12).
  - ChipFlow: both actions just preselect the dialog's Position as
    'beginning'/'end' — the new column lands at the board's beginning or
    end regardless of which column's menu was used (app.js:298-299,
    openAddColumnDialog at app.js:389-395; the Position select only offers
    "At the end"/"At the beginning", board.html:296-301).
  - Category: broken — Severity: medium

- **Load the Edit dialog with the column's saved values instead of defaults** — Surface: columns
  - KanbanFlow: the dialog reflects the column's actual saved settings.
  - ChipFlow: openEditColumnDialog hard-resets sorting to 'none' and
    unchecks column-sum/group-by-date while checking all show/hide boxes
    (app.js:422-431, with a comment admitting it "edits them from
    defaults"); saving then silently discards the stored config_json
    values — editing a column's name wipes its grouping/sorting/display
    settings.
  - Category: broken — Severity: medium

- **Rebuild "Task properties to display on board" as per-property dropdowns** — Surface: columns
  - KanbanFlow: six per-property dropdowns — Description, Labels,
    Subtasks, Due dates ("Show active due in 7 days"), Created date,
    Added to column (v1-e019 enlarged frame).
  - ChipFlow: a "Show / Hide" fieldset with three CHECKBOXES —
    Description, Task count, WIP limit (board.html:339-346). Different
    control type, five of six KanbanFlow properties missing, two
    non-KanbanFlow toggles added.
  - Category: divergent — Severity: high

- **Restore the "Archiving" checkbox with its end-state note** — Surface: columns
  - KanbanFlow: "Archiving" checkbox labeled "Group tasks by the date
    they were added to the column" plus a blue info note: "Archiving is
    ONLY recommended for columns that represent an end state for tasks,
    like the Done column. Loads the 20 most recent tasks from the start."
    (v1-e019).
  - ChipFlow: checkbox relabeled "Group tasks by date"; the note is
    missing entirely (board.html:336-338).
  - Category: divergent — Severity: medium

- **Make Column sum a dropdown, not a checkbox** — Surface: columns
  - KanbanFlow: Column sum is a dropdown showing "None" (v1-e019).
    (Its option list wasn't captured; only the control type is verified.)
  - ChipFlow: a checkbox (board.html:337).
  - Category: divergent — Severity: low

- **Label the Edit dialog button "Update column" in green** — Surface: columns
  - KanbanFlow: Cancel + green "Update column" (v1-e019).
  - ChipFlow: Cancel + blue "Save" (board.html:349-352).
  - Category: divergent — Severity: low

- **Replace window.confirm() with a styled delete-confirmation dialog** — Surface: columns
  - KanbanFlow: deleting a column goes through an in-page confirmation
    dialog (fidelity-status: "Delete via confirmation dialog").
  - ChipFlow: native `window.confirm()` (app.js:376-386).
  - Category: divergent — Severity: low

- **Render the dragged column blank/empty mid-drag** — Surface: columns
  - KanbanFlow: the column being dragged renders blank/empty while the
    cursor holds it, with a gap at the drop position (v1-e017).
  - ChipFlow: no dragClass wired — SortableJS drags a full-opacity clone
    of the column; the defined `.drag-ghost{opacity:.4}` rule
    (style.css:274) is never applied (app.js:133-161).
  - Category: divergent — Severity: low

- **Use a flat light-gray column header, not a white card** — Surface: columns
  - KanbanFlow: headers are flat light-gray table cells (hi-res
    screenshot).
  - ChipFlow: white rounded cards with border (`background:#fff;
    border:1px solid #e2e5ea; border-radius:8px`, style.css:120-130).
  - Category: divergent — Severity: low

## Surfaces that could NOT be assessed (and why)

- Narrow trailing header cell: in the hi-res screenshot each board column
  header is followed by a narrow cell showing "0" (blank for the empty
  "Do today") — its meaning is unverified (not task count: "Work To-do"
  holds 1 task yet shows "0"), so no defect filed; ChipFlow has no
  equivalent cell.
- Add-column dialog Position options: no video frame captured the dialog
  itself — only the field set (Name + Position combobox) is verified from
  the status file. KanbanFlow's option list is unknown.
- Delete-with-tasks behavior in KanbanFlow: beyond "confirmation dialog",
  what KanbanFlow does to tasks in a deleted column was not captured.
- "Show details" popup styling: content matches ("Column:" /
  "Task count:"); visual styling unverified (no frame captured it).
- Task-sorting dropdown options: KanbanFlow shows "None"; the full option
  list wasn't captured, so ChipFlow's Manual order/Name/Date created set
  can't be judged.
- Column sum dropdown options: control type verified (dropdown), options
  unknown.
- Delete menu-item color: whether KanbanFlow renders "Delete" in red
  couldn't be discerned from v1-e018; ChipFlow styles it .menu-danger.
- KanbanFlow's Boards > Layout editor view (v1-e016–e018: column list,
  drag-reorder there, per-header ⋮) is a separate surface not covered by
  this audit — ChipFlow has no layout-editor view; its board toolbar
  "+ Add column" button is an extra entry point with no KanbanFlow
  board-view equivalent.
- WIP warning at exactly count == limit: inferred from the
  "*LIMIT EXCEEDED*" DOM title, not frame-verified (noted in the defect).
