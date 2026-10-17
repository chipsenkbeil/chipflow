## verify-i3 report: agentctl round 4 (fc3c041) against baseline 95b7948

Every runtime check passed on fc3c041 and failed on 95b7948 wherever 95b7948 had the bug. **The test gate is not finished.** I was told to hand back while the gate script was still running: fmt and build pass, clippy was running, and none of the 5 libtest runs or the `--test-threads=1` run had started. So the GATE verdict is "not measured", not PASS. Everything else is measured.

### Environment
- **Fix tree:** `.dev/worktrees/verify-i3`, detached at fc3c041. Release binary md5 b71cf567dea5f4dce7bc5475d4e20c50. Port 3355, DB `/tmp/chipflow-verify-i3-3355.redb`.
- **Baseline:** `.dev/worktrees/verify-i3-base`, detached at 95b7948. Release binary md5 64c65d41102353637167bf82e4e5a562. Port 3356, DB `/tmp/chipflow-verify-i3-3356.redb`.
- **Binaries differ.** `rg -a -c` on fix vs baseline: "cross-site request refused" 3 vs 0, "dropped fields" 2 vs 0.
- **Fixtures and browser:** audit user and golden-master seed on both. Playwright 1.63.0, 1905x1000, DPR 1, a fresh context per scenario.
- **Diff and blinding:** `git diff --stat 95b7948 fc3c041` touches only the expected files (csrf.rs, db.rs, routes.rs, tests, docs, the settings and modal templates, app.js, style.css). I read only commit subjects and no builder notes.
- **Cleanup:** both servers are killed by PID and nothing listens on 3355 or 3356. I edited no tracked files.
- **Evidence:** scripts and logs in `$S/verify-i3/` (with `$S` = the scratchpad), including `t/`, `shots/` and `claimrace.sh`, `lost.sh`, `combo.sh`, `k422.sh`, `hb.sh`, `xboard.sh`. Playwright scripts are `$S/r2/pw/i3-*.mjs`.

### Gate
- `cargo` in this shell is a function that rewrites `cargo test` to `cargo nextest run`. `gates.sh` bypasses it by calling `/Users/senkwich/.cargo/bin/cargo` directly.
- **Measured:** fmt --check exit 0; debug build exit 0. The release build has one `unused import ServeDir` warning; the same line is in 95b7948, so it predates this round.
- **Not finished:** clippy --all-targets, jsc checkSyntax, 5× `TZ=UTC cargo test` and `--test-threads=1`.
- **Still running:** the script runs detached (nohup, PID in `$S/verify-i3/gates.pid`) and writes one line per step to `$S/verify-i3/gates.out` (`clippy=… warnings=…`, `jsc=…`, `test1..5=… passed= failed=`, `test-t1=…`, then `GATES-DONE`). Logs are `g-clippy.log`, `g-test{1..5}.log` and `g-test-t1.log`.
- **Contention:** another agent's cargo test (`calendar_feed_get_should…`) was building at the same time, so CPU was shared.
- Someone has to read gates.out (or re-run `gates.sh`) before this gate can PASS.

