## verify-c report: server/API batch (10 defects)

**Result: 7 PASS, 3 FAIL.**
- KF-400 FAIL: a local time at the exact second of a DST change is stored an hour off, or accepted when it should be a 400.
- KF-387 FAIL: the new date sort silently undoes some drags within the Done column. Baseline handles the same drag correctly.
- KF-398 FAIL: the edited docs promise two behaviours that the binary does not have.

### Environment
- **Tree under test:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-c`, HEAD `21cfef4`. It is origin/main `d332e14` plus the 10 commits cherry-picked in the given order with no conflicts.
  - Diff touches agent-guide.md, agents/skill.md, src/auth.rs, src/db.rs, src/routes.rs, static/app.js, templates/board_settings.html and tests/db_colors_templates.rs. That matches the defects.
- **Baseline tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-c-base` at `d332e14`.
- **Binaries (debug):**
  - fix md5 `afe178d2ea30ea184dec2b6cd0d1d15f`
  - base md5 `20865d086655d9a0ebfe2e457c695827`
  - `rg -a -c 'that local time is skipped by a daylight-saving change'` finds 1 match in fix, 0 in base.
- **Ports and DBs:** fix 3325, base 3326. Fresh DBs `/tmp/chipflow-verify-c-<port>-<tag>.redb` (tags: main, k397, k397noenv, k397blank, k400ny, fresh, upgrade, upgrade2).
  - The host TZ is Europe/London. One fix run used `TZ=America/New_York`.
  - Every server was killed by exact PID; both ports are free.
- **Browser:** Playwright 1.63.0, 1905x1000, DPR 1, fresh context per scenario, reduced motion, `timezoneId: Europe/London`.
- **Evidence directory:** `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-c/` (screenshots and result JSON under `pw/shots/`).

### Gates (fix tree)
- fmt, build, clippy --all-targets (0 warnings) and `TZ=UTC cargo test` all exit 0. That is 107 tests including the 9 new ones.
- `jsc checkSyntax(static/app.js)` exits 0.
- Logs: `gates.txt`, `clippy.log`, `test.log`.

### KF-374 (attachment body limit)
| Claim | Base | Fix |
|---|---|---|
| 1,500,000-byte file | 200 | 200 |
| 1,600,000 / 8,000,000 bytes | 413 | 200 / 200 |
| exactly 10,485,760 bytes | 413 | 200; download md5 matches the source (`a830131…`) |
| 10,485,761 bytes | 413 | 400 "attachment exceeds the 10 MiB limit" |
| body exactly at the limit (14,046,552 bytes, padded name) | 413 | 400 "file name is too long" (the handler ran) |
| limit + 1 byte | 413 | 413 |
| 10 MiB upload via env Bearer token | n/a | 200 |
| other routes keep 2 MiB: POST /api/tasks at exactly 2 MiB / +1 byte | 400 / 413 | 400 / 413 |
| PATCH task, POST comment, board-settings form at 3 MiB | 413 | 413 |

Break-attempts: the exact-byte body-limit boundary, a padded file name, Bearer auth, and default limits on three non-attachment routes. **PASS.**

### KF-396 (board name validation)
| Name sent | Base | Fix |
|---|---|---|
| spaces, "", tabs/newlines | 500 plain text | 400, General tab re-rendered |
| 101 ASCII chars, 101 emoji | 500 | 400 |
| 100 accented chars, 100 chars with padding | 303 | 303 (trimmed) |
| missing field | 422 | 422 |

- **400 page:** shows "Board name must be 1-100 characters.", keeps the stored name in the input, and the General tab is active.
- **XSS:** a stored name of `"><img onerror>` is escaped in the title, h1 and value attribute.
- **Browser:** submitting spaces through the real form shows the error page (`pw/shots/fix-settings-general-blank.png`).
  - The only console entry is Chrome's network line "Failed to load resource: 400". That comes from the specified 400 response, not a JS error.
- General tab without an error matches baseline with 0 differing pixels.

**PASS.**

### KF-400 (due_at validation)
Matrix: 30 inputs through POST and PATCH on both builds (`k400-london.txt`, `k400-dst.txt`, `k400-ny-out.txt`).

