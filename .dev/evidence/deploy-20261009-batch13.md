# Batch-13 Deploy Evidence — 2026-10-09

## Release Summary
- **Batch:** 13
- **Date:** 2026-10-09 ~05:20 AM PDT (12:20 UTC)
- **Production SHA:** `8794cb266f2d8386f67013426f6fc78f02106eca` (short: `8794cb26`)
- **Previous SHA:** `a2f9b16c6879e6e22d41f31dd6d32634707b2cf6` (short: `a2f9b16c`)
- **GitHub:** `chipsenkbeil/chipflow`, branch `main` → `8794cb26`

## Fixes Shipped (4)

### KF-312 — Inline WIP count in neutral gray rgb(91,91,91)
- **Files:** `static/style.css` (9 insertions, 1 deletion)
- **Change:** `.columnHeader-count` color `var(--muted)` (#6b7280 blue-gray) →
  `#5b5b5b` = rgb(91,91,91) per the GM-033 master image. Dark-mode guard
  keeps the muted tone; the WIP-exceeded darkred warning rule still wins.
- **Builder:** dale-vm, worktree `~/workspace/pomodoro-kanban/.dev/worktrees/batch-13`, commit `438d7d7`
- **Verifier:** dale-vm (independent); inline count measured exactly
  rgb(91,91,91); badge untouched; WIP-exceeded 4/3 shows darkred
  rgb(139,0,0) bold.

### KF-313 — Vertical separator between + button and standalone count cell
- **Files:** `templates/board.html` (3 insertions), `static/style.css` (18 insertions)
- **Change:** new `.columnHeader-separator` 1px vertical line between the
  green `+` add-task button and the standalone count badge, rendered only
  for columns without a WIP limit; `#d4d7dc` light / `#374151` dark,
  matching the KF-182 column-separator tone (GM-032 parity).
- **Builder:** dale-vm, commit `358795e`
- **Verifier:** dale-vm (independent); separator present in all 3 non-WIP
  headers, strictly between `+` and badge; zero separators in the WIP
  column; screenshot-confirmed.

### KF-314 — Folded-swimlane strip label regular weight, near-black
- **Files:** `static/style.css` (9 insertions)
- **Change:** `.folded-strip .swimlane-name` → `font-weight: 400`,
  `color: #111111` = rgb(17,17,17) per GM-037; unfolded swimlane headers
  keep bold gray-600.
- **Builder:** dale-vm, commit `264e775`
- **Verifier:** dale-vm (independent); computed style exactly 400 /
  rgb(17,17,17); unfolded header unchanged.

### KF-320 — Move toolbar icon as 2px-shaft SVG arrow
- **Files:** `templates/modal.html` (4 insertions, 1 deletion), `static/style.css` (comment update)
- **Change:** the U+2794 text glyph (4px shaft) replaced with an inline SVG
  arrow: 2px shaft, round cap, filled triangular head, `#666`, ~21px wide
  (GM-076 parity).
- **Builder:** dale-vm, commit `6bb6cd1`; raster-verified shaft exactly
  2.0 CSS px, ink 86.2 px vs master 86.
- **Verifier:** dale-vm (independent); real modal fragment: shaft exactly
  2.00 CSS px (min=max=median, no hollow rendering), ink 85.4–89 px.
  Observation (not a rejection): arrowhead bbox 19.8×16.0 vs the defect
  entry's approximate ~20×18 — hard metrics met.

## L0 Results (release tree 5c38ae2, = deployed tree)
- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — PASS, zero warnings
- `cargo test` — **85 passed, 0 failed**
- `node --check static/app.js` — PASS

## Deploy Steps
1. Merged `batch-13` → `master` locally (ff-only, b7d5fbe → 6bb6cd1), L0 green.
2. Release lineage note: remote `main` was at `a2f9b16c` (Oct-8 release of
   KF-281/298/306, app.js-only delta vs b7d5fbe). Applied the batch-13 diff
   cleanly onto `a2f9b16` as release commit `5c38ae2`; L0 re-verified on
   the exact release tree.
3. Pushed to GitHub via `.dev/push_via_api.py` → remote `main` `8794cb26`
   (tree `676ddced788ca0ae12b1e88ab6c7a036e01e86a3`, verified identical to
   local release tree).
4. Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`:
   - rev: `8794cb266f2d8386f67013426f6fc78f02106eca`
   - hash: `sha256-71PksaJhJx1WMX45OnNOaaFZXhFWA4BNcLDqVSjQwj0=`
   - Hash method validated: `git archive` + `nix hash path` reproduces the
     known-good a2f9b16c pin (`sha256-3O2CwDVynUoHf4kHlyrZsUcUESszGSWjOziw7C2QPug=`) exactly.
5. `nixos-rebuild dry-activate --flake .#green-box` — confirmed ONLY
   `chipflow.service` would stop/start.
6. `nixos-rebuild switch` — completed cleanly.

## Verification
- Green-box-local `127.0.0.1:3000/api/v1/version` → `8794cb266f2d8386f67013426f6fc78f02106eca` ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) → `8794cb266f2d8386f67013426f6fc78f02106eca` ✓

## Tracker
- KF-312/313/314/320 marked RELEASED (commit `650768c`, pushed to TrueNAS)