### Item 8: device pairing
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| curl with cookie plus forged Origin (127.0.0.1:3999, evil.example, null) | 200 approved (per i2b) | 403 `cross-site request refused` on /device, /device/approve and /device/deny | PASS |
| curl with cookie plus `Sec-Fetch-Site` same-site or cross-site | — | 403 on all three | PASS |
| Browser form POST from 127.0.0.1:3999 (cookie sent, same-site) | approved (per i2b) | 403 on all three; request still pending (poll 428) | PASS |
| Browser no-cors `fetch` with credentials, same-site | — | request still pending | PASS |
| Same-origin approve in a real browser | — | 200 "Approved"; poll 200 with a `cf_` token, scopes read+write | PASS |
| Lockout counts approve and deny | not counted (i2b) | 10 wrong codes alternating approve/deny each give 400; then the RIGHT code gets 429 on all three routes; request still 428 | PASS |
| Lockout is in memory, per server | — | Restart clears it (right code 200 after restart); state lives in `AppState.device_lockout` | PASS |
| Statuses | — | 428 pending; 429 slow_down; 403 denied (real UI deny); 410 at t+637 s; approving an expired code 400; reused device code 400 invalid_grant | PASS |
| Pending cap | — | 18 new starts plus 2 already pending = 20 at 200; the 21st and 22nd get 429 "too many pending device requests" | PASS |
| Revocation | — | Device token works (200), revoke gives 200, then 401; a cross-origin revoke gets 403 | PASS |
| Escaping | — | `<script>…"'&amp;<b>x` client_name shows as text on the confirm and denied pages; 0 injected nodes; 0 console errors, 0 failed requests, 0 dialogs | PASS |

### Item 4 and KF-408: claims and all-or-nothing writes
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| Claim race over HTTP: 4 parallel claims per round on S while M1 and M2 move in and out of S's column and M3 reorders inside it | 100 rounds: round 1's release did not stick, then 99 rounds of all-409 (stuck claim) | **300 rounds:** exactly one 200 per round; fences 1..300 with no repeats and strictly increasing; 0 grants not stored; 0 releases not sticking. Movers confirmed active (M1 and M2 each have 500 `moved` events, the history cap) | PASS |
| Metadata PATCHes racing neighbour moves (300 PATCHes, 8 parallel) | 300×200, 64 lost | 300×200, 0 lost | PASS |
| KF-422 column PATCH | name plus wip -5 gives 400 but saves; bad config_json gives 400 but `is_done` flips to true | Both 400 and nothing changes (export identical; `is_done` stays false) | PASS |
| KF-423 subtask 404 | PATCH and DELETE of an unknown subtask give 404 but write | 404 with the export identical | PASS |
| agents.md exception note | absent | Present at agents.md lines 103–104. Measured: a 409 heartbeat or foreign-fence release on an expired claim appends `claim_expired` and bumps `updated_at` | PASS |

### Item 4b: `updated_at`
- **Heartbeat:** baseline bumps `updated_at`; fix leaves it unchanged. `?since=` returns 0 tasks after 3 heartbeats, and `session_id` updates.
- **Claim, release and expiry:** each bumps it.
- **Reads after expiry:** `claim` is null, with no write and no bump.
- Verdict PASS.

### Item 5: `"fields": null` and carriage returns
- **fields:null:** PATCH gives 400 "fields must be a JSON object" (baseline 200). POST gives 400 and creates no task (baseline 201).
- **OpenAPI:** `UpdateTaskInput.fields` and `CreateTaskInput.fields` are `"type":"object"` (baseline allowed `["object","null"]`). All 123 `$ref`s resolve; 97 operationIds, all unique.
- **Carriage returns:** a text field rejects `\r` and `\r\n` on PATCH, POST and field defaults (baseline accepts `\r`). Multiline still keeps `\r`.
- Verdict PASS.

### Round 3 (built after 95b7948)
- **Cross-board move (Addendum 5):**
  - Kept: `note` (text on both boards), and `agentctl.state`, a system field defined on both.
  - Dropped: `stage:"review"` (not an option on the target), `only_here` (target has no such key), `pts:5` (number into a text field) and `tags:["x","z"]` (z not an option on the target).
  - The move's event reads `Moved from To-do to To-do; dropped fields: {"only_here":"y","pts":5,"stage":"review","tags":["x","z"]}`.
  - In-board moves leave fields alone. A 409 WIP cross-board move changes nothing. Target defaults are not applied on a move.
  - A dropped value of `<img onerror>…</textarea>` is escaped in the history view.
  - Baseline kept everything and recorded nothing.
