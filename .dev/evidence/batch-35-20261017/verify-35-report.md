## verify-35 report: batch-35 release candidate, integ/batch-35 at 22e84eb

**Result: PASS.** Every check in your list passed on the combined tree. One thing to know before the deploy: one new schema test fails when `cargo test` runs as **root** in Linux. It passes as a non-root user, which is how the Nix sandbox runs builds (details under Observations).

`$S` = `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`. All outputs are in `$S/verify-35/`, screenshots in `$S/verify-35/shots/`, and the scripts are `$S/r2/pw/v35-*.mjs`.

### Environment
- **Worktrees** (both detached and clean; integ-35 was not touched, no tracked files edited):
  - `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-35` at 22e84eb.
  - `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-35-base` at f8927b7.
- **What the diff touches:** f8927b7..22e84eb changes 10 files: `.dev/HANDOFF.md`, `.dev/fidelity-status.md`, AGENTS.md, agent-guide.md, agents/skill.md, src/{db,main,routes}.rs, static/app.js and tests/graceful_shutdown.rs. That is what the 15 commits should touch. f8927b7 and d230d76 have identical src/static/docs/Cargo files.
- **Debug binaries:**
  - fix md5 3d56c2617475b144ca72dae409025fda; base md5 b09f46273e23eb04684a65ed3f4b7899.
  - `rg -a -c 'SCHEMA_VERSION|schema version'`: fix 3, base 0.
  - The `static/app.js` served by the fix equals the tree's copy, with 5 `parseDueAt` hits.
  - All browser probes ran on debug binaries; I did not build a release (rust-embed) binary.
- **Ports and DBs:** fix on 3386, base on 3387. DBs were `/tmp/chipflow-verify-35-*.redb`, always passed with `--database-path`. All servers were killed by PID, both ports are free, and the Linux container is removed.
- **Browser:** Playwright 1.63 Chromium, 1905x1000 (1905x600 for the sticky header), DPR 1, a fresh context per scenario, animations off.
- **Clocks:** libfaketime (timestamp file, NO_CACHE, `FAKETIME_DONT_FAKE_MONOTONIC=1`) on the server, and Playwright `context.clock` in 1 s lockstep.

### 1. Gates and test suites
| Check | Result |
|---|---|
| fmt --check | rc 0 |
| build | ok |
| clippy --all-targets | 0 warnings |
| jsc checkSyntax(static/app.js) | rc 0 |
| `cargo test --no-fail-fast`, TZ=UTC | 152 passed, 0 failed, 1 ignored |
| same, TZ=America/Chicago | 152 / 0 / 1 |
| same, TZ=Pacific/Chatham | 152 / 0 / 1 |
| macOS `env -i`, each test binary | 12 of 12 binaries rc 0, 152 passed |
| Linux `rust:1.98.0`, no zoneinfo, no /etc/localtime, no network, `env -i`, no HOME, empty TZDIR, `--offline`, as **uid 1000** | rc 0, 152 passed (`linux-sandbox-nonroot.out`) |
| same, as **root** | rc 101: `failed_crash_repair_is_reported_as_such` fails in both lib and main (`linux-sandbox-test.out`) |

- Crates were fetched first; then the network was disconnected (DNS lookup failed) before the offline build and the test run.
- The root failure is environmental: root ignores the read-only bit the test relies on, so the "cannot repair" path never happens.

### 2. Schema guard on the combined tree
| Claim | Evidence | Verdict |
|---|---|---|
| A baseline-written DB is stamped 1 with no data loss | Base seeded with 25 tasks, 2 comments (one with ü/ï), pomodoro 37 / short break 7, a logged 22 s session and a running stopwatch, then SIGTERM. The baseline left the file dirty. The fix logs "not closed cleanly; redb repaired it" and "stamped … schema version 1". The full API dump (2615 lines, including timer status/log/today/stats and settings) is identical. All rows match except `meta` (+schema_version 1) and one new session. The old session cookie still works, and the running timer kept its started_at. | PASS |
| A newer-stamped file is refused byte-identically | I stamped a copy with 2 using my own redb tool (`$S/verify-35/stamp`). The fix exits 1 with the "schema version 2 … refusing" message. md5 is 327ecf98… before and after. Nothing listens on the port. | PASS |
| SIGTERM gives exit 0 and a clean file | Idle: rc 0, "shut down cleanly". A read-only redb open (no repair) succeeds afterwards. | PASS |
| A stalled request is dropped after 10 s | An authenticated POST with Content-Length 5000 and a partial body, then SIGTERM: "requests still open after 10s; dropping them", elapsed 10 s, rc 0, clean file. | PASS |

### 3. Timer after integration (server TZ America/Chicago)
| Probe | Fix | Verdict |
|---|---|---|
| KF-006 skew +360 s | Pill 24:00 at t+60. Take break appears exactly at server 0. The short break runs 300 s and ends at server time (idle at +301). TODAY reads "25m 1 Pomodoro". The output matches batch-34's verify-e3 line for line. | PASS |
| KF-006 skew −360 s | Same cycle. The second pomodoro gets a short break (count 2, every 4) and TODAY reads "50m 2 Pomodoros". | PASS |
| KF-279 TODAY (fresh DB) | Bare at 0:29, 1m at 0:30, 2m at 1:30, 25m at 25:00, 26m while waiting and after reload. "TODAY 26m 1 Pomodoro" after Take break. The row reads "6:00 AM – pending", then "6:00 AM – 6:26 AM". Identical to batch-34. | PASS |
| KF-412 local day | A 7:10 PM Chicago entry is filed under the 16th. Time spent: 50 min on the 16th, 0 on the 17th. TODAY empties at 00:10. Identical to batch-34. | PASS |

