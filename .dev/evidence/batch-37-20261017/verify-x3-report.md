verify-x3 report. fix/server-x2 @ b9f846a against baseline 2c1773b.

Result: every defect passes, no regressions, all gates green. I found two minor candidate defects and a few observations (below).

## Environment
- Candidate worktree `.dev/worktrees/verify-x3` (detached at b9f846a). Baseline worktree `.dev/worktrees/verify-x3-base` (2c1773b). Test-graft worktree `.dev/worktrees/verify-x3-tests`.
- Release binaries:
  - candidate md5 `7583375e1b5a60a2fe75c2dd3a7505a5`
  - baseline md5 `c4eefe01efb71099e65794a61ca5861b`
  - `rg -a -c 'cannot archive the last active board'`: candidate 3, baseline 0.
- Release build prints one `unused import ServeDir` warning on both trees (pre-existing).
- Ports: candidate 3412, baseline 3413. Databases: `cand.redb`, `base.redb`, `base2.redb`, `base-copy.redb`.
- All evidence is in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-x3/`. Key files:
  - scripts: t1.sh to t11.sh
  - outputs: `t1-*.txt`, `t4-*.txt`, `t5*.txt`, `t6-*.txt`, `t8.txt`
  - race logs: `race-3412.log`, `race-3413.log`
  - dumps and sweeps: `dump-*.json`, `smoke-*.txt`, `sweep-*.txt`
  - gates: `g-*.log`
  - screenshots: `ui-*.png`
- Playwright script: `r2/pw/x3-ui.mjs`. Viewport 1905x1000, DPR 1.
- Both servers are stopped. I started only these PIDs and killed only them.

## Per defect (baseline -> candidate)

**KF-433 PASS**
- Delete the target column:
  - baseline: `due_target_column_id` kept the dead id, and re-sending it gave 400.
  - candidate: the target became null, and re-sending it gave 200.
- Cross-board move:
  - baseline: the foreign column id stayed on the task, and re-sending it gave 400.
  - candidate: the target became null and `due_at` stayed.
- Named test `due_target_should_clear_when_its_column_goes_away` fails on baseline code, passes on the candidate.
- Break attempt: sending a foreign column explicitly after the move still gives 400.

**KF-434 PASS**
- Board `due_time` 08:30, task target "Review", saved as a template and applied.
  - baseline: board `due_time` 17:00, target null.
  - candidate: `due_time` 08:30, target resolves to "Review" on the new board.
- A template saved by the baseline (no `due_time` or target in its snapshot) applies cleanly under the candidate, with `due_time` 17:00.
- Duplicate column names: the target maps to the last column with that name (see observation 4).
- Named test `template_should_keep_due_target_and_due_time` fails on baseline.

**KF-435 PASS**
- A target-only change logs:
  - baseline: "Set the due date to May 05, 2031 5:00 PM" (3 times).
  - candidate: "Set the due date target column to Tgt", "Set the due date target column to Extra", "Cleared the due date target column".
- Named test `target_only_due_change_should_log_the_target_column` fails on baseline.
- Break attempt: a PATCH changing `due_at` and the target together logs only the date line (see observation 3).

**KF-489 PASS**
- Baseline ignored the filter: `board_id` was missing on list items, and an unknown board returned 200 with 34 tasks.
- Candidate acceptance run over all 10 boards: every returned `board_id == X`, the filtered counts add up to the unfiltered count of 33, and the counts match `/api/boards/X/tasks`.
- An unknown board returns 404. A Bearer token gives the same results.
- An archived board's tasks stay listed and filterable.
- Named test `task_list_should_filter_by_board_id` fails on baseline.

**KF-490 PASS**
- Baseline had `palette_hexes` in routes.rs plus the override in `get_color`.
- Candidate: `palette_hexes` is gone, and one `ColorRow::with_palette_hexes` backed by the `standard_color` / `STANDARD_COLORS` table is used by `get_color` and `task_context_in`. `list_colors` goes through `get_color`.
- The candidate's unit test `with_palette_hexes_should_use_the_palette_for_standard_colors_only` passes.
- Baseline-side check: I did not graft this test onto the baseline because it needs the new method and would only fail to compile.

**KF-492 PASS**
- Baseline: the API archived the only board and `/api/boards` returned `[]`. The page's Archive button navigated to `/boards/archived`.
- Candidate: the API returns 400 "cannot archive the last active board", and the list is unchanged.
- Candidate page: with one active board left, clicking "Archive board" stays on `/settings/delete`, shows "Could not archive board: cannot archive the last active board", and the button re-enables. `ui-archive-3412.png` shows it.
- The only browser console error was the expected 400 resource line.
- Named test `archiving_the_last_active_board_should_be_refused` fails on baseline.

**KF-493 PASS**
- The named test `task_color_update_should_never_store_a_deleted_color` also passes on the baseline, as warned.
- My live HTTP race: 300 rounds of concurrent PATCH `color_id` and DELETE of the same colour, checked via export-data.
  - baseline: 300/300 rounds left the task holding a deleted colour (update 200 and delete 200 every time).
  - candidate: 0/300 dangling. Outcomes were exclusive: 146 rounds update 200 + delete 400, 154 rounds update 400 + delete 200.
- A foreign-board colour id and a nonexistent id both give 400 on both builds. An own colour gives 200 and clearing gives 200.

**KF-494 PASS**
- Baseline task detail has neither key.
- Candidate has both: `color_id` is a uuid or null, `completed_at` is null or an RFC3339 string after the task moves into Done.
- OpenAPI `TaskDetail` lists both properties. `agent-guide.md` and `skill.md` document them.
- Named test `task_detail_should_include_color_id_and_completed_at` fails on baseline.

**KF-495 PASS**
- PUT on a task-less entry with no `task_id`:
  - baseline: 400 "missing field `task_id`", and `task_id: ""` returned 404 "task not found".
  - candidate: 200, `task_id` stays "", minutes 30 then 40, and the entry stays in the timer log.
- A task entry without `task_id`, or with `task_id` "", gives 400 "task_id is required". A bogus task gives 404.
- Assigning a task to a task-less entry gives 200. The task's `total_minutes` then reads 40 and the task history shows the edit.
- Named test `editing_a_task_less_entry_should_keep_it_task_less` fails on baseline.

**KF-498 PASS**
- Real 5-minute task-less short break, stopped with `activity_id` "act-walk":
  - baseline: `break_activities[0].minutes` 0 -> 0, and `/api/timer/log?entry_type=short_break` is `[]`.
  - candidate: 0 -> 5, `today_minutes` 5, and the log has one entry (`task_id` "", "No task", 5 min). `time-spent` shows `break_minutes` 5.
  - Under a `board_id` filter the candidate shows 0, as documented.
- Named test `task_less_break_should_reach_the_log_and_break_statistics` fails on baseline.

**KF-499 PASS**
- 3 rounds each of start A, start B in the same second, then stop with `started_at` A:
  - baseline: A == B every time. The stop returned `already_stopped:false` and the timer ended up idle (B stopped). A burst of 5 starts gave 5 identical `started_at` values.
  - candidate: A != B (one second apart). The stop returned `already_stopped:true` and B stayed active (`started_at` = B).
- Named test `stale_stop_should_not_stop_a_session_started_in_the_same_second` fails on baseline.
- Break attempt: a burst of 40 starts exposed the clock skew in observation 1.

**KF-502 PASS**
- A write with `Origin: http://127.0.0.1:PORT:80`:
  - baseline: 200.
  - candidate: 403.
