# KF-331 verification captures, 2026-10-16

Selected captures behind the four verification passes. The reports sit one
level up and cite their files by the scratch paths they were written with;
only the files below were kept.

| Pass | Report | Verdict | Worker |
|---|---|---|---|
| 1 | `../kf331-verify-pass1-20261016.md` | FAIL (header 1 px short, nav pitch and inset, Arial-metric type) | laptop session, not the builder of `77a6537` |
| 2 | `../kf331-verify-pass2-20261016.md` | FAIL (chevron 8x14 against 9x16, nav row 8 1 px high) | blind subagent |
| 3 | `../kf331-verify-pass3-20261016.md` | FAIL (back-link label 1 px right) | second blind subagent |
| 4 | `../kf331-verify-pass4-20261016.md` | PASS | the pass-3 subagent, re-run in full |

The fixes between passes are recorded in `../kf331-fix-build-20261016.md`.
The laptop session built them, so passes 2 to 4 were run by other workers.

## Files

- `pass1/`: release-candidate captures at 1905x1000, 1920x1000, 1440x900
  and 1100x900 (`cand-*`), baseline `origin/main` captures (`main-*`), the
  board page with ChipFlow's top bar (`cand-board-1920x1000.png`), and the
  "General" board capture used for the H1 width check.
- `pass2/`: the chevron and nav-row zooms behind the pass-2 FAIL.
- `pass3/`: the back-link overlay and the green-channel label profile
  behind the pass-3 FAIL.
- `pass4/`: the released tree at 1905x1000, 1905x2400 (bottom shadow
  edge), 1440x900 and 1100x900, the General tab, and `/settings` plus the
  delete page for fix and baseline. Both of those pairs compare at
  ImageMagick AE 0.

The reference for every pass is
`../kf331-kanbanflow-settings-colors-recovered-20261016.png`.
