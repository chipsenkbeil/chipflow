## verify-d3 report: KF-414 (Safari due_at parsing)

**Verdict: PASS.** All four client-side due parsers now go through one helper. In an emulated Safari browser, the fix shows lowercase-z dues everywhere they belong, and the baseline drops them. Server storage is unchanged.

### Environment
- **Fix worktree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-d3`, detached at 9cfcd03 (f5be0fe plus one commit).
- **Baseline worktree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-d3-base`, at f5be0fe.
- **Debug binaries (md5):** fix 8fd8132d3a9a0ca5e9ae46c669136d76, baseline f10ed2e1a214dad1308f7a4087526a6a.
- **Served `/static/app.js`:** fix md5 9979d8e9… (same as the worktree file) with 5 `parseDueAt` hits; baseline md5 b7dfd368… with 0 hits. A debug build serves static files from disk, so the served file is the meaningful difference.
- **Ports and DBs:** 3327 uses `/tmp/chipflow-verify-d3-3327.redb`, 3328 uses `/tmp/chipflow-verify-d3-3328.redb`. Both were fresh, set up as `audit` and seeded with `seed-golden-master.sh`. Both servers were killed by exact PID afterwards.
- **Browser:** Playwright 1.63.0 Chromium, 1905x1000, DPR 1, timezone Europe/London (the server's zone), reduced motion, a fresh context per page.
- **WebKit:** not available (only Chromium is installed), so I used an init script. It makes `new Date(str)` and `Date.parse(str)` return Invalid/NaN when the trimmed string ends in a lowercase `z`. That matches what real jsc does: jsc accepts a lowercase `t` and a space separator, and rejects only the lowercase `z`. The in-page probe confirmed it: lowercase z gave NaN, uppercase Z gave 1792483200000.

### Gates (fix tree)
| Gate | Result |
|---|---|
| fmt | OK |
| build | OK |
| clippy --all-targets | 0 warnings |
| `TZ=UTC cargo test` | all suites ok (69 + 11 + 13 + 8 + … passed, 0 failed) |
| jsc `checkSyntax` on app.js | exit 0 |

The diff touches only `static/app.js` (+13 −4).

### Check 1: does every due parse go through the helper?
I grepped `static/` and `templates/` for `new Date(`, `Date.parse`, `Date.UTC` and date libraries (dayjs, moment, luxon, date-fns). No templates use any of them, and there is no date library.

Every place that parses a due value uses `parseDueAt`:
- **Due dialog prefill:** line 4521.
- **Due report:** line 6992.
- **Calendar:** `parseLocalDay`, line 7334, called at 7470.
- **Date filter:** `dateMatches`, line 8536, which reads `card.dataset.dueAt`.

The other `new Date(` calls parse `created_at`, `completed_at`, `started_at`, ms numbers or locally built strings. The export at 6031 passes the due string through unparsed. The card and modal due text are rendered on the server by chrono (`format_due_card` / `format_due_modal`), so the browser's parser never touches them. **No due parse site is missed.**

### Check 2: the helper in real jsc
I extracted the helper's text verbatim from the fix's app.js and ran 46 cases under TZ=UTC, America/Los_Angeles and Asia/Kolkata. Results are in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-d3/jsc-results.txt`.

| Input class | Raw `new Date()` in jsc | Helper |
|---|---|---|
| `T…z`, `t…z`, `' '…z` (e.g. `2026-10-20 08:00:00z`) | Invalid | 2026-10-20T08:00:00.000Z (correct) |
| `.123z`, `.123456z`, `.123456789z` | Invalid | correct, truncated to ms |
| `t`, space, `.5Z`, 10-digit fraction with uppercase Z | valid | identical to raw |
| offsets +05:30, -07:00, ±00:00, +14:00, `.250-03:00` | valid | identical, correct instant |
| padded `"  …t…z  "`, `"\t… …z\n"` | Invalid | correct |
| years 0001 and 9999 with lowercase z | Invalid | correct |
| local forms `2026-10-20 08:00[:00]`, `T08:00`, `t08:00` | local time | same local time in all three zones |
| null, undefined, `""`, `"   "`, garbage, `2026-13-45T…Z`, `2026-13-45t…z`, `T25:00`, `zz`, `Zz`, `Tt`, `x` separator, double space, `2026-02-30t…z`, `"z"`, `"xyz"` | Invalid (null gives epoch 0) | Invalid |
| `T08:00:60Z`, `T24:00:00Z` | rolled over | rolled over, same as raw |

Of the 46 cases, 44 match expectations. The two exceptions are the `:60` and `24:00` rows. jsc itself rolls those over, and the server rejects both with a 400, so they can never be stored.

**What the server stores.** I PATCHed 37 forms and read each back. Every RFC3339 form is stored verbatim after trimming. Local forms are converted to `…+01:00` or `…+00:00`. One surprise: `2026-10-20t08:00` returns a 400. The output of the fix (3327) and the baseline (3328) is byte-identical, so storage is unchanged. I also fed every stored string back through the helper in jsc, and every one parses to the correct instant. Output is in `server-forms-3327.tsv` and `jsc-stored.txt` in the scratchpad folder.

### Check 3: browser surfaces, emulated Safari
Test tasks:
- **Lowercase z (6):** lowerz `2026-10-18t10:00:00z`, spacez `2026-10-19 09:30:00.250z`, fracz `…T08:00:00.123456789z`, padded (stored as `2026-10-21T07:00:00z`), far `2026-11-20t10:00:00z`, pastz `2026-10-10t10:00:00z`.
- **Controls:** upper `…T10:00:00Z`, offset `…t09:00:00+05:30`, and one task with no due date.

| Surface | Baseline, emulated | Fix, emulated | Verdict |
|---|---|---|---|
| Card due line (server-rendered) | all 8 shown | all 8 shown | OK; never affected |
| Modal chip (server-rendered) | all 8 shown | all 8 shown | OK; never affected |
| "Due in 7 days" filter | only upper and offset; 4 in-range z tasks hidden | lowerz, spacez, fracz, padded, upper, offset; far and pastz correctly excluded | fixed |
| Due dialog prefill | date and time empty for all 6 z tasks | correct for all, e.g. lowerz 2026-10-18 11:00, offset 2026-10-19 04:30 | fixed |
| Calendar (Oct) | "5 tasks…", only upper and offset chips | "10 tasks…", z chips on 10/18/19/20/21 Oct | fixed |
| Due report | "0 of 6", only 2 KF414 rows | "0 of 12", all 8 rows, pastz Overdue | fixed |

The baseline numbers come from a read-only run in which no task was modified (`base-emu-ro.json`).

**Additional finding on the baseline.** In emulated Safari, opening the dialog for lowerz and clicking Add without changes sent `{"due_at":null,…}` and erased the due date (the API then returned `due_at: null`). The same action on the fix re-sends the stored string. So before this fix, Safari users lost data on an unchanged save, not only display.

### Check 4: regressions
- **Native Chromium:** fix and baseline give identical output on every surface (`fix-native.json` and `base-native.json`).
- **Uppercase and offset dues:** identical on fix and baseline in both modes.
- **KF-409 unchanged save on the fix:** the PATCH body is `{"due_at":"2026-10-18t10:00:00z","due_repeat":null}` in both native and emulated runs. The stored value stays `2026-10-18t10:00:00z` and `due_done` stays false.
- **Errors:** 0 console errors, 0 page errors, 0 failed requests and 0 responses ≥400 across all 5 runs.

### Break-attempts
1. Nine-digit fraction combined with a lowercase z.
2. Padding with tabs and newlines around a lowercase z.
3. The extreme years 0001 and 9999.
4. Inputs that look nearly valid: `Tt`, `zz`, `Zz`, a double space, `2026-02-30`.
5. Three browser timezones for the local forms.
6. Feeding every string the server actually stores back through the extracted helper.
7. An unchanged-save round trip in emulated Safari, which exposed the baseline data loss.

All survived.

### Observations (not KF-414 blockers)
- **Bare date in the helper:** `parseDueAt("2026-10-20")` is UTC midnight, following the ES spec, not local time. The server never stores a bare date (it converts it to `T23:59:59+01:00`), so there is no impact today.
- **Possible new defect (calendar layout):** the calendar's day columns size to their content, so widths are unequal, and with long chips the grid runs past the header's × button. In `fix-emu-calendar.png` the grid's right edge is at x≈1492 against the header at ≈1409. The baseline also has unequal widths, so this predates the fix. I could not find it in the tracker. Screenshots are in `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-d3/shots/`.
- **Possible docs/server mismatch:** `2026-10-20t08:00` (local form with a lowercase t) gets a 400, while the RFC3339 forms accept a lowercase t. That fits the docs' "exactly one space or uppercase T" for local forms, so it is consistent, but worth knowing.

Evidence folder: `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-d3/` (gates.txt, helper.js, run.js, jsc-results.txt, jsc-stored.txt, server-forms-*.tsv, tasks-*.tsv, fix-native.json, fix-emu.json, base-native.json, base-emu.json, base-emu-ro.json, shots/). The driver script is `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/r2/pw/vd3-due.mjs`.

DONE: KF-414 PASS all 4 due parse sites use parseDueAt; jsc parses every stored form correctly; in emulated Safari the fix shows lowercase-z dues in the filter, prefill, calendar and report while the baseline drops them (and its unchanged save erased the due); server storage identical; 0 console errors