### 4. Due dates
Server TZ Europe/London; the UI used real clicks on the due chip and Add.

| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| KF-398: `26-10-20` | 201, stored `0026-10-20T23:59:59-00:01` | 400 on POST and PATCH, stored value unchanged | PASS |
| KF-398: other rules | `26-10-20 08:00` and a two-space separator were accepted | 400. Also 400 on the fix: `t` in local form, a 5-digit year, a sign, slashes, `:60`, `2026-1-05T…Z`. Whitespace is trimmed, and "" or blanks clear. | PASS |
| KF-398: docs | — | agent-guide.md lines 174-195 and agents/skill.md lines 44-51 describe every result above exactly | PASS |
| KF-409: unchanged save (`10:00:59Z`, done; browser UTC) | PATCH `10:00:00.000Z`, done false | PATCH `2026-10-20T10:00:59Z`, done true | PASS |
| KF-409: `.123Z` value, browser Los Angeles | `10:00:00.000Z`, done false | exact, done true | PASS |
| KF-410: `2026-10-25T01:30:00+00:00`, browser London | `00:30:00.000Z`, done false | exact, done true; also exact after change-and-revert | PASS |
| KF-410: cancel on task A, then save task B | B `00:30Z` | B exact, A untouched | PASS |
| KF-414: shipped `parseDueAt` under jsc | — | Raw `new Date` is Invalid for `…z`, `t…z`, `space…z`, `.123z` and padded input; the helper parses all of them to the right instant. Z/offset/date-only/local values are unchanged. In the UI, a lowercase-z due saved unchanged keeps `2026-10-20t08:00:00z`. | PASS |
| KF-400: DST rules | identical to fix | `2026-10-25 01:00` / `01:30` / `01:59:59` → +01:00; `02:00` → +00:00; `2027-03-28 01:00` to `01:59:59` → 400; `02:00` → +01:00; "next tuesday" and `2026-13-45` → 400 | PASS |

### 5. Batch-32/33 behaviours (fix and base side by side, timer running where it matters)
- **Move submenu:** the menu shows "Move right / Move to bottom / Move advanced". Move right keeps the dashed border, the live part and the avatar badge.
- **Done drag:** the timed card lands in Done and keeps its state through a reload.
- **Collapsed strip:** the strip renders, the selection survives a poll, and expanding restores the card.
- **Sticky header:** the header bottom stays at 140 and covers the scrolled timed card. Screenshots fix vs base: AE 0.
- **Board frame:** board-wrap is 1905x925 with background rgb(242,242,242), and the column bodies are 850 px each. Full-board screenshot fix vs base: AE 0.
- **Board delete confirm:**
  - The styled dialog is 420x162, titled "Delete board", with a quoted-name message and focus on Delete. Fix vs base screenshots: AE 0.
  - Cancel sends 0 requests. Delete sends one DELETE, redirects to /boards/new, and the board is no longer listed.
- **Done group order:** dragging T-C below T-B inside "Today" persists after the drop and after a reload.

The fix's integration output is identical to the base's, ids aside.

### 6. Console
- 0 console errors, 0 page errors and 0 failed requests in every fix run, with one exception: the 400 my integration probe causes on purpose with a garbage due_at PATCH (the KF-400 check). The base shows the same 400.

### Break-attempts (ones the builders probably didn't try)
1. The production upgrade shape: the old binary is SIGTERMed while a timer runs, then the guard opens the dirty file. Result: repaired and stamped, no loss.
2. A newer-stamped file that is also crash-left: refused. redb's allocator repair changes bytes, but the rows are identical and the message says so.
3. A stalled authenticated upload during shutdown. My first try was unauthenticated, and its 401 returned immediately.
4. An open board tab plus a running pomodoro across a SIGTERM restart: exit in 1 s with rc 0. After the restart started_at is unchanged, the pill counts on, and a reload logs 0 errors.
5. The Linux sandbox shape both as root and as non-root, with no network.

### Observations (candidate new defects)
1. **LOW: `failed_crash_repair_is_reported_as_such` assumes a non-root user.**
   - **What fails:** run as root, `set_readonly` does not stop the open, so the test panics with "a read-only file cannot be repaired".
   - **Production:** I expect this to be safe, because NixOS sandbox builds run as nixbld (non-root), and that shape passes. I have not checked green-box's nix.conf.
   - **Suggested fix:** skip the test when euid is 0, or run a `nix build` dry run before the deploy (as batch-33 did).
   - **Evidence:** `$S/verify-35/linux-sandbox-test.out` (root) vs `$S/verify-35/linux-sandbox-nonroot.out`.
2. Not a defect: the TODAY list shows only the 2 newest rows until it is expanded. In an earlier run, entries left from a "future" faketime made the list look truncated.

DONE: BATCH-35 PASS gates clean, 152/152 tests under UTC/Chicago/Chatham/env -i and in a no-network, no-zoneinfo Linux sandbox as non-root; schema guard upgrade/refusal/SIGTERM/10 s drain, timer (KF-006/279/412) and due-date (KF-398/409/410/414/400) probes and batch-32/33 behaviours all hold with 0 console errors; one test fails only when run as root (LOW, not the Nix shape)
