# Round-12 Deploy Evidence — 2026-10-07

## Release Summary
- **Round:** 12
- **Date:** 2026-10-07 ~01:15 AM PDT (08:15 UTC)
- **Production SHA:** `92f4d33f4f0f31c2ab88372c8dcfe1131e044cd9` (short: `92f4d33f`)
- **Previous SHA:** `a84eb56bb266686bca1f850e771afe6886212a82` (Round-11)

## Fixes Shipped (1)

### KF-309 — Solid funnel and pencil glyphs in board header
- **Files:** `templates/board.html` (4 insertions, 2 deletions)
- **Change:** 
  - Filter funnel: was hollow outline SVG (`fill="none" stroke="currentColor" stroke-width="1.7"`); now solid fill (`fill="currentColor"`) with same geometry
  - Edit pencil: was thin `✎` (U+270E) text glyph; now solid SVG pencil (single silhouette subpath, `fill="currentColor"`)
- **Builder:** dale-vm, worktree `~/workspace/wt-KF-309b`, commit `d6cdfc6`
- **Verifier:** dale-vm (independent), verified via custom SVG rasterizer — 0 hollow pixels in both glyphs
- **Note:** First build (b24320f) was REJECTED — pencil used Bootstrap detail-cutout variant with hollow interior. Rebuild uses solid silhouette only.

## L0 Results (merged master b7d5fbe)
- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — PASS, zero warnings
- `cargo test` — **85 passed, 0 failed**
- `node --check static/app.js` — PASS

## Deploy Steps
1. Cherry-picked `d6cdfc6` onto master (ccd0e00 → b7d5fbe), clean apply
2. L0 green on merged master
3. Pushed to GitHub via `.dev/push_via_api.py` → remote main `92f4d33f`
4. Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`:
   - rev: `92f4d33f4f0f31c2ab88372c8dcfe1131e044cd9`
   - hash: `sha256-jmGu9vCcU4SYJezfxpxLOWtcHo6xGPuVhNYz6O+4/wI=`
5. `nixos-rebuild dry-activate` — confirmed ONLY `chipflow.service` affected
6. `nixos-rebuild switch` — completed cleanly

## Verification
- Green-box-local `127.0.0.1:3000/api/v1/version` → `92f4d33f4f0f31c2ab88372c8dcfe1131e044cd9` ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) → `92f4d33f4f0f31c2ab88372c8dcfe1131e044cd9` ✓

## Tracker
- KF-309 marked RELEASED (commit `83d677a`, pushed to TrueNAS)
