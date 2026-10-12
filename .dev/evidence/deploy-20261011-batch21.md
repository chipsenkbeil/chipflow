# Deploy Report — Batch-21 (2026-10-11)

**Date:** 2026-10-11 ~21:00 CDT (2026-10-12 02:00 UTC)
**Production SHA:** `109aade3dee90a06a3e3a9d8a6fc6b8028223f03`
**Previous SHA:** `d10b16c5473bfd1e80ff9ed76cbfe9a118b23b38`
**GitHub main:** `109aade3dee90a06a3e3a9d8a6fc6b8028223f03` (pushed via `.dev/push_via_api.py`)

## What shipped (2 verified features, cherry-picked onto the release lineage)

**Critical lineage note:** local master `6bb6cd1` is BEHIND `origin/main` in app code
(origin/main carries the KF-306 move-target dialog + Round-6…Round-11 releases +
the batch-19 and batch-20 releases). The 2 built commits were cherry-picked onto
`origin/main` (`d06953dd`), NOT merged — no deployed fix regressed.

- **KF-343** (`e786ce5`) — Task count report: `GET /b/:board_id/task-count`,
  color × column count table rendered client-side from the shared export-data
  dataset (`templates/task_count.html`, `TaskCountPage` JS module).
- **KF-344** (`41d4734`) — Throughput report: `GET /b/:board_id/throughput`,
  tasks completed per week over the last 12 Monday-start weeks as an inline-SVG
  line chart (green line + dots, tooltips), no chart library
  (`templates/throughput.html`, `ThroughputPage` JS module).

## L0 (release tree, exact)

- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — **87 passed, 0 failed**
- `node --check static/app.js` — PASS

## Deployment

- Pushed to GitHub via `.dev/push_via_api.py` (base `d06953dd` asserted against
  remote tree; absolute worktree path used — relative paths fail with
  FileNotFoundError).
- Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`:
  - rev: `109aade3dee90a06a3e3a9d8a6fc6b8028223f03`
  - hash: `sha256-vGOmIwOuboUJwkrz2zr4GeWTQxwkJCL9OftvtqdmpQg=`
  - Hash method validated first: GitHub tarball + `nix hash path` reproduces the
    previous pin (`sha256-MUivl4HSPFUVdi33GALTKyxHh4SWmM3mXZyOmpi+KFA=`) exactly.
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed ONLY
  `chipflow.service` would stop/start (plain `nixos-rebuild dry-activate`
  without `--flake .#green-box` fails with "nixos-config not found").
- `nixos-rebuild switch --flake .#green-box`: completed cleanly.
- Green-box-local `http://127.0.0.1:3000/api/v1/version` → `109aade3...` ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) → `109aade3...` ✓
- Production's served `static/app.js` contains the new report markers
  (`TaskCountPage`, `ThroughputPage` ×4) ✓
- Both new routes respond 303 (auth redirect, not 404) on the fake-board probe ✓
- `chipflow.service` active; `:3100` stale test instance untouched (still `73b68b06`);
  no `pkill` used; Caddy/Tailscale undisturbed.

## Fix-content chain

Independent verifier tested both features live on the exact release tree →
deployed tree proven byte-identical (nar hash self-validated against the GitHub
rev) → running service reports that SHA → served assets contain the new code →
routes live. Verifier's end-to-end evidence: task-count table matched independently
computed expectations (Yellow 1/1/1, Green 1/0/1, Blue 0/1/0, Red 0/0/1, "7 tasks");
throughput chart showed 12 Monday-start week buckets with a spike of 3 in the
current week, tooltips on all 12 points, last x-label unclipped.

## Tracker

Not modified by the release worker — RELEASED prepends for KF-343/KF-344 left to
the orchestrator per process.

## Notes

- Release worktree `.dev/worktrees/batch-21-release` (detached `91a9db3`) left in
  place for the orchestrator to remove after the tracker update.
