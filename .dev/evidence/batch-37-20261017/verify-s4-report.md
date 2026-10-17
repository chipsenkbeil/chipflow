verify-s4 report: candidate fix/server-s2 @ fbbf0a4 against baseline 657ec5e. All 17 defects PASS. Three non-blocking observations are listed after the DONE lines.

Evidence folder (E): /private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-s4/

## Environment
- Worktrees: .dev/worktrees/verify-s4 (detached fbbf0a4) and verify-s4-base (657ec5e). The candidate diff touches only the 14 expected files: agent-guide.md, agents/skill.md, src/{db,main,models,routes}.rs, static/app.js, templates/board_delete.html, and 6 test files.
- Release binaries, copied to E: chipflow-cand md5 5a0a9e8a611d85085a39beaba2de6dd9 and chipflow-base md5 0e782453ea6f551d32df3de106476587. `rg -a -c 'second stop signal'` gives 1 for cand and 0 for base.
- Ports: cand 3400, base 3401. Every DB is under E (cand.redb, base.redb, reg*.redb, mal-*.redb, orph*.redb, sw-*.redb). Servers were stopped by exact PID, and no listeners remain.
- Browser: Playwright 1.63.0 via node, viewport 1905x1000, DPR 1. No Claude-in-Chrome tools, no python or perl.
- The baseline worktree carries scratch edits: extra test copies, an untracked examples/mkdb.rs (which builds baseline-format DBs), and a throwaway test insertion in routes.rs that was reverted afterwards. It is my scratch worktree, not the candidate.

## Gates (candidate)
- `fmt --check` exit 0.
- `clippy --all-targets` exit 0 with 0 warnings.
- `cargo build --release` has 0 warnings. The baseline release build has 1 (unused `ServeDir`).
- `TZ=UTC cargo test` (/Users/senkwich/.cargo/bin/cargo, plain libtest) exit 0. The suite has 23 lib, 103 bin and about 60 integration tests. `schema_fingerprint_tests::stored_schema_matches_fingerprint` passes with SCHEMA_VERSION 1.
- `env -i` libtest exit 0.
- jsc checkSyntax exit 0.
- Logs: g-fmt.log, g-clippy.log, g-test-utc.log, g-test-envi.log, g-jsc.log, build-cand.log, build-base.log.

## Per-defect results

