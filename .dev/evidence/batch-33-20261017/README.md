# Release batch-33 (2026-10-17)

Production SHA `1c0c67a5c7fe141e63c68b5b3f97abddf46a43d6`: release commit `9be2b41` (KF-374, KF-396,
KF-400, KF-402, KF-391, KF-372, KF-387, KF-397, KF-399, KF-358, KF-223) plus a test-only commit,
`1c0c67a`, that skips the local-zone DST tests when no zone database is available. The homelab
Nix build runs `cargo test` in a sandbox without tzdata, where those tests failed and blocked the
first deploy attempt. The sandbox build passing (the dry run) is the check for that commit.

`9be2b41`'s tree is identical to the integration head `a1baa41` that verify-c3 (server/API, on
`bd37f6c`) and verify-b4 (board visual) passed. The commits between `bd37f6c` and `a1baa41`
change only `static/style.css`.

Reports, oldest first:

- `verify-c-report.md`: stream C, first pass. 7 passed; KF-400, KF-387 and KF-398 failed.
- `verify-c2-report.md`: KF-387 and KF-398 passed; KF-400 failed on seconds = 60.
- `verify-c3-report.md`: the combined batch-33 tree. KF-400, KF-387, the regressions and the
  stream B pixel identity passed; KF-398 failed on the year rule, so KF-398 is not released here.
- `verify-b-report.md`, `verify-b2-report.md`, `verify-b3-report.md`: stream B rounds 1 to 3
  (KF-358 and KF-223 failures fixed one by one).
- `verify-b4-report.md`: KF-358 and KF-223 passed on `a1baa41`.

Deploy check: the database was copied first to
`/var/lib/chipflow-backups/chipflow-20261016T201818Z-before-1c0c67a.redb` on green-box, with
matching sha256. green-box `127.0.0.1:3000` and https://chipflow.chip.network both report
`build_sha` 1c0c67a, and the sha256 of the served `static/app.js` (3f6bc143…) and
`static/style.css` (b985937e…) match `git show 1c0c67a:`.
