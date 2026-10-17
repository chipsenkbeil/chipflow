## verify-s3 report: graceful shutdown and schema version (feat/schema-version, 039658c)

Both items pass. The drain deadline does what it should: every stalled case exits 0 about 10.05–10.22 s after the signal, the file is left cleanly closed, and in-flight uploads arrive intact. d7b861b still hangs under the same load. The schema guard holds at binary level, and the new fingerprint fails on every kind of change I made except a pure field reorder, which is harmless for JSON.

### Environment
- **Tree under test:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-s3`, detached at 039658c. The diff against d7b861b touches only the expected files: `.dev/fidelity-status.md`, `AGENTS.md`, `src/db.rs`, `src/main.rs`, `tests/graceful_shutdown.rs`.
- **Comparison tree:** `.dev/worktrees/verify-s3-prev`, at d7b861b.
- **Mutation worktree:** `.dev/worktrees/verify-s3-mut`, at 039658c. I added it so the mutation edits would not disturb the gate runs. It has been reverted and is clean.
- **Binaries:** debug builds, copied to the scratchpad folder `vs3/`.
  - Fix: md5 `84750c9564c4171cc715b5aef16c1678`.
  - Comparison: md5 `52a76c3f6cd680f98e037f17a1a210a9`.
  - The string "requests still open after" appears once in the fix binary and 0 times in the comparison binary.
- **Ports and databases:**
  - 3362 (fix) used `/tmp/chipflow-verify-s3-3362.redb`.
  - 3363 (comparison) used `/tmp/chipflow-verify-s3-3363.redb`.
  - Copies for the schema cases went under the scratchpad.
- **Checker:** a standalone redb 4.3.0 checker, `vs3/rocheck`. It reports whether `ReadOnlyDatabase::open` succeeds or returns RepairAborted, and can also stamp, crash-stamp, unstamp and list table lengths.
- **Hygiene:** the app was never run without `--database-path`. Nothing was written to `~/Library/Application Support/chipflow` (its timestamps are still 15:36, from the earlier verifier). No processes of mine are left running.

### Gates (039658c)

| Gate | Result |
|---|---|
| fmt --check | exit 0 |
| build | exit 0 |
| clippy --all-targets | exit 0, 0 warnings |
| `TZ=UTC cargo test` | 145 passed, 0 failed, 13 binaries (d7b861b: 142 passed, 0 failed) |
| jsc checkSyntax `static/app.js` | exit 0 |
| Empty environment (`env -i HOME PATH`, TZ unset, `--no-fail-fast`) | 143 passed, 2 failed |

- **Empty environment:** the 2 failures are exactly `format_due_card_weekday_near_absolute_far` and `format_due_modal_relative_and_done_marker`.
- **Same failures on d7b861b:** 140 passed and the same 2 failed.
- **Identical messages:** the failure texts ("got Sunday 12:00 PM" and "got Saturday 12:00 PM") diff as identical between the two trees.
- **Gate caveat:** plain `cargo test` stops at the first failing target, so the "only failures" claim needs `--no-fail-fast`.

### 1. Shutdown drain
Each "full" run opened four connections, then sent the signal 1.5 s after the upload started:
- (a) a POST /login that declares Content-Length 1000 and sends 7 bytes;
- (b) partial headers;
- (c) an idle keep-alive;
- (d) a 540,000-byte attachment upload (720,066-byte JSON body) throttled to 150 KB/s.

| Run | Signal | Exit code | Exit after signal | Upload (d) | DB after exit |
|---|---|---|---|---|---|
| term1–5 | TERM | 0 ×5 | 10.048 / 10.060 / 10.099 / 10.045 / 10.047 s | 200, 4.69–4.70 s | clean ×5 |
| int1–3 | INT | 0 ×3 | 10.177 / 10.133 / 10.221 s | 200 | clean ×3 |
| idle1–3, idleint1 (no requests) | TERM ×3, INT ×1 | 0 | 0.087 / 0.048 / 0.170 / 0.078 s | n/a | clean |
| d7b861b, same load | TERM | 137 (I sent SIGKILL at the 40 s cap) | still alive at 40 s | 200 | **DIRTY (RepairAborted)** |

- **Upload data intact:** all 11 attachments uploaded on the fix match the source md5 `a6857b0793027cf1c2a4d78ae94006f4`. The task history shows exactly 11 "Attached" events.
- **Log sequence:** "shutting down", then "requests still open after 10s; dropping them", then "shut down cleanly".
- **Sockets:** (a) and (b) read EOF after exit. (c) received its 200 response and was closed.
- **Startup behaviour unchanged:** both trees log the same stamp, listening, shutdown and port-in-use lines (exit 1, same AddrInUse error) for 127.0.0.1 and 0.0.0.0. One cosmetic change: the listening line now prints the bound socket address. `--host localhost` now logs `http://[::1]:PORT` where it used to log `http://localhost:PORT`, and `::1` is now bracketed (it used to print `http://::1:PORT`). The bind itself is unchanged. The only thing that parses this line is the new test.

### 2. Schema guard (binary level, on a copy of a populated database)

| Case | Result |
|---|---|
| Newer clean file (stamp 2) | exit 1 with the full refusal message; sha256 byte-identical |
| Newer crash-left file (stamp 2, left needing repair) | exit 1; logs "redb repaired it" and "no application data was written"; table lengths identical to the base, stamp still 2; file clean afterwards |
| Unstamped file | logs "stamped … with schema version 1"; every table length identical to the base |
| Equal version | logs "is at schema version 1"; table lengths identical. The unit test `equal_version_check_writes_nothing` passes |
| Unit tests | all 10 `schema_` tests pass |

