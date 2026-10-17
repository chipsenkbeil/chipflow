## verify-e4 report: fix/timer-goldens at b8b994c (KF-424 to KF-430)

**Result:** KF-424, KF-425, KF-427 and KF-429 PASS. KF-426, KF-428 and KF-430 FAIL, each on a narrow residual; the functional part of each works. The regression sweep PASSES.

`$S` below is `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad`. Outputs are in `$S/verify-e4/`, scripts in `$S/r2/pw/e4-*.mjs`, screenshots in `$S/verify-e4/shots/`.

### Environment
- **Worktrees** (both detached and clean; timer-goldens untouched, still at b8b994c):
  - fix: `.dev/worktrees/verify-e4` at b8b994c
  - baseline: `.dev/worktrees/verify-e4-base` at d79de3d
- **Diff d79de3d..b8b994c:** agent-guide.md, src/models.rs (the STANDARD_COLORS const only), src/routes.rs, static/app.js, static/style.css, templates/time_entries.html, templates/time_log_view.html, tests/db_colors_templates.rs. That is what these defects should touch.
- **Release binaries:**
  - fix md5 b07edb38240d4a53a5206fc14d441c99. `skipBreak` 2 hits, `log-day-total` 2 hits.
  - base md5 8f88286d2827f6719d73e53e72f534b7. Both strings 0 hits.
  - Both trees emit the same release-only warning (unused `ServeDir` import), so it predates this branch.
- **Servers:**
  - Ports: fix 3345, baseline 3346. DBs `/tmp/chipflow-verify-e4-<port>.redb`, always started with `--database-path`.
  - Server: TZ=America/Chicago, libfaketime (`FAKETIME_TIMESTAMP_FILE`, `NO_CACHE`, `FAKETIME_DONT_FAKE_MONOTONIC=1`).
  - Browser: Playwright 1.63 `context.clock`, moved in 1 s lockstep with the server.
  - Viewport 1440x612 for the golden geometry and 1905x1000 for the sweep, DPR 1, animations off.
  - Both servers were killed by PID; the ports are free.

### Gates (`gates.log`)
| Gate | Result |
|---|---|
| fmt | rc 0 |
| build | ok |
| clippy --all-targets | 0 warnings |
| `TZ=UTC cargo test` | 128 passed, 1 ignored |
| jsc checkSyntax | rc 0 |
| `env -i` + TZ=UTC | 128 passed |
| No TZ set (host zone) | 71/73: exactly `format_due_card_weekday_near_absolute_far` and `format_due_modal_relative_and_done_marker` fail. This is the expected KF-431 gap. |

### Stored data
- No stored struct or table changes: db.rs is untouched, and models.rs changes only the STANDARD_COLORS hex constants. The builder's claim holds.
- The `TimerLogEntry.seconds` response field is new and additive.
- `/api/timer/stop`'s `completed` changes meaning: the server now decides it.

### KF-424: PASS (golden 27)
| Claim | Baseline | Fix |
|---|---|---|
| Panel title | "Short break" | "Pomodoro" |
| Label | "Time left" | "Break time" |
| Button | `tp-stop` "Stop" | `tp-break-start` "Start", background #4e4e4e, ▶ #4cd964 |
| Task row × | none | present |
| Start | — | a new 25:00 pomodoro on the same task; count stays 1 |

- **The Start button is not green in the golden:** golden 27 gives #4e4e4e with a 1px #535353 border. Only the ▶ is green; its golden pixel (120,228,142) is the same as idle Start's in golden 14, which ChipFlow renders as #4cd964. The fix matches.
- Panel-relative geometry: task row (15,124,230x30), equal to the golden. The clock and button are off by 1–7 px, which is the existing KF-366 layout, unchanged.
- **Break-attempts:**
  - × during a break shows "No task selected", then Start runs a task-less pomodoro.
  - Change task during a break, then Start, runs on the new task.
  - Two tabs: Start in tab B; tab A follows after a poll.
  - The footer Stopwatch during a break: see observation 3.

### KF-425: PASS (golden 29)
- **Baseline:** at break+300 the server goes idle and the panel shows "Time until break 25:00".
- **Fix:** at break+300, +302, +905 and after a reload, the server still reports `short_break` with rem 0. The panel shows "Break time 00:00" with Start, and the chip reads "▶ 00:00".
- **Break-attempts:**
  - ±6 min client skew: identical behaviour.
  - Two tabs: both show 00:00.
  - Long-break cycle: see judgement (a).

### KF-426: FAIL (golden 28)
- **Baseline:** during a break the chip shows a red 20x20 square and has the `running` class.
- **Fix:** green ▶, `running` class dropped, background #484848 (equal to the golden), white digits.
- **The ▶ is not the golden's icon:**

| Property | Fix | Golden 28 |
|---|---|---|
| Glyph | 11.2 px text glyph, 8x9 px painted | solid triangle ≈16x19 CSS px (26x31 px at the 1.6x zoom) |
| Colour | #16a34a (22,163,74) | pixel (113,232,137), the same green as the panel ▶ (#4cd964-class) |