- Candidate results for the other origins, with Host set to match:
  - The same malformed forms for https `:8443:443` and IPv6 `[::1]:3000:80` give 403.
  - Own origin gives 200, the default-port forms and https/443 pass the check (401 from auth because of the Host override, not 403), and uppercase scheme gives 200.
  - `:80` on https and `:443` on http give 403 on both builds.
  - `Sec-Fetch-Site: cross-site` gives 403.
- A Bearer write with the malformed Origin passes, as designed.
- Named test `normalize_origin_should_keep_a_default_port_suffix_after_another_port` fails on baseline.

## DONE: REGRESSION
- Schema: `SCHEMA_VERSION` is still 2. The diff touches no stored struct or table. The added `Db.last_session_start` is not stored, and the `models.rs` change is an impl plus a test. `stored_schema_matches_fingerprint` passes in both test runs.
- GET/HEAD sweep: 66 GET routes x GET and HEAD with cookie, `Sec-Fetch-Site: cross-site` and a foreign Origin, 132 requests per build. The database md5 was unchanged on both (candidate `7083da27…`). Statuses were identical across builds (124x200, 4x303, 4x404).
- Cross-site writes: 10 write routes on each build gave 403 every time, with the database md5 unchanged.
- Baseline-written database under the candidate: it opens at schema version 2. A 260-endpoint dump (boards, columns, tasks, task details, colours, export-data, timer log, statistics, time-spent, entries) differs only by added `color_id` and `completed_at` lines (39 tasks each).
- agentctl-style smoke on a fresh baseline versus fresh candidate database: metadata set, merge and bad input, links and blocked, claim, claim conflict, stale fence, release, list filters (label, ready, claimed, since, unknown board). The only differences are the added `color_id` and `completed_at` on task detail.
- Docs:
  - served `/agents.md` is byte-identical to `agent-guide.md`, and served `skill.md` is identical to `agents/skill.md`.
  - OpenAPI parses and all `$ref`s resolve.
  - OpenAPI documents `board_id` on `TaskNameItem` and as a query parameter with 404, `UpdateTimeEntryInput.task_id` as optional, and the 400 text on `PUT /api/time/entries/{id}` and `PUT /api/boards/{id}/config`.
  - Both guides cover the new fields, the 400 on archiving the last board, and the distinct `started_at`.
  - They do not mention that a due target is cleared on column delete or cross-board move (observation 5).
