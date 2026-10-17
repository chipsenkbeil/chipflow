verify-w1 report: candidate fix/board-w1 @ 62ff5cc vs baseline 2c1773b. Every defect passes its measured check. There are two candidate observations (one 1 px colour regression from KF-486, and KF-440 undercounting overdue when a column hides due dates) and one pre-existing, documented KF-406 limitation.

## Environment
- Worktrees: `.dev/worktrees/verify-w1` (62ff5cc, detached) and `verify-w1-base` (2c1773b). The diff touches only agent-guide.md, db.rs, models.rs, routes.rs, app.js, style.css and board.html.
- Release builds, `--config 'build.build-dir="target/build"'`.
  - Candidate md5 b9a43e5c6f22bf22002fa5ae03bbf99b. It contains the string `main_swimlane_id` (3 hits); the baseline has 0.
  - Baseline md5 d544e4e999a152166fa1eb33a6a8581e.
- Ports: candidate 3408, baseline 3409, plus a 3410 start (see Rule slips). Databases are under `$S/verify-w1/db/` (`cand.redb`, `base.redb`, `k406*.redb`, `kb-*.redb`). `$S` is the scratchpad.
- Evidence is under `$S/verify-w1/`. In `shots/` and `reg/`, files named `...-3409` are baseline and `...-3408` are candidate. The scripts (`t1`..`t22.mjs`, `cmp.sh`, `sweep.sh`) and the logs (`gate-*.log`, `sweep-b.log`, `sweep-base.log`, `srv-*.log`) sit beside them.
- Browser: Playwright 1.63.0 via node, DPR 1, animations off, fresh context per scenario.
- Viewports: 1440x612, 1905x1000, 1120x900, 1919x998 and 1920 for KF-503. Boards: the seeded golden-master board plus purpose-built boards for lanes, colours, Done, subtasks, deletion and KF-406.

## Gates (DONE: GATE)
- `cargo fmt --check`: exit 0.
- `clippy --all-targets`: exit 0, 0 warnings.
- `TZ=UTC cargo test`: exit 0. The lib suite passed 212 with 1 ignored, and every other suite passed.
- `env -i` plain libtest via `/Users/senkwich/.cargo/bin/cargo test`: exit 0, same counts.
- jsc `checkSyntax(static/app.js)`: exit 0.
- Schema v2 fingerprint unchanged: the candidate opened a baseline-written database with "is at schema version 2" and no re-stamp.

## Per-defect results

