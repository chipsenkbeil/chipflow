verify-36a report: batch-36 release candidate, API/data/security half

I found no batch-36 defect. Every requested check passed except one gate: the Linux root test run fails one test. That test fails the same way on production a980a7c, so batch-36 did not cause it. The release call is yours.

**Process note (off-assignment port):** once, by mistake, I started the candidate on port 3388 (my ports are 3386/3387). The port was taken by another agent's server, so my process upgraded my own scratch file v1.redb to v2 and then exited with AddrInUse. I never sent it a request. I restored the file from a pristine copy before the real upgrade run.

**Environment**
- Candidate: worktree `.dev/worktrees/verify-36a` at 2c1773b, clean before and after. Debug binary md5 2d21f54e…; it contains "claim fence mismatch or expired" and "task_links_by_target".
- Baseline: worktree `.dev/worktrees/verify-36a-base` at a980a7c. md5 bebfe7d1…; neither string is in it.
- Ports 3386 (candidate) and 3387 (baseline). Every database lives under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-36a/`.
- Playwright 1.63 runs at 1905x1000, DPR 1. Fake clocks came from libfaketime launched directly; it is stripped when started through nohup or env.
- All my servers and containers are stopped; ports 3386 and 3387 are free.

**1. Gates**
- fmt 0; clippy --all-targets 0 warnings; jsc checkSyntax 0; `tests/js/cumulative_flow.js` 6/6.
- Tests under TZ=UTC, America/Chicago and Pacific/Chatham, and the 13 test binaries under `env -i`: 279 passed, 0 failed, 1 ignored each time. The ignored test is the same one ignored on the baseline.
- Linux rust:1.98.0, offline, no zoneinfo, no HOME:
  - uid 1000: 279 passed.
  - root: `db::schema_version_tests::failed_crash_repair_is_reported_as_such` fails (src/db.rs:5356) in both the lib and bin targets. Everything else passes.
  - Why: the test marks the file read-only, but root ignores that, so the repair succeeds and the test's expected error never comes. The same test fails as root on a980a7c (db.rs:3801) and passes as uid 1000 (`linux-root-base.out`).

**2. Schema v2**
- **Upgrade, no data lost:** a980a7c wrote a golden-master DB with comments, subtasks, time entries, a date-only due, a repeat, due_done, a renamed column, legacy column config, an attachment, a watch and a running pomodoro, then stopped on SIGTERM. The candidate logged "upgraded … from schema version 1 to 2".
- **Old keys:** a full API dump of 97 endpoints, compared on old keys only, has 0 differences. The timer was still running after the upgrade.
- **New fields** all start empty or null: metadata, links, fields, claim, claim_fence, due_target_column_id, former_names, and comment actor_kind/token_name/kind. Old rows report updated_at = created_at.
- **Refusal:** a980a7c refuses the v2 file (exit 1, the refusal message) and leaves it byte-identical (cmp). Same result on a v2 file full of agentctl data.
- **Crash repair:** a v1 file left by SIGKILL is repaired, then upgraded, and the write made just before the kill is kept.
- **Doc comment vs diff:** I diffed the fingerprint listings of both trees. The SCHEMA_VERSION v2 list (items 1–7) matches the stored changes exactly, with nothing extra. The opaque config values (board due_time, column column_sum/sorting) I confirmed in code.
- **Mutations:** the fingerprint test fails on SCHEMA_VERSION reverted to 1, on renaming the `field_defs` table, and on a serde rename of `former_names`. Restored; worktree clean afterwards.

**3. agentctl API**
- **Probe:** 242/242 checks pass (`probe-3386.out`). It covers:
  - Items 1–6, 2b and 8.
  - The readiness bump: blocker entering or leaving Done, the is_done toggle, link removal, blocker delete, cross-board blockers.
  - Cross-board moves: dropped fields are written into the "moved" event.
  - Claims: fences, renewal and heartbeat touch only the lease, lazy expiry, "reads never write", the documented 409 exception, claim_expired logged exactly once.
  - The four LOW fixes.
- **Device pairing, extra checks:** 410 on expiry, cap of 20 pending, client_name limited to 80 characters, Bearer gets 403 on all device pages. The lockout counts unknown and expired codes but not decided ones, and resets on restart.

**Concurrency (KF-408)**
- 100-round claim race (6 claimants per round, neighbours moving between two columns): exactly one 200 every round, fences 1..100 strictly increasing, 0 lost releases.
- 300 metadata PATCHes racing 600 neighbour moves: all 300 keys present, 0 lost.

**4. Security (KF-420..423)**
- There is one guard layer: `src/csrf.rs` is unchanged and nothing else checks Origin or Sec-Fetch-Site.
- GET and HEAD on all 66 GET routes, cookie plus cross-site plus foreign Origin: 132 requests, 0 writes (DB md5).
- All 60 write routes, each with a valid body, under cross-site, same-site, foreign Origin and Origin null: 240/240 gave 403 "cross-site request refused", 0 writes.
- Same-origin writes work; so does `Sec-Fetch-Site: none`.
- Bearer works cross-site, but minting a token with Bearer is 403 and no token is created.
- Bogus Bearer plus a valid cookie: 401 on GET, PATCH and approve, no write.
- **Evidence caveat:** every Bearer request rewrites the token's last_used_at (that predates batch-36). So the no-write checks used the cookie; a Bearer request changes the DB md5 even when nothing else changed.
- KF-421: 50 rounds with 0 lost PATCHes, 0 misplaced moves, 0 duplicate positions.
- KF-422: invalid column PATCHes are 400 with no write; a valid is_done toggle unblocks the dependent and bumps its updated_at.
- KF-423: four kinds of subtask 404 write nothing.

**5. Due dates (stream D)**

| Defect | Baseline | Candidate |
|---|---|---|
| KF-401 | no Delete button | Delete shows only when a due exists; it PATCHes `{due_at:null,due_repeat:null,due_done:false}` on the open task only, and the target column clears too |
| KF-404 | date-only due stored at 23:59:59 | 17:00 by default, or the board's due_time (08:30 tested); dialog pre-fills it; bad formats 400; a DST-gap time 400 with a clear message |
| KF-259 | — | target column stored; modal and card read "Wednesday 10:00 AM (Done)" and follow a column rename; another board's column is 400; "" or clearing due_at clears it |
| KF-389 | "—" | "0h" for From = To in both dialogs; reversed times still "—" |
| KF-417 | empty time saved at 09:00 | "Invalid date or time" alert, 0 PATCHes, dialog stays open |
| KF-398 | — | 27 due_at inputs give identical 200/400 results on both; only the time of a date-only due differs |

**6. Timer (stream T)**
- KF-453: a stop with a stale started_at returns 200 `already_stopped:true` and the newer session keeps running. A stop without started_at on an idle timer is 404 (baseline: the stale stop killed the running session).
- KF-451, timers left 48 h: pomodoro logs 85 min and stopwatch 720 min (baseline: 2880 both). A 70-minute pomodoro still logs 70.
- KF-464: a short break's badge is "B Short break" (baseline: "M Manually added"); manual entries keep "M".
- KF-430: unchanged; completed:true after 2 minutes still logs completed:false.

**7. History (stream H)**
- "created" and "moved" events carry column_id; a rename records former_names ["Temp"].
- Live multi-day run (fake clock, then the column deleted): one band, Temp2, with no phantom "Temp". The page shows no console errors and no failed requests.

**8. Docs**
- Served /agents.md and /agents/skill.md are byte-identical to the tree; /.well-known/agents.json matches agents.json.
- OpenAPI: 91 refs, 0 unresolved; 98 operationIds, all unique; every /api route is in the spec.
- The new fields and the 10 MiB attachment cap (9.9 MiB upload 200, 10.1 MiB 413) are described consistently across the guide, skill.md and OpenAPI.

**Candidate defects (observations)**
1. **Root-only test failure, predates batch-36** (on both 2c1773b and a980a7c). Acceptance: `cargo test --lib failed_crash_repair_is_reported_as_such` exits 0 as root in rust:1.98.0.
2. **LOW:** deleting a task's due target column leaves a dangling `due_target_column_id`. Acceptance: after `DELETE /api/columns/{target}`, `GET /api/tasks/{id}` shows `due_target_column_id: null`, or the delete is refused.
3. **LOW:** PATCH `{due_target_column_id}` on a task with no due_at returns 200 and stores nothing. Acceptance: a 400, or a docs line saying it is ignored.
4. **LOW:** `PATCH /api/fields/{id} {"kind":"text"}` returns 200 and silently ignores the change of a field the docs call fixed. Acceptance: a 400 naming kind, or the docs say it is ignored.
5. **LOW docs:** skill.md's TaskDetail key list omits `due_target_column_id`, and it says claim_expired is logged by "the first claim call", where the guide says claim, heartbeat or release. Acceptance: skill.md lists the key and uses the guide's wording.
6. **LOW:** the server upgrades the stamp before binding its port. A start that fails with AddrInUse leaves a v1 file at v2, which the old binary then refuses. Acceptance: starting on a busy port leaves a v1 file stamped 1.
7. **Note for verify-36b:** upgraded columns with no column_sum key now show no header count badge; a980a7c always showed the count. A legacy `column_sum:true` still shows the count.

DONE: BATCH-36-A PASS no batch-36 regression found across 242 probe checks, races, 372 security requests, schema v2, due/timer/history/docs; one root-only test failure that a980a7c shares
DONE: GATE FAIL root-only: failed_crash_repair_is_reported_as_such fails as root (pre-existing, same on a980a7c); fmt/clippy 0/jsc/cfd/3 TZ/env -i/Linux uid 1000 all PASS
DONE: SCHEMA-V2 PASS 97-endpoint dump 0 old-key diffs, new fields empty by default, refusal byte-identical, SIGKILL repaired then upgraded, doc list matches stored diff exactly, 3 mutations caught
DONE: AGENTCTL PASS 242/242 checks (items 1-6, 2b, 8, bump, cross-board, claims/fences/expiry/409 exception, WIP, comments, pairing, 4 LOW fixes)
DONE: CONCURRENCY PASS 100/100 rounds exactly one 200, fences 1..100 increasing, 0/300 metadata PATCHes lost
DONE: SECURITY PASS 132 GET/HEAD 0 writes; 240/240 cross-origin writes 403 with 0 writes; same-origin, Bearer, bogus-Bearer correct; KF-421/422/423 hold
DONE: DUE-DATES PASS KF-401/404/259/389/417 fixed vs baseline; KF-398 parsing identical
DONE: TIMER-SERVER PASS stale stop 200 already_stopped, idle 404, caps 85/720 (baseline 2880), break badge B, KF-430 unchanged
DONE: HISTORY PASS column_id on events, former_names recorded, renamed-then-deleted column is one band (JS test + live)
DONE: DOCS PASS served docs equal the tree, all OpenAPI refs resolve, 98 unique operationIds, no contradictions (LOW skill.md gaps noted)

Everything is in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-36a/`. The main files:
- gates: `gates.out`, `linux.out`, `linux-root-base.out`
- probe and races: `probe-3386.out`, `race4-3386.out`
- schema: `schema/dump-oldkeys.diff`, `schema/newkeys.txt`
- fingerprint: `fp/fp.out`, `fp/cand-listing.txt`, `fp/base-listing.txt`
- security: `sec/getprobe.tsv`, `sec/writeprobe.tsv`, `sec/kf421.out`
- due dates: `due/dd-3386.json`, `due/dd-3387.json`, `due/parse-3386.tsv`, `due/parse-3387.tsv`
- history: `hist/export-days.json`, `hist/cfd-days.png`
- docs: `docs/openapi.json`

