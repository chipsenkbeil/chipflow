verify-38a report: batch-38 server/data half plus the real-data upgrade audit

Bottom line: the production-data audit, the safety checks, the gates and agentctl all pass. Of the 15 server items, 12 pass. Three fail their tracker acceptance: KF-517, KF-518 and KF-499. None of the three is worse than production (each is better), so I found no regression against d2a466a.

## Environment
- **Candidate:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-38a`, detached at e9a829d. Release binary md5 8de5bbe647ea0297c6a6ff4b3d8f0c2f.
- **Baseline:** `.../worktrees/verify-38a-base` at d2a466a, binary md5 993e4ad86fb734b6e61646d442522d4c. The two binaries differ: `rg -a -c` finds "due_target_column_id needs a due date" 2 times in the candidate and 0 in the baseline, "cannot be changed" 3/0, "cannot bind" 1/0.
- **Audit trees:** `verify-38a-v1` at a980a7c (md5 8b26703be529f27b62e4d508e9724537) and `verify-38a-v2` at 9548374 (md5 40f7f7cd9739c5ee2a28f287e8288183).
- **KF-518 tree:** `verify-38a-pre458` at 7fcfa55^ (4be557c).
- All builds were release builds with `--config 'build.build-dir="target/build"'`, one cargo invocation at a time.
- **Ports:** candidate 3386, baseline 3387, audit and extra runs on 3392/3393.
- **Databases:** all test databases are under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-38a/db/` and `.../verify-38a/k507/`. Every run used `--database-path`.
- **Browser:** Playwright 1.63.0 from r2/pw, 1905x1000 at DPR 1, a fresh context per scenario.
- **Cleanup:** I stopped every server I started by exact PID (listed in `.../verify-38a/pids.txt`). All five worktrees are clean.
- **One slip, already fixed:** my first `git worktree add` loop did not split words under zsh. It created five worktrees with spaces in their names at d332e14. I removed exactly those five and recreated them correctly.

## Part 1: real-data upgrade audit — PASS

**Method.** I wrote my own dumper and comparator; I did not read anyone else's.
- The dumper, `examples/audit_dump.rs`, was compiled separately into each of the three trees. It reads every redb table through `list_tables` and `list_multimap_tables` with the real key and value types. Each row value becomes canonical JSON, and each row also gets a sha256 of its raw bytes. In `connect` mode it first opens the file with that tree's `chipflow::db::Db::connect`, which is the binary's own startup path.
- The comparator, `compare.jq`, checks every v1 leaf path for missing or changed values, counts rows whose raw bytes changed, counts lost multimap pairs, and lists added leaves by key name only.
- I never opened the original file. Its md5 is 69cb97df3dd5f2156f2bb2dbf606404a before and after.
- The a980a7c and candidate dumpers produce byte-identical dumps of the raw v1 copy.

**Results.** Pair (a) of your request is rows 2 and 3 of this table.

| Comparison | Result |
|---|---|
| a980a7c raw dump vs a980a7c `connect` (v1 code reopening its own file) | No change of any kind |
| raw v1 vs 9548374-upgraded | Only change: `meta.schema_version` 1→2. Four empty tables added (`device_auths`, `field_defs`, `fields_by_board`, `task_links_by_target`). 0 rows changed. |
| raw v1 vs candidate-upgraded | Same as 9548374, plus `boards.config_json.main_swimlane_id` added on 2 of 2 boards (documented v2 item 8). 0 changed or removed leaves; the only byte-changed rows are those 2 boards. |
| 9548374-upgraded, then candidate (the real production path) | Only `main_swimlane_id` added on 2 boards |
| Real binaries started on copies (9548374 on 3392, candidate on 3393), then dumped | Identical to the `Db::connect` dumps |
| d2a466a started on the candidate-upgraded copy (rollback) | Starts, serves the login page, no data change |

- **Per-table row counts are identical before and after** (multimap pairs in parentheses): active_timer 0, api_tokens 1, attachment_data 0, boards 2, colors_by_board 2 (20), columns 9, columns_by_board 2 (9), entries_by_task 2 (2), meta 1, sessions 4, settings 3, swimlanes 3, swimlanes_by_board 2 (3), task_colors 20, tasks 23, tasks_by_column 6 (23), templates 2, time_entries 2, usernames 1, users 1.
- **Main lane assignment:** each board's assigned main lane is its lowest-position lane. On one of the two boards that lane is not named "Default".
- **Orphan repeats:** the production data has none, in tasks or in template snapshots, so the new startup cleanup changes nothing there.

