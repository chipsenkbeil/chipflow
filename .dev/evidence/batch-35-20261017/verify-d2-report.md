# verify-d2: KF-398 (agent docs must match the binary)

**Verdict: PASS.** I found no due_at sentence, or any other date or timestamp statement, that the binary contradicts. The commit is docs-only, and 106 probes gave byte-identical results on the fix and the baseline under both time zones.

## Environment
- **Fix tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-d2`, detached at f5be0fe. Debug binary md5 e841d9fb80404e66755e2802072f1c3a. Port 3327, DB `/tmp/chipflow-verify-d2-3327.redb`.
- **Baseline tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-d2-base`, detached at 000c5ff. md5 77bb2f3e556f9aba0961dfed2d11e47c. Port 3328, DB `/tmp/chipflow-verify-d2-3328.redb`.
- **The binaries differ:** `rg -a -c 'whitespace-only clears the due date'` finds the string once in the fix binary and 0 times in the baseline.
- **Time zones:** each pair of servers ran with fresh DBs, first under TZ=UTC and then under TZ=Europe/London. Both were stopped by exact PID.
- **Browser sweep:** Playwright Chromium 153, 1905x1000, DPR 1.
- All evidence is under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-d2/`:
  - probe results: `utc-fix.tsv`, `utc-base.tsv`, `lon-fix.tsv`, `lon-base.tsv`
  - scenario results: `utc-special-*.txt`, `lon-special-*.txt`
  - pomodoro check: `pomo-fix.txt`
  - gates: `gates.log`
  - served docs: `served-agents.md`, `served-skill.md`, `served-openapi*.json`
  - probe list and scripts: `probes.txt`, `run-probes.sh`, `special.sh`, `start.sh`

## Gates (fix tree)
All pass:
- fmt --check: exit 0.
- clippy --all-targets: 0 warnings, exit 0.
- `TZ=UTC cargo test`: all targets ok, 0 failures (unit tests 11 + 69 with 1 ignored, which accounts for all 70 test functions, plus 9 integration files).
- jsc checkSyntax on static/app.js: exit 0.

## Step 1: docs-only, and behavior identical
- **Diff scope:** `git diff 000c5ff f5be0fe` touches only agent-guide.md, agents/skill.md, and two `///` doc comments in src/routes.rs (on `CreateTaskInput.due_at` and `UpdateTaskInput.due_at`). No code lines change.
- **Spec impact:** the served OpenAPI JSON for fix and baseline differs in exactly those two description strings.
- **Served docs:** /agents.md and /agents/skill.md are byte-identical to the files in the fix tree.
- **Probe identity:** each of the 106 values went through POST and PATCH on both binaries, under both time zones. I compared status code, stored value, task-count change and PATCH result.
  - `cmp` reported IDENTICAL for UTC and for London.
  - The scenario scripts (re-sends, "changes nothing", overdue) also matched, apart from ids and timestamps.

## Step 2: due_at sentences
"L" means the result under London. Every probe ran as both POST and PATCH.

