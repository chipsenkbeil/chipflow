All four defects pass, and the regression and gate checks are clean.

**Environment**
- Candidate: worktree `.dev/worktrees/verify-x4`, detached at eeeefa6.
- Baseline: worktree `.dev/worktrees/verify-x4-base`, detached at b9f846a.
- Both are release builds, and the diff touches only agent-guide.md, agents/skill.md, src/db.rs and src/routes.rs.
- Binary md5: candidate a76c24168bd5f620cf41e4e341f2940d, baseline f1067a8188337991e3dce6451dcafe4d.
- `rg -a -c 'session_id'` gives 15 on the candidate and 9 on the baseline. `rg -a -c 'Board not found'` gives 6 and 4.
- Ports: candidate 3412, baseline 3413, with fresh databases and a seeded golden-master board. I killed only my own PIDs (1631 and 1635, plus the restarted candidate), and both are gone.
- Evidence is under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-x4/`. The files are ev-499-a/b/c.txt, ev-489.txt, ev-435.txt, ev-sweep.txt, ev-agentctl.txt, ev-ui.txt, ev-gate1.txt, ev-clippy.txt, ev-test-utc.txt and ev-test-envi.txt. The UI screenshots are ui-3412.png and ui-3413.png (the latter from an earlier script run), and the scripts are t499.sh, t435.sh and ui.mjs.

**KF-499**

| Claim | Baseline (b9f846a) | Candidate (eeeefa6) |
|---|---|---|
| After 40 starts, remaining <= total and started_at <= now | remaining 1537 > total 1500; started_at 37 s ahead of now | remaining 1500 = total 1500; started_at equals now |
| Stopwatch started right after the burst, stopped 22 s later | `discarded:true` (the bug) | `discarded:false`, minutes 0 |
| Two starts in the same second | Not tested | Same started_at 1792256718, session_id 42 and 43 |
| Stop naming the first session_id | Not tested | `already_stopped:true`; status still shows session_id 43 running |
| Stop with only started_at | Not tested | `already_stopped:false` |
| Idle status | Not tested | `active:false`, `session_id:null` |
| Stop naming a session_id this process never issued (9999, while 43 was running) | Not tested | `already_stopped:true` |
| Stop after a server restart | Not tested | Status shows `session_id:null`; stop with stale id 7 and the right started_at works, and stale id 1 with a wrong started_at gives `already_stopped:true` |

- **Stale-id behaviour:** a stale session_id against a running server returns `already_stopped:true`. That is consistent with the documented fallback, which applies only after a restart, when the process holds no id for the running session.
- **UI, two tabs:** start, then tab B reloads and shows 24:58 on the pill, B stops, A polls to idle, A restarts as session 6, B sees the pill (24:54), and A stops. It also covered short_break (300/300) and long_break (900/900) starts via the API. There were 0 console errors and 0 failed requests.
- **Docs and OpenAPI:** agent-guide.md and agents/skill.md describe session_id, its in-memory nature, and the started_at fallback after a restart. The OpenAPI spec has `session_id` on TimerStatusView and on TimerStopInput, with the restart note.
- **Break-attempts:** a stop with a session_id and a wrong started_at gives `already_stopped:true` and leaves the session running. A stop with a session_id while the timer is idle gives `already_stopped:true`. A plain stop while idle is a 404 "no active timer".

**KF-489**

| Claim | Baseline | Candidate |
|---|---|---|
| `GET /api/tasks?board_id=` (empty) | 404 "board not found" | 200, 25 tasks, same as unfiltered |
| `?board_id=nope` | 404 "board not found" | 404 body exactly "Board not found" |
| A real board id | 3 tasks | 3 tasks (unchanged) |

**KF-435** (PATCH followed by history-view; evidence in ev-435.txt)

| Case | Baseline | Candidate |
|---|---|---|
| Due date and target column in one PATCH | Only "Set the due date to Nov 20, 2026 5:00 PM" | That line and "Set the due date target column to Done" |
| Clearing the due date while a target is set | "Cleared the due date" only | "Cleared the due date" only (the target is also cleared, with no extra line) |
| Target changed alone | "Cleared the due date target column" | Same |
| Date changed alone | Date line only | Date line only |

**KF-433-DOCS:** the diff and the worktree files contain the new line in both agent-guide.md and agents/skill.md ("clearing due_at clears it too, and so does deleting the target column or moving the task to another board"; "The target is cleared when its column is deleted or the task moves to another board").

**Regression**
- **GET/HEAD sweep:** I covered the 46 GET paths from the OpenAPI spec (placeholders filled where I could, parameterized ones skipped) plus 3 page routes, as GET and as HEAD, with the cookie and cross-site headers (Origin evil.example, Sec-Fetch-Site cross-site). That is 98 GET and 49 HEAD requests, with no 5xx responses. The database md5 stayed 83542b8a771de57f0ccc22b0496fa0ec and the size stayed 348160 before and after.
- **Schema:** SCHEMA_VERSION is still 2 (src/db.rs:251), and the diff touches no table definition or fingerprint. The schema_version tests pass.
- **agentctl smoke:**
  - Metadata merge and key deletion work.
  - Claim, heartbeat and release return 200, and the `claimed=true` filter goes 1 then 0.
  - The `label+since` filter returns 7 tasks and `blocked=false` returns 22.
  - A claim by another claimant without ttl_seconds returned 400 "ttl_seconds is required". That was my malformed request, not a defect.

**DONE: GATE**
- `cargo fmt --check` exits 0, `clippy --all-targets` reports 0 warnings, and jsc `checkSyntax` on `static/app.js` exits 0.
- `TZ=UTC cargo test` passes: the lib has 219 passed and 1 ignored, schema_version_tests has 26, and every other suite has 0 failures.
- The `env -i` plain libtest run gives identical numbers.

**Observations (candidate defects, with a one-line acceptance check each)**
1. `?board_id=%20` (whitespace only) returns 404 "Board not found", not "no filter". This may be intended. Acceptance check: either document it, or have `GET /api/tasks?board_id=%20` return 200 with the full list.
2. `timer_start` calls `set_active_timer` and then `begin_session`, and `timer_retarget` writes the timer without touching the session. Two near-simultaneous starts could pair a stored timer with the other request's session id. I could not provoke it, since my two starts were sequential. Acceptance check: a test that fires two concurrent starts and asserts that status session_id and started_at belong to the same request.
3. The release build prints one pre-existing compiler warning, "chipflow (bin) generated 1 warning", on both the baseline and the candidate. It does not appear under clippy. Acceptance check: `cargo build --release` shows 0 warnings.

DONE: KF-499 PASS (baseline reproduced: remaining 1537 > total 1500 and a 22 s stopwatch discarded; candidate 1500/1500, `discarded:false`, session_id semantics all as claimed)
DONE: KF-489 PASS (empty board_id returns all 25 tasks, unknown board returns 404 "Board not found"; baseline returned 404 "board not found" for both)
DONE: KF-435 PASS (both history lines on a combined PATCH; clearing the due date logs only "Cleared the due date")
DONE: KF-433-DOCS PASS (the line is present in agent-guide.md and agents/skill.md)
DONE: REGRESSION PASS (no writes from 98 GET + 49 HEAD requests with cross-site headers, SCHEMA_VERSION 2, agentctl smoke OK, timer UI two-tab with 0 console errors)
DONE: GATE PASS