**(b) Proof that the comparator detects loss.** Each control mutated a scratch copy, which was then upgraded and compared:
- Changing one task's `position` gave a changed leaf `tasks.position` (1 row) and 1 byte-changed row.
- Deleting one task gave `tasks.missing_rows=1`.
- Removing one `tasks_by_column` pair gave a missing row there.
- All three gave the verdict "LOSS-OR-CHANGE".
- My first comparator draft had two blind spots, both caught before these results: it skipped scalar rows (the meta table), and it treated 1.0 and 1 as equal. I fixed the first and added the raw-byte hashes for the second.

**Cleanup.** I deleted every working copy and dump of the private file. The original is the only copy left. The evidence in `.../verify-38a/audit-evidence/` holds only table names, key paths, counts and the two scripts.

## Gates — PASS
- `fmt --check` exit 0.
- `clippy --all-targets`: 0 warnings.
- Release build: 0 warnings.
- `/Users/senkwich/.cargo/bin/cargo test` passed 345 / failed 0 under each of TZ=UTC, America/Chicago, Pacific/Chatham and `env -i`. The 1 ignored test is the existing DST helper, also ignored on the baseline.
- `jsc checkSyntax` exit 0; `tests/js/cumulative_flow.js` reports `failures=0`.
- The diff touches only the 12 expected files.

## Part 2: server defects

Each row is: claim, then baseline result, then candidate result.

- **KF-507** (busy port, port held by `nc`): the baseline exits 1, but the v1 file is upgraded and changed, and the v2 file also changes. The candidate exits 1 with "cannot bind", and both v1 and v2 files stay byte-identical. The new `busy_port_startup.rs` test passes. **PASS.**
- **KF-508:**
  - Setting a target with no due date: baseline 200 and nothing stored; candidate 400 "due_target_column_id needs a due date".
  - Break-attempts: `due_at:null` plus a target gives 400; a rename plus a target gives 400 and the rename is not applied; due and target in the same request give 200 and stored; clearing the target with null or "" gives 200.
  - **PASS.**
- **KF-509:** the baseline returns 200 for kind, key and owner and changes nothing. The candidate returns 400 "<name> cannot be changed" for each, including `kind:null` and echoing the unchanged value. The field is unchanged afterwards and a label-only PATCH still gives 200. **PASS.**
- **KF-510:** `rg due_target_column_id agents/skill.md` finds the key in the TaskDetail list (line 201). That key was already there on the baseline. The claim_expired sentence now matches agent-guide.md ("first claim, heartbeat or release call"). Behaviour also matches: a heartbeat on an expired claim returns 409 and logs the expiry. **PASS.**
- **KF-518** — FAIL against the tracker acceptance. The 7 tests, 15 runs each, on 7fcfa55^:

  | Test | Failures on 7fcfa55^ |
  |---|---|
  | apply_template, copy_colors, create_entry, delete_color, ensure_default_swimlane | 15/15 each |
  | update_color | 8/15, then 15/15 and 13/15 on two reruns (36/45; machine load average 10–17) |
  | create_task | 0/15 |

  - On the candidate code every test passes 15/15.
  - Why create_task never fails: `insert_task` already computed positions inside its write transaction before KF-458, and 7fcfa55 did not change it. That test cannot fail on old code.
  - copy_colors (0/15 when filed) is fixed. update_color (1/15 when filed) is much better, but misses ≥14/15 in two of three batches.
- **KF-519:** the new test passes on the candidate, and it also passes when run against the baseline code. The bug is already gone in d2a466a, so this batch adds only the regression test. The acceptance is met. **PASS.**
- **KF-520:**
  - The baseline has no `maxlength`, and a 101-character name shows no message in the page.
  - The candidate has `maxlength="100"`: typing 105 characters leaves 100. A forced 101-character value shows "Board name must be 1-100 characters." with 0 console errors.
  - The server returns 400 with the same message on both.
  - **PASS.**
- **KF-521:** over HTTP, the baseline puts both tasks' column and target on the last of the two same-named columns. The candidate gives task a column 1 / target 0 and task b column 0 / target 1. The Rust test passes. **PASS.**
- **KF-522:**
  - Over HTTP, 100 rounds of two concurrent starts and 100 rounds of eight gave 0 mismatches on both builds; the race does not reproduce through HTTP.
  - The candidate's Rust test, run against the baseline code, fails 10/10 at round 0 with "the running session is retired once": both starts log the old session.
  - On the candidate it passes 10/10 plus the 4 gate runs.
  - **PASS.**
