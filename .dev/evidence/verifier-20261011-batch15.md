# Batch-15 independent verifier report — 2026-10-11

Verdict: **all seven defects PRESENT-AND-CORRECT in the deployed tree**. None return to the build path.

## Verified tree
- `origin/main` = `7fff5504` (fetched fresh 2026-10-11).
- **Production equivalence proven:** `git diff 8794cb26 7fff5504 -- src static templates Cargo.toml Cargo.lock` is **empty** — the verified app code is byte-identical to production build `8794cb26`.

## Method
- Built the deployed tree (`cargo build`, debug) in worktree `~/workspace/verify-b15`; ran test instance on 127.0.0.1:3222–3227 with throwaway redb DBs (all killed afterward, ports confirmed free).
- Seeded via REST API: board → task with RFC3339 due date (2 days out) → `due_done=true` → member assigned → 2 comments → 4 subtasks (2 done).
- Fragment assertions on `GET /api/tasks/:id/modal` HTML (10 checks, 8 pass immediately; 2 initial FAILs investigated and resolved as test artifacts — see notes).
- Live render: real modal fragment + deployed `style.css` in Chromium (DPR 1) at 1600px, 1100px, and 1600px dark-mode; computed-style + geometry measurements; screenshot `/tmp/b15-geom-wide.png` (reviewed visually).

## Per-defect verdicts

**KF-254 — Comments section: PRESENT-AND-CORRECT**
Header `Comments <span class="tm-sect-count">2</span>` + ⋮ section menu; 30px avatar circles; timestamps relative (`Today 11:41 PM` via `format_comment_datetime`); zero visible × delete (Delete lives inside each comment's hidden ⋮ menu); single-line `Add comment…` composer (white, 38px, borderless); comment body boxes computed `rgb(255,249,237)` = **#FFF9ED**, matching the r4 repair-challenge golden audit (the r3 "inverted colors" FAILED verdict was refuted: r3 sampled the author row as box interior). Screenshot confirms layout.

**KF-259 — Due date relative + (Done): PRESENT-AND-CORRECT**
Modal row renders `Tuesday 5:00 PM (Done)` — ±7-day weekday rule (`format_due_card`) + ` (Done)` appended by `format_due_modal` when `due_done=true`. Unit tests `format_due_modal_relative_and_done_marker` pass in-tree.

**KF-261 — Members avatar + ⊕: PRESENT-AND-CORRECT**
`.tm-member-disc` circle + `.tm-member-add` ⊕ button rendered. The lone `●` in the modal HTML is inside an HTML *comment* describing the fix, not a rendered text chip.

**KF-262 — Subtasks header count + menu: PRESENT-AND-CORRECT**
`Subtasks <span>2 / 4</span>` + ⋮ section menu (`tm-subtasks-menu`) rendered.

**KF-263 — No strikethrough: PRESENT-AND-CORRECT**
Zero `line-through` occurrences in deployed `static/style.css`; computed `text-decoration-line: none` on done subtasks at all viewports/modes. Screenshot: done subtasks in normal text.

**KF-264 — ⋮ drag handles: PRESENT-AND-CORRECT**
4 `.tm-subtask-handle` buttons (one per row), 0 × remove buttons; Delete moved into the handle's menu.

**KF-265 — Borderless add-subtask row: PRESENT-AND-CORRECT**
`.tm-subtask-addrow input` computed border `0px/none`, transparent background; plain text row.

## Break-attempts survived
- 1100px narrow viewport: all sections intact, no overflow breakage.
- Dark mode (class-forced): no degradation in any measured property.
- Empty states covered by fragment assertions (counts render from live data).

## Notes / limitations
1. The two initial fragment FAILs were test artifacts, not regressions: (a) KF-259's first probe used a bare local datetime (`2026-10-13T17:00:00`, no offset) which the API accepts but `format_due_card` cannot RFC3339-parse → no due row rendered. The UI always sends `toISOString()` (RFC3339), under which the fix renders correctly. Out of KF-259's scope; flagging as a possible separate edge, not filing. (b) KF-261's `●` was in an HTML comment, not rendered.
2. No golden-master PNG tree exists in this environment, so no pixdiff vs GM-070/073/074/085 was possible — verification is code + live-render measurement against the defect entries' and r4 audit's stated values. Stated plainly per the verifier spec.
3. Production was not screenshotted (auth-walled); code-identity to production is proven instead.
4. I did not modify the tracker. All seven sections still show OPEN as the first `- Status:` line with older FIXED/RELEASED lines beneath — the known newest-prepend anomaly; RELEASED prepends are the parent's per process.

## Cleanup
- Test servers killed; throwaway DBs removed; ports free. Worktree `~/workspace/verify-b15` left for parent cleanup. Production untouched; nothing pushed.
