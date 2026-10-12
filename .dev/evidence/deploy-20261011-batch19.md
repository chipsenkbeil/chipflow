# Deploy Report — Batch-19 (2026-10-11)

**Date:** 2026-10-11 ~20:15 CDT (2026-10-12 01:15 UTC)
**Production SHA:** `d1d7d0fa9459ce7f6d9c343d153f0ec705f3bf45`
**Previous SHA:** `8794cb266f2d8386f67013426f6fc78f02106eca`
**GitHub main:** `d1d7d0fa9459ce7f6d9c343d153f0ec705f3bf45` (pushed via `.dev/push_via_api.py`)

## What shipped (4 verified fixes, cherry-picked onto the release lineage)

**Critical lineage note:** local master `6bb6cd1` was BEHIND `origin/main` in app code
(`git diff 6bb6cd1 cb0bf143 -- src static templates` = 112+/13− in `static/app.js`;
origin/main carried the KF-306 move-target dialog + Round-6…Round-11 releases).
The 4 built commits were cherry-picked onto `origin/main` (`8da8054`), NOT merged —
no deployed fix regressed. Release tree `07ce4ad` is byte-identical to the deployed
tree across `src static templates agents.md` (`git diff 07ce4ad origin/main` = empty).

- **KF-308** (`d5c8fda`) — Pomodoro template's first column `"Work To-do"` → `"To-do"`
  (`src/db.rs` seed + starter-task column reference).
- **KF-348** (`cb14daf`) — `normalize_due_input` accepts bare local datetimes with
  seconds (`%Y-%m-%dT%H:%M:%S`, `%Y-%m-%d %H:%M:%S`) as server-local time; new unit
  test `normalize_due_input_bare_local_datetime_with_seconds`.
- **KF-249** (`a94a507`) — color legend per the 2026-10-02 contract correction: fills
  from the palette `background_hex`, top edges `#f5cc00/#59d600/#70b0ff/#ff858f`.
  (Independent verifier re-derived these values by pixel-sampling the unresampled
  1919×998 KanbanFlow capture — the correction is exactly right.)
- **KF-347** (`79d9f03`) — Break activities statistics tab fully implemented:
  `TimeEntryRow.activity_id`, `TimerStopInput.activity_id` (validated against defined
  activities), break-activity picker in the timer popup's break view wired into both
  stop paths, `BreakActivityStat` aggregation in `/api/timer/statistics`, tab
  rendering with per-activity totals + daily-goal progress. The "not available yet"
  placeholder is gone.

Also verified already-fixed (no code needed): KF-230, KF-246, KF-301, KF-316, KF-317.
KF-247/KF-248 marked INVALID per the documented 2026-10-02 re-triage (separate tracker commit).

## L0 (release tree, exact)

- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — **86 passed, 0 failed**
- `node --check static/app.js` — PASS

## Deployment

- Pushed to GitHub via `.dev/push_via_api.py` (base `8da8054` asserted against remote tree).
- Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`:
  - rev: `d1d7d0fa9459ce7f6d9c343d153f0ec705f3bf45`
  - hash: `sha256-9iF/IAdqLt8dLbIsCaS0IGv9iOZuQweKtVx7mfrve0I=`
  - Hash method validated first: `git archive` + `nix hash path` reproduces the
    previous pin (`sha256-71PksaJhJx1WMX45OnNOaaFZXhFWA4BNcLDqVSjQwj0=`) exactly.
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed ONLY `chipflow.service`
  would stop/start.
- `nixos-rebuild switch --flake .#green-box`: completed cleanly.
- Green-box-local `http://127.0.0.1:3000/api/v1/version` → `d1d7d0f...` ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) → `d1d7d0f...` ✓
- Production's served `static/app.js` contains the KF-347 client code
  (`timer-break-activity`, `break_activities` ×8, `tp-break-activity` ×2) ✓
- `chipflow.service` active (new build started 20:11:28 CDT); `:3100` stale test
  instance untouched (still `73b68b06`).

## Tracker

Not modified by the release worker — RELEASED prepends for the 9 batch-19 defects
left to the orchestrator per process.

## Notes

- Minor nits from the independent verifier (not failures, not blockers): KF-348's doc
  comment still lists only the no-seconds formats; the timer-log view doesn't serialize
  `activity_id`; breaks with no task log nothing (pre-existing `log_timer_session`
  behavior).
- Release worktree `.dev/worktrees/batch-19-release` (detached `07ce4ad`) left in place
  for the orchestrator to remove after the tracker update.
