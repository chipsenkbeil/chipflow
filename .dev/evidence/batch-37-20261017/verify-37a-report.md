verify-37a report: server/data half of batch-37 (candidate integ/batch-37 @313c7a0 vs baseline/production 9548374)

**Verdict: FAIL, but the release is safe.** Production data opens intact, nothing regresses, and every gate and the agentctl probe pass. The FAIL comes from two tracker acceptance lines the candidate does not meet (KF-499 and KF-458), plus three new candidate defects.

## Environment
- Worktrees: `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-37a` (313c7a0, clean) and `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-37a-base` (9548374, clean; my temporary test files are removed).
- Release binaries, frozen copies in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-37a/bin/`:
  - candidate md5 06bdf16929e013f3066221e0c2e302b8; the string "cannot archive the last active board" occurs 3 times.
  - baseline md5 56ac198bef5d31fdaa182604e02eee95; the string occurs 0 times.
- Ports 3386 (candidate) and 3387 (baseline). Every database is under `.../scratchpad/verify-37a/db/`. Playwright 1.63 at 1905x1000, DPR 1.
- Every PID I started was stopped by its exact PID. No listeners remain on 3386 or 3387.
- Evidence folder: `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-37a/` (`*.out`, `*.log`, `shots/`, `fp/`, `safety-*.json`).

## Gates (candidate)
- `fmt --check`: 0.
- `clippy --all-targets`: 0 warnings.
- jsc `checkSyntax`: 0. `cumulative_flow.js`: 0 failures.
- `cargo test`: 331 passed, 0 failed, 1 ignored (an existing test that another test runs) under TZ=UTC, America/Chicago, Pacific/Chatham, and `env -i` with no HOME and no TZ.
- Release build: 0 warnings. The baseline release build prints 1 (the unused ServeDir import).
- `tests/db_check_then_act.rs`: 15 of 15 runs pass (7 of 7 tests each run).
- nextest: 5 of 5 runs give 331 passed, 0 LEAK. The baseline gave 1 leaky test in 3 runs.
- Linux `rust:1.98.0` container (crates fetched, network off, `--offline`, no zoneinfo, no HOME):
  - uid 1000: 331 passed, 0 failed.
  - root: 331 passed, 0 failed.
  - Baseline as root: `failed_crash_repair_is_reported_as_such` FAILS (the repro).
- Fingerprint: the test passes on both trees. The v2 listing produced from each tree is identical (44 lines, `fp/listing.diff` is empty). SCHEMA_VERSION is still 2.

## Release safety (`safety-*.json`)
- **Upgrade:** I filled a database with the 9548374 binary: golden master board, 4 comments, 3 time entries, due dates with repeat and target, custom fields, metadata, 4 links, a claim, an archived board, a lanes board with a later lane named "Default", a calendar token, a saved template and a running pomodoro. I stopped it with SIGTERM. The candidate opens it at schema 2.
- **Full API dump diff (147 endpoints):** every old value is intact. The only additions are documented ones: `color_id` and `completed_at` on task detail, `board_id` on `/api/tasks` items, `session_id: null` on the timer, the hyphen in time ranges, and "1h 30m" durations.
  - The running pomodoro survives (same task and `started_at`).
  - Startup logs a main-swimlane line for each of the 4 boards.
- **GET/HEAD sweep on the very first start:** 264 requests (cookie, then cookie plus cross-site headers), 252 of them 200. md5 is unchanged.
- **Cross-site writes:** all 60 return 403, md5 unchanged.
- **Rollback:** the baseline reopens the candidate-written file and its dump is identical to the original.
- **Restart writes:** both binaries change the file's md5 on every start and stop (baseline control, a redb effect), so I judged a second start by its log instead: it logs nothing to normalise.

## Per-defect results
**Stream S**
- **KF-458:** concurrent HTTP calls are clean on the candidate, while the baseline fails every round:

  | 40 rounds over HTTP | Candidate | Baseline |
  |---|---|---|
  | Colour delete vs task colour update | 0 dangling | 40 dangling |
  | Create a task with a colour being deleted | 0 dangling | 40 dangling |

  But "each test fails on the old code" is not met: 5 of 7 tests fail against 9548374. `create_task_should_…` passes on the old code, because 9548374 already reads the cell inside its transaction. `copy_colors_should_…` passed 13 of 13 runs against the old code.
- **KF-459:** red maps to the target board's red; a missing value falls back to the target default; no colour stays none. The baseline keeps the source board's id.
- **KF-460:** in the browser, A gets 30m and B gets 45m. On the baseline, A's 30m landed on B. Break attempt: typing a name two tasks share gives "Several tasks are named…".
- **KF-467:** an orphan timer injected with the 9548374 library stops with `minutes:6` and a 6m task-less interrupted entry is logged. The baseline logs 0 entries.
- **KF-468:** 12 concurrent first POSTs return one path, the .ics returns 200, and GET returns the same path.
- **KF-469, KF-449:** the colours GET text is read-only and the "POST below" wording is gone. The OpenAPI text has 0 Rust-syntax hits (the baseline has 19).
- **KF-470:** 0 warnings against 1.
- **KF-471:** passes as root and as uid 1000 (see Gates).
- **KF-472:**
  - SIGTERM 50 ms after launch, 10 runs: exit 0 every time, no repair on reopen.
  - The baseline does not reproduce at 50 ms on this laptop, because it finishes starting sooner. It does at 10 ms (0 of 10 exit 0, 6 of 10 need repair) and at 20 ms (8 of 10 need repair). The candidate needs 0 repairs at both.
  - A second SIGINT during a stalled drain ends it in 0.03–0.06 s. The baseline takes 9.5 s.
- **KF-455:** 101 characters give 400 (with or without a template), 100 characters padded give 200, blank gives 400. The baseline accepts 101.
- **KF-407, KF-492:**
  - Archiving or deleting the last active board gives 400, the delete page carries the refusal, and the page's Archive button shows "Could not archive board: cannot archive the last active board".
  - Concurrent archive+archive and delete+archive over 20 rounds never leave zero active boards. The baseline left zero in 10 of 10 tries.
- **KF-419:** `completed_at` is set for a task created in Done, including with top placement and through a template. The baseline leaves it null.
- **KF-418:** "2026-1-5" becomes "2026-01-05", the padded-with-spaces date is rewritten, "garbage" and "" are cleared, and each change is logged. It runs after the schema guard: a v3-stamped file is refused with md5 unchanged. A second start changes nothing.
- **KF-448:** whitespace, "", null or tab clears the repeat; a repeat without a due is dropped on POST and PATCH. The baseline fails 8 of 8 of these checks.
- **KF-454, KF-466:** the documented time-entry GET keys and PUT semantics match the server. The grace is measured: 59 s of a 60 s pomodoro completes, 56 s does not, and a stopwatch echoes `completed:true`.

**Stream X**
- **KF-433:** the target clears when its column is deleted (and re-sending it gives 200) and on a cross-board move. The baseline gives 400.
- **KF-434:** the template keeps the target and due_time 08:30. The baseline loses the target and falls back to 17:00.
- **KF-435:** the history lines read "Set the due date target column to …" and "Cleared…".
- **KF-489:** the filter is exact, an empty value means no filter, and an unknown board gives 404 "Board not found".
- **KF-490:** one palette table and one override method remain.
- **KF-493:** see the KF-458 HTTP colour races.
- **KF-494:** fields present and documented in OpenAPI.
- **KF-495:** a task-less entry stays task-less; an entry with a task given no `task_id` gets 400.
- **KF-498:** a task-less break adds 1 minute to the Walk activity and appears in the log; work totals are unchanged and the board filter excludes it. The baseline adds nothing.
- **KF-502:** `:3386:80` gets 403. The baseline returns 200.
- **KF-499:** FAIL.
  - 40 rapid starts: remaining never exceeds total, 40 distinct session ids, and a 22 s stopwatch is kept. Stopping by `session_id` works.
  - The tracker check, `stop {started_at: A}` with B started in the same second, stops B (`{"discarded":true,"already_stopped":false}`).
  - The web UI still sends only `started_at` when it stops.

**Stream Y**
- **KF-480:** the seeded board shows the In Progress card's inline subtasks with "2 hidden (2 done)", matching golden board/02. The baseline seed shows 0.
- **KF-456:** 0 LEAK in 5 nextest runs.

**W1 server parts**
- **KF-479:** the last column, empty or not, is refused with 400 and the column stays. The layout view shows only the generic toast "Could not delete column." See defect 1 for a concurrency gap.
- **KF-406:** the main lane is stored and assigned at startup by first position: the renamed "Work" lane, not the later "Default". Renders do not write, and the template-made board's main lane is Work. The baseline labels Work as an ordinary lane.

## Integration points (all pass)
- The KF-448 rule in POST and PATCH routes.
- `apply_template` in one transaction, with no nested write: it succeeds and its 100-round reader test passes. No snapshot content can trigger a failure in the code, so I could not inject a partial board.
- KF-406 inside `create_swimlane`, `ensure_default_swimlane` and `apply_template`.
- `insert_task_checked`: unknown and foreign colours give 400, and the Done stamp is set.
- `move_task_logged`, in one cross-board move: the colour is remapped, the shared field is carried, the foreign field is dropped, the target is cleared and due_at is kept.
- Orphan and task-less paths in `log_timer_session`.
- OpenAPI "null" wording.

## agentctl and races
- The probe passes 242 of 242 on both binaries with identical labels. It covers detail and list filters, since/ready, metadata, fields, links and blocked, claims, fences, heartbeat, renewal, expiry, WIP, comments and device pairing.
- Claim race, 100 rounds with neighbour moves: exactly one 200 per round, fences strictly increase, 0 releases lost.
- 300 metadata PATCHes lose 0.

## Docs
- The served `/agents.md` equals `agent-guide.md`, and the served `skill.md` equals the repo file.
- The S, X and T statements agree, apart from the two items in defects 2 and 4.

## Candidate defects (each with an acceptance check)
1. **The KF-479 last-column guard races (MEDIUM).** Two concurrent DELETEs of a board's only two empty columns both return 200 and leave 0 columns, in 37–40 of 40 rounds (`race37-3386.out`). Acceptance: over 40 rounds of two concurrent `DELETE /api/columns/{id}` on a board's only two empty columns, the board never ends with 0 columns.
2. **Timer session ids restart at 1 after a server restart (LOW).** A stale `session_id` from before the restart stops a newer session, which contradicts the docs' "a stale id gets already_stopped:true". Acceptance: start session n, restart, stop it, start a new session; `POST /api/timer/stop {"session_id":n}` returns `already_stopped:true` and the new session keeps running.
3. **The KF-499 started_at path and the UI are unchanged (LOW).** Acceptance: start A, then B in the same second; a stale-tab UI stop or `stop {started_at:A}` returns `already_stopped:true` and B keeps running.
4. **agent-guide.md idle-stop sentence is wrong (LOW, docs).** It says "Without started_at, stopping an idle timer is a 404", but `stop {session_id}` on an idle timer returns 200 `already_stopped`. Acceptance: the sentence also names `session_id`.
5. **Legacy repeat-without-due rows survive (LOW).** Rows written by 9548374 or older that have `due_repeat` but no `due_at` keep it, and save-as-template copies it onto the new board. Acceptance: after upgrade, every task with `due_at` null returns `due_repeat` null, including template copies.
6. **The v2 schema doc list omits `main_swimlane_id` (LOW).** The KF-406 config-bag key is missing from the `SCHEMA_VERSION` doc list. Rollback is verified safe because the bag is opaque. Acceptance: `src/db.rs`'s v2 list names `main_swimlane_id`.
7. **KF-458 test coverage is incomplete (LOW).** The `copy_colors` race test never fails against the 9548374 code (13 of 13 pass). Acceptance: it fails at least once in 10 runs against 9548374's `db.rs`.
8. **Two informational items:**
   - A board whose main lane is deleted never gets a new one, so a lone lane keeps its label; the baseline renders this the same way. Acceptance: deleting the main lane makes the next lane by position the main one.
   - A SIGTERM within about 10 ms of launch, before the handler is installed, still exits 143 (3 of 10 runs), with no damage to the database. Acceptance: 10 SIGTERMs at 10 ms all exit 0.

## DONE lines
DONE: BATCH-37-A FAIL release-safe, but KF-499 and KF-458 miss their acceptance checks and new defect 1 (KF-479 guard race) needs a fix
DONE: GATE PASS fmt/clippy 0, 331 tests in 3 TZs and env -i, jsc, cfd, release 0 warnings, nextest 5x 0 LEAK, Linux root and uid 1000
DONE: SAFETY PASS v2 file opens with every value intact and documented additions only, rollback identical, same fingerprint and listing, GET sweep and 403 sweep leave md5 unchanged
DONE: STREAM-S FAIL KF-458 tests: 2 of 7 pass on the old code; every other S defect passes
DONE: STREAM-X FAIL KF-499 started_at acceptance fails and session ids repeat after a restart; every other X defect passes
DONE: STREAM-Y PASS KF-480 seed matches golden board/02, KF-456 0 LEAK
DONE: W1-SERVER PASS KF-479 refuses the last column (concurrency gap filed as defect 1), KF-406 stored main lane by position, renders read-only
DONE: INTEGRATION-POINTS PASS all listed paths verified live
DONE: AGENTCTL PASS 242/242 on both binaries, claim race 100/100, 300 metadata PATCHes lose 0
DONE: DOCS PASS agents.md and skill.md byte-equal to served, OpenAPI consistent; defects 2 and 4 are wording errors
DONE: KF-458 FAIL fix sound (15/15 runs, HTTP races 0), but create_task and copy_colors tests do not fail on old code
DONE: KF-459 PASS
DONE: KF-460 PASS
DONE: KF-467 PASS
DONE: KF-468 PASS
DONE: KF-469 PASS
DONE: KF-470 PASS
DONE: KF-471 PASS
DONE: KF-472 PASS
DONE: KF-455 PASS
DONE: KF-407 PASS
DONE: KF-419 PASS
DONE: KF-418 PASS
DONE: KF-448 PASS
DONE: KF-449 PASS
DONE: KF-454 PASS
DONE: KF-466 PASS
DONE: KF-433 PASS
DONE: KF-434 PASS
DONE: KF-435 PASS
DONE: KF-489 PASS
DONE: KF-490 PASS
DONE: KF-492 PASS
DONE: KF-493 PASS
DONE: KF-494 PASS
DONE: KF-495 PASS
DONE: KF-498 PASS
DONE: KF-499 FAIL stop {started_at:A} still stops same-second B; UI sends only started_at; ids repeat after restart
DONE: KF-502 PASS
DONE: KF-480 PASS
DONE: KF-456 PASS
DONE: KF-479 PASS acceptance met (single delete refused, column stays); concurrent-delete race filed as defect 1
DONE: KF-406 PASS
