All three defects PASS. Evidence is under /private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-u/. Both servers are killed by PID.

**Environment**
- Candidate worktree .dev/worktrees/verify-u at 080fd41, port 3404. Baseline worktree .dev/worktrees/verify-u-base at 657ec5e, port 3405.
- Debug binaries differ: candidate md5 c7fc0602..., baseline md5 8c885fb5.... Debug serves static/ and templates from each worktree, so the fix is live in the candidate only.
- Databases: db-3404.redb and db-3405.redb in the folder above, seeded with the golden-master fixture. I created extra boards on both servers (100-char names, CalLong, CalQuiet, throwaway delete-flow boards).
- Playwright 1.63.0, Chromium, DPR 1, animations off. Delete page and calendar at 1905, 1100 and 640; settings at 1440x612. Scripts: t473.mjs, t447.mjs, t447b.mjs, t416.mjs, treg.mjs (with lib.mjs); logs 447.log and 416.log.
- The diff `657ec5e..fix/dialogs-u` touches only static/style.css and templates/board_settings.html (+12/-4 lines). No Rust, no stored data.

**KF-473 (delete-board page, 100-character name with no spaces) — PASS**

| Viewport | Baseline scrollWidth | Fix scrollWidth |
|---|---|---|
| 1905 | 1905 (the text spills past the 600px box but not the document) | 1905 |
| 1100 | 1225 (overflows) | 1100 |
| 640 | 995 (overflows) | 640 |

- The acceptance check holds: the fix gives scrollWidth equal to the viewport at all three widths.
- Short name ("Short Name"), baseline vs fix: 0 differing pixels at 1905, 1100 and 640.
- Long name at 1905: the differing region is x 642-1627, y 241-454. That starts below the title band at y 193, so the title is unchanged.
- The region covers the alert text, which now wraps inside the box, and the layout beneath it, which moves up.
- Cancel, Archive and Delete flows on 100-character-name boards, light and dark, with 0 console errors:
  - Cancel goes to `/b/ID/settings`.
  - Archive goes to `/boards/archived`, and the board is no longer in the board list.
  - Delete shows the confirm dialog, then the board is gone.
- Break-attempts: a 100-character name at 640, and every flow in dark mode.

**KF-447 (board settings footer) — PASS**
- Every tab was measured in light, and again with `chipflow-dark=1` set. `body.dark` stayed false, the page background stayed rgb(211,232,242) and the footer stayed rgb(51,51,51) on all seven tabs: General, Layout, Colors, Task settings, Advanced, API & Webhooks, Add from email.
- Baseline has 0 `footer.site-footer` elements on every tab; the fix has exactly 1.
- Fix at 1440x612:
  - The footer is 1440x30 at y 582-612 on all six short tabs, with scrollHeight 612. This matches golden settings/02 (y 582-612).
  - Colors tab (long page): box bottom y 904, footer top y 944, so the gap is exactly 40 px. Page scrollHeight is 974 (baseline 952). Scrolled to the bottom, the footer ends at y 612 (447-colors-bottom.png). Golden 14 shows about 41 px at half scale.
  - Pixel samples of the footer in my screenshot: rgb(51,51,51) at y 582 and y 611. The golden has KanbanFlow's copyright text there; ours reads "ChipFlow".
- The Colors tab's Edit dialog still opens, even though the dialog markup comes after the new footer.
- Break-attempts: dark-mode localStorage on every tab, and the long Colors tab scrolled to the bottom.

**KF-416 (calendar day columns) — PASS**

| Check | Baseline | Fix |
|---|---|---|
| CalLong board, 1440: column widths | 551, 487, 123, 546, 119, 37, 40 | all 130.14 |
| CalLong board, 1440: document scrollWidth | 2168 | 1440 |
| CalLong board, 1905: document scrollWidth | 2400 | 1905 |
| Grid right edge vs close button right edge | equal in the grid box, but the cells overflow it | equal: 1176 at 1440, 1408.5 at 1905 |
| Long chips | 478-542 px wide, nothing truncated | 121 px wide, truncated with an ellipsis |

- The fix also holds at 1100 (all columns 130) and 640 (all columns 84), with no horizontal scroll.
- Short chips are not truncated.
- CalQuiet board (short chips only): baseline vs fix is 0 differing pixels at 1440 and 1905, in light and in dark.
- Break-attempts: a 78-character name with no spaces, 44 `W` characters, a long name with spaces, and the narrow widths.

**DONE: REGRESSION** — 0 console errors and 0 failed requests across all 7 settings tabs (light and dark), the delete page (Cancel, Archive and Delete in light and dark), the calendar (1440, 1905, 1100 and 640, light and dark), and the Colors Edit dialog.

**DONE: GATE**
- `cargo fmt --check`: exit 0.
- `cargo clippy --all-targets`: 0 warnings.
- `TZ=UTC /Users/senkwich/.cargo/bin/cargo test`: exit 0. Suites passed: 23, 95 (1 ignored), 3, 13, 3, 8, 7 and 4 tests, 0 failed.
- jsc `checkSyntax` on static/app.js: exit 0.
- The candidate built cleanly with `--config 'build.build-dir="target/build"'`.

**Observations (not blocking)**
1. The settings box on the short tabs is 416 px tall (Layout 443.6) against golden settings/02's 441 px, so the box ends about 25 px above where the golden's does. Acceptance: on the General tab at 1440x612, the box height equals 441.
2. After deleting a board, the browser goes to `/boards/new`. The fix did not change this, and I did not compare it with the baseline or a golden. Acceptance: confirm KanbanFlow's post-delete destination, then assert the redirect URL equals it.

DONE: KF-473 PASS scrollWidth equals viewport at 1905, 1100, 640 (baseline 1905/1225/995); short name 0 pixels different from baseline
DONE: KF-447 PASS footer is 30 px, #333333, y 582-612 on all 7 tabs (baseline has no footer); Colors tab gap is 40 px; dark mode stays off
DONE: KF-416 PASS day columns equal (130.14 at 1440 and 1905), scrollWidth equals viewport (baseline 2168/2400), grid right edge equals header right edge, chips truncate; CalQuiet 0 pixels different from baseline
