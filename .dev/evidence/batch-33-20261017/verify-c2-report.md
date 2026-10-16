## Verifier verify-c2: report for KF-400, KF-387 and KF-398

**Verdicts:** KF-387 and KF-398 pass. KF-400 fails. All 24 DST transition seconds now resolve correctly, but a seconds value of `:60` is still accepted, stored, and then dropped by the browser views (break-attempt B1).

### Environment
- **Fix tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-c2`, HEAD d33495c. It is origin/main fe23cc2 plus the 13 commits cherry-picked in order. All applied cleanly.
- **Baseline tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-c2-base`, HEAD 3839c87 (the same stack through 0db79af).
- **What the 3 new commits touch:** `src/routes.rs`, `agent-guide.md`, `agents/skill.md`, and `Cargo.toml`/`Cargo.lock` (a dev-only `chrono-tz 0.10`, pure Rust). `static/` is untouched, as expected.
- **Binaries (debug):**
  - Fix: md5 65114d36…, then relinked by the gate run at 18:29:48 to 59f323e6…, same source.
  - Baseline: d0e76870….
  - The string "exact transition second" appears once in the fix binary and 0 times in the baseline; "zero-padded YYYY-MM-DD" appears 2 times vs 0.
  - I re-ran the London battery on the final 59f323e6 binary: 21/21 OK.
