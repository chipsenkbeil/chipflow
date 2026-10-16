# Deploy Report — Round-8 (2026-10-05)

**Date:** 2026-10-06 ~04:15 UTC / 2026-10-05 ~21:15 PDT
**Production SHA:** `11d7734faeab44e59bcedf416d2634745034a760`
**Previous SHA:** `138574461fb5d9ef8a04d7db140f184aeed48e43` (Round-7)

## What shipped (3 verified HIGH fixes)

- **KF-003** — "Add time manually" dialog hardened end to end: Escape now dismisses the topmost overlay first (the manual-time future-time error) instead of closing the dialog out from under it, preserving dialog state; `ManualTime.open` guarded against pages without the dialog (null-guard on `#manual-time-overlay`).
- **KF-004** — Edit time-entry dialog hardened: `EditEntry.open` fetch now has a `.catch()` that toasts "Could not load the time entry." on failure, and malformed `started_at` values are guarded with an `isNaN` check (Invalid Date no longer populates NaN into the date/time fields).
- **KF-005** — "Why did you stop?" menu populated: server backfills the 15 KanbanFlow-verbatim interruption reasons for legacy settings with empty lists, so the menu opens populated instead of empty (includes 2 new tests).

## Verification

- L0 fully green on master: `cargo fmt --check` ✓, `cargo build` ✓, `cargo clippy --all-targets` (0 warnings) ✓, `cargo test` (83 passed, 0 failed) ✓, `node --check static/app.js` ✓
- All 3 fixes independently verified by separate verifiers (not the builders).
- Green-box-local `/api/v1/version` reports new SHA ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) reports new SHA ✓
- `chipflow.service`: active, sole affected unit (dry-activate confirmed)

## Git history (master)

```
d2a2ec6 KF-004: harden EditEntry.open with catch and date validation
3b17b0d KF-005: backfill interruption reasons for legacy settings
8889284 KF-003: Escape dismisses topmost overlay first; guard ManualTime.open
0eec86b Merge KF-295: Time spent Print opens KanbanFlow-style Print sheet
...
1385744 Round-7 (previous production)
```

Note: KF-004's source commit 8563656 also contained KF-003's hunks (shared working-tree contamination); only the 2 KF-004-specific hunks (isNaN guard + .catch()) were applied to master via filtered patch.

## Tracker

All 3 entries marked DEPLOYED in TrueNAS chipflow-tracker.git (commit 73244ea).

## Deployment

- Pushed to GitHub via `.dev/push_via_api.py`: remote main `11d7734faeab44e59bcedf416d2634745034a760`
- Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`: rev `11d7734faeab44e59bcedf416d2634745034a760`, hash `sha256-e0W3I3B5CqHZL8SvceFQ/oO+BYnvSfYrBq95BunhowM=`
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed only `chipflow.service` affected
- `nixos-rebuild switch --flake .#green-box`: completed
- Production live at https://chipflow.chip.network
