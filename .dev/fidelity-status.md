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
- 2026-09-28 22:01 CDT: Chip's standing rule: no feature branches/PRs (leftover fidelity/* branches deleted) — push straight from local master to GitHub main, and deploy to green-box as part of every push. Deploy of 199af93c (bugfix milestone) dispatched. (process)
- 2026-09-28 22:02 CDT: Chip raised the browser-testing bar: adversarial and comparative, not just confirmatory — click everything, keyboard shortcuts, navigation, edge cases; judge every control on both function and KanbanFlow UX parity; file bugs and UX deviations alike. Briefed into the active deploy worker and the watchdog body. (process)
- 2026-09-28 22:10 CDT: DEPLOYED 199af93c to green-box. Homelab pin: rev 199af93ca3e4790159f406a56aba7452499aaf66 + exact nix-computed NAR hash sha256-SIwhu5K9ebFE+F/AVaHcmIvtzLh14e5e7xUd6xFdols= (computed on green-box via the real fetchFromGitHub fetcher — NOT the tarball, since the module uses fetchFromGitHub; nix-prefetch-git unavailable) + one-time CHIPFLOW_BUILD_SHA=chipflowSrc.rev env wiring (verified .rev evaluates on fetchFromGitHub results). Homelab main 76e40a10 -> 916bda14 "Update chipflow pin to 199af93c"; diff verified to touch only rev/hash/env lines; invokeai.nix untouched. Dry-activate: only chipflow.service stop/start (restart, not removal); podman/InvokeAI/civitai secrets untouched; sops OK. Switch applied. VERIFIED LIVE: chipflow.service active; GET /api/v1/version -> {"version":"0.1.0","build_sha":"199af93ca3e4790159f406a56aba7452499aaf66"} (matches deploy rev — env wiring works); https://chipflow.chip.network/api/v1/version -> HTTP 200 externally; Caddy active; /setup 303 -> / (already claimed, correct). HTTP-level tests of 199af93c (local build, fresh DB): 23/24 pass — settings page 200 (was 500), API tokens list/create/revoke OK, Bearer works on API, Bearer mint -> 403, revoked -> 401, board created from Pomodoro template, board page 200 with timer markup, TimerUI in app.js, version/openapi/agents.md OK. (The 1 "fail" was a re-run artifact: /setup 303 on non-fresh DB, correct behavior.) BROWSER CLICK-THROUGH NOT DONE — no live-browser capability in this worker; delegated to parent for a real browser task (adversarial + KanbanFlow-comparative per Chip's bar). (deploy)
- 2026-09-28 22:12 CDT: deploy of 199af93c VERIFIED LIVE on green-box — chipflow.service active, public /api/v1/version returns build_sha == 199af93ca3e4790159f406a56aba7452499aaf66 (homelab pin + CHIPFLOW_BUILD_SHA wiring confirmed working). Deploy worker's browser pass did not complete (its local server died mid-task); dedicated adversarial browser-test worker dispatched against a fresh local build of the same SHA. (deploy)
- 2026-09-28 22:22 CDT: Chip directed a full systematic parity audit: screenshot every KanbanFlow surface (live capture on throwaway board + his 9 walkthrough videos), compare surface-by-surface vs ChipFlow in parallel, build a giant defect list (hundreds OK). Audit coordinator dispatched; defects merge to .dev/kf-parity-defects.md. Audit only, no code changes. (process)
- 2026-09-29 00:15 CDT: KanbanFlow→ChipFlow parity audit MERGED — 103 defects in .dev/kf-parity-defects.md (KF-001–KF-103): high 25 · medium 44 · low 34; broken 15 · divergent 51 · missing 37; surfaces: timer 21, task modal 18, settings 16, columns 16, header 12, cross-surface 10, cards 6, reports 4. Method: 7 parallel static-audit workers (templates/app.js/style.css/routes.rs vs video frame catalogs v1/v3/v4/v7/v8/v9 + 961-state frame review + hi-res board screenshot + color-settings screenshot); 10 cross-surface dupes merged; 2 video-catalog mistranscriptions corrected (v7 "Start"→"Print" in Reports menu, v3 "Repeat it"→"Reports"). Headline: the timer subsystem is fundamentally broken — TimerUI reads status fields the server never sends (KF-001, VERIFIED) and every stop 400s so no session is ever logged (KF-002, VERIFIED); manual/edit time dialogs, why-stop menu, break flow, timer-log and statistics pages are all dead shells; P shortcut stops a running timer; board Filter, Reports menu, timer-settings modal, sounds, subtasks, members, card context menu all missing. NOT covered (needs live signed-in KanbanFlow browser): avatar/account menu, bell/help destinations, break visuals, 13 report destinations, Board Settings sub-pages, swimlanes, pixel-level fidelity, and all runtime behavior (every "broken" finding is static-inference, pending behavioral confirmation). Audit only — no code changed. (audit)
- 2026-09-29 03:45 UTC: Adversarial test run against 199af93c COMPLETED (replacement run; first worker's server died). Backend runtime-verified via curl against a fresh local build + fresh redb (127.0.0.1:18081): 30+ checks — setup/login/session, boards/columns/swimlanes/tasks CRUD, task move/patch/delete, timer start/stop/status/today, manual time entry, API token mint/list/revoke + Bearer-cannot-mint 403 + revoked 401, settings GET/POST, templates save/list/delete, colors list/page, timer log/statistics pages + API, WIP-limit warning rendering, 404/401/303 auth behavior. Backend is SOLID except the already-filed KF-002 (timer stop 400s without `completed` — runtime-confirmed: `{"reason":"completed"}` → 400 "missing field `completed`"). 4 parallel static-audit workers (board chrome, cards/modal, settings/colors/reports, non-timer JS wiring) found 23 NEW defects, filed as KF-104–KF-126 (total now 126). Headline new finds: KF-118 HIGH — add-task form swaps the new card into itself then deletes itself (card never appears without reload); KF-116 — `refreshModalTimeLog()` fetches nonexistent route `/api/tasks/{id}/time-entries` into nonexistent element `#modal-time-log` (double mismatch); KF-117 — Task URL deep link has zero consumers; KF-104/KF-121 — legend is dots+task-colors vs KanbanFlow's full-width pomodoro-count band; KF-122 — invented toolbar row occupies the board-bar slot; KF-123 — board name in dark topbar vs light board bar; KF-126 — floating card grid vs flush table; KF-114 — "1h 9m" vs "1h6m"; KF-120 — move_task position collision. LIMITATION: no live interactive browser available in this worker — no click-through, screenshots, drag-drop, or pixel comparison was performed. The parent must delegate the browser pass to a browser-capable worker. (test)

## 2026-09-29 — Browser testing complete, 136 defects filed

Real Chromium browser testing completed against fresh local DB (build 199af93c):
- 39-step Playwright harness via SOCKS+nip.io (bypassed LNA block); 63 screenshots captured.
- Network monitoring found 4 timer API bugs: /api/timer/settings 404 (3x), /api/timer/status 401, unhandled JSON parse errors, null DOM writes (KF-127..130).
- Visual comparison vs KanbanFlow reference confirmed KF-104, KF-122, KF-123, KF-036, KF-037.
- New: KF-131 (swimlane labels left/horizontal vs right/vertical).

KanbanFlow live handoff received (browser-task:80778f50):
- Throwaway board created, toured, PERMANENTLY DELETED. General untouched. Only that board modified.
- KEY FINDING: Timer is a clock icon in the BOARD BAR, not the dark top header (KF-132).
- Board bar: Invite (+), Timer (clock), Filter, Edit layout (pencil), Menu (KF-133).
- Missing: Filter panel (KF-134), Board Menu (KF-135), Reports submenu 15 items (KF-136).
- Card menu: 7 items (handoff) vs 8 in KF-050 — discrepancy noted.
- Swimlanes are premium-only in KanbanFlow.

Defect inventory: 136 total (KF-001..136). Committed as 6208ad9, pushed to main as e251c4d. Live build still 199af93c (docs-only push, no deploy needed).