- **KF-523:** the baseline returns 404 for `%20`, `%20%20`, `%09` and `+`. The candidate returns 200 for each, identical to the unfiltered list. A space-padded valid id filters to its board; an unknown id still gives 404. **PASS.**
- **KF-517** — FAIL. Playwright counted `requestfailed` (ERR_ABORTED) events per path:

  | Path | Baseline | Candidate |
  |---|---|---|
  | Task delete from card context menu | 1 | 0 |
  | Task delete from task modal | 1 | 0 |
  | Task delete from timer context menu | 1 | 0 |
  | Lane move from lane menu | 1 | 0 |
  | Lane delete from lane menu | 1 | 0 |
  | Column move by layout-view drag | 1 | 1 |
  | Lane move by layout-view drag | 1 | 1 |

  - The cause: the layout-view drag reorders (app.js lines ~284 and ~299) reload without reading the response first.
  - `moveColumnBy`, the function the fix drained, has no caller. The only reloading column-move path in the UI is the layout-view drag.
  - My Playwright header drag timed out. That path does not reload on success, so this is a limit of my harness, not a finding.
- **KF-479:** over HTTP, 40 rounds of deleting a board's only two columns concurrently left 0 columns 39/40 times on the baseline. On the candidate it never did (0/40), and one of the two deletes is always refused. The Rust test passes 10/10. **PASS.**
- **KF-499** — FAIL against the tracker's literal acceptance. Everything in your scope passes:
  - **Restart:** on the baseline, ids restart at 1, and a stop carrying the previous process's id 1 stopped the new session. On the candidate, ids are microsecond-seeded, so the stale id gets already_stopped and the new session keeps running.
  - **UI stale-tab stop:** the baseline sends only `started_at` and stops B. The candidate sends `{"started_at":…,"session_id":…}`, gets already_stopped:true, and B keeps running.
  - **Docs** are updated.
  - **What fails:** the tracker acceptance stops with `{started_at: A}` only. That still stops a session B started in the same second, on both builds. This behaviour is now documented as second-granular.
- **Orphan repeats:** the unit test passes. Over HTTP a repeat cannot be set without a due date, and clearing `due_at` clears the repeat. The startup cleanup commits only when something changed. **PASS** (see observation 4).
- **KF-406 main lane hand-off** (delete the main lane, two lanes remain):
  - The baseline labels both remaining lanes, with no main.
  - The candidate makes "Second" the unlabelled main lane.
  - After the other lane is deleted, the candidate hides the lone lane's label column; the baseline still shows a label and a menu.
  - **PASS.**

## Part 3: safety — PASS
- **Schema:** SCHEMA_VERSION is 2. The fingerprints d999…ab7 and 0607…4f1 are identical in 9548374, d2a466a and e9a829d, and `stored_schema_matches_fingerprint` passes.
- **Rollback:** d2a466a opens a populated candidate-written file and applies a candidate-written template (which carries the new `column_index` keys): 200, 24 tasks, every board page 200. Existing rows are unchanged.
- **Read-only and cross-site sweep** (identical counts on the baseline):
  - The sweep covers 70 GET routes × GET and HEAD = 140 requests, sent with the cookie plus cross-site headers. Results: 132× 200, 4× 303, and 4× 404 (from the deliberate bad ids).
  - 60 write routes × 3 variants each returned 403 every time. The variants are `Sec-Fetch-Site: cross-site`, a foreign `Origin` only, and `same-site` with a sibling origin.
  - Application data is byte-identical across all five snapshots: populated, after first start, after GET/HEAD, after cross-site writes, after stop.
  - The file md5 stays constant through both sweeps. It does change on start and on clean stop, for both builds. A bare redb open with only a read transaction also changes it, so that is redb's own bookkeeping, not an application write.

## agentctl — PASS
My own probe makes 130 checks: tokens and scopes, TaskDetail keys, metadata merge and the 64 KiB cap, links and readiness, filters, claims, fences, expiry, history, WIP, comment actor kinds, fields, device pairing, and the KF-408 races. The races are 100 rounds of two concurrent claims (exactly one 200 each round) and 300 concurrent metadata PATCHes (all 300 keys kept). The candidate passes 130/130. The baseline passes 129/130; its only miss is the KF-509 check.