| Input | Base | Fix |
|---|---|---|
| "next tuesday", "garbage", "2026-13-45", "2026-02-30", "…25:00", "… +02:00", "…T08:00Z", fractional bare time, "20261020" | stored verbatim | 400; the rejected POST creates no task and the rejected PATCH changes nothing (name and description untouched) |
| every valid format: RFC3339 incl. `.000Z`/offset/lowercase, `HH:MM[:SS]` with space or T, bare date, padded date, "", null | same | same results (bare date → 23:59:59 local) |
| "2026-10-25 01:30" (ambiguous) | raw | `+01:00`, the earlier occurrence |
| "2027-03-28 01:30" (skipped) | raw | 400 |
| instance TZ env (`TZ=America/New_York`) | | honoured: the London gap time is accepted as `-04:00`; the NY gap "2027-03-14 02:30" is a 400 |
| due dialog (6 cases incl. ambiguous, gap, blank time) | | identical requests on both builds, all 200, chip renders; the browser resolves DST before sending |

**FAIL — exact transition seconds** (the times the tz library treats as ambiguous or skipped are off by one second at each boundary):
- **London fall-back:** "2026-10-25 02:00" happens only once (02:00Z), but it is stored as `2026-10-25T02:00:00+01:00`. That is 01:00Z, so the task shows 01:00, an hour early.
- **London spring-forward:** "2027-03-28 01:00" does not exist, but it is accepted as `01:00+00:00`, which displays as 02:00 BST. It should be a 400.
- **New York:** "2026-11-01 02:00" is stored as `-04:00` (an hour early), and "2027-03-14 02:00" is accepted.
- Cross-check: `TZ=Europe/London date -r` on 01:00Z gives 01:00 GMT; on 02:00Z it gives 02:00 GMT.
- Baseline stored these as raw strings (the original bug). The fix now stores the wrong time with no error.
- Suggested fix: keep only candidates whose UTC instant converts back to the same local time.

### KF-402 (re-saved due date keeps due_done)
- **API sequence** (`k402.sh`): re-sending the same instant as `.000Z`, `+01:00`, local "2026-10-20 09:00", or `.000Z` plus a new due_repeat keeps `due_done:true` and the stored string on the fix.
  - Baseline reset `due_done` on every one of those.
  - A real change (`…00.001Z`, the next day, or null) still resets it on both builds.
  - Fix logs 6 `due_updated` history events against baseline's 8, so the spurious ones are gone.
- **UI:** with the due date marked done, reopening the dialog and clicking Add unchanged sends `.000Z`. Fix stays `due_done:true`; baseline goes `false` (`pw/shots/*-results.json`, `due-dialog-resave-keeps-done`).

Break-attempts: an offset-form instant, a local-time form, a change of due_repeat only, and a 1 ms change. **PASS.**

### KF-391 (Highscores empty state)
- **API:** with an empty DB or a range with no data, base returns `best_day {pomodori:0}` and fix returns `null`.
- **UI:** fresh DB Highscores shows "No data to display" on the fix and "0 Best day — Friday, Oct 16 / 0" on base (`pw/shots/fix-fresh-stats-highscores.png`).
- **With data:** after one stopped and one completed pomodoro, both builds show the best-day and longest-streak cards.
- 0 console errors, 0 failed requests.

**PASS.**

### KF-372 (Pomodoro template colours)
**Spelling ruling: "Pomodori" is correct.**
- The KF-331 KanbanFlow capture, zoomed 4x (`kf331-labels-zoom.png`), reads "1 Pomodoro (Default)", "2 Pomodori", "3 Pomodori", ">3 Pomodori".
  - The other six are disabled, listed Brown, Cyan, Magenta, Orange, Purple, White.
  - Its header is "Board settings: General", so it is KanbanFlow's own starter board.
- KF-296's "Pomodoros" rests on GM-125, whose evidence was in `/tmp` and is gone, and whose own note flagged the spelling as unresolved.

| Claim | Base | Fix |
|---|---|---|
| Board from the built-in template | 10 plain names, all enabled | the 4 names above enabled, 6 disabled; hex, sort order and default identical to base |
| Fresh install "General" board | plain palette | same palette as the KanbanFlow reference (`pw/shots/fix-fresh-colors-k372.png`) |
| Blank board / Kanban basics template | plain | plain (unchanged) |
| Fix started on a baseline-created DB | | every existing board's colours byte-identical, template ids unchanged, new template boards get the new palette |
| User template also named "Pomodoro board" | | untouched |
| Restart of the fix | | labels kept; the KF-296 one-time backfill does not rename them |

