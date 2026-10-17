# Batch-39 release evidence

- Release commit: 7f7a63f on main (code tree identical to integ/batch-39 7dc7b79 outside .dev/).
- Contents: integ/batch-39 = 8ec4d30 + fix/final-ui a4d18b2 + fix/final-server 8858d22 (KF-517, KF-527..538).
- Blind verification: verify-39 PASS for all 13 defects, data safety (1569 GET endpoint dumps identical between 8ec4d30 and the candidate, comparator proven against planted changes), GET/HEAD no-write sweep, cross-site write 403s, light and dark regression sweep (0 console errors), gates.
- Gates on the release commit: fmt clean, release build 0 warnings, clippy 0 warnings, TZ=UTC tests all pass, node --check ok.
- Deploy: homelab 2197eab pins 7f7a63f; dry run changed only chipflow.service. Database copied before the switch to green-box /var/lib/chipflow-backups/chipflow-20261017T234921Z-before-7f7a63f.redb (sha256 bb16bca3a43e8cded946166c526b838ab97f8ef06b7c3c36f3fcb195167e8bef, equal to the source).
- Production check: /api/v1/version build_sha 7f7a63f25d52a97f807d6a786f3ce380efacffcd; served static/app.js sha256 b6940591c7dc7157f7dcb6d716876f25539cdfbf445e5ef7f1782d8fd5cf1c19 equals the release tree; cross-site POST 403; database opened at schema version 2.
- Outage: the service was stopped for the database copy at 18:49:22 CDT and the switch was held by a permission check until the owner approved; it started at 18:57:17 CDT (about 8 minutes down).
- Known: KF-528's change ships here but KF-528 is INVALID; KF-557 (batch-40) restores the task Add > Due date popover.
