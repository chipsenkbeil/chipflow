## verify-x2 report: KF-420 narrow re-verify (fix/csrf-origin 41c18dd vs 7969f65)

**Verdict: KF-420 passes.** The fix branch makes 0 database writes across 1,096 cookie-authenticated cross-site GET and HEAD requests. The baseline made 7 writes on the same fixture. All six claims hold and all gates pass. I found three low-severity candidate defects (listed at the end); none blocks the fix.

### Environment
- **Fix tree:** worktree `.dev/worktrees/verify-x2`, detached at 41c18dd. The diff from 7969f65 touches only `agent-guide.md`, `agents/skill.md`, `src/db.rs`, `src/routes.rs` and `static/app.js` (+2 lines), as expected.
- **Baseline tree:** `.dev/worktrees/verify-x2-base`, at 7969f65.
- **Release binaries:**
  - fix md5 9c1b760ed46b0dca8c71d29ff96b7b58
  - baseline md5 af9f8886d399eb0747b25e75de37c7fd
  - The fix-only strings "POST, because the first request creates the token" and "path null when none exists" each match once in the fix binary and 0 times in the baseline.
- **Ports:** fix 3392, baseline 3393. Every server ran with `--database-path`, using DBs named `/tmp/chipflow-verify-x2-*.redb`.
- **Fixture:** built on the baseline binary, snapshot at `seed-final.redb`:
  - the golden-master board (8 colours, tasks, time entries, a subtask, a comment, an attachment, a calendar-feed token)
  - a board from the "Kanban basics" template, a board copied through save-as-template, an archived board, the starter "General" board
  - an API token
  - an orphaned timer: started on a task, then the task was deleted on the baseline without any status read in between
  - a colourless board "Legacy colourless", made with a small helper crate built against the baseline library, whose `create_board` makes a bare row
- **Browser checks:** Playwright 1.63.0, 1905x1000 at DPR 1, a fresh context for each scenario.

### Gates (fix tree, `--config 'build.build-dir="target/build"'`)
- `fmt --check`: exit 0.
- `clippy --all-targets`: exit 0, 0 warnings.
- `TZ=UTC /Users/senkwich/.cargo/bin/cargo test`: exit 0. All test binaries pass (lib 12, bin 77 with 1 ignored, plus the integration tests). The ignored test is an existing deliberate one (the DST case driven by another test). The new tests `calendar_feed_get_should_never_write`, `timer_should_stop_with_its_task_and_status_reads_should_never_write` and `colorless_boards_should_get_colors_at_startup_not_on_read` pass.
- `jsc checkSyntax` on `static/app.js`: exit 0.
- The release build prints one warning, an unused `ServeDir` import that only appears in release builds.

### Claims

| # | Claim | Baseline 7969f65 | Fix 41c18dd | Verdict |
|---|---|---|---|---|
| 1 | No GET or HEAD writes | 7 of 548 wrote | 0 of 548 (orphan-timer state), 0 of 548 (live-timer state) | PASS |
| 2 | Calendar feed | GET creates a token | GET read-only; POST creates; reset is POST; cross-site 403s | PASS |
| 3 | Timer | delete leaves an orphan; status GET clears it | delete clears it in the same request; status never writes | PASS |
| 4 | Colours | backfilled on read | backfilled at startup; reads write nothing | PASS |
| 5 | Guard | (guard already present) | holds | PASS |
| 6 | Data safety | — | export identical apart from the backfill | PASS |

**1. GET/HEAD write sweep.**
- I built the route list from the router myself: 89 paths, checked by regex that every one is covered. That gave 274 URLs, covering:
  - every board's pages and APIs (seven boards, including the unknown, archived and colourless ones)
  - every task API for a live task, the deleted task and an unknown task
  - public routes and static files
  - GET and HEAD sent to every POST-, PUT-, PATCH- and DELETE-only path
- Each URL went as GET and as HEAD, with the session cookie, `Sec-Fetch-Site: cross-site` and `Origin: https://evil.chip.network`. I took the redb md5 before and after each request.
- The baseline's 7 writers:
  - `GET /api/timer/status` (clears the orphan)
  - `GET /api/boards/{id}/calendar-feed` on 5 boards without a token
  - `GET /b/{colourless board}` (colour backfill)
