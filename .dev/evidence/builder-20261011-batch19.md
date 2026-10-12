# Batch-19 builder report — 2026-10-11/12

**Worktree:** `/home/hatch/workspace/pomodoro-kanban/.dev/worktrees/batch-19`
(base `6bb6cd1`, detached HEAD). **Production untouched, nothing pushed.**

## Headline

- **Already-fixed in the 6bb6cd1 tree (zero commits):** KF-230, KF-246, KF-301, KF-316, KF-317
- **INVALID per planner re-triage — not built:** KF-247, KF-248
- **Built (4 commits, one per defect):** KF-308, KF-348, KF-249, KF-347

## Already-fixed (verified live on a test instance, 127.0.0.1:3227)

- **KF-230** — the column menu (`#column-menu`, templates/board.html:764-769) renders
  all four items including `<button data-col-act="delete">Delete</button>`,
  and app.js:518 wires `act === 'delete'` → `deleteColumn(id)`. Delete is
  reachable; the old "dead code" claim is stale.
- **KF-246** — `templates/board.html:706` has the green `btn btn-success`
  `#layout-add-column-end` "+ Add column" button at the right end of the
  layout-editor column list (KF-246 comment at 703-705).
- **KF-301** — `/b/:id/settings/colors` renders `(Default)` (capital D) once;
  zero lowercase `(default)` occurrences.
- **KF-316** — served `static/style.css`: `.columnHeader-add` is
  `background: transparent; color: #3DC54E` (bare green glyph, no button
  background); `.columnHeader-countBadge` is `color: #111111` (near-black).
- **KF-317** — the task labels dialog (`templates/board.html:1026-1027`) has
  `<span>Add labels...</span>` and `placeholder="Add labels..."`.

## INVALID — not built (flagged for the orchestrator)

- **KF-247** — the section's own `[INVALID] CORRECTION 2026-10-02 (planner
  re-triage)` marks it INVALID: the premise is contradicted by GM-157 itself,
  which shows KanbanFlow's layout editor as a full-page in-board view
  ("Layout: General", green Done button, Add column/Add swimlane buttons) —
  structurally the same as ChipFlow's `#layout-view`. "Do NOT claim/build
  KF-247 as written." No code touched.
- **KF-248** — the section's own `[INVALID] CORRECTION 2026-10-02` marks it
  INVALID: the strip is a collapsed *column* (not a swimlane), measured
  ~27px (not 70), and the real element is already implemented under
  KF-223/KF-039/KF-040. "Do NOT claim/build KF-248 as written." No code
  touched.
- Both sections still show OPEN as their first `- Status:` line (stale
  ordering); the tracker needs a status reconciliation, which is the
  orchestrator's call, not the builder's.

## Built

### KF-308 — Pomodoro template's first column "Work To-do" → "To-do"
- Commit `d5c8fda`. `src/db.rs:244` (column name) and `src/db.rs:261`
  (starter-task column reference) in `pomodoro_template_snapshot()`.
- Live: `POST /api/boards` with the "Pomodoro board" template id returns
  columns `['To-do', 'Do today', 'In progress', 'Done']`.

### KF-348 — bare local datetimes with seconds normalize to RFC3339
- Commit `cb14daf`. `normalize_due_input` (src/routes.rs) now also accepts
  `%Y-%m-%dT%H:%M:%S` and `%Y-%m-%d %H:%M:%S`, interpreting them as
  server-local time (same policy as the existing minute-precision formats).
  Added unit test `normalize_due_input_bare_local_datetime_with_seconds`,
  which also asserts `format_due_card` parses the result.
- Live: `POST /api/tasks` with `"due_at":"2026-10-20T17:00:00"` stores
  `"2026-10-20T17:00:00+00:00"` (server is UTC).

### KF-249 — legend fills/edges per the 2026-10-02 contract correction
- Commit `a94a507`. `ColorView::legend_bg()` now returns the palette
  `background_hex` (`self.bg`); `legend_edge()` returns the per-segment
  edge colors from the correction (`#f5cc00/#59d600/#70b0ff/#ff858f`,
  `#cbcbcb` fallback). The old `#ffffe2`-set fills were GM-139's raw values
  read without its embedded ICC profile; the old uniform `#cbcbcb` edge
  contradicted the unresampled capture. Geometry (33px strip, 1px rule,
  9px band, flush bottom) was already correct and untouched.
