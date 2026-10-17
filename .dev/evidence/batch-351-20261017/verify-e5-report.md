## verify-e5 report: fix/timer-goldens 131b54e compared with b8b994c

**Verdict:** all three defects pass, the regression sweep is clean and every gate passes. There are two qualifications on KF-430's docs (below) and five candidate defects that already existed on the baseline (end of report).

### Environment
- **Fix tree:** worktree `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-e5`, detached at 131b54e. Release binary md5 54a6fd48c165004414dd1e84d55c833a. Port 3345, database `/tmp/chipflow-verify-e5-3345.redb`.
- **Baseline tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-e5-base`, at b8b994c. Release binary md5 d27c8ac3a9e62d482cea765d11c44da8. Port 3346, database `/tmp/chipflow-verify-e5-3346.redb`.
- **Binaries differ:** the string `timer-pill-playbox` appears twice in the fix binary and not at all in the baseline.
- **Clocks:** the servers ran under libfaketime (FAKETIME_TIMESTAMP_FILE, NO_CACHE, DONT_FAKE_MONOTONIC=1, TZ America/Chicago). Playwright 1.63 kept `context.clock` in lockstep. The harness is copied from e4 as `$S/r2/pw/e5-*.mjs` and `e5r-*.mjs`.
- **Browser:** viewport 1440x612 (the golden size), DPR 2 for the chip and DPR 1 elsewhere, animations frozen.
- **Cleanup:** both servers were killed by PID (10050, 12695) and the ports are free.
- **Evidence folder:** `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-e5/`. All paths below are relative to it.

### Gates (fix tree)
- `fmt --check`: exit 0.
- `clippy --all-targets`: exit 0, 0 warnings.
- `TZ=UTC /Users/senkwich/.cargo/bin/cargo test`: 128 passed, 0 failed.
- The same suite under `env -i TZ=UTC`: 128 passed, 0 failed.
- `jsc checkSyntax` on `static/app.js`: exit 0.
- Logs: `gate-fmt.log`, `gate-clippy.log`, `gate-test.log`, `gate-test-envi.log`, `gate-jsc.log`.

### Diff scope
- The diff from b8b994c to 131b54e touches only `agent-guide.md`, `agents/skill.md`, `src/routes.rs`, `static/app.js` and `static/style.css` (47 lines added, 25 removed).
- `src/db.rs` is untouched.
- No stored struct changed. In `routes.rs` only doc comments, the badge format string and its unit test changed.

### KF-426: break chip (golden timer/28)
| Claim | Golden | Baseline | Fix | Verdict |
|---|---|---|---|---|
| Break icon | ▶ about 16x19 CSS px (26x30 px at the zoom's 1.6x scale = 16.25x18.75) | text glyph ▶, 9.86x11.2 px, #16a34a | CSS triangle, 16x19 px (borders 10/9/16), #4cd964. At DPR 2 the solid pixels measure 30x36 in exactly #4CD964. Vertically centred: icon centre y=18 = chip centre | PASS |
| Chip not in running style | no `--running` | not running | class `timer-pill` (no `running`) for short break, long break, after reload and at break 00:00 | PASS |
| Background / digits | #484848 / #fff | #484848 / #fff | rgb(72,72,72) / rgb(255,255,255) | PASS |
| Running pomodoro unchanged (KF-325) | red square | `running`, 20x20 #ff7070 | same, and the running-chip screenshots are byte-identical between fix and baseline | PASS |

- **Break attempts:** the long break, the break left at 00:00 for 10 minutes and a reload during the break all show the triangle. Stopwatch running gives a red square plus `running`, and idle gives the old glyph without `running`. The idle stopwatch-tab class `timer-pill-stop` cannot trip the new `stopbox` substring test.
- **Console:** 0 errors and 0 failed requests in both runs.
- **Evidence:** `chip-3345.out`, `chip-3346.out`, `chip-dark-*.out`, `shots/chip-*`, and `kf426-golden-fix-base.png` (golden, fix and baseline stacked).

### KF-428: timer log day header and badge quotes (light and dark golden 20)
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| One stopped 30 s session | "Today 30s" | "Today 30s" (no count) | PASS |
| 1 completed + 2 stopped + 1 stopwatch (dark golden) | "Today 26m" (no count) | "Today 26m 1 Pomodoro" | PASS |
| Plural | none | "2 Pomodoros"; "3 Pomodoros" (on a day with a break and a stopwatch) | PASS |
| Days with no completed pomodoro | none | count hidden ("Monday, 19 October 1m") | PASS |
| Gap from title to total | 20 px | 30.0 px (golden 494−464=30). Total to count is 20 px, which also matches the golden (540) | PASS |
| Title / total styles | same | title 20px rgb(17,17,17); total and count 13px rgb(102,102,102) | PASS |
| Badge tooltip | `Stopped with reason 'Email'` | `Stopped with reason "Email"` (log, sub-views, API) | PASS |

- **Hostile reason test:** I used the reason `Boss said "now" <img src=x onerror=alert(1)> & 'ok' x" onmouseover="alert(2)`.
  - Log modal: 0 injected elements.
  - CSV: quotes correctly doubled.
  - Excel: entity-escaped, 0 `<img>` tags.
  - `/api/tasks/{id}/time-log-view` and `/time`: the attribute is escaped (`&quot;`, `&lt;`, `&#x27;`) and 0 elements were injected.
