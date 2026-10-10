# Batch-14 independent verification — 2026-10-10

**Verifier:** independent subagent (NOT the builder). Blind to the builder's reasoning.
**Deployed tree verified:** `origin/main` = `eb3159f02ed382d8524d94e722cef65deb089f6a`
(fetched 2026-10-10; worktree `/tmp/verify-batch14`, since removed).
**Production equivalence:** `git diff 8794cb26 eb3159f0 -- src static templates Cargo.toml Cargo.lock`
is EMPTY — the verified tree's app code is byte-identical to production `8794cb26`.
**Method:** release binary built from the deployed tree, fresh throwaway redb DB,
test instance on :3210 (seeded via REST API), Playwright/Chromium screenshots +
`getBoundingClientRect`/`getComputedStyle` measurements, DPR 1.
**Result: 26/26 checks passed.** Raw JSON: `verifier-20261010-batch14/verify-batch14-results.json`.

## KF-241 — Folded-column WIP-violation styling — PRESENT-AND-CORRECT
- Violated strip count renders `rgb(139, 0, 0)` (darkred, `var(--board-headerWarningTextColor)`)
  — measured live; dark-mode break-attempt keeps darkred.
- `#ff8080` warning line present under the count badge, `rgb(255, 128, 128)`,
  21.0px wide vs 22.0px strip (full-width via negative margins).
- Strip body contains only the count badge + vertical "TO-DO" name; no spurious red "5".
- Break-attempts survived: dark mode; (strip geometry is viewport-independent).
- Evidence: `verifier-20261010-batch14/kf241-strip.png`.

## KF-250 — Task modal fits viewport — PRESENT-AND-CORRECT
- `.modal-overlay` padding fixed `25px 1rem`, `overflow: hidden` (backdrop never scrolls).
- `.task-modal` `max-height: calc(100vh - 4rem)`, flex column, `overflow: hidden`;
  `.task-modal-body` `overflow-y: auto; min-height: 0`; header `flex-shrink: 0`;
  `body.modal-open { overflow: hidden; }`.
- Measured at 1919x998 with a content-heavy task (description + 4 subtasks + 2 comments):
  modal top=25 (GM-070: y≈26), bottom=959 ≤ 998; body scrolls internally
  (scrollHeight 950 > clientHeight 859).
- Break-attempts survived: 1366x768 (top=25, bottom=729 ≤ 768); content overflow
  exercises the internal scroll path rather than clipping.
- Evidence: `verifier-20261010-batch14/kf250-modal-1919.png`.

## KF-252 — Modal toolbar outside modal edge — PRESENT-AND-CORRECT
- `.task-modal-iconbar` is a sibling AFTER `.task-modal` inside `.task-modal-wrap`
  (positioning context); `position: absolute; left: 100%; margin-left: 11px; top: 26px`.
- Measured: barLeft=1266.0 vs modalRight+11=1266.0; 6 actions; circles exactly
  44.0x44.0px; labels right of circles (labelLeft=1321.0 > circleRight=1310.0).
- **Circle-color judgment call:** the defect prose says "light-gray circles" but the
  code implements WHITE (`background: #fff`, measured `rgb(255, 255, 255)` live).
  WHITE is correct: the code documents GM-076 connected-component pixel scans with
  a >238 brightness threshold locating the 44px-diameter circle fill — a light-gray
  fill (~200-224) would not pass a >238 threshold. The auditor's "light-gray" was a
  loose visual description (white circles + soft shadow against the dimmed board
  read as light gray). Screenshot confirms clean white circles, no border ring,
  matching the documented master scan.
- Break-attempts survived: 1366px viewport (iconbar outside modal AND on-screen,
  barRight=1112 ≤ 1366).
- Evidence: `verifier-20261010-batch14/kf252-modal.png`.

## KF-286 — Timer log as modal — PRESENT-AND-CORRECT
- `#timer-log-overlay > .timer-log-modal[role=dialog][aria-label="Timer log"]`:
  white (`rgb(255,255,255)`), centered, 1080px wide, over dimmed board (z-index 300).
- Title "Timer log" + header × (`#log-close-x`); Period stepper
  (`#log-period-prev/next`, label) — stepping changes the label
  ('This month' → 'Last month'); Board "All boards" dropdown; Entry-type "All"
  dropdown; icon row print/export/reload/settings/close ×.
- Seeded entries render as day groups (2 `.log-entry` rows); `/timer/log` page
  retitled standalone "Time spent" (`<h1>Time spent</h1>`).
- Break-attempts survived: 1100px viewport (modal 1068 ≤ 1100); stepper functional;
  close × dismisses.
- Evidence: `verifier-20261010-batch14/kf286-timelog.png`.

## Limitations
- No golden-master PNG tree was available in this environment, so no pixdiff
  vs GM-040/070/076/117/129 was possible; verification is code + live-render
  measurement against the defect entries' stated values.
- Production itself was not screenshotted (auth-walled); production app code is
  proven byte-identical to the verified tree (empty diff above), and production
  reports healthy at `8794cb26`.
