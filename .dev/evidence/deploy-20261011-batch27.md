# Deploy evidence — batch-27 (KF-351 + KF-352)

- **Date:** 2026-10-12T~04:50Z
- **Release tree:** cherry-picks of `5816005` (KF-351 iCal feed), `fa3b6c8` (KF-352 due_repeat), `fff4d0e` (KF-351 spec fixes) onto origin/main `0f76f23e` — clean, no conflicts. Local master `6bb6cd1` never merged.
- **New origin/main:** `cc19c87cd600e4b5e5cd051498de522b044ca0f8`
- **New production SHA:** `cc19c87cd600e4b5e5cd051498de522b044ca0f8` (was `5244f3c`)

## L0 (release tree)
- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings, 0 errors
- `cargo test` — 95 passed, 0 failed (10+44+3+12+8+6+4+2+2+1+3)
- `node --check static/app.js` — PASS

## Deploy (Chip's standing 2026-09-30 authorization)
- Pushed via `.dev/push_via_api.py` (absolute worktree path, base `0f76f23e` asserted) — PUSHED OK
- Pin updated in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix` (rev + hash only, via sed): rev `cc19c87c…`, hash `sha256-+g1HxsAad8UTWCtET0OfbK8V6j1Bi50pQN0OE22Iqzc=` — hash method validated by reproducing the previous pin exactly via GitHub tarball + `nix hash path`
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed ONLY `chipflow.service` would stop/start
- `nixos-rebuild switch --flake .#green-box`: clean; service active
- Temp tarballs on green-box cleaned up

## Production verification (fix content in the new build)
- Green-box `:3000` `/api/v1/version` → `cc19c87cd600e4b5e5cd051498de522b044ca0f8` ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) → same SHA ✓
- `GET /api/calendar-feed/definitely-not-a-token` → 404 (feed route registered, unknown token rejected) ✓
- Served `static/app.js` contains `initFeed`/`repeatInstances` markers (4 matches) ✓
- `:3100` stale test instance untouched; no `pkill` used; Caddy/Tailscale undisturbed; `secrets/green-box.yaml` untouched
