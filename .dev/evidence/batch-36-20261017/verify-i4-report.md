verify-i4 report: blind check of feat/agentctl-api-v2 at 820dcff (on origin/main 7817bce). All six checks pass, and I found no candidate defects.

**Environment**
- Worktrees (detached, clean afterwards): `.dev/worktrees/verify-i4` at 820dcff, `verify-i4-base` at 7817bce, `verify-i4-prev` at fc3c041.
- Release builds, md5:
  - candidate: 7365b179a627ad8727fd7aede9bbc2b3
  - base: 422494284998866fd2d84ff6dac11ca7
  - prev: 60bbaa40a8ad15d8be62231b27f7a8d9
- Only the candidate binary contains "options must be single lines"; base contains no `device_auths`.
- Ports 3355, 3356 and 3357. Every DB is under `$S/verify-i4/` (`*.redb`). Server TZ is America/Chicago. UI ran at 1905x1000, DPR 1, Playwright 1.63 (`$S/r2/pw/vi4-ui.mjs`).
- Every server was killed by PID. One stray base instance I started by mistake on 3357 had a DB under `$S` and a 20 s timeout; it was killed. Nothing is listening on 3355–3357.

`$S` = `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`

**1. Rebase fidelity**
- `git range-diff ffdce86..fc3c041 7817bce..eb322a7` (`$S/verify-i4/range-diff.txt`): 28 commits pair up. Every `!` pair differs only in import lines, hunk context, or the `AppState::new` call in `serve()`.
- I ran a probe of 189 checks (`$S/verify-i4/probe.sh`) on fresh servers for the candidate and fc3c041, normalised ids and timestamps, and diffed the output (`probe.diff`). The only differences:
  - `RENEW_BUMPS` changed from yes to no (the intended LOW fix c).
  - The port inside `verification_url`.
- The probe covers:
  - **Task detail and list:** 201/HTML create and PATCH; filters for column, lane, label AND, `since`, `ready` and `claimed`, with their 400/404/401.
  - **Metadata:** merge, null key, last key gives null, `metadata: null`; body over 64 KiB and merged object over 64 KiB both 400 with nothing applied.
  - **Fields:** key regex including 64 and 65 characters, every kind, 409 on duplicate key, defaults only on new tasks, `fields: null` is 400 on both create and PATCH, each 400 names the key, a partial PATCH changes nothing, deleting a field deletes its values.
  - **Cross-board move:** only valid target-defined values are kept, and the move event records the dropped ones, e.g. `dropped fields: {"stage":"review"}`.
  - **Links and readiness:** cycle, self and 404 errors; `blocks` is stored as the inverse; `blocked` and the `updated_at` bump fire on a Done move, on an `is_done` PATCH in both directions, on link delete and on blocker delete; a cross-board link carries `board_id` and the card shows "(Other)".
  - **Claims:** 400s, 409 text, fences, lease-only heartbeat, idempotent release, `If-Claim-Fence` on PATCH, move and comment, lazy expiry, the documented 409 heartbeat exception, history sequence.
  - **WIP:** 409 (2/1) leaves the task where it was; reorder and same-column lane moves are 200; `?force=true` works.
  - **Comments:** `actor_kind`, `token_name` and `kind` for token and cookie writes; blank kind becomes null; 65 characters is 400.
  - **Device pairing:** 400s, 428, 429, approve then token exchange, spent code gives `invalid_grant`, a read-scope token gets 403 on writes, revocation gives 401, deny gives 403, the 20-pending cap gives 429, lockout gives 429.
  - **KF-420 guard:** approve and deny with a Bearer token, `Origin: evil`, `Sec-Fetch-Site: cross-site` or `same-site`, or a foreign port all get 403.
  - **Docs:** `/.well-known/agents.json` is 200; `/agents.json` is 404; `download_url` appears 0 times in agents.md and OpenAPI.
- Differences in OpenAPI and docs between fc3c041 and the candidate are main's due and timer text plus the LOW-fix docs, nothing else.

