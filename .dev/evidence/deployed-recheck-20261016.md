# Deployed-tree recheck of the 8 DEPLOYED defects — 2026-10-16

Independent verifier (built none of this code). Verification only: no code,
tracker, or production changes were made.

- Deployed tree inspected: `2246458c748cbcd516d3bc815323ffbecd26dbff`
  (`git rev-parse 2246458` = `2246458c748cbcd516d3bc815323ffbecd26dbff`;
  `origin/main` tip `2aa422eb` is a tracker-only commit on top of it).
- Method: `git show 2246458:<path>` / `git grep <pat> 2246458` against git
  objects only (working tree has unrelated uncommitted changes; untouched).
- Production cross-check (green-box `:3000` only, never `:3100`):
  - `~/workspace/bin/ssh green-box "curl -s http://127.0.0.1:3000/api/v1/version"`
    → `{"version":"0.1.0","build_sha":"2246458c748cbcd516d3bc815323ffbecd26dbff"}`
  - Served `static/app.js` sha256 `9c37ea9278b4423dfe73d9dc039c56e9a79c934bb4d34385af45077f999b1235`
    = tree `git show 2246458:static/app.js | sha256sum` — byte-identical.
  - Served `static/style.css` sha256 `17bac44aa21802a39f4858e58295e5d3aa5c25d7f2ef34c7dd552759f5040084`
    = tree — byte-identical.
  So tree findings below are the live served behavior for static assets.

---

## KF-293 — Pomodoro statistics presets, labels, tooltip — **PARTIAL**

Requirement (tracker, first occurrence): Period presets must match KanbanFlow
(the two masters conflict — GM-124: '1 day','2 days','3 days','7 days',
'14 days','This week','Last week','This month'; GM-130: "Last 7 days",
"Last 14 days", "Last 30 days" (default), "This week", "Last week",
"This month", "Last month", "Custom (absolute)", "Custom (relative)" — fix
direction says reconcile against live KanbanFlow; the verified BUILT lines
adopt the GM-130 list); bar count labels atop bars; bar tooltip format
`"Tuesday 1 Sep 2026: 2 Pomodori (30m)"` (weekday, day month year, capital
"Pomodori", duration in parentheses); `DayPomodori.minutes` backing the
duration. History: the Round-7 DEPLOYED marker was FALSE — fix absent from
Round-7 (13857446) and Round-8 (11d7734f) trees; reopened 2026-10-06T05:20Z.

Evidence — PRESENT parts:

- Statistics Period presets match GM-130 verbatim, in
  `templates/timer_statistics.html:13-23`:
  ```
  <select id="stats-period-filter">
    <option value="last-7">Last 7 days</option>
    <option value="last-14">Last 14 days</option>
    <option value="last-30" selected>Last 30 days</option>
    <option value="this-week">This week</option>
    <option value="last-week">Last week</option>
    <option value="this-month">This month</option>
    <option value="last-month">Last month</option>
    <option value="custom-absolute">Custom (absolute)</option>
    <option value="custom-relative">Custom (relative)</option>
  </select>
  ```
- `DayPomodori.minutes` exists and is populated in Rust:
  `src/routes.rs:463-468` struct fields `date, label, pomodori, minutes: i64`;
  construction at `src/routes.rs:4141-4147`:
  `minutes: daily_minutes.get(&key).copied().unwrap_or(0)`.

Evidence — ABSENT parts:

- Tooltip is still the OLD format the defect was filed against. Deployed
  `static/app.js` (TimerStatsPage.render, ~line 5758):
  ```js
  var unit = d.pomodori === 1 ? 'pomodoro' : 'pomodoros';
  return {
    label: d.label,
    minutes: d.pomodori,
    tooltip: weekday(d.date) + ', ' + d.label + ': ' + d.pomodori + ' ' + unit,
  };
  ```
  With `label` built server-side as `d.format("%b %d")` (`src/routes.rs:4144`),
  this yields exactly the filed-against string form "Thursday, Oct 01:
  4 pomodoros" — no year, lowercase "pomodoros", and NO duration: the JS
  never reads `d.minutes` (it even maps the count into a field named
  `minutes`). Searches: `git grep -n 'Pomodori (' 2246458 -- static/app.js`
  → 0 matches; same search against the live served app.js on green-box
  `:3000` → 0 matches.
- No bar count labels. `renderBarChart` (`static/app.js:7996-8038`) emits
  only gridline y-axis value labels and the day label under each bar
  (`<text ... y=H-10 ...>b.label</text>`); the bar itself is a `<rect>`
  with a `<title>` child. There is no `<text>` element above any bar
  carrying the count.