| Doc sentence | Probes | Result | Verdict |
|---|---|---|---|
| G: surrounding whitespace is trimmed | Values wrapped in \t, \n, \r, \r\n, NBSP, U+2003, U+3000, U+0085/U+2028, on all three forms | All accepted and trimmed, e.g. `\t…Z\t` stored as `…Z`. U+200B and U+FEFF prefixes give 400 (neither is whitespace). | PASS |
| G: null, "" or whitespace-only clears | null, "", "   ", "\t", NBSP alone, U+3000 alone, a mix of 7 classes, U+200B alone | All clear (POST gives null due, PATCH clears and resets due_done). U+200B alone is a 400. | PASS |
| G1: RFC3339 stored as sent once trimmed | `Z`, `.000Z`, `+01:00`, `-00:00`, a 10-digit fraction | All stored verbatim | PASS |
| G1: lowercase t or a space for T, lowercase z for Z | `t…Z`, `T…z`, `space…z`, `space…Z`, `t…z` | All 201/200, stored verbatim | PASS |
| G2: local "YYYY-MM-DD HH:MM[:SS]" with one space or uppercase T | space, T, with seconds; lowercase t, two spaces, tab, NBSP separators | Space and T accepted (UTC `+00:00`, L `+01:00`). The other four are 400. | PASS |
| G3: bare date is 23:59:59 local | `2026-10-20`, bare 2026-10-25 and 2027-03-28 (L) | `23:59:59+01:00`; L `2026-10-25T23:59:59+00:00` and `2027-03-28T23:59:59+01:00` | PASS |
| G: four-digit unsigned year | 26-, 999-, 02026-, 12026-…Z, +2026, -2026, 0026 | All 400 except 0026, which has four digits and is accepted | PASS |
| G and S: leading zeros optional in forms 2 and 3, not in RFC3339 | `2026-10-20 8:5:7`, `2026-1-5 8:05`, `2026-1-5T8:5`, `2026-1-5`, `2027-3-28` (L); RFC3339 with unpadded month, day, hour or second | Local and bare forms accepted. Unpadded RFC3339 is 400 in all four cases. | PASS |
| G, S and OpenAPI: seconds 00-59 in every form | `:60` in RFC3339 (23:59:60Z, mid-day, `.5`, with offset) and in local space, T and unpadded forms; minute 60 | All 400, with "seconds must be 00-59" | PASS |
| G: anything else is a 400 | Hour 24 (RFC3339 and local), fractional seconds in local forms, offset without colon, offset with seconds, +24:00, RFC3339 without seconds, Feb 29 2026, slashes, "next tuesday", 2026-13-45, a trailing T, and JSON number, bool and object | All 400 | PASS |
| G: …and the request changes nothing | PATCH with name, description, size, labels, due_repeat, due_done, grouping_date and a bad due_at (garbage, a gap time, :60, lowercase-t local); POST with a gap time (L) | 400, and the full task snapshot is unchanged; the POST creates 0 tasks | PASS |
| G: a repeated fall-back time resolves to the earlier instant | L: 2026-10-25 00:59:59, 01:00, 01:30, 01:59:59 | All `+01:00` (the first occurrence) | PASS |
| G: a time skipped by spring-forward is a 400 | L: 2027-03-28 01:00, `T01:30`, `1:30`, 01:59:59 | All 400 ("skipped by a daylight-saving change"). Under UTC the same values are accepted, which fits "the server zone's". | PASS |
| G: exact transition second, "02:00" once (02:00Z) | L: 2026-10-25 02:00 and `T2:0`; 2027-03-28 00:59:59 and 02:00 | `02:00:00+00:00`; `00:59:59+00:00`; `02:00:00+01:00` | PASS |
| G: same instant in another format keeps the stored string and due_done | Start from `08:00:00Z` with done set, then send `.000Z`, `09:00+01:00`, `t…z`, `space…Z`, a whitespace-padded copy, the local form, `-05:00`; also bare date then `23:59:59` local; and at the DST edge (L), local 01:30 then `00:30Z` | String and due_done kept in every case | PASS |
| (same sentence, negative case) | `08:00:01Z`; at the edge, `01:30+00:00`, the second occurrence | Both reset due_done | PASS |
| G: overdue only once the full timestamp has passed | Due now−30s, now+90s, bare today | `card-due-overdue` class: 1, 0, 0 | PASS |
| G: POST unparseable value is a 400 and creates no task | All 400 probes | Task-count change 0 for every one | PASS |
| S: one sentence restating the rules above (trimmed, stored as sent, uppercase T, 00-59, end of day, unsigned year, zeros, fall-back earlier, spring-forward or other format is 400) | Same probes as above | Matches | PASS |
| OpenAPI CreateTaskInput.due_at (forms, zeros, trimmed, :60 and gap are 400, absent/null/empty/whitespace means none) | Same probes as above | Matches | PASS |
| OpenAPI UpdateTaskInput.due_at (null, "" or whitespace clears; absent leaves it unchanged) | only_* probes on PATCH; PATCH without due_at | Matches | PASS |

