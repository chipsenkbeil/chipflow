# Builder report — batch-13 (KF-312, KF-313, KF-314, KF-320)

Date: 2026-10-09. Worktree: `~/workspace/pomodoro-kanban/.dev/worktrees/batch-13`
(branch `batch-13`, base `b7d5fbe` = master). All work in the worktree;
master untouched, nothing pushed, nothing deployed.

## L0 gate (full suite, final tree — byte-identical to per-unit trees)

- `cargo fmt --check` — PASS
- `cargo build` — PASS (Askama recompiled `board.html` + `modal.html` cleanly)
- `cargo clippy --all-targets` — zero warnings
- `cargo test` — all pass:
  `9 + 35 + 3 + 12 + 8 + 6 + 4 + 2 + 2 + 1 + 3 + 0 = 85 passed; 0 failed`
- `node --check static/app.js` — PASS

## L1 gate (test instance, port 3217, fresh throwaway redb)

- `/api/v1/version` responds; fresh DB redirects `/` → `/setup` (200);
  setup + login flow works; board page 200 (79 KB).
- Served `style.css` contains all new rules. No app-caused console/page/
  request errors (only sandbox-expected CDN tunnel failures for
  cdn.jsdelivr.net, which this VM cannot reach — unrelated to the changes).
- Test server killed by exact PID after checks; no stray processes.

## KF-312 — Inline "n / 3" WIP count neutral gray

- Contract: `.columnHeader-count` (the inline "count / limit" span) changes
  from `var(--muted)` (#6b7280, blue-gray) to `#5b5b5b` = rgb(91,91,91) per
  the GM-033 master image. Standalone count badge untouched (KF-316).
- Files: `static/style.css`. Commit: `438d7d7`
  "KF-312: render inline WIP count in neutral gray rgb(91,91,91)"
- Dark-mode guard: `body.dark .columnHeader:not(.columnHeader--warning)
  .columnHeader-count { color: var(--muted); }` preserves the dark tone and
  lets the WIP-exceeded darkred rule (0,2,0, `.columnHeader--warning`)
  keep winning. Light-mode warning override untouched.
- Verified live: board page renders `1 / 3` inline count; CSS rule confirmed
  in served stylesheet.

## KF-313 — Vertical separator between + button and count cell

- Contract: 1px vertical separator between the green `+`
  (`.columnHeader-add`, right:2.5rem) and the standalone count badge
  (`.columnHeader-countBadge`, right:0.65rem), tone matching the KF-182
  column separators (`#d4d7dc`; dark `#374151`). Rendered only for columns
  without a WIP limit (the badge exists only there; the element sits inside
  the `{% if col.wip_limit.is_none() %}` block).
- Files: `templates/board.html` (new `<span class="columnHeader-separator"
  aria-hidden="true">`), `static/style.css`. Commit: `358795e`
  "KF-313: add vertical separator between + button and count cell"
- Geometry: `position:absolute; top:0.55rem; bottom:0.55rem; right:2rem;
  width:1px` — centered in the ~16px gap between badge text and the +
  button, spanning the padded header height (41px header).
- Verified live: header screenshot shows the separator line between `+`
  and the count in all three non-WIP columns (`/tmp/kf320/header.png`).

## KF-314 — Folded-swimlane strip label regular / near-black

- Contract: folded-strip label changes from `font-weight:700`,
  `rgb(75,85,99)` to `font-weight:400`, `rgb(17,17,17)` (#111111) per
  GM-037. Scoped to `.folded-strip .swimlane-name` — unfolded swimlane
  headers keep the bold gray-600 styling.
- Files: `static/style.css`. Commit: `264e775`
  "KF-314: folded-swimlane strip label regular weight, near-black"
- Verified via computed style against the served stylesheet:
  strip label → `400 rgb(17, 17, 17)`; unfolded header → `700 rgb(75, 85, 99)`
  (unchanged). Strip background is `#f3f4f6` in both modes, so no dark-mode
  override needed.

## KF-320 — Move toolbar icon 2px-shaft SVG arrow

- Contract: replace the U+2794 text glyph (4px shaft, 26×19px ink, 184 ink
  px) with a lighter arrow matching the GM-076 master (2px shaft,
  ~20×18px ink, 86 ink px, #666).
- Files: `templates/modal.html` (inline SVG in the Move `.tm-icon` span;
  `tm-glyph-lg` class removed from that span), `static/style.css`
  (GM-076 comment updated — only the `+` glyph still scales up).
  Commit: `6bb6cd1` "KF-320: Move toolbar icon as 2px-shaft SVG arrow"
- Glyph design (viewBox 0 0 21 19): shaft `line (1.5,9.5)→(13.3,9.5)`,
  `stroke-width:2`, round cap; filled triangular head
  `polygon 12.3,1.5 20.3,9.5 12.3,17.5`, `fill:#666`.
- Raster verification (headless Chromium @10x, PIL pixel scan):
  shaft = exactly **2.0 CSS px**; bbox **19.7 × 15.8** (master 20×18);
  total ink **86.2 px** (master 86). Screenshot: `/tmp/kf320/glyph.png`.
- Note: the modal template is rendered on task-open via htmx; it is not in
  the board page HTML. Askama compile (via `cargo build`) validates the
  template; the verifier's blind pass will exercise the open modal.

## Diffs

Full combined diff saved at `/tmp/kf320/batch13-full.diff` (45 insertions,
4 deletions across `static/style.css`, `templates/board.html`,
`templates/modal.html`). Final tree verified byte-identical to the L0-green
tree (`git diff b7d5fbe..HEAD` matches).

## Iteration count

1 attempt per defect; no verifier round-trips (first build). No tracker
edits made — Dale inspects the diffs and updates the tracker.

DONE: batch-13 built — 438d7d7 (KF-312), 358795e (KF-313), 264e775 (KF-314), 6bb6cd1 (KF-320); L0 green (fmt/build/clippy clean, 85 tests pass, node --check clean); L1 boot + page-load verified on port 3217.
