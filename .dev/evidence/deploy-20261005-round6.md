# Deploy Report — Round-6 (2026-10-05)

**Date:** 2026-10-05 ~20:55 UTC / 15:55 CDT  
**Production SHA:** `a073a65dc865f0600ccd3feefa3c05cee144f85d`  
**Previous SHA:** `8de92e60f645c079755f320b743636ae4d1cbcc9` (Round-5)

## What shipped (4 verified MEDIUM fixes)

- **KF-281** — Stopwatch popup: idle tab now shows big "00:00" readout with green "▶ Start" button; Settings footer icon visible in all states (matches KanbanFlow GM-105).
- **KF-298** — Pressing `T` in the task modal now opens the task's Timer menu (not the global timer popup).
- **KF-306** — Column delete confirmation: verbatim "Are you sure you want to delete this column?" with "Move the tasks to the column" checkbox + target dropdown.
- **KF-318** — Board-filter option values no longer render with literal quotes (Askama `{{ }}` Display trait fix).

## Verification

- L0 fully green on master: `cargo fmt --check` ✓, `cargo build` ✓, `cargo clippy --all-targets` (0 warnings) ✓, `cargo test` (81 passed, 0 failed) ✓, `node --check static/app.js` ✓
- All 4 fixes independently verified by separate verifiers (not the builders).
- Green-box-local `/api/v1/version` reports new SHA ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) reports new SHA ✓
- `chipflow.service`: active, sole affected unit (dry-activate confirmed)

## Git history (master)

```
aa7ddf9 KF-306: Column delete confirmation (cherry-picked from 4514346)
b841047 Revert unverified KF-293 commits f2c4ed2 e37ce37
3bff769 Revert partial KF-293 1bb0cd4
2f9dffe KF-281: Stopwatch popup idle readout (cherry-picked from fcadc86)
ab83c32 Revert contaminated 99c3e6a
59555ce KF-318: Fix board-filter option values
4d41322 KF-298: T opens task modal Timer menu
8de92e6 Round-5 (previous production)
```

## Incident: contaminated commit 99c3e6a

During the build phase, commit 99c3e6a (KF-281) accidentally swept in changes from 4 other defects via a broad `git add`:
- KF-293 (partial, builder in-flight)
- KF-295 (partial, builder in-flight)  
- KF-306 (complete, verified)
- KF-292 (refactor of already-deployed code)

The KF-281 verifier caught the contamination and rejected it. Remediation:
1. Stopped KF-293 and KF-295 builders immediately.
2. Extracted clean KF-281 to branch `fix/KF-281-isolated` (fcadc86), verified, cherry-picked to master.
3. Extracted clean KF-306 to branch `fix/KF-306-clean` (4514346), cherry-picked to master.
4. Reverted contaminated 99c3e6a and partial KF-293 commits from master.
5. Fresh builders dispatched for KF-293 and KF-295 (in progress, not in this release).

**Lesson:** Builders must use targeted `git add` (specific files), never `git add -A`.

## Tracker

All 4 entries marked DEPLOYED in TrueNAS chipflow-tracker.git.

## Deployment

- Pushed to GitHub via `.dev/push_via_api.py`: remote main `a073a65dc865f0600ccd3feefa3c05cee144f85d`
- Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`: rev `a073a65d...`
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed only `chipflow.service` affected
- `nixos-rebuild switch --flake .#green-box`: completed
- Production live at https://chipflow.chip.network
