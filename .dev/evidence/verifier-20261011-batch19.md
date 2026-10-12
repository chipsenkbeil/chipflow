# Batch-19 independent verifier report — 2026-10-11/12

**Verifier:** independent (not the builder). **Deployed tree:** `origin/main` = `cb0bf143`
(fetched fresh). **Production:** `https://chipflow.chip.network/api/v1/version` → build_sha
`8794cb266f2d8386f67013426f6fc78f02106eca` (healthy).
**Production equivalence proven:** `git diff 8794cb26 cb0bf143 -- src static templates Cargo.toml Cargo.lock`
is **empty** — the deployed app code is byte-identical to production.

**Batch-19 worktree:** `.dev/worktrees/batch-19`, detached HEAD `261b1e7` (base `6bb6cd1`),
commits `d5c8fda` (KF-308), `cb14daf` (KF-348), `a94a507` (KF-249), `79d9f03` (KF-347).

## Verdicts

### Already-fixed claims (verified against the DEPLOYED tree, live instance on :3231)

- **KF-230 — PRESENT-AND-CORRECT.** Deployed `templates/board.html:768` has the 4-item column
  menu (Edit / Collapse / Show details / Delete); `static/app.js:518` routes
  `act === 'delete'` → `deleteColumn(id)`; `deleteColumn` implemented at `app.js:897`.
  Live board page: all 4 `data-col-act` items present. Break attempts: dark mode + 1100px —
  4 items still present. The "dead code" claim is stale.
- **KF-246 — PRESENT-AND-CORRECT.** Deployed `templates/board.html:706`:
  `<button class="btn btn-success" id="layout-add-column-end">+ Add column</button>` at the
  right end of the layout-editor column list. Live: present with `btn-success`.
- **KF-301 — PRESENT-AND-CORRECT.** Deployed `templates/board_settings.html:100` renders
  `(Default)` (capital D); zero lowercase `(default)` occurrences across templates+static.
  Live colors page: exactly 1 `(Default)`, 0 lowercase.
- **KF-316 — PRESENT-AND-CORRECT.** Deployed `static/style.css:634`: `.columnHeader-add`
  has `background: transparent; border: none`. Live dark mode at 1100px: computed
  `rgba(0,0,0,0)` / `rgb(61,197,78)` (#3DC54E) — bare green glyph, no button chrome.
- **KF-317 — PRESENT-AND-CORRECT.** Deployed `templates/board.html:1026-1027`:
  `<span>Add labels...</span>` + `<input placeholder="Add labels...">`. Live: both present;
  dark mode + 1100px: input still present.

### Built fixes (verified in the batch-19 worktree, live instance on :3232)

- **KF-308 — PRESENT-AND-CORRECT** (`d5c8fda`). Pomodoro template seed now names the first
  column `"To-do"` (was `"Work To-do"`); the starter-task column reference updated too.
  Live: new board from the Pomodoro template has columns `['To-do', 'Do today',
  'In progress', 'Done']`.
- **KF-348 — PRESENT-AND-CORRECT** (`cb14daf`). `normalize_due_input` now accepts bare
  local datetimes with seconds (`%Y-%m-%dT%H:%M:%S`, `%Y-%m-%d %H:%M:%S`) as server-local
  time. Live: `"2026-10-20T17:00:00"` → stored `2026-10-20T17:00:00+00:00`;
  `"2026-10-20 17:00:00"` → same; `"garbage-xyz"` → passed through unchanged (pre-existing
  conservative behavior, documented). New unit test
  `normalize_due_input_bare_local_datetime_with_seconds` passes. Nit (not a failure): the
  function's doc comment still lists only the no-seconds formats.
- **KF-249 — PRESENT-AND-CORRECT** (`a94a507`). Legend fills now return the palette
  `background_hex`; top edges use the 2026-10-02 contract correction's per-color values.
  **Independently re-derived the correction's values** by pixel-sampling the unresampled
  1919x998 KanbanFlow capture (`~/workspace/user/media_library/browser_screenshots/30/3085c577….png`):
  row 965 = `#c3c3c3`, rows 966-974 = `#f2f2f2`, row 975 edges = `#f5cc00/#59d600/#70b0ff/#ff858f`,
  fills = `#ffffe0/#dbffc2/#cce3ff/#ffccd0` — the correction's values are exactly right
  (the builder's caution is resolved; the edge hexes are measured KanbanFlow values, not
  palette borders). Live: all four segments render `background #ffffe0/#dbffc2/#cce3ff/#ffccd0`
  with `border-top-color #f5cc00/#59d600/#70b0ff/#ff858f`. Break attempts: dark mode +
  1100px — identical computed colors.
- **KF-347 — PRESENT-AND-CORRECT** (`79d9f03`). Break activities statistics tab fully
  implemented and verified end-to-end:
  - Defined activity "Walk" (daily_goal 30, daily_limit 60) via `PUT /api/settings`.
  - Started a `short_break` tied to a task, waited 65s, stopped with
    `{"completed":true,"activity_id":"walk-1"}` → `{"discarded":false,"minutes":1}`.
  - `GET /api/timer/statistics` → `break_activities: [{id: walk-1, name: Walk, minutes: 1,
    today_minutes: 1, daily_goal: 30, daily_limit: 60}]`.
  - Invalid `activity_id` (`"nonexistent-id"`) is ignored, not stored (no phantom activity).
  - Statistics page `#stats-breaks` renders "Walk / 1m / today 1m / 30m goal" with bar;
    "not available yet" is gone from the page. Break attempts: dark mode + 1100px — tab
    renders with the activity row.
  - Note: a break with no task logs nothing (pre-existing `log_timer_session` behavior —
    entries are only written when `timer.task_id` is Some); out of KF-347's scope.
  - Note: the timer-log entry view does not serialize `activity_id` (statistics reads the
    row field directly, which is correct). Minor gap, out of scope.

### Excluded
KF-247 / KF-248: INVALID per the documented 2026-10-02 re-triage — not verified, not built.

## Critical release note

**Local master `6bb6cd1` is BEHIND `origin/main` in app code.** `git diff 6bb6cd1 cb0bf143
-- src static templates` = 112 insertions / 13 deletions in `static/app.js`: origin/main
carries the KF-306 column-delete move-target dialog plus the Round-6..Round-11 releases
(KF-281 stopwatch, KF-298 T shortcut, KF-309 glyphs, KF-277/280/282, KF-291/296/299,
KF-300/303, KF-003/004/005, KF-293/295) that local master lacks. The four batch-19
commits were built on the stale base. **The release worker must apply them onto
origin/main's app tree (rebase/cherry-pick), NOT merge local master** — otherwise the
deployed fixes regress.

## Method & evidence
- Built the deployed tree (`cb0bf143`) and the batch-19 tree (`261b1e7`) in release mode;
  served `static/app.js` sha256 matched each tree byte-for-byte.
- Test instances on 127.0.0.1:3231 (deployed) and :3232 (batch-19) with throwaway DBs
  (XDG_DATA_HOME-isolated); seeded via REST API; live DOM assertions + Playwright
  Chromium renders (1600px, 1400px, 1100px, dark mode) with computed-style measurements
  and visually reviewed screenshots.
- Break attempts survived for every defect: dark mode and 1100px narrow viewport.
- Servers killed by port PID, ports confirmed free, throwaway DBs removed, dep-verify
  worktree removed. Production untouched, nothing pushed, tracker not modified.
