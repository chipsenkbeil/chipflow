# ChipFlow KanbanFlow-fidelity rebuild — project status

Source of truth for the rebuild. Updated by Dale; the implementation
agent appends phase-log entries to the END of this file. NEVER delete or
overwrite this file — append only.

## Hard rules (from Chip, 2026-09-28)

- Rust single standalone binary. Pure-Rust redb storage (no SQL, no C
  dependencies, no migrations, no Docker requirement).
- The existing redb database layer stays; extend it, don't redesign it.
- Internal implementation may differ from KanbanFlow. The user-facing
  experience — navigation, visuals, layout, controls, interactions — must
  match KanbanFlow as exactly as practical. Columns and how they work are
  the top priority.
- Everything stays runtime-configurable (columns, swimlanes, WIP limits,
  colors, templates). Never hard-code behavior to seeded names; resolve
  special columns (e.g. Done) from runtime configuration.
- AI-accessible extras must survive: API tokens (documented in agents.md),
  utoipa-generated OpenAPI at /api/v1/openapi.json, GET /api/v1/version
  (returns build SHA), agents.md, agents/skill.md, agents.json, discovery
  endpoints.
- Browser auth = session cookie; API auth = bearer tokens; API tokens must
  never be allowed to mint tokens.
- Chip authorized modifying his KanbanFlow account for TESTING on
  throwaway boards only. Never touch his real boards or tasks.
- Commit AND push milestones to chipsenkbeil/chipflow `main` (remote
  branch is `main`, not `master`). Plain `git push` has no credentials in
  this environment — push via .dev/push_via_api.py (Git Data API):
  `push_via_api.py <repo> chipsenkbeil/chipflow <base> <head> main
  "<message>"`, where <base> is the local commit whose tree matches
  remote main's tree. Never rewrite pushed history.
- Short imperative commit messages.

## Definition of DONE (ALL required, evidence-based)

1. [ ] Visual comparison: real-browser 1440px screenshots of the ChipFlow
   board vs KanbanFlow covering the header, column headers, task cards,
   swimlane labels, estimate/color legend, and timer pill — no material
   differences.
2. [ ] Interaction pass: fresh-database browser exercise of card drag/drop
   and reordering; column creation, editing, deletion, reordering, and WIP
   limits; swimlane operations; task modal; timers; scrolling.
3. [ ] Behavioral parity: every KanbanFlow board behavior in "Discovered
   behaviors" below is implemented.
4. [ ] Project checks pass: cargo fmt --check, cargo build, cargo clippy,
   cargo test, node --check static/app.js.
5. [ ] Release and deployment: committed, pushed, homelab pin (rev+hash)
   updated, deployed to green-box with --switch, live service verified
   (chipflow.service active, /setup returns 200, /api/v1/version
   build_sha matches the deployed SHA).
6. [ ] AI accessibility: API tokens, generated OpenAPI, agents.md,
   agents/skill.md, agents.json, and discovery endpoints all correct and
   working.
7. [ ] Storage validation: fresh-database and persistence/restart tests
   pass.
8. [ ] Board templates + task colors: the Pomodoro template reproduces
   Chip's exact color scheme; creating a board from a template and saving
   a board as a template both work; verified in a real browser.

Never claim the board matches KanbanFlow until the browser comparison and
interaction pass actually occur.

## Additional features (Chip, 2026-09-28)

- [ ] Task colors (KanbanFlow parity): tasks carry a color. Per-board
  enable/disable, rename (≤50 chars), description (legend tooltip),
  default color, sort order. Ten standard colors with FIXED hex values
  (see Discovered behaviors). Task color picker lists only enabled colors;
  color legend bar at board bottom; task detail modal tints with the
  task's color.
- [ ] Board templates: board creation offers a template picker; any board
  can be saved as a template; ships with a built-in "Pomodoro board"
  template carrying Chip's exact color codings (yellow "1 Pomodoro"
  default, green "2 Pomodori", blue "3 Pomodori", red ">3 Pomodori") plus
  sensible default columns/swimlanes. Templates are user-extensible data
  in redb — not hard-coded one-offs.
