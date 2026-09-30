# Header defects — KanbanFlow vs ChipFlow (build 199af93c)

Audit date: 2026-09-28. Scope: global header only (board page `.topbar`).
References: kf-video-catalog v1/v3/v4/v8/v9, kf-frame-review-report.md, board
screenshot `media_library/image/36/36656cbadcf994fff6766d509b096a1d5253ad8e6fd80f312ca2a672b411283c.png`,
survey frames /tmp/kf-survey, ChipFlow `/tmp/chipflow-audit-src`
(templates/board.html, static/app.js, static/style.css).

KanbanFlow header structure (verified in frames): TWO rows. Dark top bar —
left: hamburger "Boards" tab + active-board tab ("General", with CS avatar chip
and "+" add-board); center: KanbanFlow logo; right: bell, "?" help, "CS" avatar
circle (v1-e001). Light board bar — left: board title "General" + CS chip + "+"
add-board circle; right: timer pill, filter funnel icon, pencil (edit layout)
icon, "≡ Menu" (screenshot PNG; v1-e001; v9-e002). ChipFlow: ONE dark bar —
left: "☰ Boards" link + board name; right: timer pill, username text, ⚙
settings link, "Log out" button (templates/board.html:5-28).

- **Add the KanbanFlow brand logo to the header** — Surface: header
  - KanbanFlow: KanbanFlow logo centered in the dark top bar (v1-e001 frame; board screenshot PNG).
  - ChipFlow: no brand mark anywhere in the board header; the only "ChipFlow" text is on the settings page (templates/settings.html:5).
  - Category: missing — Severity: medium

- **Add board tabs with active-board tab and add-board "+" to the top bar** — Surface: header
  - KanbanFlow: dark bar shows "Boards" tab plus the active board tab ("General" with CS chip and "+" add-board icon) left of the centered logo (v1-e001).
  - ChipFlow: a single "☰ Boards" link (href="/boards/new", title="All boards") and no board tabs or add-board affordance in the header (templates/board.html:7-10).
  - Category: missing — Severity: medium

- **Move the timer pill to the board-bar right, before the filter icon** — Surface: header
  - KanbanFlow: pill sits in the light board bar at the right, immediately left of the filter funnel icon (v1-e001; v9-e002; board screenshot PNG).
  - ChipFlow: pill sits in the dark top bar, left of the username text (templates/board.html:14-25).
  - Category: divergent — Severity: medium

- **Show the configured duration on the idle pomodoro pill, not the word "Pomodoro"** — Surface: header
  - KanbanFlow: idle pill reads "▶ 25:00 ▾" — green play triangle, white configured work duration, chevron (v1-e001; v3-e022; report §(e)).
  - ChipFlow: `pillLabel()` returns the literal word "Pomodoro" for every idle state; the configured duration never appears (static/app.js:937-939; markup templates/board.html:14-17).
  - Category: divergent — Severity: high

- **Add the ▾ dropdown chevron to the timer pill** — Surface: header
  - KanbanFlow: pill always shows a ▾ chevron after the time in every state (v1-e001 idle; v9-e002 running stopwatch; v1-e023 running pomodoro).
  - ChipFlow: `#timer-pill` markup contains only the status dot and time spans — no chevron element (templates/board.html:14-17).
  - Category: divergent — Severity: low

- **Show a green play triangle on the idle pill, not a red stop square** — Surface: header
  - KanbanFlow: idle pill icon is a green ▶ play triangle (v1-e001; v3-e022).
  - ChipFlow: `.timer-pill-dot` is a static `#e57373` red rounded square that never changes color or shape (static/style.css:1028-1033) — it reads as a stop icon even when idle.
  - Category: divergent — Severity: medium

- **Give the running pill a visible running state** — Surface: header
  - KanbanFlow: running pomodoro pill shows a red ■ stop icon plus countdown digits on the dark pill (v1-e023; report §(e) notes red digits and a likely subtle red fill); running stopwatch shows red ■ + counting-up "00:02" + ▾ (v9-e002).
  - ChipFlow: `renderPill()` toggles a `running` class (static/app.js:953) but no `.timer-pill.running` CSS rule exists — the pill looks identical whether idle or running.
  - Category: broken — Severity: high

- **Render the running pill as icon + countdown, not "Stop (24:56)"** — Surface: header
  - KanbanFlow: running pill shows the stop icon and the time only (v1-e023; v9-e002).
  - ChipFlow: running label is the literal string "Stop (24:56)" / paused "Resume (24:56)" (static/app.js:940-946).
  - Category: divergent — Severity: medium

- **Show "00:00" on the stopwatch-idle pill** — Surface: header
  - KanbanFlow: stopwatch-mode idle pill is a dark pill with red ■ + "00:00" + ⌄ (report §(d), v4-00142).
  - ChipFlow: idle label is always the word "Pomodoro" regardless of the selected mode tab (static/app.js:938).
  - Category: divergent — Severity: medium