| KF | Baseline | Candidate |
|---|---|---|
| 458 | Live: 12 concurrent POST /api/tasks into one cell gave duplicate positions in 25/25 rounds. Named test file: 5 of 7 tests fail (see CONCURRENCY). | Live: 0/25 rounds with duplicates. 7/7 tests pass, 24/24 runs. |
| 459 | A red task moved to a board where the default is blue keeps the source board's red colour id, which does not exist on the target. A "default-colour" task is also off-board. | Moved task gets the target board's red (`value=red`). With target red deleted it gets the target default (`value=blue`). Moving back restores the source red. Test `move_task_across_boards_takes_the_target_boards_color` fails on baseline. |
| 460 | Time added from task A's dialog lands on the same-named task on the other board. Run 1: A log [], B [45m,30m]. Run 2: both entries on A. Typing the shared name "Same" from another task's dialog silently attached 10m to one of them. | A [30m], B [45m]. The edit-entry dialog left unchanged keeps its task. Typing shared "Same" from the Zzz dialog shows the error "Several tasks are named "Same"; add the time from the task itself." and logs nothing. Typing the dialog's own unique name works. 0 console errors. |
| 467 | An orphaned pomodoro timer (written via the baseline lib) stopped with `{"discarded":false,"minutes":6}`. Timer-log total stays 2, so nothing was logged. | Same response, but total goes 2 to 3 with `{task_id:"",minutes:6,task_name:"No task",kind:"pomodoro"}`. The response matches the log. |
| 468 | 12 concurrent first POSTs gave 5 and 6 distinct tokens in rounds 1 and 2. Only one .ics returned 200 and the rest returned 404. | 5 rounds: 1 distinct path each, .ics 200, stored token equals all 12 responses. |
| 469 | OpenAPI says "Backfills the standard palette…" and "with the POST below". | Colors summary says "Read-only: boards that predate colors get the standard palette when the server starts." Feed summary names `POST /api/boards/{id}/calendar-feed`. The stale strings no longer appear. |
| 470 | `cargo build --release` prints 1 warning. | 0 warnings. |
| 471 | Container rust:1.98.0, as root, `failed_crash_repair_is_reported_as_such` fails with panic "a read-only file cannot be repaired". | Root: `cargo test --offline` exit 0 (23+103+all integration tests). uid 1000: exit 0. |
| 472 | Release binary, SIGTERM 50 ms after launch, 10 runs: exit 143 each time, and the reopen log shows "redb repaired it" 10/10. A second SIGINT during a stalled drain exits after 9759 ms. | 10/10 runs exit 0, reopen needs no repair (0 messages). Second SIGINT 0.3 s into a stalled drain: exit 0 after 68 ms. |
| 455 | A 101-character name is accepted: POST /api/boards returns 200, and 101-char and 100-char boards were stored. | 101 characters gives 400 "Board name must be 1-100 characters." (same with a template id). Blank gives 400. 100 characters with padding gives 200 and the name is trimmed. 100 and 101 characters of "é" give 200 and 400, so it counts characters, not bytes. Web new-board page: 101 characters stays on /boards/new and shows "Could not create the board." |
| 407 | One active and one archived board: DELETE of the active board returns 200. | 400 "cannot delete the last active board; unarchive another board first". Deleting an archived board returns 200. The delete page carries `data-last-board` with that text. Playwright click on Delete shows "Could not delete board: cannot delete the last active board; unarchive another board first", with 0 console errors. |
| 419 | A task created in the Done column has `completed_at:null` and groups under "Completed". | `completed_at` is stamped and the group is "Today". A board made from a saved template also stamps the Done-column task, while the To-do task stays null. |
| 418 | Malformed grouping dates stay as stored. | See the KF-418 detail below. |
| 448 | `due_at` sent as "   ", "" or null leaves `due_repeat` "every week". | All three give `[null,null]`. A repeat-only patch on a task with no due and a create with a repeat but no due both drop the repeat. A repeat-only patch on a task with a due, a null repeat, and an unrelated patch behave as before. |
| 449 | Rust tokens appear in 11 descriptions: `Some(None)`, `None`, `[`ColorRow`]`, `#[utoipa::path]`. | 0 Rust tokens in any description or summary of the served openapi-cand.json. Test `openapi_text_should_not_show_rust_syntax` fails on baseline. |
| 454 | Not applicable (docs). | skill.md documents `PUT /api/time/entries/<entry>`, HH:MM:SS second precision, the 404 for an unknown task, and task-less entries. Every claim held live. |
| 466 | The docs say "reaches the duration". | agent-guide.md, skill.md and the OpenAPI text for TimerStopInput.completed and TimerStopResult.completed state the 2 s grace, the stopwatch echo, and "False when discarded". `COMPLETION_GRACE_SECS`=2 and is used at routes.rs:7427. No stale backfill text remains. |

Tests that fail on baseline (cherry-picked onto verify-s4-base):
- 7 of 8 added routes.rs unit tests (KF-449, 466, 469, 468, 467, 455, 448). The exception is `manual_time_should_stay_on_the_named_task_across_boards`, which passes on baseline and so does not discriminate for KF-460.
- `tasks_created_in_a_done_column_should_be_complete` (KF-419).
- `move_task_across_boards_takes_the_target_boards_color` (KF-459).
- `sigterm_during_startup_leaves_a_cleanly_closed_database` and `second_signal_drops_open_requests_at_once` (KF-472).
- `grouping_date_normalize` and `settings_delete_board` fail to compile on baseline because `normalize_grouping_date`, `LAST_BOARD` and `LAST_ACTIVE_BOARD` do not exist there.
- Logs: base-unit-tests.log, base-int-tests.log.

KF-418 detail:
- The startup log (mal-cand.redb.cand.log) names each changed task:
  - "2026-1-5" is rewritten as "2026-01-05".
  - " 2026-1-05 " is rewritten as "2026-01-05".
  - "garbage", "2026-02-30", "2026-13-01" and fullwidth digits "２０２６-1-5" are cleared with the message "is not a date; cleared it".
  - The strict "2026-10-05" is untouched.
- The rewrite runs after the schema guard. In `Db::open`, `ensure_schema_version` precedes `normalize_grouping_dates` (src/db.rs ~873 versus ~905), and the existing test `newer_db_is_refused_and_left_byte_identical` passes. I did not run a live newer-schema DB with a malformed date.
- It is idempotent. Two further starts printed 0 grouping lines, and the API dump after the 3rd start was identical to the 1st. File md5 changes on every open under both baseline and candidate (login writes a session), so I compared API dumps instead.
- The board UI shows finished1 under "Monday, 5 January".

