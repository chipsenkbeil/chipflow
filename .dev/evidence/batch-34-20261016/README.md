# Release batch-34 (2026-10-16): timer stream

Production SHA `d230d76ec7390da2004c86114a10d47626eaa4fd`, whose tree is identical to `d79de3d`
(branch fix/timer-r3), the tree verify-e3 passed. 18 defects: KF-001 KF-004 KF-006 KF-010 KF-015
KF-067 KF-103 KF-278 KF-279 KF-289 KF-325 KF-366 KF-375 KF-378 KF-380 KF-390 KF-412 KF-413.

- `verify-e-report.md`: first pass on fix/timer (10 passed, 6 failed, including a KF-006 regression).
- `verify-e2-report.md`: second pass (5 of 8 passed; KF-001, KF-103 and KF-279 had residual gaps
  against the 10-17 golden set kept outside the repo at ~/Pictures/chipflow-goldens/).
- `verify-e3-report.md`: the integrated release candidate. KF-001, KF-103 and KF-279 passed, the
  18-defect sweep passed, and the batch-32/33 integration checks passed. No stored struct or table
  changes.

Deploy check: the database was copied first to
`/var/lib/chipflow-backups/chipflow-20261016T224616Z-before-d230d76.redb` with matching sha256.
green-box `127.0.0.1:3000` and https://chipflow.chip.network both report `build_sha` d230d76, and the
sha256 of the served `static/app.js` (d0089a75…) and `static/style.css` (13db8ef4…) match
`git show d230d76:`.

Date note: the batch-33 evidence directory is named `batch-33-20261017` and some status lines filed
today say 2026-10-17; the real date was 2026-10-16 (a clock error in the release session).
