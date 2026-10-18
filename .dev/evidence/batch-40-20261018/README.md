# Batch-40 release evidence

- Release commit: ebd8ae6 on main (code tree identical outside .dev/ to fix/h5-subtask-due c9389f3; applied as the diff 7dc7b79..c9389f3 onto main d0081e5).
- Contents: integ/batch-40 (builders H1 board, H2 timer/modal, H3b settings and subtask due dates) + KF-557 + KF-558 + KF-561 + KF-545 follow-ups. KF-539..558 (KF-559 closed by decision), KF-561, KF-545.
- Schema: SCHEMA_VERSION 3 (v3 = v2 + Subtask.due_at, mutation-proven). Production upgraded 2 -> 3 on start.
- Blind verification:
  - verify-40a PASS: 18 UI/parity defects, full parity harness 29/29 twice (deterministic), baseline 24/29 failing, regression sweep, gates.
  - verify-40b: schema, real-data upgrade audit on a private copy of the production database (291 endpoints; only the 38 expected JSON lines differ; the v2 binary refuses the v3 file and leaves it byte-identical; copy deleted afterwards), data safety (232 cross-site writes 403, GET/HEAD writes nothing), gates PASS; one KF-545 UI fail (overdue colour) fixed in builder-h5.
  - verify-40c PASS: KF-561, KF-558, KF-545 follow-ups 1-5, harness 29/29, schema unchanged, 55/58 dumps identical plus 3 deliberate, regression, gates.
- Gates on the release commit: fmt clean, release build 0 warnings, clippy 0 warnings, TZ=UTC tests all pass (17 suites), node --check ok. Attribution check on pushed commits: clean.
- Deploy: homelab 243af12 pins ebd8ae6 (nar sha256-6QPTveWevtLhhXpuHfqxVcWR+9DS9VkpGuzXakAFTqI=); dry run changed only chipflow.service. Database copied before the switch to green-box /var/lib/chipflow-backups/chipflow-20261018T014238Z-before-ebd8ae6.redb (sha256 da787cc22d5ae0fd1a2934e1d4244fcb38f76864e7de62b8837afc8100bdf8b5, equal to the source).
- Production check: build_sha ebd8ae6351d900e4fc2fffacafb9d09632a1dc7e; served static/app.js sha256 ff0173553ec7413ef79ece7081edb54533d5f6c6d5c8b578c3a9add190c8c28b equals the release tree; cross-site POST 403; OpenAPI SubtaskDetail.required includes due_at; log "upgraded database ... from schema version 2 to 3"; public URL 200.
- Owner check: KF-543 real (native) column drag by the header was not exercised headless; drag one column by hand.