## Step 3: other date statements, checked on the fix tree
- **Done stamp:** moving a task into the Done column (found through `is_done`) sets `completed_at` to an RFC3339 value. The card has no stamp. The modal shows `&#10003; Done (Oct 16)`. Moving the task out sets `completed_at` back to null. PASS.
- **grouping_date:**
  - zero-padded date: accepted
  - surrounding tab or spaces: trimmed
  - `2026-1-5` or a timestamp: 400
  - null or "": clears
  - whitespace-only also clears, which the docs do not contradict
  - PASS
- **Timestamp formats:**
  - RFC3339: task `created_at`/`completed_at` (export-data), comment and attachment `created_at`, history `created_at`, time-entry `started_at` (`/api/time/entries/{id}`, e.g. `2026-10-15T09:00:00+01:00`)
  - Unix seconds: timer status `started_at` (1792178874) and token `created_at` (1792178921)
  - PASS
- **Pomodoro count:** one stopped (`completed:false`) and one completed pomodoro of 22 s each give `pomodori_completed` 1 and `interruptions` 1. PASS.
- **Attachment cap:** 8,000,000 bytes and 10,485,760 bytes give 200; 10,485,761 gives 400 "exceeds the 10 MiB limit". `GET /api/tasks/{id}` has no `attachments` key. PASS.
- **Manual time:** past entry 200; future entry 400; zero-length and negative 400. PASS.
- **The six tracker errors:** all six are present in main's (d332e14) agent-guide ("Unix seconds", "✓ <date>", "attachments"/"download_url", stopped pomodori count, among others). All six are corrected in the fix tree.

## Break-attempts the builder probably did not try
- Every Unicode whitespace class, including U+0085 and U+2028.
- U+200B and U+FEFF, which are not whitespace.
- An offset without a colon, `+24:00`, and an offset with seconds.
- A 10-digit fraction.
- Leap second combined with a non-UTC offset.
- DST-edge re-sends: the earlier occurrence keeps due_done, the later one resets it.
- Non-string JSON values.
- A 7-field PATCH that is rejected with nothing written.

## Regressions
- Chromium, fix tree, board holding all probe tasks: the board, /calendar, /due-date, /agents.md and /agents/skill.md loaded with 0 console errors and 0 failed requests.

## Observations (candidate new defects, outside KF-398's docs scope)
1. **Safari cannot parse a lowercase "z" due date (worth filing).** The docs now advertise lowercase z, and the server stores it as sent. JavaScriptCore returns Invalid Date for `2026-10-20T08:00:00z`, `… 08:00:00z` and `t…z`. Chromium parses all of them. Three client-side code paths use `new Date(due_at)`:
   - `parseLocalDay` in the calendar, app.js:7325
   - the date filter, app.js:8527
   - the due dialog, app.js:4512

   In Safari, those three would drop such a due date. This is a follow-up to KF-400's "reject what the renderer cannot parse" rule. Evidence is the jsc output in this run.
2. **Ancient years get a truncated offset (trivial).** Under London, `0026-10-20` is stored as `…23:59:59-00:01`. Local mean time is −00:01:15, so the stored instant is 15 s off.
3. **Rust syntax in the public spec (pre-existing).** The OpenAPI UpdateTaskInput description shows `Some(None)` to API readers.
4. **Whitespace-only clear leaves due_repeat in place.** Clearing due_at with whitespace keeps `due_repeat`; the docs neither promise nor deny this.
5. **The test default board is named "General".** This was the default board in my fresh test DBs; no owner board was touched.

Both worktrees are left in place and clean, and both servers are stopped.

DONE: KF-398 PASS every due_at sentence in agent-guide.md, skill.md and both OpenAPI descriptions holds against 106 POST/PATCH probes under UTC and London, the commit is docs-only (identical probe results on fix and baseline), and the other date statements check out