## DONE: CONCURRENCY
- `tests/db_check_then_act.rs` on the candidate, plain libtest via `cargo test --test db_check_then_act`, 24 runs in a row: 24/24 ok, "7 passed; 0 failed", 11 to 25 s each. Results are in loop458.results and loop458.runN.log.
- I could not reproduce the builder's `copy_colors` failure. A failure during a disk-full window is consistent with an `.expect()` on a failed redb commit, but I cannot prove that. Disk was 188 GB free throughout my runs.
- On baseline, 5 of 7 tests fail: create_task positions, delete_color, create_entry, ensure_default_swimlane, apply_template. Over 15 reruns of the other two:
  - `update_color_should_leave_one_default_under_concurrent_changes` failed 1/15.
  - `copy_colors_should_copy_a_consistent_source` failed 0/15.
  - So the tracker acceptance "each test fails on the old code" is only weakly met for those two (observation 1).
- Live HTTP races on 3400 and 3401:
  - create_task max position: baseline had duplicate positions in 25/25 rounds, candidate in 0/25.
  - Default colour: 60 and 80 rounds of concurrent create and set-default found `defaults != 1` in 0 rounds on both baseline and candidate. The race window is too small over HTTP, so the Rust test is the only evidence for that function.
- Result: PASS.

## DONE: REGRESSION
- No stored struct or table change: the fingerprint test passes with SCHEMA_VERSION 1, and `git diff` shows no `TableDefinition` change.
- GET and HEAD sweep: every route in routes.txt (88 routes, 176 requests) with a session cookie and `Sec-Fetch-Site: cross-site`. The DB md5 never changed (0 changes). Codes were 57 GET 200, 2 303, 3 404, 26 405, and HEAD the same. The baseline gave identical counts.
- Cross-site writes: 336 POST/PUT/PATCH/DELETE requests, 336 returned 403, DB unchanged. Baseline identical (sweep-cand.codes, sweep-writes-*.codes).
- A baseline-written DB (reg.redb, seeded with the golden-master script plus a 7-task board) opens under the candidate. The two API dumps (55 files each) differ only in the 6 grouping dates KF-418 rewrote or cleared, and that diff is the whole of it.
- UI smoke: 26 pages across 3 boards plus task modals, 0 console errors, 0 page errors, 0 failed requests, 0 HTTP >= 400. Screenshots: smoke-cand-*.png.
- Result: PASS.

## DONE: GATE
PASS: fmt, clippy 0 warnings, release build 0 warnings, TZ=UTC libtest, `env -i` libtest, jsc, and the container runs at root and uid 1000.

## Judgements
1. **Orphaned work session logged task-less: accept.** It matches how pomodoros and stopwatches with no task were already handled, and the response now equals the log. The orphaned break is not logged, as for any task-less break. Live proof with an orphaned short_break: stop returns `{"discarded":false,"minutes":6}` while the log total stays 2. That is the old reports-minutes-but-logs-nothing inconsistency for breaks only, and I accept it as scope-limited (observation 3).
2. **Typed shared task name refused: accept.** It fails safe, and the message tells the user what to do. The cost is that from a dialog opened without a task, such as the timer-log "Add time entry", a task whose name is duplicated cannot be reached at all. The datalist also lists a duplicated name once, so users cannot tell the tasks apart. The dialog's own task still wins while its name is unchanged. Showing the board name in the option would be the longer-term fix.
3. **Malformed grouping dates cleared rather than parsed: accept.** Valid unpadded dates are padded, so no information is lost. Only non-dates are cleared, and those were already ignored for grouping. Each change is logged with the task id and the original value. The padded path also trims whitespace.
4. **Repeat for a task with no due date dropped: accept with a caveat.** It is documented in agent-guide.md, and it keeps the model consistent. It is silent and order-dependent: a client that sends the repeat before the due in separate requests loses it. A 400 would be more explicit.

