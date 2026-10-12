# Independent verifier report — batch-16 (timer cluster), 2026-10-11

**Verifier:** independent (not the builder). Builder claimed all six already-fixed, zero commits.
**Verified tree:** `origin/main` = `190d7479` (fetched fresh at verification time).
**Production equivalence:** `git diff 8794cb266f2d8386f67013426f6fc78f02106eca origin/main -- src static templates Cargo.toml Cargo.lock` is **empty** — the verified app code is byte-identical to production `8794cb26`.

**Method:** Deployed-tree worktree (`.dev/worktrees/dep-verify`, since removed), full `cargo build` green, test instance on 127.0.0.1:3223 with throwaway DB (killed afterward). Served `static/app.js` sha256 `67fe55e4…` matches `git show origin/main:static/app.js` byte-for-byte. Live checks via Playwright (Chromium) against the real pages + node eval of the deployed formatter. Break attempts: dark mode, 1100px narrow viewport. Golden-master PNGs are absent from this environment, so no pixdiff was possible; verification is code-identity + live DOM/render measurement. Production was untouched, nothing pushed, tracker not modified.

## Verdicts

### KF-231 — PRESENT-AND-CORRECT
`TimerUI.fmt` at deployed `static/app.js:2851` always zero-pads minutes: `var mm = (m < 10 ? '0' : '') + m;`. Independent node eval of the deployed formatter: `fmt(3)`→`"00:03"`, `fmt(122)`→`"02:02"`, `fmt(1499)`→`"24:59"`, `fmt(3903)`→`"1:05:03"`, `fmt(0)`→`"00:00"`, `fmt(59)`→`"00:59"`, `fmt(600)`→`"10:00"` — 7/7 pass. Live browser: `TimerUI.fmt(3)` = `"00:03"`. The defect's exact case (`"0:03"` vs KanbanFlow's `"00:02"`) is resolved.

### KF-232 — PRESENT-AND-CORRECT
Deployed `closePopup` (`static/app.js:3279`) unhides the pill, then calls `this.renderPill()` (KF-232 comment at the exact site named in the defect's root-cause analysis), then `this.syncTick()`. `renderPill` on idle renders green play glyph `&#9654;` + `fmt(25*60)` = `"25:00"`. **Exact defect repro, live:** started a pomodoro (phase=pomodoro), opened the pill popup (pill hidden per KF-097), clicked the real Stop button `#tp-stop` (phase→idle), closed the popup by clicking outside. Pill after close: visible, `timer-pill-icon timer-pill-play`, label `"25:00"`, tab title `"General — ChipFlow"` (normal). The frozen `"24:59"` + red stop icon + stale tab title does NOT occur. Break attempt: dark mode + 1100px viewport, idle pill still green play + `"25:00"`.

### KF-276 — PRESENT-AND-CORRECT
Deployed `templates/board.html:1065–1078` `#shortcuts-dialog` has exactly 9 rows: V, T, P, `.`, ↑↓, Cmd+↑↓, Esc, Cmd+Enter, Delete. No "Add time manually (Y)" / "Add time estimate (E)" rows anywhere in the dialog markup (`git show origin/main:static/app.js` has no such dialog text; the only "Add time estimate" hit is an unrelated code comment). Live board page DOM: 9 rows, zero extra rows. Matches KanbanFlow's GM-093 list.

### KF-283 — PRESENT-AND-CORRECT
Deployed `src/models.rs:449` seeds `long_break_minutes: 10`. Live: fresh DB, authenticated `GET /api/settings` → `"long_break_minutes": 10`.

### KF-284 — PRESENT-AND-CORRECT
Deployed `templates/board.html:375`: `<button … class="btn btn-sm btn-success" id="ts-add-activity" …>`. Live board page: same markup; computed `background-color` = `rgb(22, 163, 74)` (green) in both light and dark mode. Break attempt (dark mode): still green.

### KF-289 — PRESENT-AND-CORRECT (via KF-294 restructure)
Deployed `templates/timer_log.html:18–20` timer-log export menu, in order: **Excel (Detailed), Excel (Summary), PDF (Summary)** — Excel first, no CSV item. Live timer/log page DOM: same order. The defect's ordering complaint (CSV before Excel) cannot recur. Note: resolved by the KF-294 menu restructure to KanbanFlow's verbatim GM-170 items, not by the literal CSV/Excel swap the defect's fix direction suggested. The verbatim wording now matches KF-294's released set; any future GM-119 wording re-check is a separate fidelity question, out of KF-289's scope.

## Summary
All six are **PRESENT-AND-CORRECT in the deployed tree**. None return to the build path. Tracker RELEASED prepends are the orchestrator's per process.
