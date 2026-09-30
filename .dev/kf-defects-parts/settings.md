# Settings / colors / templates / board-list defects — parity audit (settings surface)

Audited against KanbanFlow references: video catalogs v1/v3/v4/v7/v8/v9, color-config
screenshot a8/a84a34feb48214dbac691c3187d470b2eabb47eb50674a27fed6a8a271eb7901.png,
fidelity-status.md Discovered behaviors. ChipFlow source: deployed build 199af93c at
/tmp/chipflow-audit-src. Audit-only; no code touched.

## Timer settings

- **Ship the Timer settings modal with four tabs instead of linking to /settings** — Surface: settings
  - KanbanFlow: the Pomodoro panel's Settings icon opens a "Timer settings" modal with a left
    tab list — General (selected), Interruptions, Break activities, Sounds — per v7-e023.
  - ChipFlow: the timer popup footer Settings button and the topbar gear both link to the
    full-page `/settings` form; there is no timer-settings modal anywhere
    (templates/board.html:61 links the footer Settings to `/settings`; `grep` of
    static/app.js finds no timer-settings modal markup or logic).
  - Category: missing — Severity: high

- **Rebuild the General tab with labeled dropdowns and a Picture-in-Picture toggle** — Surface: settings
  - KanbanFlow: General tab shows "Pomodoro durations" with dropdowns: "Work time" 25 minutes,
    "Short break time" 5 minutes, "Long break interval" "Every 4th break", "Long break time",
    plus a "Picture-in-Picture" toggle switched ON (v7-e023).
  - ChipFlow: /settings has bare number inputs ("Pomodoro length (minutes)", "Short break
    (minutes)", "Long break (minutes)", "Long break every N pomodori") and no PiP control
    (templates/settings.html:14–27).
  - Category: divergent — Severity: medium

- **Rebuild the Interruptions tab with drag-reorder, inline rename, and a green Add reason** — Surface: settings
  - KanbanFlow: Interruptions tab lists the 15 default reasons with drag-reorder handles,
    inline rename via click-to-yellow-highlighted input, and a green "Add reason" button
    (kf-frame-review-report.md §a; reason order verified verbatim, v4 "Why did you stop?" menu).
  - ChipFlow: /settings has a plain one-reason-per-line `<textarea>`; reorder is cut/paste,
    no inline rename, no add button (templates/settings.html:44–46).
  - Category: divergent — Severity: medium

- **Drop "Task done" from the seeded interruption reasons** — Surface: settings
  - KanbanFlow: exactly 15 default reasons, Boss interrupted … Workchat; "Task done" was
    Chip's own custom-added reason seen live in logs, not a default
    (kf-frame-review-report.md §a; v4 menu shows "Task done" after "Add new reason…").
  - ChipFlow: `Settings::default()` seeds 16 reasons including "Task done"
    (src/models.rs:260–277).
  - Category: divergent — Severity: low

- **Add the Break activities tab and its Add-break-activity dialog** — Surface: settings
  - KanbanFlow: "Break activities" tab with info text "Configure activities for your Pomodoro
    breaks to keep body and mind fresh.", a light-blue Examples box (Meditate, Take a short
    walk, Switch sitting/standing at your desk), a green "Add activity" button, and an
    "Add break activity" dialog with Name, Description, Daily goal = 1, Daily limit = "No limit"
    (v7-e024–e026).
  - ChipFlow: no break-activities UI, model, or routes exist anywhere
    (nothing in src/models.rs, src/routes.rs, or templates).
  - Category: missing — Severity: medium

- **Add the Sounds tab: ticking mode, 9 alarm sounds, volumes, and toggles** — Surface: settings
  - KanbanFlow: Sounds tab has "Ticking mode" dropdown (Always / Timer start / Never), 9 alarm
    sounds (Bell ✓, Chime, Beeps, Blip, Glass, Microwave, Egg timer, Grandpa clock, Melodic),
    "Alarm volume" 70%, "Points volume" 70%, Sounds ON, PiP ON
    (kf-frame-review-report.md §113–115).
  - ChipFlow: no sounds UI exists; /settings only offers an invented "Play a ding when a
    timer ends" checkbox with no sound choice, ticking mode, or volume sliders
    (templates/settings.html:31–40).
  - Category: missing — Severity: high

## Board Settings → Colors admin

- **Restore the Board Settings shell with left nav around the Colors page** — Surface: settings
  - KanbanFlow: "Board settings: General" page with a left nav (General, Layout, Colors,
    Task settings, Advanced, Custom fields, Custom roles, API & Webhooks, Add task from email)
    and a "← View board" back link (a8 screenshot).
  - ChipFlow: no board-settings page or nav exists; the colors admin is a standalone page
    with only a "Back to board" link (templates/board_colors.html:8–12).
  - Category: missing — Severity: medium

- **Split the color list into Enabled colors and Disabled colors sections** — Surface: settings
  - KanbanFlow: two tables — "Enabled colors" (with Edit/Disable per row) and "Disabled
    colors" (with Enable per row) (a8 screenshot).
  - ChipFlow: one combined list; enable/disable is a per-row "Enabled in picker" checkbox
    (templates/board_colors.html:14–61).
  - Category: divergent — Severity: low

- **Reorder colors by drag handles, not ▲▼ buttons** — Surface: settings
  - KanbanFlow: ⋮⋮ drag handles on each enabled-color row reorder the palette (a8 screenshot).
  - ChipFlow: per-row ▲/▼ buttons that move the color one slot and reload the page
    (templates/board_colors.html:52–56).
  - Category: divergent — Severity: medium

