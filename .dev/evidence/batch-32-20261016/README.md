# Release batch-32 (2026-10-16)

Production SHA `45b9cda249f141f0dfb007142170b6cc910a985e`: KF-304, KF-334 (with KF-141) and KF-359.
The release tree is identical to the tree verify-a2 tested (tree `9b62d46`).

- `verify-a-report.md`: first blind verification. KF-304 and KF-359 passed; KF-334 failed.
- `verify-a2-report.md`: re-verification of the KF-334 rework. It passed, and KF-304 and KF-359 passed again.
- `kf334-move-submenu.png`: the card menu with the Move submenu open beside it.

Deploy check: green-box `127.0.0.1:3000` and https://chipflow.chip.network both report `build_sha` 45b9cda. The sha256 of the served `static/app.js` (fb3d7f95…) and `static/style.css` (eb1c6030…) match `git show 45b9cda:`.
