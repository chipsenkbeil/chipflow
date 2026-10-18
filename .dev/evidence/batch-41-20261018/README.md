# Batch-41 release evidence

- Release commit: 3c57d24 on main (code tree identical outside .dev/ to integ/batch-41 e1ce0bc; diff ebd8ae6..e1ce0bc applied onto main 7379052).
- Contents: KF-562 (completed_at follows the current column's done flag on move and on toggling a column's done flag; builder-j1 7c2b757), KF-560 (graceful_shutdown tests serialize child spawns; macOS pipe()+set_cloexec gap leaked a child's stdout pipe into sibling spawns, so stdout EOF waited for another server; builder-i2, evidence in its lsof runs: 5 of 40 unfixed runs leaked, 0 of 40 fixed), and a comment-only style-review commit e1ce0bc.
- Style review (style-reviewer): 4 comment issues fixed; 3 optional smells left for the owner.
- Blind verification: verify-41 PASS (KF-562 live repro baseline vs candidate incl. renamed/runtime done columns, toggles, atomic failing PATCHes; KF-560 30 consecutive loaded runs 0 failures, builder's 50 loaded runs 0 failures; 298 dump files identical; GET writes nothing; cross-site writes 403; parity harness 29/29; gates).
- Gates on the release commit: fmt clean, release build 0 warnings, clippy 0 warnings, TZ=UTC tests all pass (17 suites), node --check ok; attribution check clean.
- Final parity harness on the release binary (harness from main): 29/29 entries, 292/292 comparisons.
- Deploy: homelab 253c12e pins 3c57d24 (nar sha256-TWU9Obnpi4hm1ypkponco6cUQ3+PNn63UWjpn8aY080=); dry run changed only chipflow.service. Database copied first to green-box /var/lib/chipflow-backups/chipflow-20261018T021947Z-before-3c57d24.redb (sha256 d6a07116d6d4f2ca10903b65d0ac9d1787ef558bc34503dee8019f106f366eb9, equal to the source).
- Production check: build_sha 3c57d24eac6f905c02bb3462722f6f343d4097fe; served app.js sha256 ff0173553ec7413ef79ece7081edb54533d5f6c6d5c8b578c3a9add190c8c28b equals the release tree; cross-site POST 403; database at schema version 3; public URL 200.
- Known: toggling a done column off and on re-stamps completed_at with the toggle time; stale stamps already in the database stay until the task moves or its column is toggled.