### 3. Fingerprint
Mutations were made in the mutation worktree and reverted after each run.

| Mutation | Result |
|---|---|
| `TaskAttachment.size` u64 → i64 | **Fails.** The listing shows `size:i64`, inside the nested `TaskRow` attachments too. The message names the change and gives the new hash to append |
| Rename a stored field (`#[serde(rename="uid")]`) | **Fails**, `SessionRow: {created_at:string,uid:string,}` |
| Add a field (`ip: Option<String>`) | **Fails**, `ip:option<string>` |
| Rename a TimerMode variant | **Fails** |
| Add a table opened in `connect` but not listed | **Fails** with "every table Db::connect creates must be in the fingerprint table list" plus both sets |
| Same table, also listed | **Fails** (hash mismatch) |
| META key `&str` → `u64` | **Fails**, `table meta: u64 -> u64` |
| META value u64 → u32 | **Fails**, `&str -> u32` |
| USERNAMES value `&str` → `String` | **Fails** |
| Reorder fields | **Passes.** This is correct, since rows are JSON and field order does not matter |

- **Correct bump:** I added the field, set `SCHEMA_VERSION = 2` and appended the hash the test printed. The full `TZ=UTC` suite then passes, 145 passed and 0 failed, with no other test changes needed. fmt and clippy are clean.
- **End to end with the v2 build:** the v2 binary upgrades the file from 1 to 2. The v1 binary then refuses it with exit 1 and leaves it byte-identical.
- **Covered types:** I checked every `read_one`, `write_one`, `mutate` and raw insert against the `src/db.rs` tables. Every stored type is listed: UserRow, SessionRow, ApiTokenRecord, BoardRow, ColumnRow, SwimlaneRow, TaskRow, TimeEntryRow, Settings, ActiveTimer, ColorRow and BoardTemplateRow. The nested types are listed too (Subtask, TaskAttachment, TaskComment, TaskEvent, BreakActivity, TimerMode), and the fingerprint also walks into them through their parents.
- **Not covered, but they round-trip:**
  - `BoardTemplateRow.snapshot` (shows as `any`);
  - the `config_json` strings;
  - the bool flags under the SETTINGS keys.

  These are schema-less JSON, so an older binary keeps unknown keys rather than dropping them.

### 4. Log message, docs wording and test start

| Check | d7b861b | 039658c |
|---|---|---|
| Read-only crash-left file | "redb is repairing it", then "Permission denied" | a single stderr line: "…: was not closed cleanly and needs redb's crash repair, which failed: I/O error: Permission denied". Exit 1 on both; file unchanged on both |
| `.dev/fidelity-status.md` | "no migrations" | "additive schema guarded by `SCHEMA_VERSION`". No stale "no migrations" wording is left in the repo |
| Shutdown tests | — | start on `--port 0` and read the bound port from the startup line, so the bind/drop/rebind race is gone |

### Break-attempts I added
- An upload that would need about 20 s, longer than the deadline. It was dropped at 10.18 s (curl 000), the file stayed clean, and no partial attachment or history event was written.
- SIGINT sent twice. The second is ignored and it still exits 0 at 10.46 s.
- Two zero-byte connections plus a keep-alive plus an upload. Exit came at 2.998 s and 3.263 s, just after the upload finished, so these connections do not wait for the deadline.
- SIGTERM sent the instant the listening line appeared, 8 times. All exited 0 and left the file clean.
- A second instance on the same database. It is refused with "Database already open", and the first instance shuts down cleanly.

### Observations (candidate new defects, none of them a regression)
1. **Startup window, present on d7b861b too, low severity.** A SIGTERM after the schema log line but before the listening line kills the process with code 143 and leaves the file needing repair (fix: 2 of 3 tries dirty; d7b861b: 2 of 3). The signal handler is only installed after the bind, so seeding and backfills run without it. The next start repairs the file.
2. **The v1 fingerprint entry was rewritten** in 039658c, even though its doc comment says "never edit an old entry". The hashing algorithm changed, so this is unavoidable, and it only matters if d7b861b's fingerprint ever shipped.
3. **A Ctrl-C does not force-quit during the drain**, so a developer may wait up to 10 s.
4. **New stored types must be added to the type list by hand.** `AGENTS.md` documents this, and nothing enforces it.

The evidence is in the scratchpad folder `vs3/`, at `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/vs3/`:
- `results.txt`, `runs.out`, `log-*.txt` and `fds-*.txt`: the shutdown runs;
- `mut.out`, `mut2.out` and `mutout-*.txt`: the fingerprint mutations;
- `bump-test.txt`: the correct-bump suite run;
- `g-*.txt`: the gate runs;
- `startlog.sh`: the startup-log comparison script (its output was only printed to the terminal).

DONE: GRACEFUL-SHUTDOWN PASS stalled-body/header/keep-alive drain bounded at 10.05–10.22 s (5×TERM, 3×INT, exit 0, file clean, uploads intact); idle stop 0.05–0.17 s; d7b861b still alive at 40 s and left the file dirty
DONE: SCHEMA-VERSION PASS guard cases hold at binary level; fingerprint fails on type, rename, add-field, variant, table, key and value changes with helpful messages; correct v2 bump passes 145/145; only the 2 known TZ-unset tests fail, identically on d7b861b

