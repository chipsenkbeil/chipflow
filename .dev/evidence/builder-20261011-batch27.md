# Batch-27 builder evidence — KF-351 (iCal feed) + KF-352 (due_repeat expansion)

Date: 2026-10-11. Worktree: `.dev/worktrees/batch-27` (base `6bb6cd1` + all 8
report release commits cherry-picked cleanly). Production untouched, nothing
pushed, tracker not modified.

## KF-351 — Calendar iCal feed subscription — commit `5816005`

KanbanFlow parity: the calendar supports subscribing to an external iCal
feed. No public KanbanFlow documentation of the exact feed format was found;
the design mirrors the standard per-board secret-URL pattern.

### Security design (threat model)

The feed URL is a bearer credential for the board's due dates — anyone with
the URL can read them. Design decisions:

- **Token**: 32 CSPRNG bytes (`rand::thread_rng`), base64url-encoded
  (43 chars) → 256-bit unguessable space. Generated server-side only.
- **Storage**: the board's opaque `config_json` bag (`calendar_feed_token`
  key). Verified the bag is NEVER serialized to clients: export-data exposes
  only board id/name; the board config view exposes only
  legend_visible/archived. A live check confirmed the raw token is absent
  from the export-data JSON.
- **Reveal**: only via authenticated `GET /api/boards/:id/calendar-feed`,
  called on explicit user action (opening the iCal panel on the calendar
  page). Never embedded in page source or other API responses.
- **Bearer endpoint**: `GET /api/calendar-feed/:token[.ics]` is
  UNAUTHENTICATED by design (calendar clients can't do cookie auth;
  registered after the auth middleware layer, like `/api/v1/version`).
  Unknown token → 404.
- **Rotation**: `POST /api/boards/:id/calendar-feed/reset` (authenticated)
  issues a new token; the old URL stops working immediately. Verified live:
  old token → 404 after reset, new token → 200.
- **Minimal payload**: one VEVENT per task with a parseable due date —
  SUMMARY (name), DTSTART/DTEND, DESCRIPTION ("Column: X"),
  STATUS (COMPLETED/CONFIRMED), UID (`<task-id>@chipflow`, stable across
  polls so clients update rather than duplicate), RRULE for recognized
  repeats. NO comments, history, members, user data, or internal ids.
  A unit test asserts none of those strings appear in the feed.
- **Headers**: `Content-Type: text/calendar; charset=utf-8`,
  `Cache-Control: private, max-age=3600` (feed pollers), `X-Robots-Tag:
  noindex`.
- No rate limiting (same as the rest of this single-admin app).

### What was built

- `src/db.rs`: `get_board_by_feed_token` (linear scan; boards are few, feed
  clients poll infrequently).
- `src/models.rs`: `BoardRow::feed_token()` reading the config bag.
- `src/routes.rs`: `generate_feed_token`, `parse_repeat_cadence` /
  `RepeatCadence` (shared with KF-352; RRULE:FREQ=DAILY/WEEKLY/MONTHLY/YEARLY),
  `ical_escape` (RFC 5545 §3.3.11), `ical_fold` (75-octet CRLF folding),
  `build_ical_feed` (timed dues → UTC date-time + 1h; midnight dues →
  all-day DATE with exclusive DTEND), handlers `get_calendar_feed_link`,
  `reset_calendar_feed`, `serve_calendar_feed`; routes registered
  (authenticated pair before the auth layer, feed after it); OpenAPI
  `#[utoipa::path]` annotations + paths list.
- `templates/calendar.html`: iCal feed row (toggle → panel with Subscribe
  [webcal:], Copy link, Reset link, bearer-warning note, status line).
- `static/app.js` CalendarPage: `initFeed` — fetches the link only when the
  panel opens, builds absolute https/webcal URLs from
  `window.location.origin` (server never needs its public host), clipboard
  copy with fallback, reset with confirm.
- `static/style.css`: `.calendar-feed-row` light + `body.dark` variants.
- `agents.md`, `agents/skill.md`: documented the three endpoints.

### Local confirmation (test instance 127.0.0.1:3247, throwaway redb — killed)

- `GET /api/boards/:id/calendar-feed` (auth) → `{"path":"/api/calendar-feed/<43-char-token>.ics"}`; 401 without auth; 404 unknown board.
- `GET` the feed path → 200 `text/calendar`, valid ICS (`file` identifies
  it as "iCalendar calendar file"; all lines ≤ 75 octets, CRLF, verified in
  Python). Timed task → `DTSTART:20261015T143000Z`/`DTEND:+1h`;
  midnight task → `DTSTART;VALUE=DATE`/`DTEND` next day; task without due
  date excluded; `;`/`,`/`\` in names escaped; `RRULE:FREQ=WEEKLY` present.
- Reset → new token works (with and without `.ics` suffix), old → 404.
- Export-data JSON contains no token (threat-model check).
- Calendar page HTML contains all 5 feed element ids; `CalendarPage.init`
  wired.

## KF-352 — Calendar expands `due_repeat` — commit `fa3b6c8`

KanbanFlow parity: repeating due dates appear on the calendar for their
recurrence instances.

### What was built (`static/app.js`, CalendarPage only)

- `parseRepeatCadence`: "every day"/"daily" → daily, "every week"/"weekly"
  → weekly, "every month"/"monthly" → monthly, "every year"/"yearly"/
  "annually" → yearly (case-insensitive, trimmed). Mirrors the server's
  `parse_repeat_cadence`. Unrecognized text → no expansion (pre-KF-352
  single-date behavior).
- `repeatInstances(dueDay, repeatText, start, end)`: steps from the due day
  (exclusive — the literal due date is rendered by the existing path),
  emitting instances within the visible grid range; capped at 400 iterations.
  Month stepping clamps to month end (Jan 31 → Feb 28).
- `render()` buckets each instance via the existing `addChip`, so chips land
  on the right days with correct "On today → Column" sublines and filters
  still apply.

### Local confirmation

- Shipped functions extracted verbatim into node and exercised: weekly due
  Oct 15 → instances Oct 22 + Oct 29 in the Oct grid; daily → 17 instances;
  monthly/yearly behave; unknown/null repeat → []; past dues skip
  pre-range instances; Jan-31 monthly clamps to Feb 28.
- The iCal feed (KF-351) covers the same cadences via RRULE for external
  clients.

## L0 (final tree, both commits)

- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — **94 passed, 0 failed** (6 new: feed token uniqueness,
  repeat cadence parsing, ICS escaping, ICS folding, feed minimality/
  stability, db token round-trip)
- `node --check static/app.js` — PASS

## For the verifier / release

- Commits `5816005` (KF-351) + `fa3b6c8` (KF-352); cherry-pick onto current
  `origin/main`; never merge local master `6bb6cd1`.
- The release worker must copy this file to the main repo's `.dev/evidence/`.
- Note: the KF-351 commit includes `parse_repeat_cadence`/`RepeatCadence`
  in Rust (needed for the feed's RRULE); the KF-352 commit is the
  client-side calendar expansion only.
- Known limitation: unrecognized `due_repeat` free text renders as a
  single due date (documented, by design).