## Observations (candidate defects), each with a one-line acceptance
1. **KF-517 residual:** layout-view drag reorder of columns and lanes reloads before reading the response (app.js ~284, ~299). Add-column placement (~1358) and Copy here (~982) follow the same reload-on-ok pattern and are likely affected too (not exercised). `moveColumnBy` is dead code. Acceptance: a Playwright drag of a column and of a lane in the layout view raises 0 `requestfailed`.
2. **KF-518 residual:** the update_color race test fails only about 80% of runs on 7fcfa55^ under load, and the create_task test cannot fail on old code. Acceptance: three batches of 15 update_color runs on 7fcfa55^ each fail ≥14, and the tracker text excludes create_task.
3. **KF-499 residual:** a stop with `started_at` only still stops a session started in the same second. Acceptance: start A then B in the same second; `POST /api/timer/stop {started_at:A}` returns already_stopped:true and B keeps running. Alternatively, amend the acceptance to session_id.
4. **Legacy-template orphan repeat:** `apply_template` copies `due_repeat` without checking `due_at` (db.rs, `due_repeat: task.get("due_repeat")`). A snapshot saved before KF-448 therefore creates an orphan repeat that lasts until the next restart. Production has 0 such templates. Acceptance: applying a snapshot task with `due_repeat` set and `due_at` null yields a task with `due_repeat` null.
5. **KF-520 client limit:** the page counts UTF-16 units. A 60-emoji name, which the server accepts, cannot be entered at /boards/new. Acceptance: a 60-emoji name creates a board from /boards/new.
6. **Stale schema docs:** db.rs still says "Until version 2 ships… replaces the last SCHEMA_FINGERPRINTS entry" and "stays editable until version 2 ships", though v2 shipped in 9548374. That text invites editing a released fingerprint. Acceptance: `rg -i 'until version 2 ships' src/db.rs` finds 0 lines.
7. **API consistency (low):** a `due_repeat` without a due date is silently dropped (documented), while a `due_target_column_id` without one is now a 400. Similarly, KF-509 refuses an echo of an unchanged kind or key, so a client that round-trips a FieldDef gets a 400 (documented). Acceptance: the owner decides one rule for both.

## Evidence
Everything is in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-38a/`. It holds no private data:
- Audit: `audit-evidence/` (the cmp-*.json files, `compare.jq`, `audit_dump.rs`).
- Gates: `gates.out` and the `g-*.log` files.
- Defect probes: `kf-3386.out`, `kf-3387.out`, `race.out`, `k499.out`, `k518.out`, `k522-base-run.log`, `k522-cand-run.log`, `ui-3386.json`, `ui-3387.json`, `k507/`.
- Safety: `sweep-cand/`, `sweep-base/` (get.tsv, write.tsv), `sweep-cand.out`, `rollback.log`.
- agentctl: `agentctl-3386.out`, `agentctl-3387.out`.
- Scripts: `kf.sh`, `race.sh`, `race8.sh`, `k499.sh`, `ui.mjs`, `sweep.sh`, `agentctl.sh`, `gates.sh`.

DONE: AUDIT PASS: real prod v1 file upgraded by 9548374 and the candidate loses nothing; only the schema stamp 1→2, 4 empty tables and `main_swimlane_id` on 2 boards change; the mutation controls all fail as they should
DONE: BATCH-38-A FAIL: KF-517, KF-518 and KF-499 miss their acceptance (none regresses vs d2a466a); every other server item, safety, gates and agentctl pass
DONE: GATE PASS: fmt, clippy 0 warnings, release build 0 warnings, 345/0 tests under UTC, Chicago, Chatham and env -i, jsc 0, cumulative_flow 0
DONE: SAFETY PASS: schema v2 and fingerprints unchanged, d2a466a rollback opens candidate files, GET/HEAD and first start write no application data, 60/60 cross-site writes 403 in 3 variants
DONE: KF-507 PASS: busy port exits 1; v1 and v2 files byte-identical (baseline upgrades and changes them)
DONE: KF-508 PASS: target without a due date is a 400 naming the field and stores nothing
DONE: KF-509 PASS: key, kind and owner are each a 400 "<name> cannot be changed"; nothing written
DONE: KF-510 PASS: TaskDetail key present; claim_expired wording matches agent-guide.md
DONE: KF-518 FAIL: update_color fails 8, 15 and 13 of 15 on 7fcfa55^ (needs ≥14); create_task cannot fail on old code; copy_colors fixed
DONE: KF-519 PASS: regression test passes; bug already absent on d2a466a
DONE: KF-520 PASS: maxlength="100" and "Board name must be 1-100 characters." shown
DONE: KF-521 PASS: template keeps column and target by position among same-named columns
DONE: KF-522 PASS: Rust test fails 10/10 on baseline code (double retire), passes on candidate; 200 HTTP rounds with 0 mismatches
DONE: KF-523 PASS: blank board_id values return 200 with all tasks; unknown id still 404
DONE: KF-517 FAIL: task delete (3 paths), lane menu move and lane delete fixed (1→0), but layout-view column and lane drag still raise ERR_ABORTED
DONE: KF-479 PASS: 0 of 40 concurrent double deletes leave 0 columns (baseline 39 of 40)
DONE: KF-499 FAIL: restart ids and the UI's session_id stop fixed and documented, but the tracker's started_at-only same-second stop still stops B
DONE: KF-ORPHAN-REPEATS PASS: unit test passes, production has 0 orphans, HTTP cannot create one (legacy template apply gap noted)
DONE: KF-406 PASS: deleting the main lane hands the role to the next lane by position
DONE: AGENTCTL PASS: 130/130 checks on the candidate (baseline 129/130, the KF-509 check), KF-408 claim race 100/100 single winner, 300/300 metadata keys kept