- Board UI smoke on the candidate, 10 pages plus board, task modal and timer panel: 0 console errors, 0 page errors, 0 failed requests, 0 HTTP errors (`ui-board-3412.png`).

## DONE: GATE
- `cargo fmt --check`: exit 0.
- `cargo clippy --all-targets`: exit 0, 0 warnings.
- `TZ=UTC cargo test`: 292 passed, 0 failed.
- `env -i` plain libtest: 292 passed, 0 failed.
- jsc `checkSyntax` on `static/app.js`: exit 0.
- Release build: ok.
- Baseline with the candidate's tests grafted on, `TZ=UTC`: exactly 10 failures, all as claimed. These are the tests for KF-433, 434, 435, 489, 492, 494, 495, 498, 499 and 502. The KF-493 test passes there, as the brief warned.

## Judgements
- **Task-less breaks logged: accept.** It is consistent with task-less work sessions already being logged. Statistics count only breaks with an `activity_id`, and a `board_id` filter excludes them, which is documented.
- **Same-second starts stamped +1 s from an in-memory counter: acceptable as a tactical fix, with a real side effect.** See observation 1. A per-session id would be the lasting fix.
- **Unknown `board_id` returns 404: accept.** It matches `/api/boards/{id}/tasks`. Two small inconsistencies are in observation 2.
- **Target cleared on column delete with no history entry: accept, but I would log it.** It does bump `updated_at`, so `since=` pollers see the change. A history line would match KF-435's logging of target changes.

## Candidate defects / observations

1. **Rapid timer starts push `started_at` into the future, which can discard a real session.**
   - After a 40-start burst the status reported `remaining_seconds` 1539 against `total_seconds` 1500.
   - A task-less stopwatch started afterwards and stopped 22 s later returned `discarded:true`.
   - Rapid starts only come from scripts and agents; UI starts don't hit it.
   - Acceptance: after 40 consecutive `POST /api/timer/start` calls, `remaining_seconds <= total_seconds`, and a 22 s stopwatch started right after stops with `discarded:false`.
2. **`GET /api/tasks?board_id=` (empty value) returns 404 instead of acting as no filter.**
   - The 404 body says "board not found", while `/api/boards/{id}/tasks` says "Board not found".
   - Acceptance: `GET /api/tasks?board_id=` returns 200 with all tasks, or 400, and the 404 body text matches the board route's.
3. **A PATCH changing both `due_at` and `due_target_column_id` logs only "Set the due date to …".** The target change is not recorded.
   - Acceptance: after such a PATCH, the history contains both "Set the due date to" and "Set the due date target column to".
4. **Templates resolve a due target by column name.** With two columns named "Review", the target goes to the last one regardless of which was original.
   - Acceptance: a template of a task whose target is the first of two same-named columns applies with the target on the first.
5. **Docs gap.** `agent-guide.md` and `skill.md` do not say that a due target is cleared when its column is deleted or the task changes board.
   - Acceptance: `rg 'due_target_column_id' agent-guide.md` finds a line stating that behaviour.

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
DONE: KF-499 PASS
DONE: KF-502 PASS
DONE: REGRESSION PASS
DONE: GATE PASS