- Note: the old preset list cited in the defect still exists verbatim in a
  different surface — TimerLogPage's stepper `PERIODS` at
  `static/app.js:4964-4972` starts
  `{ value: 'this-plus-last-week', label: 'This + Last week' }` (confirmed
  present once in the live served app.js). That stepper belongs to the
  timer-log surface (KF-286 comments); the statistics page's own select
  (above) is the GM-130 list. Filed for the repair batch to decide scope.

Verdict: **PARTIAL** — presets (GM-130 reconciliation) and the Rust
`minutes` field are in the deployed tree; the tooltip format and the bar
count labels — two of the defect's three fix components — are ABSENT.
The Oct-6 reopen's core claim ("JS fix absent") is still true for the
tooltip/labels in the current production tree.

---

## KF-005 — Populate the "Why did you stop?" menu — **PRESENT**

Requirement: menu populated with KanbanFlow's items verbatim in order:
Boss interrupted, Colleague interrupted, Context switch, Dog, Email, Family,
Finished with no new task, Food Delivery, Meeting, Other, Phone call,
Restroom, Sleep, Web browsing, Workchat, "Add new reason…", "Task done"
(15 stored reasons + the two UI items), including backfill for legacy
settings with an empty list.

Evidence (`2246458`):

- `src/models.rs:481-497` — `Settings::default().interrupt_reasons` is
  exactly the 15 reasons, verbatim and in order (`"Boss interrupted"`,
  `"Colleague interrupted"`, `"Context switch"`, `"Dog"`, `"Email"`,
  `"Family"`, `"Finished with no new task"`, `"Food Delivery"`, `"Meeting"`,
  `"Other"`, `"Phone call"`, `"Restroom"`, `"Sleep"`, `"Web browsing"`,
  `"Workchat"`).
- `src/db.rs:2680-2686` — backfill in `get_settings`:
  ```rust
  // KF-005: backfill KanbanFlow's verbatim interruption reasons for
  // databases whose settings predate them (empty list would render
  // an empty "Why did you stop?" menu).
  if settings.interrupt_reasons.is_empty() {
      settings.interrupt_reasons = Settings::default().interrupt_reasons;
  }
  ```
- `static/app.js:3624` `renderWhyReasons` (called from `beginWhy` after
  `ensureSettings` fetches `/api/settings`) renders one button per reason,
  then appends `<button ... id="why-add-new">Add new reason…</button>` and
  `<button ... id="why-task-done">Task done</button>` as the final item,
  filtering any stored "Task done" out of the reason list so it appears
  exactly once, last.

Verdict: **PRESENT** — all 17 menu items, order, and the legacy backfill
match the requirement.

---

## KF-295 — Print button opens native dialog instead of styled sheet — **PRESENT**

Requirement: the Time spent report's Print button must open a styled sheet
titled "Print" showing the task table with Filter/Print buttons (GM-169),
not a bare `window.print()`.

Evidence (`2246458`):

- `templates/timer_log.html:87-96`:
  ```html
  <div id="spent-print-overlay" class="spent-print-overlay" hidden>
    <div class="spent-print-sheet" role="dialog" aria-modal="true" aria-label="Print">
      <header class="spent-print-head">
        <h2>Print</h2>
      ...
        <button type="button" id="spent-print-filter" class="btn btn-sm" title="Filter">Filter</button>
        <button type="button" id="spent-print-do" class="btn btn-sm" title="Print">&#128438; Print</button>
      ...
      <div id="spent-print-body" class="spent-print-body"></div>
  ```
- `static/app.js:5570-5593`: `qs('spent-print').addEventListener('click',
  openSpentPrintSheet);` — the sheet fills `spent-print-body` and unhides
  the overlay; only the sheet's own Print button fires `window.print()`;
  the Filter button closes the sheet back to the report (filters live
  there). Sheet CSS at `static/style.css:5058-5116` including an
  `@media print` block scoped to the sheet.

Verdict: **PRESENT**.

---

## KF-326 — Done column flat "Completed" list instead of date-grouped headers — **PRESENT**

Requirement: Done-column cards grouped under date headers ("Today",
"Wednesday, 30 September" convention) instead of a flat list under a
"Completed" label.

Evidence (`2246458`):

- `src/routes.rs:1012-1021`:
  ```rust
  /// Grouping label for completed tasks: Today / Yesterday / "Friday, 10 July".
  fn done_group_label(rfc3339: &str) -> String {
      let today = Local::now().date_naive();
      match ... {
          Ok(date) if date == today => "Today".to_string(),
          Ok(date) if date == today - Duration::days(1) => "Yesterday".to_string(),
          Ok(date) => date.format("%A, %-d %B").to_string(),
  ```
