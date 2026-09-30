# Task cards — parity defects (KanbanFlow vs ChipFlow)

Audited 2026-09-28 against deployed build 199af93c (/tmp/chipflow-audit-src).
References: kf-video-catalog v1/v3/v4/v7/v8/v9, kf-frame-review-report.md,
board screenshot (media_library/image/36/36656c…), fidelity-status.md
(Known defects + Discovered behaviors — no duplicates filed here).

Verified matches (no defect filed):
- `taskColor-{value}` background + `taskBorderColor-{value}` border classes and
  ALL 10 hex pairs exact (style.css:277-325 vs fidelity-status Discovered
  behaviors: yellow #ffffe0/#f5cc00, green #dbffc2/#59d600, blue #cce3ff/#70b0ff,
  red #ffccd0/#ff858f, orange #ffeac2/#faa200, purple #eddbff/#c994ff,
  magenta #ffe0ff/#ff85ff, cyan #dbffff/#00d6d6, brown #f6ddcb/#e49b67,
  white #fbfbfb/#d4d4d4; light variants match too).
- `taskBorderColorBg-{value}` used for legend dots + color-picker dots
  (board.html:211,231).
- `.sortable-placeholder{border:2px dashed rgba(0,0,0,.2)}` exact
  (style.css:266-271; ghostClass wiring app.js:63).
- No custom right-click menu on cards in ChipFlow (app.js:279 contextmenu
  handler returns early for non-.columnHeader) — matches KanbanFlow, which
  has none (v1-S2 note, fidelity-status).
- Done-column date-group labels ("Friday, 10 July") render (board.html:193-195).
- Accumulated-time format "1h 9m" (with space) matches the hi-res screenshot
  (task_card.html:8-10 renders `Nh Nm` with space).

## Defects

- **Add the card context menu** — Surface: cards
  - KanbanFlow: right-clicking a card opens a menu with verbatim items in
    order: **Timer** ▶ (submenu: **Start timer**, **Select in timer**),
    **Move** ▶, **Edit grouping date**, **Color** ▶, **Assign members**,
    **Copy here**, **Task URL**, **Delete** (v1-S2, v1-e003; Timer submenu
    re-verified in kf-frame-review-report §(e)7).
  - ChipFlow: no card context menu exists at all. Left-click opens the task
    modal directly (app.js:1784-1791); right-click on a card yields only the
    browser default menu. "Task URL"/"Copy" exist only inside the task modal's
    More menu (modal.html:40-41), not on the board surface.
  - Category: missing — Severity: high

- **Use a uniform colored border on cards, not a 4px left accent bar** — Surface: cards
  - KanbanFlow: cards carry a uniform ~2px border in the task's border color
    all around, rounded corners (verified on zoomed crops of the hi-res board
    screenshot — /tmp/kf-card-zoom2.png, /tmp/kf-done-card.png).
  - ChipFlow: `.task-card { border: 1px solid; border-left-width: 4px; }`
    (static/style.css:329-336) — a 1px border plus a thick left accent bar,
    a visibly different card anatomy.
  - Category: divergent — Severity: medium

- **Show a dashed outline on the timer-selected card** — Surface: cards
  - KanbanFlow: the task selected in the timer renders with a dashed outline
    on the board, even while idle (v1-e001: dashed-outline "Retro
    categorization" with timer at 25:00; v8 persistent context: "'Retro
    categorization' sits dashed/outlined in Work To-do").
  - ChipFlow: no visual marker at all. `.card-timer-running`
    (style.css:359-362) and `.card-live` (style.css:355-357) are dead styles —
    never applied by any JS or template; `<span class="card-live" hidden>`
    (task_card.html:15) is always hidden; the `.card-timer-indicator` loop
    (app.js:915) queries elements that don't exist in the markup.
  - Category: missing — Severity: medium

- **Add card metadata/footer (dates, labels, subtasks, members)** — Surface: cards
  - KanbanFlow: Edit column dialog's "Task properties to display on board"
    exposes Description, Labels, Subtasks, Due dates, Created date,
    Added to column (v1-S13); cards also carry member assignments
    ("Assign members" in the card context menu, v1-S2).
  - ChipFlow: task_card.html renders title + time/pomodori/done indicators
    only. No description, labels, subtask counts, due dates, created/added
    dates, and no member concept anywhere in the data model (models.rs,
    routes.rs have no assignee/member fields).
  - Category: missing — Severity: medium

- **Live-increment the card time badge while a session runs** — Surface: cards
  - KanbanFlow: the card's time badge live-increments during a run
    ("42m + 1m") and reverts on discard (kf-frame-review-report §P1-7,
    v4-00001–00002).
  - ChipFlow: badges are server-rendered only; app.js has no client-side
    update of card badges while a timer session is active (only the header
    pill and popup re-render).
  - Category: missing — Severity: low

- **Use a clock-outline icon for the card time badge, not ⏱ emoji** — Surface: cards
  - KanbanFlow: badge row shows a clock-outline SVG icon + gray time text
    ("🕐 14m", "🕐 1h 9m" in the hi-res screenshot zoom).
  - ChipFlow: `&#9201;` stopwatch emoji before the time
    (task_card.html:8).
  - Category: divergent — Severity: low

- **Match the tomato badge format ("🍅 1m")** — Surface: cards
  - KanbanFlow: tomato badge reads "🍅 1m" — space + "m" suffix
    (kf-frame-review-report, v1-00102 / v1-00391 / v1-02173; low-res frames,
    semantics unverified — see "could not assess").
  - ChipFlow: renders "🍅{N}" with no space and no "m" suffix, e.g. "🍅1",
    titled "N pomodori completed" (task_card.html:11-13).
  - Category: divergent — Severity: low

## Surfaces that could NOT be assessed and why

- Card context menu submenus other than Timer: the Move ▶ and Color ▶
  submenu contents were never captured in the video frames or the frame
  review, so exact submenu parity cannot be specified from references.
- "Play-icon + accumulated time" badge: the task brief cites a play icon,
  but the hi-res screenshot shows a clock-outline icon; the play icon may
  appear only on the card with a running timer (the live "42m + 1m" state).
  No frame shows a card mid-run clearly enough to resolve this.
- Tomato counter semantics: "🍅 1m" comes from low-res 1fps frames; whether
  it means pomodori count or pomodoro minutes could not be disambiguated,
  so only the format divergence is filed.
- Card hover quick-actions: v1-e001 shows a green checkbox on the
  timer-selected (dashed) card; it is unclear whether hovering any card
  reveals quick actions — unresolved in all references.
- Card drag-ghost appearance: no reference frame shows a card mid-drag
  (v1-e017 shows only a column mid-drag); only the dashed drop placeholder
  is verified.
- Done-card checkmark: the v3 catalog claims Done cards have a checkmark
  icon, but the hi-res screenshot shows no checkmarks on Done cards —
  treated as a low-res catalog error, not filed as a defect.
- Exact card border width/padding/corner radius: the screenshot supports
  "~2px uniform border" qualitatively; ChipFlow uses 1px + 4px left bar.
  Pixel-exact values would need the live KanbanFlow DOM.