- First-class Pomodoro categorization (a dedicated field/UI beyond colors)
  is blessed as a possible follow-up; not required for done.

## Known defects (reported by Chip, 2026-09-28)

- [ ] Settings icon → error about API tokens not existing. Settings →
  API tokens must work end to end (list/create/revoke).
- [ ] Pomodoro timer dropdown icon (left of username) does nothing — it
  must open the timer menu.

## Discovered behaviors (from live KanbanFlow review, 2026-09-28)

- Task colors = KanbanFlow's built-in color feature with renamed colors
  (not categories/labels/plugins). Configured at Board Settings → Colors.
  Standard colors have FIXED hex (not user-editable); the edit dialog only
  renames (≤50 chars) and sets a description (legend tooltip). Custom
  colors (Custom 1–10) are premium-only.
- Chip's General-board Pomodoro scheme (4 enabled): Yellow bg #ffffe0 /
  border #f5cc00 → "1 Pomodoro" (DEFAULT for new tasks, sortOrder 1);
  Green #dbffc2 / #59d600 → "2 Pomodori" (2); Blue #cce3ff / #70b0ff →
  "3 Pomodori" (3); Red #ffccd0 / #ff858f → ">3 Pomodori" (4). Disabled
  (available via Enable): Orange #ffeac2/#faa200, Purple #eddbff/#c994ff,
  Magenta #ffe0ff/#ff85ff, Cyan #dbffff/#00d6d6, Brown #f6ddcb/#e49b67,
  White #fbfbfb/#d4d4d4. Light variants: yellow #ffffe0, green #e4ffd1,
  blue #d6e9ff, red #ffe0e3.
- CSS: task cards use `taskColor-{value}` (background) +
  `taskBorderColor-{value}` (border); `taskColorVars-{value}` blocks define
  --taskColor/--taskBorderColor/--taskColorLight; `taskBorderColorBg-{value}`
  (border color as background) for dots/pills. Legend items reuse the same
  classes. Task color picker lists only enabled colors (color dot + label
  + menu); the task detail modal tints itself with the task's color.
- Drag-and-drop: mid-drag shows a dashed placeholder
  (.sortable-placeholder{border:2px dashed rgba(0,0,0,.2)}). Columns
  reorder by dragging.
- WIP-limit violation: header shows "3 / 2" in darkred + red warning line.
  DOM: th.columnHeader.columnHeader--warning.columnHeader--limitWarning,
  count div title="*LIMIT EXCEEDED*…", div.columnHeader-warningLine
  injected. Tokens: --board-headerWarningTextColor:darkred,
  --board-warningLineColor:#ff8080.
- Swimlanes are premium-only in KanbanFlow (blocked there). ChipFlow keeps
  its own swimlanes as a superset feature.
- Timer: task modal buttons Add/Move/Timer/Reports/More/Delete. Timer menu:
  Start Pomodoro / Start Stopwatch / Time log. Stopwatch shows
  "Session time 00:03…", red Stop button, task row with color dot,
  "TODAY / 9:28 PM — pending". Tab title + dark header timer pill count up
  live. Stopping before 20s → "Session discarded"; a 25s session was kept.
- Columns: add dialog = Name + Position combobox. Header ⋮ menu: Edit,
  Move left/right, Add to left/right, Delete. Edit dialog: Name,
  Description, WIP limit, Column sum, Task sorting, group-by-date checkbox,
  per-property Show/Hide. Delete via confirmation dialog.
- Column header right-click menu: Edit, Collapse, Show details. Collapse →
  narrow strip with vertical text (e.g. "IN PROGRESS") + count "3" in red.
  Show details → popup "Column: To-do", "Task count: 0".
- Task "More" menu: Watch, Task URL, Copy, Keyboard shortcuts. No
  right-click menu on task cards.
- Board deletion: Settings → Delete board → red confirmation page →
  final "cannot be undone" dialog.

## Reference material

- KanbanFlow board screenshot:
  ~/workspace/user/media_library/image/36/36656cbadcf994fff6766d509b096a1d5253ad8e6fd80f312ca2a672b411283c.png
