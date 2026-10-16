# Deploy Report — Round-11 (2026-10-07)

**Date:** 2026-10-07 ~08:05 UTC / 2026-10-07 ~01:05 PDT
**Production SHA:** `a84eb56bb266686bca1f850e771afe6886212a82`
**Previous SHA:** `c559e90cd75cc06d1b395e71d2bb74c311f47f26` (Round-10)

## What shipped (2 verified LOW fixes)

- **KF-300** — Board-colors page no longer renders stray horizontal rules under each color-row cell (`border-bottom: none` on `.color-table td`). Matches KanbanFlow (GM-131).
- **KF-303** — Disabled colors on the board-colors page are now listed alphabetically by color name; enabled colors keep palette order. Matches KanbanFlow (GM-131, GM-144).

Note: KF-309 was excluded from this release — its verifier rejected the fix (pencil glyph not solid) and it is being rebuilt separately.

## Verification

- L0 fully green on merged master: `cargo fmt --check` ✓, `cargo build` ✓, `cargo clippy --all-targets` (0 warnings) ✓, `cargo test` (85 passed, 0 failed) ✓, `node --check static/app.js` ✓
- Both fixes independently verified by separate verifiers (not the builders) before release.
- Green-box-local `/api/v1/version` (127.0.0.1:3000) reports new SHA ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) reports new SHA ✓
- `chipflow.service`: active, sole affected unit (dry-activate confirmed); restarted cleanly.

## Git history (master)

```
ccd0e00 KF-303: Sort disabled colors alphabetically
4cc4276 KF-300: Remove stray horizontal rules under color-row cells
6fec570 KF-299: Two-stage Escape discards edits before closing modal (Round-10)
...
c559e90 (GitHub) Round-10 (previous production)
```

Both cherry-picked cleanly from isolated worktrees (different files: `static/style.css` vs `src/routes.rs`; no conflicts). Pushed to GitHub as a single rebased commit via `.dev/push_via_api.py` (established pattern).

## Tracker

Both entries marked RELEASED in TrueNAS chipflow-tracker.git (commit 4d43258).

Note: the remote tracker tip had moved (local verifier/builder commits for KF-300/303/309 were pushed by another worker, superseding an older divergent tip). Reconciled via rebase; Round-10 RELEASED markers for KF-291/296/299 confirmed intact in the merged tree.

## Deployment

- Pushed to GitHub via `.dev/push_via_api.py`: remote main `a84eb56bb266686bca1f850e771afe6886212a82`
- Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`: rev `a84eb56bb266686bca1f850e771afe6886212a82`, hash `sha256-vaGDxUs/9Yq5I1KxO+kvCmJNBtD0qoMJGQFZj1p26SA=` (nar hash validated by reproducing the known Round-10 hash with the same `nix hash path --sri` method)
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed only `chipflow.service` affected (would stop/start chipflow.service)
- `nixos-rebuild switch --flake .#green-box`: completed
- Production live at https://chipflow.chip.network
