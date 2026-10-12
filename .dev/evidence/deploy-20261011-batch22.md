# Deploy evidence — batch-22 (KF-337 Burndown + KF-339 Cumulative flow)

**Date:** 2026-10-11/12 (UTC). **Release worker:** independent of builder and verifier.

## What shipped
- **KF-337** (`c9887f3` → cherry-picked as `2c9eb23`) — Burndown report: `GET /b/:board_id/burndown`, 30-day remaining-tasks line chart
- **KF-339** (`ab4823e` → cherry-picked as `42cf90f`) — Cumulative flow: `GET /b/:board_id/cumulative-flow`, 30-day stacked area chart

## Lineage (batch-19/20/21 pattern)
Local master `6bb6cd1` is behind `origin/main` in app code — cherry-picked both commits onto `origin/main` (`36f19a5`) in release worktree `.dev/worktrees/batch-22-release`. Local master never merged; no deployed fix regressed.

## L0 (release tree, exact)
- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — **87 passed, 0 failed**
- `node --check static/app.js` — PASS

## GitHub
- Pushed via `.dev/push_via_api.py` (absolute worktree path; base `36f19a5` asserted)
- **New origin/main SHA:** `4a900de4b9aa1bcfd410954f77a5b974ededdf32`

## Green-box deploy (per Chip's standing 2026-09-30 authorization)
- Hash method validated first by reproducing the previous pin exactly via GitHub tarball + `nix hash path`
- Pin updated in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix` (rev + hash ONLY):
  - rev `4a900de4b9aa1bcfd410954f77a5b974ededdf32`
  - hash `sha256-4TlDifRYcRLI95RTK44i8JUebmmYrV2qCUNFK5XeyUE=`
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed ONLY `chipflow.service` would stop/start
- `nixos-rebuild switch --flake .#green-box`: clean; service active

## Production verification
- Green-box `:3000` `/api/v1/version` → `4a900de4b9aa1bcfd410954f77a5b974ededdf32` ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) → same SHA ✓
- Served `static/app.js` contains `BurndownPage`/`CumulativeFlowPage` markers (4 matches) ✓
- Both report routes return 303 (auth redirect = registered, not 404) ✓
- Verifier's end-to-end evidence on the exact release tree: burndown math independently recomputed (exact match, 30 dots); CFD band math independently recomputed from history events with SVG geometry decoded (exact match, bottom band = last column)
- `:3100` stale test instance untouched; no `pkill` used; Caddy/Tailscale undisturbed; temp tarballs in /tmp cleaned by session end

## Not done
- Tracker RELEASED prepends for KF-337/KF-339 — left to the orchestrator.
