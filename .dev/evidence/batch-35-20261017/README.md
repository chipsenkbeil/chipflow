# batch-35 evidence (2026-10-17)

Production 9476359 (tree equals integ/batch-35 22e84eb): schema-version guard
(SCHEMA_VERSION 1, refuse newer files, repair crash-left files in place),
graceful shutdown with a 10 s drain (KF-415), due-date fixes KF-398, KF-409,
KF-410, KF-414, and host-zone-independent due tests (KF-431).

- `verify-35-report.md`: blind check of the integrated release candidate.
- `verify-s3-report.md`: schema guard and graceful shutdown.
- `verify-d1-report.md`: KF-409 and KF-410.
- `verify-d2-report.md`: KF-398.
- `verify-d3-report.md`: KF-414.

The production database was copied before the deploy to
`/var/lib/chipflow-backups/chipflow-20261017T002529Z-before-9476359.redb` on
green-box. On first start the new binary logged that redb repaired the file
(the old binary never closed it cleanly) and stamped it with schema version 1.