- An earlier HEAD-only run (`sweepA-*-headonly.tsv`) shows HEAD triggers the same 7 writes on the baseline and 0 on the fix.
- HTTP status codes are identical between fix and baseline for every request.
- **Bearer, noted separately:** 8 of 10 Bearer GET/HEAD requests to authenticated routes write, and this is only the `last_used_at` stamp. For example, `/api/boards/KB/calendar-feed` stays `{"path":null}` afterwards. The public `/agents.md` does not write.

**2. Calendar feed.**
- GET on a board with no token returns `{"path":null}` and the md5 is unchanged.
- POST is refused with 403 and writes nothing for each of: cross-site, same-site (`other.chip.network`), a foreign Origin only (form content type), and `Origin: null`.
- A same-origin POST creates the token. A second POST returns the identical path without writing, and GET then returns that path.
- The `.ics` returns 200 `text/calendar`.
- A cross-site reset gets 403. A same-origin reset issues a new token: the old `.ics` returns 404 and the new one 200.
- A Bearer POST with a cross-site header returns 200.
- **UI:** the iCal feed toggle sends POST (200), the `webcal://` link resolves to a 200 `.ics`, and the same link comes back after a reload. There were 0 console errors and 0 failed requests. The baseline UI sends GET.
- **Docs:** `agent-guide.md`, `agents/skill.md` and the served `/agents.md` describe GET as read-only (null until created) and POST as create. OpenAPI has both operations, and the schema's `path` is `["string","null"]`.

**3. Timer.**
- **Orphan from the old DB:** on the fix, status reads idle, the md5 is unchanged, and the stored row is still there afterwards. On the baseline, the status read deletes it.
- **Deleting a task with a running timer:** the fix stores None in the same DELETE request. The baseline leaves the orphan.
- **Deleting the copied board while its task's timer runs:** the fix stores None; the baseline leaves the orphan.
- **Negative cases on the fix:** deleting a different task keeps the timer, deleting another board keeps the timer, and deleting a board under a task-less stopwatch keeps it.
- **UI on the fix:**
  - Starting a Pomodoro from the modal works, and the pill shows 24:58 after a reload.
  - Stop gives idle.
  - Starting on a new task and deleting that task from the same modal works: status goes idle, the pill shows "▶ 25:00", the stored row is None and the card is gone after a reload.
  - 0 console errors.
  - One `net::ERR_ABORTED` on that DELETE. The DELETE itself returned 200; the browser reports the abort because the modal reloads the page on close. The baseline shows the same thing.

**4. Colours.**
- The fix's first startup on the baseline-written DB changes the md5. The colourless board then has the full 10-colour palette, identical to the baseline's lazy backfill apart from row ids. Reads after that do not change the md5.
- Palettes from every creation path are identical between fix and baseline, each with 0 duplicate values and exactly 1 default:

| Creation path | Colours |
|---|---|
| Plain new board | 10 |
| "Kanban basics" template | 10 |
| "Pomodoro board" template | 10, the template's own labels ("1 Pomodoro"…) |
| Copied golden-master template | 8, equal to the source palette |
| copy-from onto a new board | 8 |

- All existing boards' colours, columns and export-data are byte-identical after the fix starts.

**5. Guard.** All with the cookie, and the md5 shows nothing written on any 403:

| Request | Cross-site | Same-site | Foreign Origin only | Same-origin |
|---|---|---|---|---|
| POST /settings (form) | 403 | 403 | 403 | works |
| POST /api/tasks (form) | 403 | 403 | 403 | works |
| PUT /api/settings | 403 | 403 | 403 | works |
| POST /api/timer/start | 403 | 403 | 403 | works |
| POST /api/timer/stop | 403 | 403 | 403 | works |
| POST /logout | 403 | 403 | 403 | works |

- Bearer with cross-site headers: task create 201, settings PUT 200 (persisted).
- A bogus Bearer token plus a valid cookie, cross-site: 401, no write.
- There are no device-approval routes on this branch, so that case doesn't apply.

**6. Data safety.**
- I diffed a baseline export against a fix export of the same seed DB: boards, columns, export-data, labels, colours, templates, settings, timer log, tokens, members and tasks.
- They are identical except for:
  - the colourless board's palette, where only the ids differ
  - the calendar-feed results, where the baseline's own GETs had just created tokens (the claim-1 bug) and the fix returns null
