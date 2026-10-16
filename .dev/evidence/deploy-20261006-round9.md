# Deploy Report — Round-9 (2026-10-06)

**Date:** 2026-10-07 ~06:10 UTC / 2026-10-06 ~23:10 PDT
**Production SHA:** `a52ed4f46c3e7a70d13faa607b49c6cc634f8e4b`
**Previous SHA:** `11d7734faeab44e59bcedf416d2634745034a760` (Round-8)

## What shipped (3 verified LOW fixes)

- **KF-277** — Task-modal Time log view decluttered: removed the "Total: 2h 30m" line under the title, the explicit per-entry "Edit" link, and the "M" badges on manual entries. Matches KanbanFlow GM-096 (edit still opens by clicking the entry itself, per GM-084).
- **KF-280** — Running pomodoro popup task row now shows the task's color dot before the task name, matching KanbanFlow GM-101. Server `/api/timer/status` now returns `task_color` (documented in `agents.md`). This is a rebuild: the 2026-10-04 muse-pi build was rejected by the verifier.
- **KF-282** — Right-clicking the timer pill / timer popup area now opens a context menu with Comment, Label, Edit, Delete, matching KanbanFlow GM-109 (thin catalog-text evidence). This is a rebuild: the 2026-10-04 muse-pi build was rejected by the verifier.

## Verification

- L0 fully green on merged master: `cargo fmt --check` ✓, `cargo build` ✓, `cargo clippy --all-targets` (0 warnings) ✓, `cargo test` (83 passed, 0 failed) ✓, `node --check static/app.js` ✓
- All 3 fixes independently verified by separate verifiers (not the builders) before release.
- Green-box-local `/api/v1/version` reports new SHA ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) reports new SHA ✓
- `chipflow.service`: active, sole affected unit (dry-activate confirmed)

## Git history (master)

```
e981ece KF-282: Add right-click context menu to timer area
8caa844 KF-280: Add task color dot to running pomodoro popup task row
bfbd831 KF-277: Remove Total line, Edit link, M badges from time log view
d2a2ec6 KF-004: harden EditEntry.open with catch and date validation (Round-8)
...
11d7734 Round-8 (previous production)
```

All three cherry-picked cleanly from isolated worktrees (no conflicts requiring manual resolution beyond git auto-merge).

## Tracker

All 3 entries marked RELEASED in TrueNAS chipflow-tracker.git (commit 8714234).

## Deployment

- Pushed to GitHub via `.dev/push_via_api.py`: remote main `a52ed4f46c3e7a70d13faa607b49c6cc634f8e4b`
- Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`: rev `a52ed4f46c3e7a70d13faa607b49c6cc634f8e4b`, hash `sha256-z5d/kwCKgDMroC5GDs2+6wZKHjMUXghBTUKB6/KISA4=`
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed only `chipflow.service` affected
- `nixos-rebuild switch --flake .#green-box`: completed
- Production live at https://chipflow.chip.network