- **Extras in one transaction:** every rejection of watch, log time, subtask add/edit/delete and attachment upload/delete (14 cases) leaves the export identical. Each success records its event: watched, time_logged, subtask_added/done/deleted, attachment_added/deleted. Code check: each route uses one `apply_task_change` write transaction.
- **Column delete from the UI:**
  - Fix: 4 moves sent with `position:1000000000` all got 200, then DELETE 200. Tasks landed in order (X, T1..T4) and the column is gone, with 0 console errors.
  - The page reload aborts the DELETE response (net::ERR_ABORTED after the 200). It is benign but does show as a failed request.
  - Baseline: 4×400 and "Could not delete column".
- **OpenAPI rustdoc links:** none (baseline had "[`ColorRow`]"); the only hit is the literal `#[utoipa::path]` text.
- Verdict PASS.

### LOW UI items
- **Long kind tag:** 64×W renders 192 px wide with an ellipsis and a full 64-character title; the dialog has no horizontal scroll. Baseline: 638 px, scroll 697/598, title "Comment kind".
- **Options one per line:** `Paris, France` / `Austin, TX` / blank / `Berlin` is stored as 3 options; the edit dialog shows them one per line and saves them unchanged. Baseline split them into `["Paris","France Austin","TX    Berlin"]`.
- **Stale choice (liberty 13, for you to review):** a required select holding a removed option shows "b (not an option)" selected, gets the `tm-field-invalid` class with an amber border, and the card shows "fields incomplete". The multiselect shows "No longer options: y". Choosing a valid option sends one PATCH and clears the mark. Baseline showed "—" and no badge.
- **Field delete:** opens the in-page overlay with the field name and fires 0 `window.confirm`. Cancel keeps the field; OK deletes it. Baseline used `confirm()`.
- Verdict PASS.

### Docs
- The served /agents.md is byte-identical to agent-guide.md.
- The exception note (lines 103–104) and the export-data raw-state note (line 657) are both present.
- README has a `## Roadmap` section with the events-feed sentence.
- skill.md notes heartbeats and the cross-site refusal.
- Verdict PASS.

### Regression
- **Item 1:** POST with JSON gives 201 with every field; `label` filter is AND; a bad `since` is 400; an unknown board is 404.
- **Item 2:** metadata merge and null-delete work; a non-object is 400; null clears.
- **Item 2b:** duplicate key 409; bad key 400; a bad value is a 400 naming the key.
- **Item 3:** link, `blocked` and `ready` work; cycle, self and unknown kind are 400s.
- **Readiness bump:** fires when the blocker moves to Done, when `is_done` is toggled by PATCH, and when the blocker is deleted (its links go too).
- **Item 5:** 409 "(1/1)"; force 200; unknown column 400.
- **Item 6:** actor_kind and token_name are server-filled (a spoof is ignored); the cookie path gives user/null.
- **KF-420 guard on core forms:** browser login 303, settings save 303, logout same-origin 303 and cross-origin 403. Cross-site POST /login is 403, which is login-CSRF protection; check any sibling page that posts a login.
- Verdict PASS.

### Data safety
- I copied a DB written by 95b7948 (md5 3d5c6679…) and opened it with the fix binary. A full dump of every board (export-data, fields, columns, task details, comments, tokens, settings) is byte-identical before and after: 457,282 bytes, 0 diff lines.
- A later PATCH plus move on an old row kept its 236 metadata keys and its claim and fence.

### Break attempts
1. A parallel burst of 40 wrong codes gave exactly 10×400 and 30×429, so the lockout has no race window.
2. Concurrent approve and deny on one code: exactly one decision wins each round.
3. A bogus Bearer header plus cookie plus a forged Origin gets 401, so it cannot bypass the guard.
4. A combined race: claims, S's own cross-column moves, metadata PATCHes and subtask adds together. 120 rounds clean: 802/802 metadata keys, 806/806 subtasks, 694 moves all 200.
5. Metadata PATCHes racing neighbour create/delete at position 0 and swimlane moves: 200/200 kept.
6. An If-Claim-Fence PATCH or comment on an expired claim gives 200, with no stray `claim_expired`.
7. Dropped-field values with HTML in the history view are escaped.
8. An option containing a newline: broke the UI, see observation 1.