## Observations (candidate defects, each with a one-line acceptance check)
1. The KF-458 tests for `update_color` (failed 1/15 on baseline) and `copy_colors` (0/15) do not reliably fail on the old code. Acceptance: on baseline, `cargo test --test db_check_then_act` 15 times, each of the 7 tests fails in at least 14 runs.
2. The KF-459 tracker acceptance names `GET /api/tasks/{id}` returning `color_id`, but that route has no `color_id` field (it returns id, member_ids, grouping_date, watched, subtasks, labels, due_*). I verified through `GET /api/boards/{id}/export-data` instead. Acceptance: the tracker wording is changed to export-data `tasks[].color_id` being in `colors[].id`, or `GET /api/tasks/{id}` gains `color_id`.
3. An orphaned break session reports minutes but logs nothing. Acceptance: with an active short_break on a deleted task, POST /api/timer/stop returns `discarded:true` or the timer log total increases.
4. The KF-460 server-side test passes on baseline, because the bug was client-only. Acceptance: a browser-level test that adds time from task A's dialog with a same-named task on another board and sees the entry only in A's log.
5. The new-board page shows only "Could not create the board." for 101 characters, and the input has no `maxlength`. Acceptance: at /boards/new the input has `maxlength=100` or the error contains "1-100".
6. Pre-existing, also on baseline: archiving every board (PUT config `{"archived":true}`) leaves 0 active boards, and the delete-page Archive button does the same. Acceptance: archiving the last unarchived board returns 400.
7. The KF-471 root skip prints to stderr only, so `cargo test` reports "ok". Acceptance: n/a, low.
8. A SIGTERM about 10 ms after launch can still kill the process with exit 143 (1 of 2 runs at 10 ms). The handler goes in right after the runtime is created, and the DB file does not exist yet at that point. At 20 ms and 30 ms it was 16/16 exit 0 with no repair. Acceptance: n/a, informational.

DONE: KF-458 PASS 7/7 db_check_then_act tests 24/24 runs on candidate, 5 of 7 fail on baseline (update_color 1/15 and copy_colors 0/15 are weak discriminators), live create_task positions 25/25 duplicate rounds on baseline vs 0/25 on candidate
DONE: KF-459 PASS red task moved to a board with default blue gets the target's red, falls back to the target default (blue) when its red is deleted; baseline keeps the off-board colour id
DONE: KF-460 PASS (client-side, browser-verified) baseline sends time to the same-named task on the other board and accepts an ambiguous typed name; candidate keeps A [30m] and B [45m] and refuses the shared typed name; 0 console errors
DONE: KF-467 PASS orphaned pomodoro stop logs a task-less entry (timer-log total 2 to 3, "No task", 6 min), baseline logged nothing
DONE: KF-468 PASS 12 concurrent first POSTs give 1 distinct path with .ics 200 in 5 rounds, baseline 5 and 6 distinct paths with 404s
DONE: KF-469 PASS served OpenAPI has no "Backfills" and no "POST below"
DONE: KF-470 PASS release build 0 warnings, baseline 1
DONE: KF-471 PASS rust:1.98.0 container offline, root exit 0 and uid 1000 exit 0, baseline test fails as root
DONE: KF-472 PASS SIGTERM at 50 ms gives exit 0 and no repair on reopen in 10/10 runs (baseline 143 and repair 10/10); second SIGINT in a stalled drain exits in 68 ms (baseline 9759 ms)
DONE: KF-455 PASS POST /api/boards with 101 characters gives 400 (baseline 200); 100 gives 200; "é" boundary counts characters
DONE: KF-407 PASS deleting the only active board with an archived one left gives 400 (baseline 200); the delete-page UI shows the refusal text with 0 console errors
DONE: KF-419 PASS Done-column create stamps `completed_at` and groups under "Today" (baseline null and "Completed"); template-applied Done tasks are stamped too
DONE: KF-418 PASS startup rewrites "2026-1-5" and " 2026-1-05 " and clears 4 non-dates, logs 6 changes, runs after the schema guard, touches nothing else, idempotent on the 2nd and 3rd start
DONE: KF-448 PASS clearing the due with "   ", "" or null clears the repeat (baseline kept "every week"); repeat-only on a due-less task is dropped
DONE: KF-449 PASS 0 Rust tokens in any OpenAPI description or summary (baseline 11)
DONE: KF-454 PASS skill.md documents PUT /api/time/entries/{id}, second precision, 404 for an unknown task, and task-less entries; all claims verified live
DONE: KF-466 PASS agent-guide.md, skill.md and the OpenAPI timer text state the 2 s grace and the stopwatch echo
DONE: CONCURRENCY PASS 24/24 libtest runs (7/7 each), live race reproduced on baseline for create_task, copy_colors failure not reproduced
DONE: REGRESSION PASS fingerprint ok, 176 GET/HEAD requests with 0 DB writes, 336/336 cross-site writes 403, baseline-DB API dump differs only by the 6 KF-418 dates, smoke 0 console errors
DONE: GATE PASS fmt, clippy 0 warnings, release 0 warnings, TZ=UTC and env -i libtest, jsc, container root and uid 1000