- **Instances:** fix on 3327, baseline on 3328. Each run used a fresh DB at `/tmp/chipflow-verify-c2-<port>.redb`.
- **Zone handling:** the server resolves local times with chrono `Local`, which reads the process `TZ`. I confirmed this by running the same binary under different `TZ` values and getting different results.
- **Browser:** Playwright 1.63.0 Chromium, 1905x1000, DPR 1, reduced motion, a fresh context for each scenario. All servers are killed by PID and nothing is listening on 3327 or 3328.
- **Evidence:** everything is under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-c2/`:
  - `k400-*.txt`
  - `shots/d1..d10-*.{png,json}`
  - `shots/leap-*.{png,json}`
  - `openapi-*.json`
  - gate logs

### Gates (fix tree)
- `fmt --check`: 0
- `build`: 0
- `clippy --all-targets`: 0, with 0 warnings
- `TZ=UTC cargo test`: 0. Every suite is ok; the new DST, Done-drop and grouping_date tests pass.
- jsc `checkSyntax static/app.js`: 0

### KF-400: due_at at DST transitions
I created a task with POST and also applied the same value with PATCH, then converted the stored value to UTC. POST and PATCH agreed in every row.

| Zone and input | Baseline | Fix | Verdict |
|---|---|---|---|
| London 2026-10-25 00:59:59 / 01:00 / 01:00:01 / 01:30 / 01:59:59 (repeated hour, earlier instant) | 23:59:59Z / 00:00Z / 00:00:01Z / 00:30Z / 00:59:59Z | same | ok |
| London 2026-10-25 02:00 (also `T02:00` and `2:0`) | **01:00Z (wrong)** | 02:00Z | fixed |
| London 2026-10-25 02:00:01 | 02:00:01Z | 02:00:01Z | ok |
| London 2027-03-28 00:59:59 | 00:59:59Z | 00:59:59Z | ok |
| London 2027-03-28 01:00 | **201, stored 01:00Z** | 400 | fixed |
| London 2027-03-28 01:00:01 and 01:59:59 | 400 | 400 | ok |
| London 2027-03-28 02:00 / 02:00:01 | 01:00Z / 01:00:01Z | same | ok |
| NY 2026-11-01 00:59:59 / 01:00 / 01:00:01 / 01:59:59 | 04:59:59Z / 05:00Z / 05:00:01Z / 05:59:59Z | same | ok |
| NY 2026-11-01 02:00 | **06:00Z (wrong)** | 07:00Z | fixed |
| NY 2026-11-01 02:00:01 | 07:00:01Z | 07:00:01Z | ok |
| NY 2027-03-14 01:59:59 | 06:59:59Z | 06:59:59Z | ok |
| NY 2027-03-14 02:00 | **201, stored 07:00Z** | 400 | fixed |
| NY 2027-03-14 02:00:01 and 02:59:59 | 400 | 400 | ok |
| NY 2027-03-14 03:00 / 03:00:01 | 07:00Z / 07:00:01Z | same | ok |
| Card / modal / iCal for London "02:00" | card "1:00 AM", iCal DTSTART 010000Z | card "2:00 AM", modal "25 October 2:00 AM", iCal 020000Z | fixed |
| garbage, 2026-13-45, bare dates, unpadded forms | — | as the docs say | ok |

Break-attempts:
- **Lord_Howe (30-minute DST): survived.** On 2027-04-04, 01:30 resolved to 14:30Z and 02:00 to 15:30Z; the baseline gave 15:00Z for 02:00. On 2026-10-04, 02:00 and 02:29:59 returned 400; the baseline accepted 02:00.
- **Pacific/Apia's skipped day 2011-12-30: survived.** It returns 400, and the edges are correct.
- **Europe/Dublin (negative DST in tzdata): survived.** It matches London; the baseline was wrong in the same way.
- **B1, leap-second `:60`: did not survive.**
  - Accepted by the fix:
    - `"2026-10-20 08:15:60"` on a `TZ=UTC` server, stored as `2026-10-20T08:15:60+00:00`
    - `"2026-10-21 23:59:60"`, accepted the same way
    - RFC3339 `"2026-10-22T10:30:60+02:00"`, stored as sent
    - `"2026-12-31 23:59:60"` and `"2026-10-25 01:59:60"` in Dublin
  - **The browser can't parse what gets stored.** Chromium's `new Date("…:60…")` returns NaN, so these dues show on the card ("Oct 20, 2026 8:15 AM") but are missing from the calendar view (`shots/leap-3327.json`: `calendarHas` is false for all three `:60` tasks and true for the `:59` control). The due-date dialog prefill (`app.js` ~4430) and the due report (`classify`, ~6896) silently drop them too.
  - **Acceptance depends on the server's zone.** The same local `"08:15:60"` returns 400 on a London server (offset +1) and 201 on a UTC server. Dublin `"01:59:60"` on the fall-back night resolves to the later instant (02:00:00Z), which contradicts the earlier-instant rule.
  - **It is not new.** The baseline and origin/main accept these too. But it is a stored value the renderer cannot parse, which is exactly what KF-400 says to reject, and it produces the defect's own symptom (a due visible in one view and missing from another).

### KF-387: Done drag order and grouping_date
All drags below are real SortableJS mouse drags. Every scenario had 0 console errors, 0 page errors, 0 failed requests and 0 responses ≥400.

| Scenario | Baseline | Fix | Verdict |
|---|---|---|---|
| D1: drag T-B below T-C inside "Today" (sent position 1) | **snaps back to T-B T-C after the drop and after reload** | T-C T-B, persists after reload and in server HTML | fixed |
| D2: reorder O-1/O-2 inside the older "2 October" group (on-screen index 7) | — | O-2 O-1, persists | ok |
| D3: drag the last Today card to the top of Today | — | persists | ok |
| D4: drag Y-1 into a different date group | — | returns to its own group, as documented | ok |
| D6: drag a Today card below the last group | — | keeps its place, as documented | ok |
| D5: drag a card from another column into the middle of the older group | — | joins the top of Today, as documented | ok |
| D7: reorder inside a second swimlane's Done cell | — | persists | ok |
| D8: drag from lane 1 Done into lane 2 Done | — | lands after T-C (same group) | ok |
| D9: reorder inside the undated "Completed" group (tasks created directly in Done) | — | persists | ok |
| D10: drag in a non-Done column (regression) | — | persists | ok |

grouping_date checks:
- **Rejected on the fix, accepted on the baseline:**
  - `"2026-1-5"`, `"2026-01-5"`, `"2026-1-05"`, `"26-01-05"`, `"+2026-01-05"`: fix 400, value unchanged. The baseline returns 200 and stores each one verbatim.
- **Rejected on both:**
  - `"2026-01-05T00:00:00Z"`, `"02026-01-05"`, `"2026-02-30"`, `"2026/01/05"`, and the number 12345: 400.
- **Accepted on the fix:**
  - `" 2026-01-05 "` returns 200 and is stored trimmed.
  - `null` and `""` both clear the value.

### KF-398: agent docs vs the binary
- **Served docs match the files:** `/agents.md` and `/agents/skill.md` are byte-identical to `agent-guide.md` and `agents/skill.md`; `/.well-known/agents.json` returns 200.
- **Move claims hold:**
  - A move without `position` or `column_id` returns 400.
  - Leaving out `swimlane_id` keeps the task's lane (it stayed in Lane2).
  - Position is rounded and clamped: 1.6 → index 2, 1.4 → 1, -5 → 0, 999 → end.
  - The Done on-screen-order rules (beside a same-group neighbour / keeps its place / top of its group when arriving) match D1–D9.
- **Done stamp:** moving into Done shows `✓ Done (Oct 16)` in the modal and no stamp on the card; moving out clears it.
- **grouping_date text** matches the probes above.
- **due_at claims hold:**
  - RFC3339 is stored as sent (`.000+02:00` and lowercase `t…z` are kept verbatim).
  - Unpadded month, day, hour and minute are accepted ("2026-1-5 9:5" → 09:05).
  - Anything else is a 400 and changes nothing.
  - The London transition example in the guide is exact.
  - `""` and `null` clear the due date.
  - Re-sending the same instant as `.000Z` or `+01:00` keeps `due_done`.
- **skill.md** DST, position and grouping_date lines are also correct.
- **OpenAPI:** the `MoveTaskInput.position` description is new and accurate. The `due_at` and `grouping_date` descriptions are terse but not false.
- **Break-attempt:** unpadded seconds and an unpadded bare date are accepted. The docs don't mention them, but they don't contradict them either. Survived.

### Regression sweep (fix and baseline give identical results)
- **KF-374:** an exactly 10 MiB attachment (13,981,078-byte JSON) returns 200.
- **KF-396:** renaming to blank or 101 characters returns 400; 100 characters returns 303.
- **KF-402:** `due_done` stays true after re-saving the same instant as `.000Z`.
- **KF-391:** `best_day` is null on a fresh DB.
- **KF-372:** a Pomodoro-template board has the colours 1 Pomodoro / 2 Pomodori / 3 Pomodori / >3 Pomodori.
- **KF-397:** before setup, the env token gets 200, a wrong token 401, no token a 303 to /setup, and minting with the token returns 403.
- **KF-399:** OpenAPI lists `/api/tasks/{id}/card`, `/api/tasks/{id}/modal` and `/api/timer/settings`.

No regressions found.

### Candidate new defects (outside the three verdicts)
1. **Old grouping_date values are now ignored (LOW).** origin/main stored any string as `grouping_date`. The fix only groups by strictly padded dates, so a value like "2026-1-5" already in a production DB now falls back to `completed_at`. It is neither migrated nor flagged, and the API still returns the old string.
2. **Tasks created directly in Done have no completion time (pre-existing).** A task POSTed straight into the Done column has `completed_at: null` and groups under "Completed" rather than "Today".
3. **Docs nit.** The guide lists month/day/hour/minute as zero-optional; seconds and the bare-date form also accept unpadded values.

DONE: KF-400 FAIL DST transition seconds now correct in London/NY/Lord_Howe/Apia/Dublin (baseline wrong at 02:00 and the first skipped second), but `:60` seconds are still accepted and stored (local forms on a zero-offset server, RFC3339 everywhere); Chromium can't parse them, so those dues vanish from the calendar, due dialog and report while the card shows them
DONE: KF-387 PASS real SortableJS drag inside a Done group now persists (baseline snapped back), 9 Done drag scenarios incl. older group, swimlanes, cross-lane, undated group all behave as documented; unpadded grouping_date ("2026-1-5", "26-01-05", "+2026-01-05") now 400
DONE: KF-398 PASS guide, skill, served copies and OpenAPI position text match the binary for DST resolution (London example exact), grouping_date strictness and trimming, Done on-screen position rules, rounding and clamping, swimlane keep and the Done stamp
