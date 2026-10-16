# Deploy Report — Round-10 (2026-10-07)

**Date:** 2026-10-07 ~07:45 UTC / 2026-10-07 ~00:45 PDT
**Production SHA:** `c559e90cd75cc06d1b395e71d2bb74c311f47f26`
**Previous SHA:** `a52ed4f46c3e7a70d13faa607b49c6cc634f8e4b` (Round-9)

## What shipped (3 verified LOW fixes)

- **KF-291** — Date picker weekday row now uses three-letter names ("Sun Mon…") instead of single letters, and Today / Yesterday / This month quick buttons added. Matches KanbanFlow (GM-122/GM-171).
- **KF-296** — Color filter labels now use KanbanFlow's "Pomodoros" plural ("2 Pomodoros", "3 Pomodoros", ">3 Pomodoros") instead of the Italian "Pomodori". Includes a one-time lazy backfill (`migrate_pomodoro_plural_labels`, runs in `Db::connect` behind the `pomodoro_plural_backfill_done` marker) that renames stored color labels from pre-Round-2 boards; only exact matches are rewritten, user-customized labels untouched, "1 Pomodoro" unchanged. Documented in-code as a lazy backfill, not a schema migration (same pattern as the existing `migrate_legend_default`).
- **KF-299** — Escape in the task modal is now two-stage: first Esc discards in-progress edits (comment composer, subtask input) while keeping the modal open; a second Esc with nothing pending closes the modal.

## Verification

- L0 fully green on merged master: `cargo fmt --check` ✓, `cargo build` ✓, `cargo clippy --all-targets` (0 warnings) ✓, `cargo test` (85 passed, 0 failed) ✓, `node --check static/app.js` ✓
- All 3 fixes independently verified by separate verifiers (not the builders) before release.
- Green-box-local `/api/v1/version` reports new SHA ✓
- Public `https://chipflow.chip.network/api/v1/version` (tunnel proxy) reports new SHA ✓
- `chipflow.service`: active, sole affected unit (dry-activate confirmed); restarted cleanly, migration ran inside `Db::connect` with no errors.

## Git history (master)

```
6fec570 KF-299: Two-stage Escape discards edits before closing modal
6f5b99a KF-296: Use Pomodoros plural for 2/3/>3 pomodoro-count colors
b50d0f5 KF-291: Three-letter weekday names and quick buttons in date picker
e981ece KF-282: Add right-click context menu to timer area (Round-9)
...
a52ed4f (GitHub) Round-9 (previous production)
```

All three cherry-picked cleanly from isolated worktrees (KF-291 + KF-299 both touch `static/app.js`; git auto-merged with no conflicts; both fix markers confirmed present in the merged tree). Pushed to GitHub as a single squashed commit via `.dev/push_via_api.py` (established pattern).

## Tracker

All 3 entries marked RELEASED in TrueNAS chipflow-tracker.git (commit 64fa37e).

## Deployment

- Pushed to GitHub via `.dev/push_via_api.py`: remote main `c559e90cd75cc06d1b395e71d2bb74c311f47f26`
- Updated pin in `/home/senkwich/homelab-dale-chipflow/modules/chipflow.nix`: rev `c559e90cd75cc06d1b395e71d2bb74c311f47f26`, hash `sha256-glzLXGCDeKkCQNkjELzeef4s+1fGdd596jL83Cw/jhs=` (nar hash validated by reproducing the known Round-9 hash with the same method)
- `nixos-rebuild dry-activate --flake .#green-box`: confirmed only `chipflow.service` affected
- `nixos-rebuild switch --flake .#green-box`: completed
- Production live at https://chipflow.chip.network