**PASS.**

### KF-387 (grouping_date drives Done groups)
**Working:**
- Fix groups by grouping_date when set (Yesterday, "Thursday, 1 October", a future date) and by completion date when unset.
- Groups are newest first; undated tasks go last under "Completed".
- null, "" and spaces all clear it. Baseline ignored null.
- "garbage", "2026-02-30" and a full datetime are 400 (baseline stored them).
- Baseline's duplicate "Today" group after a cross-group move (`k387pos.sh`) is fixed.

**FAIL — drag within Done:**
- Setup: board order G-B, G-A (grouped to 2 Oct), G-C, G-direct. On screen that shows Today [G-B, G-C].
- Dragging G-B below G-C sends `position:1`, which is its index on screen. After the reload G-B is still above G-C, so the drag was silently undone.
- Baseline, same steps: G-B moves below G-C.
- Cause: the position index comes from the sorted on-screen order but is applied to board order, and the two now differ.
- Evidence: `pw/shots/fix-reorder-B-below-C.png`, `fix-drag2.json`, `base-drag2.json`.
- Minor: "2026-1-5" is accepted and stored unpadded.

### KF-397 (env token before /setup)
**Before setup, env token configured** (`k397probe.sh`):
- The env token can GET and POST /api and load HTML pages. Baseline sent everything to /setup with a 303.
- Rejected with 401: a random `cf_` token, the token's prefix, the token plus one character, the uppercased token, and a random token alongside a garbage cookie.
- Still sent to /setup with a 303: no auth, a lowercase "bearer" scheme, Basic auth, an empty Bearer, a cookie only, and HTML pages without a token.
- The env token cannot list, mint or revoke tokens (403).

**Before setup, other configurations:**
- No env token: every token gets 401 and anonymous requests get the 303.
- Env token set to spaces: never matches.

**After setup, both builds identical:**
- Read-scope token cannot write (403).
- Tokens cannot list, mint or revoke, even with a cookie attached (403).
- Revoked token: 401.
- A token with 1-day expiry, server run with `faketime +3d`: 401.
- The env token reaching HTML pages after setup is existing baseline behaviour, so pre-setup access is not wider than what the token already had.
- A revoked or expired token with zero users is unreachable: tokens need a cookie, and there is no user deletion.

**PASS.**

### KF-399 (OpenAPI coverage)
- Spec has 83 operations on the fix against 80 on base.
- The card, modal and timer-settings paths now have GET entries.
- A router-vs-spec diff finds no `/api` route missing.
- Timer settings returns the same body as `/api/settings`.
- Card and modal return 200 HTML, 404 for an unknown id and 401 without auth, matching the spec.
- Not addressed: the fix direction's "decide whether HTML pages belong in the spec". Report pages and /setup are still absent while board-settings pages are present.

**PASS.**

### KF-398 (docs accuracy)
Served `/agents.md` and `/agents/skill.md` are identical to the files. Checked against the running fix:

**Correct:**
- Env token before setup.
- RFC3339 times on tasks, comments, attachments, history and time entries; Unix seconds for timer status `started_at` and token times.
- A create with a bad due date is a 400 and makes no task.
- Moving into Done stamps the modal "✓ Done (Oct 16)" with no stamp on the card; moving out clears `completed_at`.
- The task detail key set is exact.
- Upload returns the six documented keys; GET on the attachments list is 405; DELETE works.
- The 10 MiB decoded-file cap.
- `best_day` is null with no data.
- A stopped pomodoro gives `pomodori_completed` 1 and `interruptions` 1; sessions under 20 s are discarded.
- Re-saving the same instant keeps `due_done`.

**FAIL — two described behaviours don't hold:**
- "one skipped by a spring-forward gap is a 400": "2027-03-28 01:00" returns 201 (from the KF-400 bug).
- "grouping_date … anything else is a 400": "2026-1-5" returns 200.

This flips to PASS once KF-400 handles the boundaries and grouping_date validation is strict (or the docs are loosened).

