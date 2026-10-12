# Batch-28 builder evidence — KF-353 / KF-354 / KF-355

Worktree: `/home/hatch/workspace/pomodoro-kanban/.dev/worktrees/batch-28` (base `6bb6cd1`, detached HEAD).
Production untouched, nothing pushed, tracker not modified.

## KF-353 — Board menu toggle switches + item icons (`a30f961`)

**Reference:** Chip's 2026-10-12 KanbanFlow menu screenshot
(`~/workspace/user/media_library/image/92/9234e05c70aa49b52889df6c1efb0a23d0298b6db3cae2ea3c0b385545688772.png`).
Pixel measurements: panel bg #f6f6f6; switch ~40x19px in-image (~34x16 CSS at
~1.19 scale); off track grey ~(180,180,180); white knob left in off state;
right-aligned switches; thin separators around the toggle group; dark-grey
leading icons per item.

**Built:**
- `templates/board.html`: every `#board-menu` item gets a leading
  `.menu-item-icon` inline SVG (filter funnel, reports bars, pencil, gear,
  person, trash, half-moon, palette, bookmark, text-size T, help ?). The
  three toggle items (Dark mode, Color legend, Large task names) are now
  `role="menuitemcheckbox"` with a right-aligned `.menu-switch` span
  (knob span inside); section separators (`div.menu-sep`) around the toggle
  group, matching the reference. Kept existing item order and behavior
  (menu still a `.menu-pop` popup; clicking a toggle still closes the menu).
- `static/style.css`: `.menu-switch` (34x19 track, #b3b3b3 off, #56b74f on —
  KanbanFlow's measured add-button green), 15px white knob with shadow,
  0.15s transitions; `#board-menu button` switched to `justify-content:
  flex-start` with `.menu-switch`/`.menu-sub-arrow` `margin-left: auto`
  (right-aligned); `.menu-sep` 1px #e5e5e5; icons #555 (dark mode #9ca3af /
  #374151 seps). Removed the KF-183 `::before` checkmark rule — the switches
  now show state (verified no other menuitemcheckbox usage).
- `static/app.js`: `toggleDark`/`toggleLargeNames` now sync their menu
  item's `aria-checked`; menu-open handler syncs both from body classes
  (legend item is server-rendered, re-synced defensively). The switch visual
  is pure CSS from `aria-checked`.

**Local confirmation** (test instance :3248, throwaway redb — killed, port
free): seeded 8 color tasks, enabled legend. Playwright screenshots:
menu shows 11 icons, 3 right-aligned switches, 2 separators; Color legend
switch ON (green) reflecting `legend_visible=true`; Dark mode/Large names
OFF (grey). Clicking Dark mode flips `body.dark` and the switch reads
`aria-checked=true` on reopen; same for Large task names. Dark-mode menu
screenshot reviewed (icons/separators readable).

## KF-354 — Color legend shows all board colors (`9c08038`)

**Reference:** Chip's 2026-10-12 full-board screenshot
(`~/workspace/user/media_library/image/47/474e30c864c8adcf602f09d2eda90d0eab85b7136135392dbe3cd2619368e3e9.png`).
Pixel measurements (bottom strip): 1px #bbbbbb top rule, 9px #f0f0f0 band,
23px segment row; per-segment 1px SATURATED edge + 22px fill:
yellow #edc82f/#ffffde, green #6bca2d/#dcfcbe, blue #71a4fb/#c9ddfd,
red #f08685/#f7c9ca, orange #ed9f27/#fce8bd, purple #bb8efa/#e8d7fd,
magenta #f087fa/#fadefd, cyan #4ec9d0/#dcfdff.
This supersedes the stale GM-139 reading (4 pomodoro segments, uniform
#cbcbcb edge).

**Built:**
- `src/routes.rs`: `legend_bg()` returns the 8 fresh fills (fallback:
  board `bg`); `legend_edge()` returns the 8 saturated edges (fallback
  #cbcbcb); removed `is_pomodoro_legend_color()`; new unit test
  `legend_colors_match_kanbanflow_all_eight` covering all 8 pairs.
- `templates/board.html`: dropped the 4-color `{% if %}` filter; all board
  colors render one segment each with `display_label()`.
- `static/style.css`: strip rule #c3c3c3→#bbbbbb, band/footer #f2f2f2→#f0f0f0;
  geometry unchanged (33px total).

**Local confirmation:** rendered legend shows all 10 board colors; pixel
probe of all 8 standard segments: fills AND edges match the reference
exactly (8/8 OK); rule #bbbbbb, band #f0f0f0 confirmed.

## KF-355 — White column bodies / #ebebeb headers (`2876bfb`)

**Reference:** same full-board screenshot: column bodies #ffffff, headers
#ebebeb.

**Built:** `static/style.css` only — `.board-cell` #f7f8fa→#ffffff,
`.columnHeader` #eef0f2→#ebebeb. Dark mode unaffected (`body.dark
.columnHeader` keeps its own #1f2937; no `body.dark .board-cell` rule
exists). `#f7f8fa`/`#eef0f2` have no other board-surface users.

**Local confirmation:** pixel probe on the rendered board: column body
#ffffff, header #ebebeb — exact.

## L0 (final tree)

- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings, 0 errors
- `cargo test` — **86 passed, 0 failed** (85 base + 1 new legend test)
- `node --check static/app.js` — PASS

## For the verifier / release

- Cherry-pick `a30f961` + `9c08038` + `2876bfb` onto current `origin/main`;
  never merge local master `6bb6cd1`.
- Evidence file: this file (copy to the main repo's `.dev/evidence/`).
