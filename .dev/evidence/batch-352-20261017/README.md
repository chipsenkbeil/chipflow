# batch-35.2 evidence (2026-10-17)

Production a980a7c (code tree equals integ/batch-35.2 ee0b86f): stream F board
visuals (19 defects), KF-457 responsive board (with KF-442), KF-484 optimised
argon2 in debug builds only, and KF-446 (board Menu switch colours, via KF-367).

- `verify-352-report.md`: blind check of the integrated release candidate.
- `verify-f-report.md`, `verify-f2-report.md`: stream F.
- `verify-r-report.md`: KF-457.

The production database was copied before the deploy to
`/var/lib/chipflow-backups/chipflow-20261017T135306Z-before-a980a7c.redb` on
green-box; the new binary opened it at schema version 1.