- `CellView.done_groups: Vec<DoneGroup>` (`src/routes.rs:1027-1038`) —
  Done cells carry grouped tasks, and `templates/board.html:608-613`
  renders, for done cells only:
  ```html
  {% for group in cell.done_groups %}
  <div class="done-group-label">{{ group.label }}</div>
  {% for task in group.tasks %}{% include "task_card.html" %}{% endfor %}
  {% endfor %}
  ```
  Styled by `.done-group-label` at `static/style.css:865`.

Verdict: **PRESENT**.

---

## KF-331 — Board settings page chrome (H1, background, card styling) — **PARTIAL**

Requirement: H1 convention "Board settings: <board name>" (was hardcoded
"General"); page background and card styling per GM-134. The defect itself
carries the caveat that the two captures showed different tabs, so tab
content was not directly comparable and an aligned re-capture was needed.

Evidence (`2246458`):

- H1 — PRESENT. `templates/board_settings.html:4-8`:
  ```html
  <div class="page settings-page" data-board-id="{{ board_id }}">
    <div class="page-header">
      {# KF-153: the H1 names the current board (was hardcoded "General"). #}
      <h1>Board settings: {{ board_name }}</h1>
  ```
- Card styling exists: `.settings-section { background: #fff; border:
  1px solid #e5e7eb; border-radius: var(--radius); padding: 1rem 1.2rem; }`
  (`static/style.css:3862-3868`), page shell `.settings-page` at
  `style.css:3848-3852`, plus the `.bshell` nav/content shell
  (`style.css:4570-4595`).
- NOT PROVABLE from the tree: the defect never specifies target values for
  background or card styling (no hex/size values are stated for GM-134
  anywhere in the section), and no GM-134 spec/evidence file exists in the
  deployed tree (`git ls-tree -r 2246458 -- .dev/evidence | grep -i 134`
  → empty). The tracker's own RELEASED content-verification (2026-10-06)
  likewise cited only the H1. An exact background/card match to GM-134
  therefore cannot be established by content inspection; it needs the
  aligned same-tab screenshot re-capture the defect itself called for.

Verdict: **PARTIAL** — H1 convention PRESENT and verified in the tree;
background/card styling present as white bordered cards but its match to
GM-134 is NOT PROVEN (no stated target values exist to check against).

---

## KF-333 — Card Color submenu shows pomodoro-count labels — **PRESENT**

Requirement: card Color submenu lists the 8 plain color names — Yellow,
Green, Blue, Red, Orange, Purple, Magenta, Cyan — each row tinted its
color, instead of "1 Pomodoro / 2 Pomodori / 3 Pomodori / >3 Pomodori" for
the first four.

Evidence (`2246458`):

- `src/models.rs:311-322` `STANDARD_COLORS` default labels are exactly
  `"Yellow", "Green", "Blue", "Red", "Orange", "Purple", "Magenta",
  "Cyan"` (then Brown, White).
- `src/db.rs:202-215` `pomodoro_color_specs()` seeds every new board's
  color rows with those same 8 labels (no pomodoro-count labels).
- `src/routes.rs:1076,1147` — the colors API/view carries `standard_name`
  resolved from `STANDARD_COLORS`.
- `templates/board.html:750` — `#board-colors` spans carry
  `data-label="{{ color.label }}"`; the card context submenu builder,
  `static/app.js:671-681`, builds each row from `s.dataset.label ||
  s.dataset.value` and tints it: `b.style.borderLeft = '0.9rem solid ' +
  (s.dataset.bg || '#fff')`.

Note: the submenu text is the board-configured label by design (KanbanFlow
allows renaming colors); the defaults — what the Golden Master board and
any un-renamed board show — are the 8 names. A board whose labels a user
deliberately renamed would show its own names, which is parity behavior,
not this defect.

Verdict: **PRESENT**.

---

## KF-334 — Card Move submenu flat column list — **PRESENT**

Requirement: Move submenu offers KanbanFlow's relative-move options
"Move right", "Move to bottom", "Move advanced" instead of a flat list of
destination column names.