- **Hide the timer pill while the timer popup panel is open** — Surface: header
  - KanbanFlow: pill is absent while the timer popup panel is open/docked — the panel occupies the pill's spot (report §(e); v9-e001).
  - ChipFlow: `renderPill()` forces `pill.hidden = false` unconditionally, so the pill stays visible above the open popup (static/app.js:944-953).
  - Category: divergent — Severity: medium

- **Render the user identity as an initials avatar circle, not plain username text** — Surface: header
  - KanbanFlow: identity is a "CS" initials avatar circle in the dark top bar (and board bar); no username text is shown (v1-e001 header crops).
  - ChipFlow: `<span class="user">{{ username }}</span>` renders the raw username as inert text (templates/board.html:21); no avatar circle, no account menu.
  - Category: divergent — Severity: low

- **Add the notifications bell icon to the top bar** — Surface: header
  - KanbanFlow: bell icon at the right of the dark top bar, left of the help icon (v1-e001 header crop).
  - ChipFlow: no notifications bell anywhere in the header (templates/board.html:5-28).
  - Category: missing — Severity: low

- **Add the help "?" icon to the top bar** — Surface: header
  - KanbanFlow: "?" in a circle at the right of the dark top bar, between bell and avatar (v1-e001 header crop).
  - ChipFlow: no help icon in the header (templates/board.html:5-28).
  - Category: missing — Severity: low

- **Add the filter funnel icon to the board bar** — Surface: header
  - KanbanFlow: filter funnel icon in the board bar right, between the timer pill and the pencil icon; opens the Filter panel (User/Color/Date/Labels, "Remember filter", Bookmarks) (v1-e001; v1 catalog S14, v1-e021).
  - ChipFlow: no filter control anywhere in the header or board toolbar (templates/board.html).
  - Category: missing — Severity: high

- **Add the edit-layout pencil icon to the board bar** — Surface: header
  - KanbanFlow: pencil icon in the board bar right, between filter and Menu; opens the layout-edit view ("Layout: General" with Add column / Add swimlane buttons — a view in which the timer pill is absent, report §(e)) (board screenshot PNG; v1-e001 crop; v1 catalog S10).
  - ChipFlow: no layout-edit view and no pencil control; column/swimlane adding lives in a separate board toolbar (templates/board.html:30-40).
  - Category: missing — Severity: medium

- **Add the "≡ Menu" button and its Reports menu to the board bar** — Surface: header
  - KanbanFlow: "≡ Menu" (hamburger + "Menu" text) at the board-bar far right; opens the Reports menu with items verbatim: Pomodoro statistics, Time spent, Print, Board history, Burndown, Calendar, Cumulative flow, Cycle & lead time, Due date performance, Monte Carlo forecasting, Task count, Throughput, Time estimate, Time in column, Export (v1-e023; v1 catalog S15).
  - ChipFlow: no Menu button and no Reports menu in the header (templates/board.html:5-28).
  - Category: missing — Severity: high

- **Remove the settings gear and Log out button from the board header** — Surface: header
  - KanbanFlow: the board header shows no settings gear and no logout button; settings/logout live behind other surfaces (v1-e001; board screenshot PNG).
  - ChipFlow: exposes a ⚙ gear link to /settings and a "Log out" submit button directly in the board header (templates/board.html:22-26).
  - Category: divergent — Severity: low

- **Make T toggle the timer popup, Y open manual time entry, P open the reports menu** — Surface: header
  - KanbanFlow: T opens/closes the timer menu (popup); Y = manual time entry; P = reports menu (frame review, v2-02558, v2-02705; v7:52).
  - ChipFlow: T immediately STARTS a pomodoro on the open task (static/app.js:1679-1682); Y smooth-scrolls to the first task card (static/app.js:1676-1678); P triggers stopClicked() (static/app.js:1683-1685). The shortcuts dialog even misdescribes T as "Open / close the timer popup" while the code starts a session (templates/board.html shortcuts dialog vs static/app.js:1681).
  - Category: broken — Severity: medium

## Assessed parity (no defect filed)

- Pill click toggles the Pomodoro popup panel: KanbanFlow's pill ▾ toggles the dark Pomodoro panel (v1 S4; v9-e001); ChipFlow's pill click toggles `#timer-popup` (static/app.js:861-867, 1051-1062). Behavior matches; only the pill's own rendering diverges (filed above).
- No separate small dropdown menu exists under KanbanFlow's pill — the ▾ chevron is an affordance for the panel toggle. No pill-dropdown menu items to compare; nothing observed in any reference.

## Surfaces that could NOT be assessed

- **Username/account menu contents**: no catalog, frame, or the board screenshot shows KanbanFlow's avatar/account menu open, so its items and order are unverifiable from the references. ChipFlow currently has no account menu at all (username is inert text; logout is a separate header button). Needs a live KanbanFlow capture of the avatar menu.
- **Timer pill in the layout-edit view**: KanbanFlow's pill is absent there (report §(e), v1-02504–02516), but ChipFlow has no layout-edit view at all (covered by the missing-pencil defect), so there is no corresponding state to compare.
- **Bell / help icon destinations**: the icons are visible in frames but no reference shows what their panels/menus contain.
