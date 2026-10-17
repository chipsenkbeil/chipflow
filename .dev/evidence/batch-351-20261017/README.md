# batch-35.1 evidence (2026-10-17)

Production 657ec5e (code tree equals integ/batch-35.1 6468308): the cross-site
write guard KF-420 (cookie writes must be same-origin; every GET and HEAD is
read-only), KF-421 to KF-423, stream G (dark mode themes only the board with
KanbanFlow's values; settings fixes), and the timer goldens KF-424 to KF-430.

- `verify-351-report.md`: blind check of the integrated release candidate,
  including a full UI sweep in light and dark.
- `verify-x-report.md`, `verify-x2-report.md`: the security hotfix.
- `verify-g-report.md`, `verify-g2-report.md`: stream G.
- `verify-e4-report.md`, `verify-e5-report.md`: timer goldens.

The production database was copied before the deploy to
`/var/lib/chipflow-backups/chipflow-20261017T022056Z-before-657ec5e.redb` on
green-box. The previous binary shut down cleanly (no repair on start), and the
new one opened the file at schema version 1.