**2. LOW fixes** (`low.sh`; `low-cand.txt` vs `low-prev.txt`)
- **(a) Option line breaks.** Options containing `\n`, `\r`, `\r\n`, U+0085, U+2028 or U+2029, for both select and multiselect, now get 400 `field "<key>": options must be single lines` on create and on PATCH. A rejected PATCH leaves the field unchanged, label included. fc3c041 returned 200 for all 22 cases.
  - A raw (unescaped) NEL byte is refused too.
  - In the settings UI, typing U+2028 into the options textarea shows the error in the page and saves nothing.
- **(b) Lockout counting**, raced for real (`devrace.sh`, `devrace-results.txt`):
  - Candidate: 40 rounds, each with 4 approves and 4 denies on one code, then two repeat submits. Every round had exactly one 200, and the 360 losing or repeat submits never locked the page. After that, 9 unknown codes still left a live lookup at 200; the 10th locked lookup and approve with 429.
  - fc3c041: locked from round 2.
  - Expired codes (both a pending one and an approved one, `expiry-lockout.txt`): 9 submits leave the page at 200 and the 10th locks it with 429. Twelve resubmits of an already exchanged code are not counted.
- **(c) Lease-only renewal.** A same-claimant renewal with the live fence keeps the fence and `started_at` and sets a new `heartbeat_at` and `expires_at`. The stored and raw `updated_at` are unchanged, no history event is added, and `?since=` (with or without `claimed=true`) returns nothing. Optional fields (`session_id`, `model`) are stored. fc3c041 bumped `updated_at` and the task appeared in `?since=`.
  - A renewal against an expired claim becomes a new claim: fence +1, a bump, then `claim_expired` and `claimed`.
  - agents.md and skill.md both describe the renewal as lease-only.
- **(d) Text fields.** Text values with `\n`, `\r`, U+0085, U+2028 or U+2029 get 400 on create, on PATCH and as a field default; a raw U+2028 byte is refused too. Multiline fields still accept all five. Tab and U+2027 are still accepted. fc3c041 accepted U+0085, U+2028 and U+2029.
- **Renewal under races** (`renewrace-cand.txt`):
  - On an expired lease, the old holder's renewals raced a rival's claims for 40 rounds: exactly one 200 each round and fences strictly increasing.
  - 100 renewals raced 100 metadata PATCHes: 100/100 keys kept and the claim kept.

**3. Schema v2**
- Seeded a DB with the 7817bce binary: golden master, a token comment and a cookie comment, a binary attachment, a manual time entry, a subtask, a due date, labels and a running pomodoro. Stopped it with SIGTERM ("shut down cleanly").
- The candidate logged "upgraded … from schema version 1 to 2".
- Full API dump diff (`schema-dump.diff`):
  - 0 lines removed or changed.
  - The only `<` lines are 8 comment `"id"` lines that gained a trailing comma.
  - Every `>` line is a new default (`updated_at`, `metadata`, `links`, `fields`, `claim`, `claim_fence`, `blocked`, `actor_kind`, `token_name`, `kind`) or an item-1 field on the detail view.
  - The running timer and the attachment md5 are preserved.
- The 7817bce binary refuses the upgraded file with exit 1 and the message "has schema version 2 … supports up to schema version 1". The file stays byte-identical (md5 5fd3eb57… before and after).
- Crash-left v1 file (base killed with -9): the candidate logged "was not closed cleanly; redb repaired it" and then the upgrade. The dump diff is again additions only. The base binary refuses that file too and it stays byte-identical.
- The `SCHEMA_VERSION` doc comment's v2 list matches the diff 7817bce→820dcff exactly: 6 TaskRow fields, 3 TaskComment fields, `field_defs`/FieldDefRow, `fields_by_board`, `task_links_by_target`, `device_auths`/DeviceAuthRow. The auth.rs `token_name` is not stored. The v1 fingerprint is unchanged.
- Fingerprint test, mutated in my worktree and then restored (0 dirty):
  - Reverting to version 1: the test fails and reports exactly the committed v2 hash c452003d…
  - Renaming the `field_defs` table: the test fails.