### Observations (candidate defects)
1. **LOW: the API accepts an option containing `\n`, and the settings editor then splits it.** `POST /api/boards/{id}/fields` with options `["one\ntwo","three"]` gives 200. A label-only edit in Settings → Custom fields then saves `["one","two","three"]` (PATCH captured by `$S/r2/pw/i3-nlopt.mjs`). Tasks holding "one\ntwo" become stale. Fix direction: reject newlines in options, or keep options the user didn't touch.
2. **LOW: losing a duplicate submit counts as a wrong code.** The losing approve/deny racers (and any double-click or resubmit on an already-decided code) count toward the 10-code lockout. Two double-submitted rounds locked the page for 10 minutes.
3. **LOW: a lease renewal bumps `updated_at`.** A same-claimant renewal through `POST /claim` with a matching fence changes `updated_at`, although it only extends the lease, so a renewal can bring a task back into `?since=` results. The docs say only that heartbeats leave `updated_at` alone.
4. **LOW:** text fields accept U+2028 and U+0085 line separators; only `\n` and `\r` are rejected.
5. **Noise, not regressions:**
   - The column-delete and field-delete reloads abort the 200 DELETE response (ERR_ABORTED).
   - The /login page logs two 401 console errors before login (timer polling).
   - The cross-board move event reads "Moved from To-do to To-do" with no board names.

DONE: GATE FAIL not finished: fmt and debug build pass, but clippy, jsc, the 5 plain-libtest runs and the --test-threads=1 run had not run at handback; results land in $S/verify-i3/gates.out (cargo test is a nextest alias, bypassed by calling ~/.cargo/bin/cargo)
DONE: ITEM-8 PASS forged Origin, same-site/cross-site Sec-Fetch-Site and a real browser form from 127.0.0.1:3999 all get 403 on /device, /device/approve and /device/deny with the request still pending; same-origin approve 200 with a token; lockout counts and enforces on approve and deny (10 then 429, even for the right code; 40-way burst gives exactly 10), in memory per server; 428/429/403/410/400 invalid_grant, single use, 20-pending cap, revocation and escaping all hold
DONE: ITEM-4 PASS 300-round neighbour-move claim race clean (fences 1..300, no repeats, every grant stored, every release sticks; baseline stuck after round 1); heartbeat leaves updated_at alone while claim, release and expiry bump it; the documented 409 expired-claim exception is measured and stated in agents.md
DONE: KF-408 PASS 300 metadata PATCHes racing neighbour moves lose 0 (baseline 64); column PATCH all-or-nothing (KF-422: baseline renamed and flipped is_done); subtask 404 writes nothing (KF-423); fields:null gives 400 on PATCH and POST with OpenAPI object-only; text fields reject \r
DONE: ROUND-3 PASS cross-board move keeps only valid target-defined values and lists the dropped ones in the move event (escaped); 14 extras rejections leave nothing and successes record their events; UI column delete sends valid positions and works; no rustdoc links in OpenAPI
DONE: LOW-UI PASS kind tag 192 px with ellipsis and full title (baseline 638 px overflow); options one per line with commas kept; stale choice shown "(not an option)", marked invalid and counted incomplete (liberty 13); field delete uses the in-page confirm with 0 window.confirm; LOW new: an API option containing \n is split on a settings edit
DONE: DOCS PASS agents.md exception note and export-data raw note served; README Roadmap with the events-feed sentence
DONE: REGRESSION PASS items 1, 2, 2b, 3, 5, 6 and the readiness bump (Done move, is_done toggle, blocker delete) all hold; a 95b7948 DB loads under the fix with a byte-identical full dump