Evidence (`2246458`) — `static/app.js:655-669`:
```js
} else if (act === 'move') {
  openCardSubmenu(btn, function (sub) {
    // KF-334: KanbanFlow-style relative-move submenu.
    ...
    if (currentIndex >= 0 && currentIndex < headers.length - 1) {
      var nextColumnId = headers[currentIndex + 1].dataset.columnId;
      sub.appendChild(cardSubmenuButton('Move right', 'move-right', { 'data-column-id': nextColumnId }));
      sub.appendChild(cardSubmenuButton('Move to bottom', 'move-bottom'));
    }
    // "Move advanced" (always available).
    sub.appendChild(cardSubmenuButton('Move advanced', 'move-advanced'));
```
"Move right"/"Move to bottom" are correctly omitted in the last column;
"Move advanced" is always offered. No flat destination-column list is
built in this submenu path.

Verdict: **PRESENT**.

---

## KF-335 — Board lacks KanbanFlow's bottom pomodoro-count legend strip — **PRESENT** (remaining scope; part superseded by KF-354)

Original requirement: a persistent bottom legend strip with 4 pomodoro
segments — "1 Pomodoro" (yellow) / "2 Pomodori" (green) / "3 Pomodori"
(blue) / ">3 Pomodori" (pink) — GM-139 fills `#ffffe2/#e1fec7/#d1e1fd/
#f7cdd1`, gray `#cbcbcb` top edge; the strip was entirely absent.

**Superseded by KF-354 (batch-28, in this same deployed tree)** — these
parts of KF-335's original scope no longer apply and were deliberately
replaced, per Chip's 2026-10-12 KanbanFlow screenshot:
- the 4-segment set (now: one segment per board color, all colors);
- the GM-139 fills (now yellow `#ffffde`, green `#dcfcbe`, blue `#c9ddfd`,
  red `#f7c9ca`, orange `#fce8bd`, purple `#e8d7fd`, magenta `#fadefd`,
  cyan `#dcfdff`);
- the uniform gray top edge (now saturated per-color edges: yellow
  `#edc82f`, green `#6bca2d`, blue `#71a4fb`, red `#f08685`, orange
  `#ed9f27`, purple `#bb8efa`, magenta `#f087fa`, cyan `#4ec9d0`);
- the pomodoro-count segment labels (now each color's display label).
Evidence: `src/routes.rs:1103-1145` (`legend_bg()` / `legend_edge()` with
the KF-354 comment), unit test `legend_colors_match_kanbanflow_all_eight`
at `src/routes.rs:8401-8434` asserting all 8 fill/edge pairs, and
`git grep -c 'is_pomodoro_legend_color' 2246458 -- src/` → 0 matches (the
old 4-segment filter is gone).

**Remaining KF-335 scope — PRESENT:**
- The strip itself exists and renders at the board bottom.
  `templates/board.html:660-672`:
  ```html
  {% if legend_visible %}
  <footer class="color-legend" aria-label="Task color legend">
    <div class="color-legend-rule" aria-hidden="true"></div>
    <div class="color-legend-band" aria-hidden="true"></div>
    <div class="color-legend-segments">
    {% for color in colors %}
      <span class="color-legend-segment" style="background-color: {{ color.legend_bg() }}; border-top-color: {{ color.legend_edge() }}" title="{{ color.description }}">{{ color.display_label() }}</span>
    {% endfor %}
    </div>
  </footer>
  {% endif %}
  ```
- Strip geometry/styling in `static/style.css:458-514`: `.color-legend`
  sticky at `bottom: 0`, full-bleed; `.color-legend-rule` 1px `#bbbbbb`
  inset 10px each side; `.color-legend-band` 9px `#f0f0f0`;
  `.color-legend-segments` 23px row; segment labels 12px `#222222`
  centered. (These rule/band values are the KF-249/KF-354 re-measured
  values, superseding the older `#cbcbcb`-edge reading in KF-335's
  verification notes.)
- Toggle: `legend_visible` is per-board persisted config, read at
  `src/routes.rs:1656-1668`, written via the Menu's Color legend toggle
  (`src/routes.rs:5374-5433`) — matching the defect's note that the strip
  is controlled from the board menu.

Verdict: **PRESENT** for all remaining (non-superseded) scope; the
superseded parts were replaced by KF-354 in the same tree, as instructed.

---

## Summary

| Defect | Verdict |
|---|---|
| KF-293 | **PARTIAL** — GM-130 presets + `DayPomodori.minutes` present; tooltip format and bar count labels ABSENT |
| KF-005 | PRESENT |
| KF-295 | PRESENT |
| KF-326 | PRESENT |
| KF-331 | **PARTIAL** — H1 present; GM-134 background/card match NOT PROVEN (no target values exist; needs aligned re-capture) |
| KF-333 | PRESENT |
| KF-334 | PRESENT |
| KF-335 | PRESENT (remaining scope; 4-segment scope superseded by KF-354 in the same tree) |