- **Count follows the server's verdict:** a 1498 s stop sent with `completed:false` and a reason is still counted as completed.
- **Evidence:** `log-3345-0.out`, `log-3345-1.out`, `log-3346-*.out`, `shots/log-3345*.png`, and `log-3345*.csv` / `.xls`.

### KF-430: docs for the server-decided completion rule
- **What the fix docs say:**
  - `agents/skill.md` and `agent-guide.md` say a pomodoro or break completes only when its elapsed time reaches the duration, whatever `completed` says, and that the flag matters only for a stopwatch.
  - The served `/agents.md` is byte-identical to `agent-guide.md`.
  - OpenAPI `TimerStopInput.completed` reads "Honoured for a stopwatch only…", `TimerStopResult.completed` reads "The server's verdict…", and the `timer_stop` summary describes the server rule.
  - On the baseline these fields said "True when the timer ran to zero on its own" or nothing.
- **No doc still says the client's flag is trusted.** `agent-guide.md` line 454 explicitly says "even with `completed:true`". `agents.json` has no behaviour text.
- **Behaviour is unchanged:** `kf430-3345.out` and `kf430-3346.out` are identical apart from the task UUID.
  - A 2-minute pomodoro stopped with `completed:true` is logged as interrupted and does not count.
  - At 1497 s a pomodoro is interrupted; at 1498 s it completes.
  - A stopwatch honours the flag.
  - Four fake early completions still leave the count unchanged.
- **Qualification 1:** none of the docs mention the 2 s grace. They say "reaches the duration", but the server accepts 1498 of 1500 s. This errs on the safe side for agents.
- **Qualification 2:** `TimerStopResult.completed` says "true when the session reached its duration", which is not accurate for a stopwatch, where it echoes the flag.

### Regression sweep (fix vs baseline, same scripts)
- **KF-424, KF-425, KF-427 (`cyc-*.out`):** the break panel shows "Pomodoro" with Start, waits at "Break time 00:00" (including after reload), and the title blinks "--------" while a finished pomodoro waits. Output matches the baseline except for the chip icon.
- **KF-429 (`colors-*.out`):** card colours (yellow border #f5cc00, and so on) are identical to the baseline.
- **Older surfaces (`reg-*.out`):** KF-004, 010, 015, 278, 289, 366, 375, 378, 380 and 390 are unchanged. The only differences are the intended quote style and the added count.
- **Timer smoke:** start, stop, break and log all worked.
- **Console:** 0 errors and 0 failed requests in every fix run.

### Observations (candidate defects, all present on the baseline too)
1. **Dark-mode chip colours:** the break and idle chip render #1f2937 with #e5e7eb digits under `body.dark .timer-pill`. KanbanFlow's dark golden 22 keeps the chip at #484848. There is no dark break golden, so this is inferred. Evidence: `chip-dark-3345.out`.
2. **Dark log rows in light mode:** timer-log modal rows render as dark navy cards (#1f2937, from the unscoped `.log-entry`). Golden 20 has white rows with dashed separators, and its modal is 660 px wide where ChipFlow's is 1080 px. Evidence: `shots/log-3345.png`.
3. **Log dot colour:** a yellow task's dot in the log modal is #f59e0b; the golden `taskColor` is #f5cc00. Evidence: `colors-3345.out`.
4. **Break entries badged as manual:** short-break log entries carry the badge tooltip "Manually added". Evidence: `log-3345-0.out`, "Tuesday" group.
5. **Chip geometry differs from the golden:** the chip is 96x32 with 13.6px digits and the icon is inset 11 px. KanbanFlow's is 145x34 with 20px bold digits and the triangle starts about 17 px in. This falls under KF-325's sizing, not KF-426.

DONE: KF-426 PASS — break chip shows a 16x19 #4cd964 CSS triangle without the running class on #484848 with white digits; baseline had a 10x11 #16a34a glyph; running pomodoro still has the 20x20 #ff7070 square and running class (screenshots byte-identical to baseline)
DONE: KF-428 PASS — day header counts completed pomodori only, hidden at 0, pluralised, 30 px title gap, 20px #111 title and 13px #666 total; tooltip reads Stopped with reason "Email"; hostile reason escaped in log, CSV, Excel, time-log-view and /time
DONE: KF-430 PASS — skill.md, agent-guide.md, served /agents.md and OpenAPI (TimerStopInput.completed, timer_stop, TimerStopResult.completed) describe the server-decided rule and no doc trusts the client flag; behaviour identical to b8b994c (2 min → interrupted, 1498 s → completed); docs omit the 2 s grace and TimerStopResult.completed is imprecise for a stopwatch
DONE: REGRESSION PASS — KF-424, 425, 427 and 429 plus the timer smoke match the baseline apart from the intended changes, with 0 console errors and 0 failed requests; diff limited to the five expected files; src/db.rs untouched
DONE: GATE PASS — fmt, clippy --all-targets (0 warnings), TZ=UTC libtest 128/0, env -i TZ=UTC 128/0, jsc checkSyntax exit 0
