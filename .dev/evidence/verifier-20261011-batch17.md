# Batch-17 independent verifier report — 2026-10-11

**Verdict: all six defects PRESENT-AND-CORRECT in the deployed tree. None return to the build path.**

## Verified tree

- `origin/main` = `52bf7626` (fetched fresh 2026-10-11).
- Production equivalence: `git diff 8794cb26 52bf7626 -- src static templates Cargo.toml Cargo.lock` is **empty** — deployed app code is byte-identical to production `8794cb26`.
- Test instance built from the deployed tree (worktree `.dev/worktrees/dep-verify17`, since removed); served `static/app.js` sha256 matches the deployed tree byte-for-byte.

## Method

- Deployed-tree code inspection (`git show origin/main:...`) for each fix marker.
- Live test instance on 127.0.0.1:3226 (throwaway DB; admin created via /setup; board+task seeded via REST API).
- Live modal fragment (`GET /api/tasks/:id/modal`) assertions + Playwright Chromium render at 1919×998 with computed-style and geometry measurements, visually reviewed screenshots.
- Break attempts: dark mode (`body.dark`), 1100×800 narrow viewport, 1400×1200 tall viewport.

## Verdicts

### KF-256 — Modal subline trailing icons: PRESENT-AND-CORRECT
- Deployed `templates/modal.html` contains `tm-subline-icons` (1×); deployed `static/style.css` has the rule (1×).
- Live modal HTML: 1 `tm-subline-icons` span, 3 `tm-subline-ico` SVGs.
- Screenshot: "Work To-do · Created: Today" followed by the subtask-list, comment-bubble, and eye icons.

### KF-260 — Neutral label chips: PRESENT-AND-CORRECT
- Deployed `.tm-label-chip`: `background: #fff5e1; border: 1px solid #bfb7a8; color: #3d3d3c` — neutral, no blue tint.
- Live: added "backend" label via the labels dialog; 2 `.tm-label-chip` elements; computed `background-color: rgb(255, 245, 225)`, `color: rgb(61, 61, 60)`.
- Screenshot: chip renders as a cream/beige chip with dark text.

### KF-266 — Reports stacked-layers icon: PRESENT-AND-CORRECT
- Deployed `templates/modal.html:235`: 22×20 SVG with diamond polygon + two chevrons, stroke-width 2; no play triangle.
- Live modal HTML: `width="22" height="20"` SVG present; `polygon points="11,0.5` present.
- Screenshot: Reports toolbar button shows the stacked-layers glyph.

### KF-267 — Delete icon gray, not red: PRESENT-AND-CORRECT
- Deployed `templates/modal.html`: zero `tm-danger` occurrences.
- Live: delete button class is exactly `tm-icon-btn`; computed color `rgb(0, 0, 0)`.
- All six toolbar icons (Add, Move, Timer, Reports, More, Delete) compute to the identical `rgb(0, 0, 0)` — the delete icon matches its siblings; the defect's intent (not red, same as the other icons) is satisfied.

### KF-268 — Labels dialog Save button: PRESENT-AND-CORRECT (defect's intent satisfied)
- The KF-328 rebuild removed the Save button entirely: deployed `templates/board.html:1019–1027` shows the KanbanFlow GM-080 dropdown pattern (title + × close, "Add labels..." input, suggestions list) with a code comment citing GM-080; zero Save buttons in the dialog (only the ×).
- Live: opened the labels dialog; it contains exactly 1 button (the ×); screenshot confirms no Save button, blue or otherwise.
- Judgment: the defect complained the Save button was blue instead of green. KanbanFlow's dialog has no Save button (changes apply immediately); ChipFlow now matches KanbanFlow. The ordering complaint cannot recur. PRESENT-AND-CORRECT.

### KF-319 — Fixed 25px modal top offset: PRESENT-AND-CORRECT
- Deployed `static/style.css`: `.modal-overlay { padding: 25px 1rem; ... }` — fixed, not `4vh`.
- Live measurements of the modal's top edge via `getBoundingClientRect`:
  - 1919×998: **25.0px**
  - 1100×800: **25.0px**
  - 1400×1200: **25.0px** (a 4vh rule would give 48px here — discriminates fixed vs viewport-proportional)

## Break attempts survived

- Dark mode: label-chip background unchanged (`rgb(255, 245, 225)`); delete icon color unchanged (`rgb(0, 0, 0)`).
- 1100×800 narrow viewport: KF-319 top offset still 25px.
- 1400×1200 tall viewport: KF-319 top offset still 25px.

## Notes

- Golden-master PNGs are absent from this environment (same as batches 14/15/16) — no pixdiff possible; verification is code-identity + live DOM/render measurement.
- The tracker's six sections still show OPEN first with older FIXED/VERIFIED/RELEASED lines beneath; RELEASED prepends are left to the orchestrator per process.
- Production untouched; nothing pushed. Test server killed, port confirmed free, throwaway DB and scratch removed, verification worktree removed.