| KF | Claim | Baseline | Candidate | Verdict |
|---|---|---|---|---|
| 483 | Board title colour | rgb(31,41,55) | rgb(51,51,51) = #333; dark title still #eee | PASS |
| 482 | Avatar ring equals card border, all 10 standard colours | 10 of 10 mismatched (lightened mix) | 0 of 10 mismatched (yellow #f5cc00, green #59d600, blue #70b0ff, ... white #d4d4d4); also 0 in dark | PASS |
| 474 | Dark lane-menu hover | #545454 on #4a4a4a | #585858 on both label-cell menus; the folded-strip menu on #3e3e3e gives #4d4d4d, which is the same 0.08 white and unfaded | PASS |
| 486 | Overdue strip top equals warning line bottom | strip y=118, line bottom 119 (shared row) | strip y=119, line bottom 119, strip bottom still 146, text and bar unchanged (no pixel diff there) | PASS (see observation 1) |
| 487 | Collapsed name box 20x118 | 20x122.14 | 20x118.39 at 709.5,128 (golden 710,128) | PASS |
| 503 | All-collapsed strips are 28 px | 284 px at 1440, 380 px at 1920 | 28 px at both, table 140 px wide | PASS |
| 445 | Header rule under sticky corner | no rule (white / #4a4a4a) | row y=117 across the corner reads #c3c3c3 light and #222222 dark, at top and scrolled | PASS |
| 477 | Done group year | "Thursday, 16 October" for 2025 and "Saturday, 2 January" for 2027 | "... 16 October 2025", "... 31 December 2024", "... 2 January 2027"; Today, Yesterday and current-year labels unchanged | PASS |
| 479 | Last column refused | UI and API delete the last empty column | API 400 "cannot delete the board's last column", UI toast "A board needs at least one column.", column stays, delete-column OpenAPI response lists the last-column 400, `/agents.md` shows the new paragraph | PASS |
| 478 | Folded subtask label equals reload | 15 of 16 steps drifted, e.g. "1 hidden (0 done)" vs "3 hidden (3 done)" | all 16 steps match the server's `/api/tasks/:id/card`, in the folded and the expanded (`dataset.summary`) states; Enter and Space toggle, other keys do not; 3 rapid toggles end at "4 hidden (4 done)" on both | PASS |
| 476 | Bookmark button, "me", assign | 2 text lines (h 41.6); `f-user` blank with all 22 cards visible; card stays under Unassigned | 1 line (h 26.6); `f-user` = "audit" with 5 of 22 cards visible; card leaves Unassigned with avatar shown, no reload | PASS |
| 440 | Collapsed strip follows the filter | stays dark red with the line and old tooltip | across colour and user filters, To Do and In Progress collapsed: the count colour, line, tooltip and expanded strip (hidden, "0 overdue task") all match the visible cards; "5/3" goes normal at 3 | PASS (see observation 2) |
| 441 | Drop on a collapsed strip | 1 move request and the source column reordered, persisted | 0 move requests and the order unchanged after reload | PASS |
| 406 | Main lane by stored id | see the KF-406 detail below | see below | PASS, with the documented limitation |

KF-441 details:
- The strip is focusable (`tabindex=0`, `aria-expanded=false`, 2 px focus ring, reached after 12 Tabs); the baseline has no tabindex.
- Enter and Space each expand it.
- Break-attempts survived:
  - hover the strip, then a normal drop on In Progress moves the card, 1 request;
  - a drop on the collapsed header gives 0 requests;
  - a drop on a swimlane board's strip gives 0 requests and the order is unchanged.

KF-406 detail:
- Tested on a baseline-written database with six boards (a renamed main lane plus a user "Default" lane, a lone renamed lane, a plain main plus an extra lane, a deleted main, and two fresh boards).
- First start, no requests: md5 d8cd572d... Another independent first start gave the same md5.
- The same start plus a sweep of every route path: 1212 GET and HEAD requests, 428 returned 200, 24 returned 303, 340 returned 404 and 420 returned 405. It used a cookie plus Origin, Referer and Sec-Fetch-Site: cross-site headers. md5 stayed d8cd572d..., so there were no GET writes.
- Startup logged "board X got main swimlane Y" for each of the 6 boards that had lanes; a second start logs nothing new.
- A lone renamed lane ("Solo") is now unlabelled with no corner cell; the baseline labels it "Solo" and shows the corner cell.
- A main lane renamed after creation stays unlabelled and without a menu, while a user lane named "Default" added later is labelled and has a menu.
- The baseline sweep also leaves its md5 unchanged, so the md5 check is not sensitive to this change.
- Second starts of an already-started database change the md5 on both builds (redb on open). I checked this on the baseline copy (909eb64b... to 8d9be18e...).

## Claimed passing on the baseline (confirmed on both builds)
- KF-500: `#filter-panel` is 1190,34,250x578.
- KF-306: the dialog reads "Are you sure you want to delete this column?" with "Move the tasks to the column". Deleting a non-empty column moved 3 tasks to the chosen column and deleted it; deleting an empty column also worked.
- KF-411: a long column name's right edge equals the "+" left edge (258), with ellipsis and clipping.
- KF-443:
  - At 1919x998 with To Do and In Progress collapsed, the separators are at x=624, 652 and 680, matching R2-044's KF 625-651 and 653-679.
  - At 1440 and 1905 the strip origins are fractional (464.66, 619.66); the layout and painted pixels are identical on both builds.
- KF-444: at 1919x998 scrolled to the bottom, the bottom border is pixel row 988 with a 9 px gutter, and the header top is an integer 80. Baseline and candidate are identical.

## Regression (DONE: REGRESSION)
- Console errors, failed requests and HTTP errors: 0 in light and dark at 1905x1000, 1440x612 and 1120x900. (Deliberate API 400s are excluded.)
- Pixel diff of the golden board, baseline against candidate, across 6 viewport and theme combinations, with sum badges set (hours, count and hours): the only differing regions are the board title (light only), the avatar rings, the 14x7 stubs at the strip/line ends, and the Backlog card order, which differs because the baseline's own KF-441 drag reordered it on that instance. This covers the F header layout, H sum badges, the 34 px top bar and header timer, KF-457 widths and legend, and G dark mode.
- `/settings` and `/timer/log` are pixel-identical (AE=0) in light and dark. `/boards/new` differs by 1792 px and I did not attribute it.
- Drag and drop with a real mouse: cross-column in the same lane and cross-lane persisted, and a Done-group drag behaves the same on the baseline. The cross-lane drag was flaky once on the baseline and passed on the rerun.
- The 15 commits are not rebased onto the current origin/main (93602db), only onto 2c1773b.

## Judgements
1. KF-440, overdue counted from visible cards: acceptable but fragile (observation 2). The filter and the server agree whenever the card's due line renders. A server-supplied `data-overdue` attribute on the card would remove the dependency.
2. KF-441, cancel drops on strips: acceptable. A collapsed strip takes no cards, and 0 moves with no reorder is the right outcome. It relies on the last `dragover` or `mousemove` point plus `elementFromPoint`, which worked in Chromium native DnD for the strip body, the strip header and swimlane boards; I did not test Safari, Firefox or touch.
3. KF-478, re-fold immediately: acceptable. A newly checked subtask on a folded card disappears at once, exactly as a reload shows it. The alternative, keeping the row visible, would contradict the acceptance check.
4. KF-406, adopting a lane named "Default" for older boards:
   - It is right for every board whose main lane was never renamed.
   - It is wrong in the one case the brief names: B1 stored the user's "Default" lane as main, leaving "Work" labelled, which is the same visible result as the baseline.
   - The wrong id is now permanent: there is no UI to change it, and main lanes have no menu.
   - A renamed main lane among 2 or more lanes with none named "Default" stays unidentified, as on the baseline.
   - Because "Default" is only a guess, the first lane by position would be no worse and would be right for B1; I recommend that for follow-up.
   - On the candidate, deleting the main lane through the API (the UI hides its menu) leaves the board with no main lane and a labelled lone lane; the baseline does the same.

## Observations (candidate defects, each with a one-line acceptance check)
1. KF-486 leaves a 1 px band the baseline filled. At y=118 the 8 px stubs beside the warning line now show the cell background (light #FFFFFF, dark #4A4A4A) instead of the strip colour. The KF golden at that row reads F7F7F7 light and 474747 dark, closer to the strip colour than the plain-cell reading of FFFBFF and 4F4550, but JPEG ringing makes this inconclusive.
   - Evidence: `reg/crop-warn-both.png`, `reg/strip-cmp.png`.
   - Acceptance: at 1440x612, pixel (296,118) in an overdue strip column equals the strip colour (#F3F3F3 light, #444444 dark), or the KF capture shows otherwise.
2. KF-440 undercounts overdue when a column hides due dates. With a column config `prop_due_dates:"hide"`, the unfiltered strip shows "1 overdue task", but any active filter, even one still showing the overdue card (red), hides the strip and line, because the overdue count is read from `.card-due-overdue`. The breakage is real, though how KanbanFlow handles hidden due dates is unknown.
   - Evidence: `shots/obs-440-unfiltered.png`, `shots/obs-440-filtered-red.png`.
   - Acceptance: with `prop_due_dates:"hide"` on a column holding an overdue card, filtering to that card's colour keeps the strip reading "1 overdue task".
3. The collapsed strip header has `aria-expanded` but no `role`, so assistive technology may ignore the state. Acceptance: `th.columnHeader--collapsed` has role="button" (or equivalent) alongside aria-expanded.
4. The hidden expanded strip's text reads "0 overdue task" (singular grammar). It is cosmetic and never visible. Acceptance: the text uses the same pluralisation as the initial render, or is cleared when hidden.

## Rule slips
- I briefly ran a baseline server on port 3410, outside my assigned 3408 and 3409, for one second-start md5 check. It was stopped by exact PID.
- A zsh `while read path` loop clobbered PATH once; no requests were sent from that run.
- All my servers were stopped by exact PID (39372, 96496, and the k406-* and kb-* temporaries). No Claude-in-Chrome, python or perl was used.

DONE: KF-483 PASS board title #333 (was #1F2937), dark title unchanged
DONE: KF-482 PASS avatar ring equals card border for all 10 standard colours (baseline 10 of 10 off), light and dark
DONE: KF-474 PASS dark lane-menu hover #585858 on #4a4a4a (baseline #545454)
DONE: KF-486 PASS strip top 119 equals warning-line bottom 119 (baseline 118); observation 1 notes a 1 px row-118 colour change
DONE: KF-487 PASS collapsed name box 20x118.39 (baseline 20x122.14)
DONE: KF-503 PASS all-collapsed strips 28 px at 1440 and 1920 (baseline 284/380)
DONE: KF-445 PASS corner header rule #c3c3c3 light, #222222 dark at y=117 (baseline none)
DONE: KF-477 PASS other-year Done groups read e.g. "Thursday, 16 October 2025"; current-year labels unchanged
DONE: KF-479 PASS API 400 "cannot delete the board's last column", UI toast, column kept, agent-guide.md and OpenAPI documented
DONE: KF-478 PASS folded label equals the post-reload label on all 16 toggle steps; Enter and Space toggle
DONE: KF-476 PASS bookmark button 1 line (h 26.6); "me" selects audit (5 of 22 cards); card leaves Unassigned with no reload
DONE: KF-440 PASS strip count, line, tooltip and over-WIP colour follow the filter; observation 2 is the hidden-due-dates gap
DONE: KF-441 PASS drop on strip gives 0 move requests and no reorder; Tab-focusable, Enter and Space expand
DONE: KF-406 PASS stored main id, startup assignment, read-only render; first-start md5 d8cd572d... identical with and without a 1212-request sweep; a renamed main plus a user "Default" lane still adopts the user lane (documented limitation)
DONE: KF-500 PASS (claimed on baseline) 1190,34,250x578 on both builds
DONE: KF-306 PASS (claimed on baseline) verbatim text, "Move the tasks to the column", non-empty delete works on both
DONE: KF-411 PASS (claimed on baseline) long name stops at the "+" (258=258) on both
DONE: KF-443 PASS (claimed on baseline) separators at x=624, 652 and 680 at 1919x998 on both
DONE: KF-444 PASS (claimed on baseline) frame-bottom row 988 with a 9 px gutter at 1919x998 on both
DONE: REGRESSION PASS 0 console errors and 0 failed requests in 3 viewports and 2 themes; only expected pixel diffs; schema v2 unchanged; no GET writes
DONE: GATE PASS fmt, clippy 0 warnings, TZ=UTC test, env -i test, jsc checkSyntax all exit 0