- The archived board is identical.
- Every startup changes the md5, on the baseline too (two baseline restarts both did), so this predates the fix.

### Break-attempts I don't think the builder tried
1. The colourless board was made by the real baseline data layer, not the private test helper.
2. A real orphan left by the baseline binary, then stop and start against it.
3. HEAD as well as GET, and GET/HEAD sent to every non-GET route.
4. Same-site sibling-subdomain, `Origin: null`, and a bogus Bearer token riding a valid cookie.
5. Concurrent first POSTs to the calendar feed.
6. Deleting a task or board unrelated to the running timer.
7. A UI delete from the modal while the timer runs.

### Candidate defects (observations, not KF-420 failures)
- **[LOW] Stale OpenAPI summary.** `src/routes.rs:6199`, `GET /api/boards/{id}/colors`, still says "Backfills the standard palette on boards that predate colors". It no longer does. The GET calendar-feed summary also says "with the POST below", which reads oddly in the spec. Evidence: `openapi-fix.json`.
- **[LOW] Stop on an orphan misreports.** With an orphan from an older DB, `POST /api/timer/stop` on the fix returns 200 `{"discarded":false,"minutes":6}`, but nothing is logged because the task is gone. The baseline returned 404 there, since its status read had already cleared the orphan. Evidence: `orphan.sh` output.
- **[LOW, existing] Racing first creations break the feed link.** 12 concurrent first creations returned 8 different tokens on the fix, and only the last one stored works; the others 404. The baseline's GET has the same race (4 distinct tokens from 12). Two tabs opening the iCal panel at once could hand out a dead link.

### Evidence
Folder: `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-x2/`
- Sweep: `routes.txt`, `router-paths.txt`, `sweepA-fix.tsv`, `sweepA-base.tsv`, `sweepB-fix.tsv`, `sweepB-base.tsv`, `sweepA-*-headonly.tsv`, `sweepBearer-fix.tsv`
- Feed: `feed-checks.txt`, `openapi-fix.json`
- Timer: `timerdel-fix.txt`, `timerdel-base.txt`
- Colours and data: `colors-fix.txt`, `colors-base.txt`, `export-base/`, `export-fix/`
- Guard: `guard-fix.txt`, `guard-base.txt`
- UI: `ui/ui-fix.json` plus screenshots, `ui-base/ui-base.json`
- Gates: `gates.log`, `clippy.log`, `test.log`
- Fixture and helper: `seed-final.redb`, `ids.env`, `mkcolorless/`

All my servers were killed by PID; nothing is listening on 3392 or 3393. The chipflow processes on ports 3375 and 3379 belong to other agents and were left alone.

DONE: KF-420 PASS (GET/HEAD write sweep: fix 0/548 with the orphan timer and 0/548 with a live timer; baseline 7/548: timer status, calendar-feed GET on 5 boards, colourless board page; Bearer GETs write only the last-used stamp)
DONE: FEED PASS (GET returns {"path":null} and writes nothing; same-origin POST creates, repeat POST is idempotent; reset is POST; cross-site, same-site, foreign-Origin and null-Origin POSTs get 403 and write nothing; UI toggle POST 200, .ics 200, persists after reload, 0 console errors; docs, served /agents.md and OpenAPI updated)
DONE: TIMER PASS (status never writes, with a live timer or an orphan; the orphan reads idle; deleting the task or its board clears the timer in the same request, the baseline leaves an orphan; unrelated deletes keep it; UI start, stop, delete-from-modal and reload work with 0 console errors)
DONE: COLOURS PASS (colourless board backfilled at fix startup, reads write nothing; new boards get 10 colours with no duplicates; templates keep their own palette (8 for the copied board, 10 Pomodoro labels); copy-from gives 8; existing palettes byte-identical)
DONE: GUARD PASS (cross-site, same-site and foreign-Origin form/JSON writes on 6 cookie routes all 403 with no write; same-origin works; Bearer unaffected; bogus Bearer plus cookie gets 401)
DONE: DATA PASS (baseline-written DB under the fix exports identically apart from the colourless board's colour ids and the baseline's own GET-created feed tokens)
DONE: GATE PASS (fmt 0, clippy --all-targets 0 warnings, TZ=UTC cargo test all green, jsc checkSyntax 0)
