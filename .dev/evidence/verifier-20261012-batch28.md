# Batch-28 independent verifier evidence — KF-353 / KF-354 / KF-355

**Verified commits:** `a30f961` (KF-353), `9c08038` (KF-354), `2876bfb` (KF-355) in `.dev/worktrees/batch-28` (base `6bb6cd1`, detached HEAD).
Built the tree, ran a live test instance (127.0.0.1:3249, throwaway redb — killed by PID, port free, DB removed). Production untouched, nothing pushed, tracker not modified.
Note: the environment restarted mid-verification and wiped /tmp (server + seed DB lost); re-seeded with the DB inside the persistent worktree.

## KF-353 — Board menu toggle switches + item icons: PRESENT-AND-CORRECT

- Menu shows 11 leading `.menu-item-icon` SVGs, 3 `.menu-switch` spans, 2 `.menu-sep` separators; 12 menuitem roles (11 icon items + icon-less "Get Premium", matching KanbanFlow where Get Premium is a button).
- Initial states matched reality: Dark mode `aria-checked=false`, Color legend `=true` (legend_visible=true), Large task names `=false`.
- Clicking each toggle flipped BOTH the app state and the switch, verified on menu reopen (aria-checked re-sync works):
  - Dark mode → `body.dark` true/false; switch OFF 34×19 `rgb(179,179,179)` knob x=2 (left); ON `rgb(86,183,79)` (#56b74f) knob x=17 (right), 15px white knob.
  - Color legend → legend DOM nodes 0 when off, restored when on.
  - Large task names → `aria-checked` true/false (app-level class mechanism unchanged, pre-existing).
- Switch visuals match KanbanFlow's reference (right-aligned, iOS-style, grey off / green on, white knob left-off / right-on). Knob uses `left: 17px` when checked, pure CSS from `aria-checked`.
- Dark-mode menu screenshot reviewed: dark panel, light text, icons/separators readable, switches readable.
- Old KF-183 `::before` checkmark rule is gone; no other menuitemcheckbox usage.
- Observation (accepted, not a defect): "Save board as template" sits between Color legend and Large task names, splitting the toggle group. The defect says to keep the existing menu structure; KanbanFlow has no such item. Not a regression.

## KF-354 — Color legend shows all board colors: PRESENT-AND-CORRECT

- Legend renders 10 segments (all board colors: 8 standard + brown + white).
- Pixel-probed all 8 standard segments (fill sampled away from centered labels, edge at top 1px row): **8/8 exact matches** —
  yellow #edc82f/#ffffde, green #6bca2d/#dcfcbe, blue #71a4fb/#c9ddfd, red #f08685/#f7c9ca,
  orange #ed9f27/#fce8bd, purple #bb8efa/#e8d7fd, magenta #f087fa/#fadefd, cyan #4ec9d0/#dcfdff.
- Geometry: 33px total, 1px #bbbbbb rule, 9px #f0f0f0 band, 23px segment row — matches the reference structure exactly.
- Non-standard colors fall back sanely (brown #cbcbcb/#f6ddcb, white #cbcbcb/#fbfbfb).
- Reference screenshot measured independently: rule #bbbbbb, band #f0f0f0, column body #ffffff above the legend — all consistent.
- New unit test `legend_colors_match_kanbanflow_all_eight` passes.

## KF-355 — White column bodies / #ebebeb headers: PRESENT-AND-CORRECT

- `.board-cell` computed background: `rgb(255,255,255)` (#ffffff) ✓
- `.columnHeader` computed background: `rgb(235,235,235)` (#ebebeb) ✓
- Dark mode: `.columnHeader` dark override intact (`rgb(31,41,55)` = #1f2937); no `body.dark .board-cell` rule ever existed (verified in 6bb6cd1 too) — no dark regression.
- `#f7f8fa`/`#eef0f2` have no other board-surface users (CSS grep).

## Break-attempts survived

- Dark mode (menu readable; report shell covered by KF-349).
- Narrow viewport 1100px: 0px horizontal overflow, menu usable, zero JS errors.
- Empty board: legend shows all 10 color segments, zero JS errors.
- Legend on/off cycling; toggles re-sync on every menu open.
- Observation (pre-existing, out of scope): in dark mode, column bodies stay white (no dark override ever existed) and task-card text washes out — unchanged by batch-28, not a KF-353/354/355 regression.

## L0 re-run (worktree, exact)

- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — **83 passed, 0 failed** (9+36+3+12+8+6+4+2+2+1; this worktree is on base 6bb6cd1 so counts differ from release trees)
- `node --check static/app.js` — PASS

**Release note:** cherry-pick `a30f961` + `9c08038` + `2876bfb` onto current `origin/main`; never merge local master `6bb6cd1`.

**DONE: KF-353 PRESENT-AND-CORRECT, KF-354 PRESENT-AND-CORRECT, KF-355 PRESENT-AND-CORRECT**
