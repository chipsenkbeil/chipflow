## verify-c3 report: integ/batch-33 at bd37f6c

**Verdicts:** KF-400 PASS, KF-387 PASS, KF-398 FAIL (LOW, one narrow docs-vs-binary contradiction about the year in due dates), REGRESSION PASS, B-IDENTITY PASS. Two pre-existing due-dialog bugs are worth filing; they are in the observations.

### Environment
- **Fix tree:** worktree `.dev/worktrees/verify-c3`, detached at bd37f6c. The diff from ffdce86 touches only the expected files (Cargo.toml/lock, agent-guide.md, agents/skill.md, src/auth.rs, db.rs, routes.rs, static/app.js, style.css, templates/board.html, board_settings.html, tests/db_colors_templates.rs).
  - bd37f6c differs from fa408bc only in server/docs files, plus 12 lines of app.js (the KF-391 Highscores change) and one line of board_settings.html (the KF-396 error).
- **Other trees:**
  - Baseline: `.dev/worktrees/verify-c3-base` at ffdce86.
  - B reference: `.dev/worktrees/verify-c3-b` at fa408bc.
  - Extra: `.dev/worktrees/verify-c3-pre6c` at 43f4e25, the tree just before 6c91aaf, used to show the cross-board bug before the fix.
- **Debug binary md5:** fix fa9b1f5242cca863a7d43343708d0e38, base abc1cb1a9a2a144b795c117fd92ec73c, B 88f8e98da859c004beb8af559952718b.
- **Ports and DBs:** 3327 fix, 3328 base (and briefly pre6c), 3329 B and fresh-DB probes. Fresh DBs at `/tmp/chipflow-verify-c3-<tag>-<port>.redb`.
- **Viewports:** 1905x1000 for the macOS Playwright runs. 1919x998 at DPR 1 in my own Linux container `vc3-cap` (same pinned image digest).
- **Cleanup:** all servers killed by PID, container removed, 0 console errors and 0 failed requests in every Playwright run. Server logs show 0 panics or errors.
- All files below are under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-c3/`.

### Gates on bd37f6c (`gates.log`)
All pass:
- fmt rc 0
- build rc 0
- clippy --all-targets rc 0, with 0 warnings after a forced re-check
- `TZ=UTC cargo test` rc 0: 122 passed, 1 ignored, 0 failed
- jsc checkSyntax on app.js rc 0

### A. KF-400 (due dates at DST transitions and seconds = 60)
POST and PATCH were probed for every value; the PATCH sentinel stays unchanged on every 400.

| Claim | Baseline | Fix |
|---|---|---|
| London 2026-10-25: 00:59:59 / 01:00:00 / 01:00:01 / 01:30 / 01:59:59 | 00:59:59 resolves; the rest are stored verbatim (`"2026-10-25 01:00:00"`, etc.) | all resolve to `+01:00`, the earlier instant |
| London 2026-10-25 02:00:00 / 02:00:01 | 02:00:00 stored verbatim | `+00:00` |
| London 2027-03-28: 01:00:00 / 01:00:01 / 01:30 / 01:59:59 / `T01:00` | 201, stored as `+00:00` or verbatim | all 400 |
| London 2027-03-28 00:59:59 and 02:00:00 | resolve | `+00:00` and `+01:00` |
| New York 2026-11-01 01:00:00–01:59:59 (repeated hour) | stored verbatim | `-04:00`, the earlier instant; 02:00:00 gives `-05:00` |
| New York 2027-03-14 02:00:00 / 02:00:01 / 02:59:59 (skipped hour) | 201 | 400; 03:00:00 gives `-04:00` |
| Seconds = 60 under UTC, London and Dublin (local, `T` local, `Z`, `+01:00`, `.5Z`, `23:59:60.999999999Z`) on POST and PATCH | 201, stored | all 400 (`sec60.log`) |
| Seconds :59, and valid RFC3339 stored exactly as sent (`.999Z`, `.123456789+05:30`, `-00:00`) | n/a | accepted, byte-identical |

- **Transition month:** New York has no DST change in 2026-10; its fall-back is 2026-11-01, so I tested that date and also 2026-10-25 New York (normal, `-04:00`).
- **Browser views (Playwright, timezone Europe/London; `shots/a-views-3327.json`):** for 01:30 first occurrence, 01:30 second occurrence (`+00:00`), 00:59:59 and 02:00, the card, the card title, the modal, the due dialog (date/time and selected calendar day) and the calendar chip all agree.
  - Example: `2026-10-25T01:30:00+01:00` shows "25 October 1:30 AM" on the card and modal, "2026-10-25 / 01:30" with day 25 selected in the dialog, and "October 2026: day 25" on the calendar.
- **Baseline reproduces the bug:** the verbatim `"2026-10-25 01:30"` value is missing from the card and modal but present on the calendar (`a-views-3328.json`).
- **Break-attempts:**
  - Dublin, which tzdata models with negative DST: same results as London.
  - Lowercase `t`/`z`, and a space instead of `T` in RFC3339: accepted verbatim, and every view parses them.
  - `+0100` without a colon, `:61`, minute 60, hour 24, `2026-02-29`, fractional seconds in local form: all 400.
  - A bad `due_at` on POST creates no task (74 tasks before and after).

### B. KF-387 (Done date groups and drag positions)

| Claim | Baseline | Fix |
|---|---|---|
| `grouping_date` decides the Done group | ignored: all tasks under "Today" | groups Today / Wednesday 14 October / Monday 12 October |
| `null` clears | ignored (value kept) | clears; `""` clears too |
| Strict zero-padded YYYY-MM-DD | stores `2026-1-5`, timestamps, `2026-02-30`, `+2026…`, `02026…`, `26-10-05`, `2026/10/05`, `20261005` | all 400; `" 2026-10-06 "` is trimmed; an invalid value alongside a valid `due_at` writes nothing |
| Real SortableJS drag inside a group persists after reload | n/a | G14c before G14a gives 14 Oct: G14c, G14a, G14b after reload; also 12 Oct and Today drags (`shots/k387-drag1..d5.json`) |
| Cross-board API move into Done with `position` and no `swimlane_id` lands beside the card | pre-6c91aaf tree: XT1 (position 2) and XT3 (position 1) land at the top of Today; XT2 (position 5) lands at the top of 14 Oct | XT1 between Launch and Set up; XT3 between Migrate and Launch; XT2 (14 Oct) between G14b and G14c |

- **Fixture:** the board order was deliberately interleaved (G14a, G14b, G12b, G14c, Launch, Setup, Migrate, G12a), so a wrong translation would show.
- **Break-attempts:**
  - Moved the second lane to first place: the cross-board drop translated in the new first lane (XT4 at position 0 before L2a; XT5 at position 3 after L2b).
  - Dragged a card into another group's slot: it keeps its place, as documented.
  - Position 99 from another cell joins the top of its group; position 1.6 rounds to 2.
  - Dragging within a normal column still persists.
- **UI cross-board path:** Move advanced offers only Board and Column, no position, so there is no drop-beside-a-card in the UI. It sends the target count or 1e9, and the server puts the task at the top of its group.

### C. KF-398 (agent docs vs binary): FAIL, LOW
- **Matches:**
  - The served `/agents.md` is byte-identical to agent-guide.md, and `/agents/skill.md` to agents/skill.md.
  - The London DST example is exact.
  - Seconds 00–59 in every form.
  - Single-digit day/hour/minute/second forms work (`2026-1-5`, `2026-10-20 8:5:7`, `T8:05`); RFC3339 needs zero padding (`T8:05:00Z` and `2026-1-05T…Z` are 400).
  - The grouping_date rules.
  - The Done position wording (neighbor in its own group, keeps its place, top of group from another cell, rounded and clamped).
  - KF-334: a cross-board move with no lane, Done or not, lands in the target board's first lane.
  - The task detail keys are exactly as documented.
  - Comment and attachment times are RFC3339; timer status `started_at` is Unix seconds.
  - The attachment upload returns exactly the documented keys.
  - A stopped pomodoro gives `pomodori_completed` 0 with 1 interruption; `completed:true` gives 1.
  - The modal shows "✓ Done (Oct 16)" and the card has no stamp.
  - `best_day` is null.
  - The env token works before setup and cannot mint tokens.
  - The OpenAPI move summary, position and grouping_date descriptions.
- **Contradiction (the reason for FAIL):**
  - The docs say every field but the year may drop its leading zero (agent-guide), and that leading zeros are optional except in RFC3339 and the year (skill.md). Both say anything else is a 400.
  - The binary accepts `26-10-20` and `026-10-20` as year 0026, stored as `0026-10-20T23:59:59-00:25` under Dublin. The offset is local mean time, and to_rfc3339 truncates its seconds.
  - On the card this shows as "20 October 11:59 PM" with no year, and the dialog shows `26-10-20`.
  - It also accepts `+2026-10-20` and a tab or double space as the separator.
  - Same on baseline, but the docs written in this batch now promise a 400. An agent sending `26-10-20` silently gets the year 26.
  - Fix: reject years that are not 4 digits, a sign and other whitespace in `normalize_due_input`, or correct both doc sentences.
- **Omission:** the OpenAPI `due_at` descriptions do not mention the DST 400 or the `:60` 400. Strict RFC3339 allows a leap second, so the spec under-describes; the info block points to /agents.md.

### D. Regression (fix vs baseline)
- **KF-374:** 10,485,760 bytes returns 200 and the download md5 matches; one byte more returns 400 "exceeds the 10 MiB limit". Baseline: 413.
- **KF-396:** a blank or 101-character name returns 400 with the General tab re-rendered and the message; 100 characters is accepted. Baseline: 500.
- **KF-402:** re-sending the same instant as `.000Z` or `+01:00` keeps `due_done` true; a different instant clears it. Unchanged re-save through the real UI dialog also keeps it. Baseline loses it.
- **KF-391:** `best_day` null on a fresh DB, and Highscores shows "No data to display". Baseline: best day 0 for Friday.
- **KF-372:** the Pomodoro template's enabled colors are 1 Pomodoro (Default), 2 Pomodori, 3 Pomodori, >3 Pomodori; the other 6 are disabled. Baseline: 10 plain colors.
- **KF-397:** before setup, a Bearer token gets 200 on GET and POST and 403 when minting a token; no auth gets 303 to /setup and a bad token 401. Baseline: 303 for all.
- **KF-399:** the card, modal and timer-settings GET routes are in the spec (83 operations; baseline 80).
- **Batch-32:**
  - The card Move submenu is Move right / Move to bottom / Move advanced; the parent menu stays open, and Move right persists into the next column.
  - Board delete shows the styled confirm "Delete board"; Cancel closes it with 0 requests and 0 native dialogs.
  - On a lane board every row has its label cell (the main lane uses the hidden "Default" gutter) and the cells align with the headers (`shots/b32-fix-*.png`).

### E. B-IDENTITY
- **Captures:** bd37f6c on 3327 and fa408bc on 3329, fresh DBs seeded identically with `--as-of 2026-10-16`, in the Linux container at 1919x998:
  - default seeded board
  - board with a collapsed column
  - lane board ("Default" plus "Lane Two" with tasks), with "In Review" collapsed and a long column name
  - Edit layout open on the default board
  - Edit layout open on the lane board
- **Result:** ImageMagick AE is 0 for the board region (0,75, 1919x923) and 0 for the full frame in all 5 pairs; the PNGs are md5-identical. Files are in `lin/`.

### Observations (not caused by this batch)
1. **Due dialog drops seconds on save.** Opening the dialog and pressing Add unchanged on a due with seconds (for example `T10:00:59Z`, which KF-400 now accepts) saves `T10:00:00.000Z`, which clears `due_done`. Same on baseline. Severity MEDIUM. Evidence: the c3resave output in this session (secs59 task).
2. **Due dialog moves the second occurrence of a repeated hour.** For `2026-10-25T01:30:00+00:00`, pressing Add unchanged sends `00:30Z`: the due moves one hour earlier and `due_done` clears. The client rebuilds the time from the local wall clock. LOW.
3. **Long column names collide with the add button.** The column title's ellipsis runs under the green "+" add button. It is on baseline too, so it is not a B regression (`lin/zoom-longname.png`, `lin/base-lanes.png`). LOW; relevant to the KF-223/KF-358 verifier.
4. **The main swimlane is still identified by the name "Default"** (the known KF-359 caveat).

DONE: KF-400 PASS (DST seconds before, at and after each transition under London, New York and Dublin resolve to the earlier instant or 400; :60 is a 400 in local and RFC3339 on POST and PATCH under UTC, London and Dublin; :59 accepted; RFC3339 stored byte-exact; card, modal, dialog and calendar agree near the transition; baseline stores verbatim)
DONE: KF-387 PASS (grouping_date drives the groups and null clears; strict YYYY-MM-DD; real SortableJS drags in Done persist after reload; cross-board API drops land beside the target card in the target board's first lane, including after a lane reorder; the pre-6c91aaf tree lands them at the top)
DONE: KF-398 FAIL (LOW: docs say the year may not drop leading zeros and anything else is a 400, but the binary accepts "26-10-20"/"026-10-20" as year 0026, plus "+2026-10-20" and tab separators; all other items match: DST, seconds, grouping_date, Done position, cross-board lane rule)
DONE: REGRESSION PASS (KF-374, 396, 402, 391, 372, 397, 399 hold vs baseline; batch-32 Move submenu, styled delete confirm and lane label cells work; 0 console errors and 0 failed requests)
DONE: B-IDENTITY PASS (0 differing pixels and md5-identical PNGs, board region and full frame, bd37f6c vs fa408bc at 1919x998 in Linux: default, collapsed column, lanes with collapsed column and long name, Edit layout x2)
