# batch-37 evidence (2026-10-17)

Production d2a466a (code tree equals integ/batch-37 1e1c1f8): server and API
fixes (streams S and X), board grid (W1), timer and dialogs (W2), tooling (Y)
and the quick-add Escape fix.

- `verify-37a-report.md`: blind check of the integrated candidate, server/data half.
- `verify-37b-report.md`: blind check of the integrated candidate, UI half.
- `verify-37c-report.md`: the quick-add Escape fix added after 37b.
- `verify-s4-report.md`, `verify-x3-report.md`, `verify-x4-report.md`,
  `verify-w1-report.md`, `verify-w2-report.md`: the streams on their own bases.

KF-499 and KF-481 stay open (partial). The production database was copied
before the deploy to
`/var/lib/chipflow-backups/chipflow-20261017T193902Z-before-d2a466a.redb`; on
start the new binary assigned the main swimlane of two boards (KF-406).