- **Add "Set default color" and "Copy from board" actions** — Surface: settings
  - KanbanFlow: "Set default color" and "Copy from board" buttons sit above the Enabled
    colors table, next to the green "Add custom color" button (a8 screenshot).
  - ChipFlow: neither action exists; the default is set with a per-row radio button and
    there is no palette copy between boards (templates/board_colors.html; no matching
    routes in src/routes.rs).
  - Category: missing — Severity: medium

- **Rename colors through an Edit dialog, not always-visible inline inputs** — Surface: settings
  - KanbanFlow: per-row "Edit" button opens an edit dialog that only renames (≤50 chars)
    and sets the description/legend tooltip; hex is fixed (fidelity-status.md Discovered
    behaviors; a8 screenshot).
  - ChipFlow: Label and Description are always-visible inline text inputs on every row
    (templates/board_colors.html:37–45).
  - Category: divergent — Severity: low

- **Add the premium upsell box and green "Add custom color" button** — Surface: settings
  - KanbanFlow: green "Add custom color" button above the table and a bottom box: "Looking
    for more or different colors for your tasks? … Upgrade to the premium version to get
    access to this feature." (a8 screenshot).
  - ChipFlow: neither the button nor the upsell text is present
    (templates/board_colors.html).
  - Category: missing — Severity: low

- **Match the Colors admin intro copy** — Surface: settings
  - KanbanFlow: "The colors are commonly used to indicate the type or priority of a task.
    Rename them to better represent their meaning. Show more…" (a8 screenshot).
  - ChipFlow: "Labels appear in the legend and tooltips; disabled colors are hidden from
    the task color picker. The default color is pre-selected on new tasks."
    (templates/board_colors.html:13–14).
  - Category: divergent — Severity: low

- **Show the standard color name inside the swatch** — Surface: settings
  - KanbanFlow: each row's swatch shows the standard name text ("Yellow", "Green", "Blue",
    "Red"…) inside the colored box (a8 screenshot).
  - ChipFlow: the swatch is an empty colored span with the value only in a title attribute
    (templates/board_colors.html:35).
  - Category: divergent — Severity: low

Notes (verified correct, not defects): the 10 standard hex values match KanbanFlow exactly
(yellow #ffffe0/#f5cc00, green #dbffc2/#59d600, blue #cce3ff/#70b0ff, red #ffccd0/#ff858f,
orange #ffeac2/#faa200, purple #eddbff/#c994ff, magenta #ffe0ff/#ff85ff, cyan
#dbffff/#00d6d6, brown #f6ddcb/#e49b67, white #fbfbfb/#d4d4d4; light variants
yellow #ffffe0, green #e4ffd1, blue #d6e9ff, red #ffe0e3) — src/models.rs:160–172.
Label max-50-char rule is enforced (input maxlength + client check,
templates/board_colors.html:39–41). The built-in "Pomodoro board" template carries Chip's
exact scheme (yellow "1 Pomodoro" default sortOrder 1, green "2 Pomodori" 2, blue
"3 Pomodori" 3, red ">3 Pomodori" 4) — src/db.rs:202–211. Template picker on board
creation (templates/new_board.html:32–52, with "Blank board" + "Built-in" badge +
per-template delete) and "Save as template" dialog on the board page
(templates/board.html:28,415) both exist and match the required flows.

## Board list / delete board

- **Add a persistent Boards sidebar with sections, search, and favorites** — Surface: settings
  - KanbanFlow: an always-visible left sidebar — "Boards" header, search box, "Drag to add
    to Favorites" hint, boards grouped in sections (Personal: Home, Organization & Work,
    Chats & Forums, Finances, Health, Shopping, Travel, Games & Misc, Education &
    Learning, News & Articles, Pull Requests) (/tmp/kf-survey/v4/e001.jpg).
  - ChipFlow: no sidebar; switching boards requires leaving the board for the flat "Open
    a board" list on /boards/new (templates/board.html:13 links there; the list is at
    templates/new_board.html:22–35).
  - Category: missing — Severity: medium

- **Add the Settings → Delete board flow** — Surface: settings
  - KanbanFlow: Settings → Delete board → red confirmation page → final "cannot be undone"
    dialog (fidelity-status.md Discovered behaviors).
  - ChipFlow: board deletion is entirely absent — no DELETE board route or handler, no
    delete button in any template (`grep -ri "delete board"` across templates, static,
    and src/routes.rs returns nothing outside color-slot deletion).
  - Category: missing — Severity: high

## Surfaces that could not be assessed

- **KanbanFlow's board template picker UI**: no capture of KanbanFlow's board-creation
  template picker exists in the references, so ChipFlow's picker (blank + built-in +
  deletable custom templates) could only be checked against the written requirements,
  which it meets.
- **KanbanFlow's board list page**: only the left-sidebar list is visible in frames; no
  dedicated list page was captured, so beyond the missing sidebar the page-level
  comparison is incomplete.
- **KanbanFlow's /setup and /login**: no references — both are ChipFlow inventions for
  self-hosted single-admin onboarding (KanbanFlow is SaaS). ChipFlow's setup.html
  (username/password/confirm) and login.html are reasonable on their own terms; nothing
  to compare against.
- **KanbanFlow's Board Settings sub-pages** (General, Layout, Task settings, Advanced,
  Custom fields, Custom roles, API & Webhooks, Add task from email): only the left-nav
  labels are visible in the a8 screenshot; their contents were not captured, so only the
  missing shell/nav is filed above, not per-page content gaps.
- **Break-activity behavior during an actual break transition**: no reference frame shows
  a break starting/ending, so only the settings UI gap is filed.
