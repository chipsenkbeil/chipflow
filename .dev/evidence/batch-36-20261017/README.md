# batch-36 evidence (2026-10-17)

Production 9548374 (code tree equals integ/batch-36 2c1773b): the agentctl API
with SCHEMA_VERSION 2 (KF-408 all-or-nothing writes), stream D due dates with
the due dialog fixes, stream H board behaviours with the help dialog, the page
fixes KF-473/447/416, the timer UI (stream T) and the 34 px top bar with the
timer in the board header row (stream V).

- `verify-36a-report.md`: blind check of the integrated candidate, API, schema
  and security half.
- `verify-36b-report.md`: blind check of the integrated candidate, UI half.
- `verify-i3-report.md`, `verify-i4-report.md`: agentctl API.
- `verify-d4-report.md`, `verify-d5-report.md`: stream D.
- `verify-h-report.md`, `verify-h2-report.md`: stream H.
- `verify-u-report.md`, `verify-t-report.md`, `verify-v-report.md`: streams U, T, V.

The production database was copied before the deploy to
`/var/lib/chipflow-backups/chipflow-20261017T163325Z-before-9548374.redb` on
green-box. On start the new binary logged "upgraded database ... from schema
version 1 to 2". A rollback to any earlier binary needs that copy, because
earlier binaries refuse a version-2 file.