- Rejected ChipFlow UI:
  ~/workspace/user/media_library/image/48/48ca4248f3ce5e36a1d5f003b95d9e2af1714017c8c195de49f289a3c54b820d.png
- Timer deep-dive: .dev/kf-frame-review-report.md
- KanbanFlow color-config screenshot:
  ~/workspace/user/media_library/browser_screenshots/a8/a84a34feb48214dbac691c3187d470b2eabb47eb50674a27fed6a8a271eb7901.png

## Deployment

- Homelab module modules/chipflow.nix pins `rev` + `sha256` of
  chipsenkbeil/chipflow.
- Deploy: reconstruct the homelab tree via the GitHub API, run
  `scripts/deploy-green-box.sh dale-chipflow` (dry-activate) then
  `--switch` with ~/workspace/bin first on PATH (SSH via the Tailscale
  proxy, user senkwich). Verify chipflow.service active, /setup returns
  200, and /api/v1/version build_sha equals the deployed SHA.
- Never hand-edit secrets/green-box.yaml (it invalidates the sops MAC).
- When rebasing via the Git Data API: apply the branch's actual diff to
  the new base; never attach a stale tree to a new parent. Verify both
  the invokeai and chipflow registrations survive in the resulting tree.

## Phase log

- 2026-09-28: live-account exploration complete — every interaction
  exercised and documented, test board deleted, real boards untouched.
- 2026-09-28: Pomodoro color scheme documented from the live account
  (4 renamed colors, exact hex captured).
- 2026-09-28: implementation started — fidelity rebuild (board UI,
  interactions, task colors, board templates).
- 2026-09-28: continuous delivery wired — builder pushes milestones to
  the repo; watchdog cron deploys passing pushes to green-box, verifies
  live, and reports each deploy in chat.
- 2026-09-28: Chip filed two defects (settings/API-tokens error, dead
  timer dropdown) → Known defects; builder instructed to fix both and add
  GET /api/v1/version (build SHA) for deploy verification. Watchdog now
  runs a post-deploy browser click-through (fresh local build of the
  deployed SHA) against the KanbanFlow reference screenshots after every
  deploy.
- 2026-09-28 21:53 CDT: d76e709 KanbanFlow-parity UI milestone — restyled
  board/modal/task-card templates, rewrote board JS (drag-drop,
  column+swimlane menus, task modal, preserved timer), added boards and
  task-color admin pages; merged with fidelity/data-layer for template
  view fields. (UI)
- 2026-09-28 21:53 CDT: push of master to origin FAILED — plain `git
  push` has no credentials in this environment ("could not read Username
  for 'https://github.com'"). Correct mechanism: .dev/push_via_api.py
  (Git Data API) targeting chipsenkbeil/chipflow `main`, with <base> =
  the local commit whose tree matches remote main's tree (initially
  f52010d). Milestone commit d76e709 is local-only until pushed. (UI)
- 2026-09-28 22:05 CDT: restored full status file (it had been overwritten
  with phase-log-only content). Reminder: this file is append-only —
  never overwrite it.
- 2026-09-28 21:58 CDT: Dale pushed the KanbanFlow-parity UI milestone to GitHub main via .dev/push_via_api.py (plain git push has no credentials in this env; the worker's 21:53 auth failure was a wrong-mechanism miss, not a token problem). Remote main now 34b6023f "Rebuild UI to match KanbanFlow"; remote tree 3c64d572beb2072b12b57321f1b21a2d14b68487 verified equal to local master 04060b7. Worker notified that its push base (f52010d/da0dce7) is stale. (UI)
- 2026-09-28 22:10 CDT: pushed bugfix milestone to GitHub main via .dev/push_via_api.py. Remote main now dd67e12c "Fix API tokens page and timer dropdown; add GET /api/v1/version" (base 04060b7 verified: remote tree matched local tree before push). Contains: API_TOKENS table created in Db::connect (settings page 500 fix; verified end-to-end list/create/revoke + Bearer-cannot-mint 403), timer pill always visible + single TimerUI.init guard + renderPill span fix, GET /api/v1/version {version, build_sha} with utoipa + agents.md docs. Local master ee9fa8b. (bugfix)