**4. KF-408 concurrency**
- Claim race (`claimrace-cand.txt`): 150 rounds × 8 claimants while three neighbours moved and reordered (M1 and M2 each logged 500 history moves). Exactly one 200 every round, fences 1..150 with no duplicates or reversals, every grant, renewal and heartbeat stored, every release stuck.
- Metadata race (`mdrace-cand.txt`): 300 parallel PATCHes during neighbour create/delete and lane and column moves. 300 returned 200, 300 keys stored, 0 lost.
- Harness sensitivity, run against the pre-KF-421 binary 95b7948: 69 of 300 lost, and 27 of 30 claim rounds without exactly one 200.

**5. Main's features**
- Due-date battery of 22 inputs plus a same-instant resend and a bad due date on create: identical between base and candidate (`due-*.txt`), including DST spring-forward gives 400, fall-back takes the earlier instant, second 60 gives 400, whitespace clears.
- Graceful shutdown (`shutdown.sh`):
  - SIGTERM during a 4 MB upload: exit 0 in 4 s, upload 200 and committed.
  - An upload slower than 10 s: exit 0 at 10 s with "requests still open after 10s; dropping them".
  - New connections are refused after SIGTERM.
- Timer smoke: stopwatch start, 21 s status, stop, entry logged; the timer log and statistics pages return 200.
- UI: the golden-master board and a card modal have 0 console errors, 0 failed requests and 0 responses ≥400. The candidate's screenshots are pixel-identical to the base's (`magick compare` AE 0 for board and modal).
- Settings custom fields page: add with options one per line (blank lines dropped), required flag, edit, in-page delete confirmation, and the API state matches each step.

**Gates** (`gates.out`, tree clean before and after)
- fmt 0; build 0 with 0 warnings; clippy `--all-targets` 0 with 0 warnings; jsc `checkSyntax` 0.
- `TZ=UTC` plain libtest ×3: 253 passed, 0 failed, 1 ignored each time (the ignored test is main's DST helper).
- `--test-threads=1`: 253/0.
- `env -i TZ=UTC`: 253/0. This run also passed `HOME` and `PATH=/usr/bin:/bin`, because cargo cannot start without them.

**Break attempts I added:** expired leases raced against the old holder's renewals; renewals raced against metadata writes; raw unescaped separators; option breaks that trimming would remove (accepted, stored trimmed); U+2028 typed into the UI textarea; resubmitting an exchanged code; a crash-left v1 file; a drain longer than 10 s.

**Observations** (none caused by this candidate)
- Settings field delete: the browser reports `DELETE /api/fields/<id> net::ERR_ABORTED` although the server returns 200, because the page reloads before the response is read. fc3c041 does the same (`ui-prev.json`), so it predates the rebase. Cosmetic only.
- Release builds warn about an unused `tower_http::services::ServeDir` import (`src/routes.rs:26`). Main has the same warning.
- Code smell: in `claim_task`, the `if expired { Stamp }` inside the held-claim branch looks unreachable, since `expire_claim` already cleared any expired claim. Harmless.

DONE: REBASE PASS (only differences from fc3c041 are the intended renewal no-bump and main's due/timer docs; range-diff is context-only)
DONE: LOW-FIXES PASS (a) 22/22 option breaks 400 naming the field on create and PATCH (b) 360 race losers and repeats not counted, unknown and expired codes lock at 10 (c) renewal lease-only, `?since=` unaffected, docs updated (d) text refuses all five breaks, multiline accepts them
DONE: SCHEMA-V2 PASS (v1 file stamped 2, dump additions only, 7817bce refuses it byte-identical, crash-left v1 repaired then upgraded, fingerprint mutations fail as they should, v2 list exact)
DONE: CONCURRENCY PASS (150 claim rounds with neighbour moves: one 200 each, fences 1..150 strictly increasing; 300 metadata PATCHes lost 0; harness catches 69 lost on 95b7948)
DONE: MAIN-FEATURES PASS (due battery identical to 7817bce, SIGTERM exit 0 with a 10 s drain, timer smoke OK, board UI 0 console errors and pixel-identical, custom fields settings page works)
DONE: GATE PASS (fmt, clippy 0 warnings, 3×253 TZ=UTC, test-threads=1 and env -i runs 253/0, jsc 0)