- Side-by-side crop: `shots/chip-pair.png`.
- The running square was sized to the golden (KF-325); the break ▶ reuses the small dark idle glyph instead.

### KF-427: PASS (golden 26)
- **Baseline:** steady "00:00" title; the clock is blank (empty string).
- **Fix, panel open, sampled every 250 ms for 6 s:** the clock is visible for 1 s and hidden for 1 s, a 2.0 s period. The title alternates "00:00" and "--------" (8 dashes) in phase with the clock.
- **Fix, panel closed:** the title still alternates. The chip's "00:00" does not blink; KanbanFlow's chip behaviour here is unknown.
- **Not blinking where it shouldn't:** the break at 00:00 does not blink.
- **Break-attempt:** a reload in the waiting state keeps the blink, and there is no re-chime (chimedFor is set from the status).

### KF-428: FAIL (golden 20; dark golden 20)
| Claim | Baseline | Fix | Golden |
|---|---|---|---|
| Day header | "Today — 0m — 1 Pomodoro" | "Today 30s" | "Today 30s" |
| Title style | — | 20px #111, line-height 30 | same |
| Total style | — | 13px #666, margin-left 20 | same |
| Title-to-total gap | — | 20 px | 30 px (the title's 10px right margin is missing) |
| Timer log reason | ‘Email’ | "Email" | "Email" |
| CSV reason | — | "" escaping correct | — |
| Excel reason | — | `&quot;` | — |
| Task sub-views (`/api/tasks/{id}/time-log-view`, `/time`) | — | "Email" | — |

- **Escaping:** a reason of `Boss said "now" <b>&amp;, ok` is HTML-escaped everywhere and valid in the CSV.
- **The count is missing.** `dark/timer/20-timer-log` shows "Today 27m 1 Pomodoro" for a day with 1 completed and 2 stopped pomodori. KanbanFlow counts completed pomodori only and hides the count at 0, which is why light golden 20 has none.
- The fix drops the count everywhere. Its "Sunday, 18 October 27m" group should read "27m 1 Pomodoro".
- The tracker's "no count" wording came from a day with no completed pomodoro.

### KF-429: PASS (theme psv)
- **New boards:** all 10 colours (bg, border, light) from `/api/boards/{id}/colors` equal the psv exactly on a new 10-colour board. The baseline differs on yellow, green, blue and red borders and on purple, magenta, cyan and brown lights.
- **Existing boards, no migration:** I copied a baseline-created DB to the fix binary.
  - The colours API equals the psv.
  - Card computed values:
    - borders: yellow #f5cc00, green #59d600, blue #70b0ff, red #ff858f
    - lights: purple #f2e6ff, magenta #ffebff, cyan #e6ffff
  - Modal tint: purple #f2e6ff. Timer panel and TODAY dots: #f5cc00 and #70b0ff.
  - 0 old border hexes across the board, settings/colors, the modal, the card, timer status/today/log, the sub-views and the CSS. The baseline on the same data serves the old values.
- **Break-attempt:** template instantiation and board copy rebuild hexes from `standard_color`. A saved template snapshot still stores the old hexes, but nothing renders them.

### KF-430: FAIL (docs only; behaviour correct)
| Case | Baseline | Fix |
|---|---|---|
| Pomodoro, 2 min, `completed:true` | completed, count +1 | interrupted, `completed:false` |
| 1497 s, claimed true | completed | interrupted |
| 1498 s, claimed true | completed | completed |
| 1498 s, `{"completed":false,"reason":"Email"}` | interrupted "Email" | completed, reason dropped |
| 1500 s, `{}` | completed | completed |
| Stopwatch, claimed true | completed | completed |
| Stopwatch, claimed false | interrupted | interrupted |
| Short break, 60 s, claimed true | — | `completed:false` |
| 19 s | — | discarded |
| 20 s | — | interrupted |
| Start replaces a pomodoro at 1499 s | interrupted "Switched task" | completed |
| 4 fake early completions: count | 9 | 5 |

- **Break-attempt:** lowering `pomodoro_minutes` to 1 mid-session doesn't help, because the duration is fixed at start; a 2-min claim stays interrupted.
- **The FAIL:** AGENTS.md requires agents/skill.md to stay accurate, and two docs still describe the old client-trusted rule:
  - `agents/skill.md:168-170` still says "Only `completed:true` pomodori count … `completed:false` records an interruption".
  - The OpenAPI doc on `TimerStopInput.completed` (`src/routes.rs:7258`) still says "True when the timer ran to zero on its own".
  - Only agent-guide.md was updated.

### Judgements
- **(a) Breaks never end on their own.**
  - Golden 29 supports what you see: "stays in break state at Break time 00:00 … (no auto-start); header chip shows green ▶ 00:00".
  - The long-break cycle is intact. With the fast clock, breaks 1–3 were short (5:00), break 4 long (15:00) and break 5 short; each stayed at 00:00 until Start (`break-fix.out`).
  - A user who walks away keeps a server-side break at 00:00 indefinitely. A kf325 run's break was still there 4 h later, so every later page load lands in it.
  - UI breaks carry no task, so no time is logged and the data stays clean. The only exit is Start (a pomodoro) or an API stop. API agents see `active:true` with rem 0 forever. See observations 3 and 4.
- **(b) Quotes in the task sub-view too:** reasonable. The KanbanFlow log text is the same component, and the one-style change keeps the CSV, Excel, log and sub-views consistent. The badge tooltip still uses single quotes (observation 5).

### Regression sweep (fix; 0 console errors except the deliberate ones in KF-001)
- **KF-001:** A–G match verify-e3. The only console errors are D's race 404, tab B's 404 and the injected 500, as before.
- **KF-004:** 24 s loads 16:00:00–16:00:24; an unchanged Update keeps 24.
- **KF-006:** ±6 min skew:
  - Take break comes at server 0, the break at server time, and then it stays at 00:00 (the intended KF-425 change).
  - One run showed the client 1 s ahead. That was my harness: the timestamp file is written before the browser clock advances. The 2 s grace covers it.
- **KF-010:** "Select open task".
- **KF-015:** the badge titles are correct.
- **KF-067:** 1 Pomodoro 33m, then reassign, then delete.
- **KF-103:** 0:29 none, 0:30 "+ 1m", 1:30 "+ 2m", waiting "+ 25m"/"+ 26m", reload, Take break 26m equals the server card.
- **KF-278:** tooltip and 1 menu node.
- **KF-279:** "TODAY 2m" at 1:35 in the golden-25 rebuild.
- **KF-289:** the export menu text.
- **KF-325:** 3px dashed in all states, now #f5cc00. This matches the dark golden 22 note "3px #f5cc00".
- **KF-366:** 260 px, rgb(62,62,62).
- **KF-375:** the title ticks with the popup open.
- **KF-378:** avatar-timing on the header and card avatars.
- **KF-380:** a signed-out /login logs 0 errors.
- **KF-390:** "Manually added time".
- **KF-412:** day buckets; TODAY empties at 00:10.
- **KF-413:** 15 minutes in the modal.

### Observations (candidate defects; none block the PASSes)
1. **Timer log rows** (`shots/log-pair.png` against golden 20):
   - The fix renders dark navy rows with a "P" badge and 13px non-italic #f87171 status.
   - The golden has white rows, dashed separators and italic 11px #ff0000 status.
   - The row dot is a status colour (#f59e0b or green) on both trees. The golden uses the task's taskBorderColor (#f5cc00).
2. **KF-428 geometry:** the title-to-total gap is 20 px against the golden's 30 px.
3. **Stopwatch unreachable during a break:** the footer Stopwatch flips its label to "Pomodoro", but the break panel stays, and Start still forces a pomodoro. With breaks now indefinite, a stopwatch can't be started from the panel until a pomodoro is started and stopped. KanbanFlow's behaviour is unknown.
4. **Orphaned break-activity select:** it still renders during a break, but nothing submits it now (no Stop, no auto-end). Task-less breaks were never logged on either tree, so KF-347 attribution via the UI was already dead.
5. **Badge tooltip quotes:** it still reads "Stopped with reason 'Email'" (single quotes), unlike the new straight double quotes.
6. **Breaks started with a task via the API** are logged with badge "Manually added". Pre-existing.

DONE: KF-424 PASS break panel reads "Pomodoro / Break time", Start is #4e4e4e with a #4cd964 ▶ (golden 27; not a green button), × present, Start begins the next pomodoro on the selected task (baseline: Short break / Time left / Stop / no ×)
DONE: KF-425 PASS a finished break stays at "Break time 00:00" with Start through +905 s, a reload and ±6 min skew; long-break cycle intact (4th break 15:00) (baseline: idle "Time until break 25:00" at +300)
DONE: KF-426 FAIL a green ▶ now shows in a non-running #484848 chip, but it is an 8x9 px #16a34a text glyph against golden 28's ≈16x19 px #4cd964-class triangle (baseline: red square)
DONE: KF-427 PASS the waiting clock is visible 1 s and hidden 1 s (2 s period) and the title alternates "00:00"/"--------" in phase; no blink at a break's 00:00 (baseline: steady)
DONE: KF-428 FAIL "Today 30s" and straight quotes are right in the log, CSV, Excel and both task sub-views, but the completed-pomodoro count was removed: dark golden 20 shows "Today 27m 1 Pomodoro"; the title-to-total gap is 20 px against 30 px
DONE: KF-429 PASS all 10 colours (bg/border/light) equal the psv; a baseline-created DB served by the fix shows the new values on cards, the modal, timer dots and the API with no migration and 0 old hexes
DONE: KF-430 FAIL behaviour correct (a 2-min `completed:true` is interrupted, 1498 s completes with the 2 s grace, a 1497 s claim does not, stopwatch honours the claim, fake count 9 becomes 5), but agents/skill.md:168 and the OpenAPI doc for `completed` still describe the old client-trusted rule
DONE: REGRESSION PASS all 18 timer defects pass one probe each, including ±6 min skew and the fast-clock full cycle; gates green (128/128 with TZ=UTC and in an empty env; without TZ only the two KF-431 due tests fail); no stored-struct changes; 0 console errors outside KF-001's deliberate 404/500 cases