- Live (`legend_visible` enabled via `PUT /api/boards/:id/config`): the four
  segments render `(#ffffe0,#f5cc00) (#dbffc2,#59d600) (#cce3ff,#70b0ff)
  (#ffccd0,#ff858f)` with labels Yellow/Green/Blue/Red.

### KF-347 — Break activities statistics tab (real implementation)
KanbanFlow's Pomodoro Statistics has a working Break activities tab;
ChipFlow's rendered "not available yet" and — critically — never recorded
break-activity usage anywhere (activities were definable in settings but
never selectable or logged).

Server (`src/`):
- `models.rs`: `TimeEntryRow.activity_id: Option<String>` (serde default;
  old rows deserialize fine).
- `db.rs`: `create_entry_full` takes `activity_id: Option<&str>`; both
  callers updated (`create_entry` → None; `create_entry_at` row literal
  → None).
- `routes.rs`:
  - `TimerStopInput.activity_id: Option<String>` (also in the utoipa
    `ToSchema`, so `/api/v1/openapi.json` stays accurate).
  - `timer_stop` threads it into `log_timer_session`, which validates the
    id against `settings.break_activities` (unknown ids are ignored, not
    stored); the other `log_timer_session` caller (timer_start replacement)
    passes None.
  - `TimerStatisticsReport.break_activities: Vec<BreakActivityStat>` with
    `BreakActivityStat { id, name, minutes, today_minutes, daily_goal,
    daily_limit }` (ToSchema); `api_timer_statistics` aggregates
    short_break/long_break entries by activity over the requested range
    (board filter respected via the existing `in_scope`).

Client (`static/app.js`, `templates/timer_statistics.html`):
- The "not available yet" paragraph is replaced by `<div id="stats-breaks">`.
- `TimerStatsPage.render` draws per-activity rows (name, total time in
  range, bar scaled to the max, "today X / Y goal" when a daily goal is
  set) or "No break activities defined".
- The timer popup's running-break session view shows a "Break activity"
  `<select>` (populated from `settings.break_activities`) when activities
  are defined; the choice is sent as `activity_id` on both stop paths
  (`stopAndLog` and the run-to-zero `finishSession`).

Docs: `agents.md` documents the new `activity_id` stop param and the
`break_activities` statistics field. `agents.json` is metadata-only (no
change); the OpenAPI spec regenerates from the annotations.

Live end-to-end (test instance, throwaway DB):
- `PUT /api/settings` with one activity (`Walk`, goal 30m) → 200.
- `POST /api/timer/start` short_break → 65s → `POST /api/timer/stop`
  `{"completed":true,"activity_id":"act-walk"}` → `{"discarded":false,
  "minutes":1,"completed":true}`.
- `GET /api/timer/statistics` → `break_activities:
  [{id act-walk, name Walk, minutes 1, today_minutes 1, daily_goal 30,
  daily_limit 60}]`.
- A 25s session truncates to 0 minutes (existing minute-truncation policy,
  unchanged) — the first probe's `minutes: 0` was correct behavior, not a
  bug; the 65s run is the definitive proof.
- Statistics page loads 200 with `#stats-breaks` present and zero
  "not available yet" occurrences.

## L0 (final tree, exact)

- `cargo fmt --check` — PASS
- `cargo build` — PASS, no errors
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — **86 passed, 0 failed** (85 baseline + 1 new KF-348 test)
- `node --check static/app.js` — PASS

## Commits (worktree only, not pushed)

- `d5c8fda` KF-308: Name the Pomodoro template's first column To-do
- `cb14daf` KF-348: normalize bare local datetimes with seconds to RFC3339
- `a94a507` KF-249: legend fills use palette background_hex; per-color top edges
- `79d9f03` KF-347: implement the Break activities statistics tab

## Notes for the verifier / orchestrator

- KF-247/KF-248 need a tracker-status decision (their own correction
  sections say INVALID; first Status lines still say OPEN).
- The KF-249 edge colors (`#f5cc00` etc.) come verbatim from the 2026-10-02
  contract correction; they are not in `STANDARD_COLORS` (whose borders are
  `#eecd45` etc.) — the correction calls them "established palette values"
  measured from the unresampled capture. Flagging in case the verifier
  wants to re-derive them.
- Test instance (127.0.0.1:3227) killed by port PID; port confirmed free;
  throwaway DB removed. Production untouched, nothing pushed, tracker not
  modified.
