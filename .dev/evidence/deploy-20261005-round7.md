# Deploy Report — Round-7 (2026-10-05)

**Date:** 2026-10-05 ~20:55 UTC / 15:55 CDT  
**Production SHA:** `138574461fb5d9ef8a04d7db140f184aeed48e43`  
**Previous SHA:** `a073a65dc865f0600ccd3feefa3c05cee144f85d` (Round-6)

## What shipped (2 verified MEDIUM fixes)

- **KF-293** — Pomodoro statistics: Period presets match KanbanFlow verbatim list ("Last 7 days", "Last 14 days", "Last 30 days" (default), "This week", "Last week", "This month", "Last month", "Custom (absolute)", "Custom (relative)"); bar count labels atop bars; tooltip format "Tuesday 1 Sep 2026: 2 Pomodori (30m)".
- **KF-295** — Time spent Print button now opens KanbanFlow-style "Print" sheet (titled "Print", with Filter and Print buttons top-left, task table body) instead of firing the browser's native print dialog directly. Sheet's Print button fires `window.print()` with print CSS scoped to the sheet.

## Verification

- L0 fully green on master: `cargo fmt --check` ✓, `cargo build` ✓, `cargo clippy --all-targets` (0 warnings) ✓, `cargo test` (81 passed, 0 failed) ✓, `node --check static/app.js` ✓
- Both fixes independently verified by separate verifiers (not the builders).
- Green-box-local `/api/v1/version` reports new SHA ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) reports new SHA ✓
- `chipflow.service`: active, sole affected unit (dry-activate confirmed)

## Git history (master)

```
0eec86b Merge KF-295: Time spent Print opens KanbanFlow-style Print sheet
7971b4c Re-apply verified KF-293 (e37ce37, f2c4ed2)
aa7ddf9 KF-306: Column delete confirmation (Round-6)
...
a073a65 Round-6 (previous production)
```

## Tracker

Both entries marked DEPLOYED in TrueNAS chipflow-tracker.git (commit 984b58b).

## Deployment

- Pushed to GitHub via `.dev/push_via_api.py`: remote main `138574461fb5d9ef8a04d7db140f184aeed48e43`
- Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`: rev `138574461fb5d9ef8a04d7db140f184aeed48e43`, hash `sha256-SA3Btu8B/bf2Dmsupix+0iMlOsLk1Z2jDjjZGkzPFDs=`
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed only `chipflow.service` affected
- `nixos-rebuild switch --flake .#green-box`: completed
- Production live at https://chipflow.chip.network