### Regression sweep
- **Golden board:** fix vs base differ by 2 px (max channel delta 1). Fresh General board: 48 px in the top bar, delta ≤ 2, anti-aliasing only.
- **Task modal and due dialog:** 0 console errors. The only screenshot difference is the expected "(Done)" kept by KF-402.
- **Statistics, board settings General/Colors:** 0 console errors, 0 failed requests. The one exception is the intended 400 on KF-396.
- **API smoke:** 20 create/update/delete cases with good and bad payloads give identical statuses on both builds.
- **Upgrade path:** board, calendar and modal pages return 200 on a baseline DB holding legacy garbage due strings.

### Observations (candidate new defects)
1. Pre-existing, not from this batch: `format_due_card_weekday_near_absolute_far` and `format_due_modal_relative_and_done_marker` fail under TZ=Europe/London and America/New_York on both baseline and fix. They pass only under TZ=UTC.
2. The new test `env_token_works_before_setup` calls `std::env::set_var` on a process-wide variable while other tests run in parallel. Low risk.
3. The tracker has a label conflict. KF-296 shipped "Pomodoros" plus a backfill, but the only surviving KanbanFlow reference shows "Pomodori". DBs that ran the backfill will mix spellings with new template boards. Recommend reopening KF-296.
4. Both builds accept the RFC3339 leap second "2026-12-31T23:59:60Z" and lowercase "t…z". JS `Date` may not parse the leap second; I did not check that in the UI.
5. Both builds count stopped pomodori in stats `total_pomodori` and the timer pill (2), while the task's `pomodori_completed` counts only completed ones (1).
6. Both builds: a task created directly in a Done column gets no `completed_at`, so it lands in the "Completed" group.
7. Legacy unparseable due strings written by older builds remain in the DB with no cleanup.
8. The Colors settings layout still differs a lot from the KF-331 reference: rows about 71 px against 47 px, no Color/Name/Actions header row, different buttons. The content is right; the layout belongs to the KF-331 work.
9. A future grouping_date sorts above Today. No KanbanFlow capture to confirm either way.

DONE: KF-374 PASS 10 MiB file 200, 10 MiB+1 400, body limit exact (limit 400 from the handler, limit+1 413); other routes keep 2 MiB (2 MiB+1 413); baseline 413 above ~1.5 MB
DONE: KF-396 PASS blank, whitespace and 101-char names return 400 and re-render the General tab with the message and the stored name (escaped); baseline 500
DONE: KF-400 FAIL garbage and gap times now 400 and every valid format still works through the API and the dialog, but local times at the exact DST second are stored an hour off or accepted (London "2026-10-25 02:00" stored as 01:00Z; "2027-03-28 01:00" accepted; NY same)
DONE: KF-402 PASS a re-sent same instant (.000Z, offset or local form, also via the dialog) keeps due_done and the stored string; a real change still resets it; baseline reset it
DONE: KF-391 PASS best_day is null with no pomodori and Highscores shows "No data to display"; data case unchanged; baseline showed "0 Best day"
DONE: KF-372 PASS template and fresh General board get "1 Pomodoro (Default)/2 Pomodori/3 Pomodori/>3 Pomodori" with the other 6 disabled, matching the KF-331 KanbanFlow capture ("Pomodori" is correct); existing boards and user templates untouched on a baseline DB
DONE: KF-387 FAIL grouping_date now picks the Done group and null clears it, but sorting makes on-screen order differ from board order, so a drag within a Done group can be silently undone after reload (G-B below G-C reverts; baseline moves it)
DONE: KF-397 PASS before /setup only the configured CHIPFLOW_API_TOKEN gets through (random, prefix, case-changed and empty tokens 401; no-auth, cookie-only and HTML 303 /setup); no token can list, mint or revoke before or after setup; revoked and faketime-expired tokens 401
DONE: KF-399 PASS spec now 83 ops including GET card, modal and timer/settings with correct 200/404/401; no /api route missing (HTML-page policy still undecided)
DONE: KF-398 FAIL most edited claims verified against the binary, but two describe behaviour that does not exist: "a time skipped by spring-forward is a 400" ("2027-03-28 01:00" gets 201) and "grouping_date anything else is a 400" ("2026-1-5" gets 200)
