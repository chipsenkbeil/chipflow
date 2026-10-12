# Batch-27 independent verifier report — 2026-10-11

**Verified commits:** `5816005` (KF-351) and `fa3b6c8` (KF-352), worktree
`.dev/worktrees/batch-27`. Built the tree, ran a live test instance
(127.0.0.1:3247, throwaway redb — killed by PID after verification, port
free, DB removed). Production untouched, nothing pushed, tracker not modified.

## KF-351 — Calendar iCal feed: PRESENT-BUT-WRONG (2 small spec bugs)

Core feature works end-to-end and the security design holds up (see below),
but two objective RFC 5545 violations were reproduced live; both are one-line
fixes. **Return to the build path** for these two items only:

1. **`ical_fold` emits 76-octet continuation lines** (RFC 5545 §3.1: lines
   MUST NOT exceed 75 octets excluding the line break). The fold loop budgets
   75 bytes per segment but prefixes continuation lines with a space, so a
   full continuation segment is 1 + 75 = 76 octets. Reproduced live: a 200-char
   task name produced a 76-octet line (verified byte-exact with python).
   Fix: continuation segments must be ≤74 bytes
   (`let budget = if first { 75 } else { 74 };`).
2. **`STATUS:COMPLETED` on VEVENT for done tasks** (RFC 5545 §3.8.1.11:
   VEVENT status is TENTATIVE / CONFIRMED / CANCELLED; COMPLETED is VTODO-only).
   Reproduced live: marking the seeded task done produced
   `STATUS:COMPLETED` inside its VEVENT. Fix: emit `STATUS:CONFIRMED` for all
   VEVENTs (or omit STATUS on done tasks).

The builder should add regression tests for both (a >149-char SUMMARY for the
fold boundary; a done task for the STATUS value).

### What verified clean (do not regress)
- **config_json bag never serialized to clients** — confirmed by grep over
  every serialization path: export-data serializes board as
  `ExportBoardRef{id,name}`; `list_boards` → `BoardListItem{id,name,archived}`;
  `PUT /api/boards/:id/config` returns only
  `BoardConfigView{legend_visible,archived}` and its input accepts only those
  two keys (token cannot be overwritten or read back through it); no template
  renders the *board* config bag (`board.html` touches only *column* config);
  `create_board`/`save_as_template` return id-only results. The raw token
  leaves the server only via the authenticated link endpoint.
- **Route auth layering** — verified in `router()`: the link/reset routes are
  registered *before* `.route_layer(auth_middleware)` (authenticated, 401
  without session confirmed live); `/api/calendar-feed/:token` is registered
  *after* the layer (public by design, like `/api/v1/version`).
- **Token mechanics** — 43-char base64url from 32 CSPRNG bytes; unauthenticated
  link endpoint → 401; authenticated → 200 `{"path":"/api/calendar-feed/<tok>.ics"}`;
  unknown board → 404; unknown token → 404; `.ics`-less URL also 200;
  `strip_suffix(".ics")` unambiguous (base64url has no `.`).
- **Rotation** — `POST .../reset` → new token; old URL → 404 immediately; new
  URL → 200. `set_board_config` *merges*, so reset cannot wipe
  legend_visible/archived.
- **Feed payload minimal** — one VEVENT per due task: SUMMARY (RFC-escaped),
  DTSTART/DTEND (timed → UTC +1h; midnight → all-day DATE with exclusive DTEND),
  DESCRIPTION "Column: X", STATUS, RRULE for recognized cadences, stable
  `UID:<task-id>@chipflow`. No comments, history, members, or user data.
  No-due tasks excluded. Headers: `text/calendar; charset=utf-8`,
  `Cache-Control: private, max-age=3600`, `X-Robots-Tag: noindex`.
- **Attack surface review** — no SQL injection (`get_board_by_feed_token` is a
  redb linear scan with `==`, no query string built from the token); no path
  traversal (axum path segment; token alphabet has no `/`); 256-bit space is
  not enumerable and 404-vs-200 is the inherent bearer-URL oracle; no
  constant-time comparison but HTTP timing on a 43-char random token is not
  exploitable. Reset is POST + `SameSite=Lax` session cookie with no CSRF
  token — consistent with every other state-changing endpoint in this app
  (Lax blocks cross-site POST); not a new hole. Raw (unhashed) token storage
  is ACCEPTED: the bag is provably never exposed (above), and the link must be
  persistently re-viewable, which hashing would break.
- **KanbanFlow parity** — calendar iCal subscription exists per the defect;
  the per-board secret-URL pattern is the standard approach and matches the
  fix direction. No public KanbanFlow feed-format docs found; the emitted
  format is standards-based.
- Client: token fetched only on panel open, never in page source; bearer
  warning shown; light + dark CSS present and scoped.
- Break-attempts survived: empty board (minimal valid VCALENDAR), 404s,
  dark mode, 1100px, special chars (`;`, `,`, `\` escaped correctly),
  200-char name (folded — modulo bug 1 above).

### L0 re-run
`cargo fmt --check` PASS · `cargo build` PASS ·
`cargo clippy --all-targets` 0 warnings / 0 errors ·
`cargo test` **94 passed / 0 failed** · `node --check static/app.js` PASS.

## KF-352 — Calendar due_repeat expansion: PRESENT-AND-CORRECT

Extracted `parseRepeatCadence`/`addMonthsClamped`/`stepRepeat`/`repeatInstances`
verbatim from the shipped `static/app.js` into node; all cases pass:
- weekly Oct 15 → Oct 22, Oct 29 (in-window only)
- daily → 30 instances in October (original day excluded)
- monthly Jan 31 → Feb 28 (clamped), Mar 28, Apr 28
- yearly across a 3-year window
- past-due weekly from Sep 1 → first in-window instance Oct 6 (pre-range skipped)
- unknown text / null → no expansion (single date, by design)
- 400-iteration cap present in code
- instances render through the existing `addChip`, so per-day sublines
  ("On tomorrow → Column") and the 4 filters apply to recurrences

Live: calendar page 200 with all 5 feed-panel element IDs; 404 on unknown
board; `reportAction` `case 'calendar'` present; dark-mode CSS scoped;
empty-board and narrow-viewport sane.

## Release note
Cherry-pick `5816005` + `fa3b6c8` onto current `origin/main` (batch-19–26
pattern); never merge local master `6bb6cd1`. **Do not release until the two
KF-351 spec fixes above are applied and re-verified** (fold boundary +
VEVENT STATUS). The fixes are server-side only (`src/routes.rs`); no client
changes needed.

DONE: KF-351 PRESENT-BUT-WRONG (2 spec bugs, specifics above),
KF-352 PRESENT-AND-CORRECT
