# Batch-23 deploy evidence — 2026-10-11

## Release
- origin/main: `520ac4b415fcb600823ea1c040e3ee7eac4a8857` (was `1fc6dadc`)
- Cherry-picked onto origin/main (batch-19/20/21/22 pattern; local master `6bb6cd1` never merged):
  - `1712382` KF-340 — Cycle & lead time report
  - `4265692` KF-341 — Due date performance report
- Conflict resolution note: the KF-340 cherry-pick conflicted with origin/main's
  KF-337/KF-339 report commits in routes.rs, app.js, style.css, agents.md,
  agents/skill.md. All conflicts were additive (independent report pages);
  resolved by keeping both sides. One orphan `=======`/`>>>>>>>` pair in
  app.js (CumulativeFlowPage fetch block vs a duplicate CycleLeadPage fetch
  body) was resolved by keeping origin's CumulativeFlowPage block — the
  CycleLeadPage module inserted from the cherry-pick was already complete.
  Folded into the KF-340 commit via fixup+autosquash.

## L0 (release tree)
- cargo fmt --check: PASS
- cargo build: PASS
- cargo clippy --all-targets: 0 warnings, 0 errors
- cargo test: 89 passed, 0 failed (9+37+3+12+8+6+4+2+2+1+3+0)
- node --check static/app.js: PASS

## Deploy (green-box, standing 2026-09-30 authorization)
- Pin bumped in homelab-dale-chipflow/modules/chipflow.nix (rev + hash only):
  rev `520ac4b415fcb600823ea1c040e3ee7eac4a8857`,
  hash `sha256-xNLyBIu7Z5HPVQQccY4IjZRzfFH/213ceuroK8ZJxwY=`
  (hash method validated by reproducing the previous pin exactly)
- nixos-rebuild dry-activate --flake .#green-box: only chipflow.service stop/start
- nixos-rebuild switch --flake .#green-box: clean, service active

## Production verification
- green-box :3000 /api/v1/version -> 520ac4b415fcb600823ea1c040e3ee7eac4a8857
- public https://chipflow.chip.network/api/v1/version (tunnel proxy) -> same SHA
- /b/:id/cycle-lead -> 303 (route registered); /b/:id/due-date -> 303
- served static/app.js contains CycleLeadPage/DueDatePage markers (4 matches)
- :3100 stale test instance untouched; no pkill used; Caddy/Tailscale undisturbed
