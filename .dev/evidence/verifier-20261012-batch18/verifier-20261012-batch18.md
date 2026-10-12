# Independent verifier report — ChipFlow batch-18 (board-surface visual cluster)

**Date:** 2026-10-12T00:30Z. **Verifier:** independent (did not build batch-18).
**Builder claim:** all eight defects already fixed in the 6bb6cd1 tree, zero commits.
**Verdict: all eight PRESENT-AND-CORRECT in the deployed tree — none return to the build path.**

## Production equivalence

- `git fetch origin main` → `origin/main` = `7b9db095ddf1311538a65dbda93ef94f5d7b8c76`.
- `git diff 8794cb26 7b9db095 -- src static templates Cargo.toml Cargo.lock` → **empty**.
  The deployed app code is byte-identical to production `8794cb26`.
- Served `static/app.js` sha256 from the test instance matches the deployed tree
  byte-for-byte (`67fe55e4…f7669a` both).

## Method

Built the deployed tree in an isolated worktree (`.dev/worktrees/dep-verify-18`,
since removed), ran a test instance on `127.0.0.1:3226` with a throwaway DB
(admin via `/setup`, board + columns + fully-attributed task seeded via REST API:
description, due `2026-10-20T17:00:00Z`, 2 subtasks, 1 comment, 150m time,
assignee `verifier`). Live board-page DOM assertions + Playwright Chromium
render at 1600×900 with computed-style and geometry measurements, plus visually
reviewed screenshots (`board-light.png`, `dropdown-open.png`). Test server
killed by PID afterward, port confirmed free, throwaway DB removed. Production
untouched, nothing pushed, tracker not modified.

One environment event: the VM was replaced mid-verification (uptime reset,
`/tmp` wiped, test server died). The worktree and build under
`.dev/worktrees/` survived on durable storage; the server was restarted from
the surviving binary and the board re-seeded. All measurements below were
re-taken after the replacement against the same deployed tree.

## Verdicts

- **KF-233 — PRESENT-AND-CORRECT.** Deployed `.topbar`: `background: #333333`,
  `height: 36px`. Deployed `.boards-btn`: `background: #515151`, 28px tall at
  y=5 with 2px emboss bands top/bottom (visually fills the 36px bar).
  Live: topbar bg `rgb(51,51,51)`, h=36; button bg `rgb(81,81,81)`, h=28, y=5,
  label "Boards". Screenshot: button fills the bar vertically.
- **KF-234 — PRESENT-AND-CORRECT.** Deployed `.user-avatar`: 1.5rem circle,
  `background: #ffffff`, `color: #98540f`. Live: bg `rgb(255,255,255)`, color
  `rgb(152,84,15)`, 24×24, border-radius 50%, initials "VE". Screenshot:
  white disc with brown initials in the dark top bar.
- **KF-238 — PRESENT-AND-CORRECT.** Deployed `templates/board.html:116`:
  `#board-menu-btn .menu-bars` SVG with three solid 2px `#494949` rects
  (y=0/5.5/11). Live DOM confirms the same markup. Screenshot: thick solid
  bars + "Menu" label in the gray board row.
- **KF-239 — PRESENT-AND-CORRECT.** Deployed `templates/board.html:93-97`:
  `button#timer-pill` sits inside `header.topbar` (KF-239 comment at the move
  site). Deployed `.topbar .timer-pill`: `background: #484848`, white text,
  32px. Live: `inTopbar: true`, bg `rgb(72,72,72)`, color white, rect
  (1495,2,89,32) — top-right of the 36px dark bar. Screenshot: dark chip
  "▶ 25:00 ▾" in the dark top bar.
- **KF-240 — PRESENT-AND-CORRECT** (defect's intent satisfied by supersession).
  The old drawer is gone: zero `#boards-sidebar`/`.boards-sidebar` markup in
  the deployed tree. The KF-330 light dropdown is present
  (`#boards-dropdown`, hidden until toggled) with search, board list,
  "View archived boards", blue "Create board" button — matching the KF-330
  L3-verified GM-021 design. Screenshot of the open dropdown confirms.
  NOTE (not a failure): the current-board marker is a blue 3px left border
  (`#2f7de1`, `.bs-current`), introduced after the KF-330 L3 verification in
  commit `99a7fac`. Whether KanbanFlow's actual boards dropdown uses a blue
  indicator is an unverified residual fidelity question — no GM-021 PNG exists
  in this environment to pixdiff against.
- **KF-242 — PRESENT-AND-CORRECT.** Deployed `templates/task_card.html` renders
  icons in GM-069 order: description, duedates, subtasks, comments, time chip.
  Live card with all five attributes: `data-sec` = description, duedates,
  subtasks, comments + `card-ico-time` present. Screenshot: all five icons
  visible in order.
- **KF-243 — PRESENT-AND-CORRECT.** Deployed fix (commit `04334495` + ring-color
  repair) present: conditional avatar placement (`has_icon_content` → icon
  row, else title row) and `.card-avatar` 27px white disc, `color: #ac5d00`,
  `color-mix(...75%...)` theme border. Live: avatar bg `rgb(255,255,255)`,
  color `rgb(172,93,0)`, 27×27, inside `.card-iconrow` at the icon row's
  height (avatarY == rowY), right end — matching the pixel-settled
  interpretation of GM-069 (verified + spotchecked 2026-10-03; the tracker's
  "top-right of the title row" prose was found imprecise by golden pixel
  analysis).
- **KF-244 — PRESENT-AND-CORRECT.** Deployed `.card-dueline { color: #222222 }`.
  Live: `rgb(34,34,34)`, text "Due: 20 October 5:00 PM". Screenshot: dark
  due line. Overdue-red rule preserved in the CSS (untested live).

## Break-attempts survived

- **Dark mode** (`body.dark`): avatar stays white, card-avatar initials stay
  `#ac5d00`, dueline stays `#222222`; pill restyles to its dark-mode rule
  (`#1f2937`) without breaking.
- **Narrow viewport (1100×800):** topbar 36px, Boards button 28px, pill still
  visible (89px wide) in the dark bar.

## Notes

1. Golden-master PNGs are absent from this environment (same as batches
   14/15/16/17) — no pixdiff possible; verification is code-identity + live
   DOM/render measurement + visual screenshot review.
2. The eight tracker sections still show OPEN first with older
   FIXED/VERIFIED/RELEASED lines beneath; RELEASED prepends are left to the
   orchestrator per process.
3. Environment: `/tmp` is a 512MB tmpfs — build worktrees under
   `.dev/worktrees/`, never `/tmp`. `cargo` is at `~/.cargo/bin/cargo`.

## Screenshots

- `board-light.png` — full board render (top bar, Menu button, card with
  5 icons, avatar, due line)
- `dropdown-open.png` — open boards dropdown (light, search, board list with
  blue current indicator, archived link, Create button)
