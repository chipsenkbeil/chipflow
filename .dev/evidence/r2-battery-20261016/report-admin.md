I could not write `$R/results/admin/report.md`. The harness refused the .md Write ("Subagents should return findings as text"), so the full report is below. Save it to that path as-is. Every evidence file it cites is in `$R/results/admin/`.

Result: 40 PASS, 20 FAIL, 4 N/A and 1 BLOCKED over 65 rows. The worst finding is a regression of RELEASED KF-304: the Delete board button throws `ReferenceError: showConfirmDialog is not defined`, so you cannot delete a board from the UI at all.

# R2 battery, batch `admin`

All paths are relative to `$R/results/admin/`, where `$R=/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/r2`.

## 1. Environment

- **Binary:** `.dev/worktrees/r2-main/target/release/chipflow`, origin/main `62d573d`, not rebuilt.
  - `/api/v1/version` returns `build_sha:"unknown"` because this local build did not set `CHIPFLOW_BUILD_SHA`. Production sets it in `homelab/modules/chipflow.nix:29`.
- **Port and PIDs:** port 3270, bound to 127.0.0.1. I killed each server by its exact PID after each phase. Nothing is listening on 3270 now.
- **Main DB:** `/tmp/chipflow-r2-3270.redb`.
- **Second DB:** `headless.redb`, started with `CHIPFLOW_API_TOKEN=cf_headless_bootstrap_test_0001`. I used it for the FRESH, headless and single-board states, then swapped back to the main DB.
- **Clock:** 2026-10-16, server TZ Europe/London (BST, +01:00). The seeder ran with its default `--as-of` (today), so fixture dates shift by +16 days.
- **Boards on the main DB:**
  - GM `52b5f56d-8ed3-4b6a-b543-5d78b7904ee5`
  - General `97bb03f3-f928-4715-844a-cdcde4ddf65f`
  - POMO `da0d052f-d94d-4655-b8f2-792705cb21f1` (Pomodoro board template)
  - "API Pomodoro board" `d4c76ede-b0d0-4f15-8c10-a3a0b9c349bc` (actually created from the Kanban basics template)
  - D1 `98436f65-fadb-4f7c-bb06-f494d3c61b23`, named `Dale's "fidelity" <test> & co`. It still exists because the UI delete is broken.
  - D2 `da5d9283-160d-42e2-b42a-cd1b7916a0a9`, "Archive me", archived then unarchived.
- **Fixture adjustments, all through the API:**
  - The seeder deletes Brown and White. I re-added both as disabled to match the inventory's GM profile.
  - I collapsed To Do through the API.
  - The LOG profile comes from `seed-log.sh` with real timer sessions:
    - two 1-minute completed pomodori on #11
    - pomodori stopped with reasons Email and Meeting
    - a 31 s stopwatch session
    - a short break attributed to break activity "Stretch"
    - manual entries on 10-12, 10-13 and 10-14, one with labels and a note
  - Output is in `seed-log.out`.
- **Browser:** Playwright 1.63.0, Chromium headless shell 153.0.8010.12, DPR 1, fresh context per scenario, motion reduced.
  - Viewport is 1440x900 unless a row says otherwise. Others used: 1100x900, 1905x900, 500x900, 400x800, and 767 to 1024 for R2-225.
- **Scripts and logs:**
  - API: `api-battery.sh` (logs `api-battery.log` and `api-battery2.log`), plus `token-auth.log` and `session.log`.
  - Browser drivers: `d-*.mjs` on the shared helper `lib.mjs`. Each scenario writes `shots/<name>.log.json` (console errors, page errors, failed requests, HTTP >= 400) and `shots/<name>-NN-*.png`.
- **Secrets:** raw tokens in `tok-*.json` are redacted.

## 2. Verdicts

| Row | Verdict | Evidence | Break-attempt (PASS) | KF ids |
|---|---|---|---|---|
| R2-146 | PASS | Board Menu > Reports lists the 15 KF-099 items verbatim and in order. Each item routes correctly (`rep-items.log.json`). The submenu opens on hover, a second click or ArrowRight. A click right after the hover toggles it shut (`submenu-*.log.json`). | Clicked all 15 items in fresh page loads. Tested at 1440 and 1905. | KF-099 KF-136 |
| R2-147 | PASS | Presets match the KF-092 list. Last 30 days shows 30 bars, Sep 17 to Oct 16, each with a count label. The tooltip reads `Friday 16 Oct 2026: 4 Pomodori (2m)`, which matches KF-293 (still a native SVG title). | Custom absolute single day, custom absolute empty range ("No data to display"), inverted range (server swaps it), custom relative 1 week, board filter set to General (empty). Totals match `/api/timer/statistics`. | KF-092 KF-293 |
| R2-148 | PASS | Interruptions tab: Meeting 1, Email 1. | The empty DB shows "No data to display". | KF-092 |
| R2-149 | PASS | Break activities tab: "Stretch 0m today 0m / 2m goal". | The empty DB shows "No break activities defined". | KF-347 |
| R2-150 | PASS | Highscores: "4 Best day — Friday, Oct 16" and "1 Longest streak (days)". | Matched against the API for the same range. | KF-092 |
| R2-151 | FAIL | On the empty DB, Highscores fabricates "0 Best day — Friday, Oct 16". The other tabs correctly show "No data to display". `shots/single-board-01-stats-empty.png` | | KF-092 |
| R2-152 | FAIL | The table, bars, % column, Tasks/Breaks/Duration columns and footer totals all render. The "Total: 5h 7m" line above the table is invisible: `.spent-total{color:#f3f4f6}` (`static/style.css:3723`) gives contrast 1.04. `shots/rep-spent-01-summary.png` | | KF-091 KF-287 KF-292 |
| R2-153 | PASS | The Export menu lists Excel (Detailed), Excel (Summary), PDF (Summary) verbatim. Both Excel files download with the right rows. PDF opens a print popup. | Opened every download and popup. | KF-294 |
| R2-154 | PASS | The Print sheet is titled "Print", has Filter and Print buttons and the table. `window.print()` is called once. Filter closes the sheet. | Stubbed `window.print` and counted calls. | KF-295 |
| R2-155 | FAIL | Summary/Detailed and Group by work. "Bookmarks (0)" is a static div with no handler. `shots/rep-spent-05-bookmarks-click.png` | | KF-091 |
| R2-156 | PASS | Created, renamed and moved events render with value diffs and working filters. The accepted gap holds: after the task is deleted, its events vanish and no deletion event appears. | A probe task was created, renamed, moved, then deleted, and the page reloaded. | KF-336 |
| R2-157 | PASS | Burndown: "25 tasks remaining." (29 tasks minus 4 done). | Checked against export-data. | KF-337 |
| R2-158 | FAIL | After renaming "In Review" to "Reviewing", the legend lists both names. The status says "29 tasks across 6 columns" on a 5-column board. The tooltip says "In Review: up to 0 tasks". `shots/br-cumflow-02-after-rename.png` | | KF-339 KF-350 |
| R2-159 | PASS | "4 tasks completed in the last 12 weeks". Both toggles render. | Matches Throughput. | KF-340 |
| R2-160 | PASS | Summary: 0 on time, 0 late, 1 overdue, 6 upcoming. | Tasks with garbage `due_at` were present. The page did not error and skipped them. | KF-341 |
| R2-161 | PASS | Three runs for 10 tasks put the 50% date at Mar 10, 18 and 19, 2027. The 95% date ranges from Nov 2027 to Jan 2028. Results vary per run, as accepted. | 0 tasks clamps to 1. 500 tasks also runs. | KF-342 |
| R2-162 | PASS | Color x column table, 29 tasks. | Sum checked. | KF-343 |
| R2-163 | PASS | 4 completed tasks, 12 weekly bars. | | KF-344 |
| R2-164 | PASS | 33h 00m across 6 tasks. | A task with its color cleared is listed under No color. | KF-345 |
| R2-165 | PASS | Excel, CSV, XML and JSON all download with 29 tasks (`shots/export-*`). | Opened each file. | KF-346 |
| R2-166 | N/A | Premium in KanbanFlow. The toast reads "Time in column report is not available in ChipFlow yet." | | KF-275 |
| R2-167 | PASS | Print calls `window.print()` once. | Stubbed `window.print`. | |
| R2-187 | PASS | Enabled: 8 colors in palette order, Yellow marked default. Disabled: Brown, White. The KF-086 intro text, Show more toggle, upsell box and "?" help icon are present. | Disabled Orange and Cyan. The disabled list became Brown, Cyan, Orange, White (alphabetical). | KF-303 KF-151 |
| R2-188 | FAIL | The Edit dialog opens prefilled with a 50-character limit, and Save persists. Cancel and × do nothing. Only Escape closes it. `bs-colors.log.json`, `shots/bs-colors-03-*` | | KF-084 |
| R2-189 | PASS | The Set default dialog lists the 8 enabled colors. Picking one moves the "(Default)" tag, and Cancel closes. | Changed the default to Green. The Task settings tab followed. | KF-083 |
| R2-190 | FAIL | Copy from board lists the other boards. With a single board it says "There are no other boards to copy from." A copy done through the UI reproduced the GM palette. With no board picked, the error is painted under the overlay, so the user sees nothing. `shots/bs-colors-06-copy-nopick.png` | | KF-083 |
| R2-191 | FAIL | The premium dialog shows the KF-085 text. Its Close button does nothing. Escape and a backdrop click do close it. | | KF-085 |
| R2-192 | FAIL | Disable and Enable work. Drag reorder does nothing, before or after reload. `Sortable.get(tbody)` is undefined because the inline script runs before the deferred `Sortable.min.js` loads (`base.html:9`). `colors-followup.log.json` | | KF-082 |
| R2-193 | FAIL | Save shows "Board settings saved." and the rename persists. A spaces-only name passes the browser's required check. It, or any name over 100 characters, returns a bare HTTP 500 text page. `shots/bs-general-03-blank.png` | | KF-080 KF-153 |
| R2-194 | PASS | Layout shows WIP 3 on In Progress, Done = Yes on the Done column, Collapsed = Yes on To Do, and the Default swimlane. | Collapsed To Do through the API. A column created through the API also showed up. | KF-080 |
| R2-195 | PASS | Task settings shows Yellow as default color and Done as the Done column. | After changing the default to Green, the page showed Green. | KF-080 |
| R2-196 | PASS | Board ID, Board position 0, and a Danger zone link to `/settings/delete`. | Followed the link. | KF-080 |
| R2-197 | N/A | Shows API pointers and a no-webhooks note. | | KF-080 |
| R2-198 | N/A | Points to the API tokens page. | | KF-080 |
| R2-200 | FAIL | Archive works end to end: the board moves to `/boards/archived`, leaves the lists, and Unarchive restores it. Delete board throws `ReferenceError: showConfirmDialog is not defined` and does nothing. The function is private to the app.js IIFE, and the page has no `#confirm-dialog` markup. The last-board refusal exists only in the API (400). `del-board.log.json`, `single-board.log.json` | | KF-089 KF-304 KF-305 KF-139 |
| R2-201 | PASS | Timer, Alerts, Interruption reasons and API tokens render. Saves persist: pomodoro set to 30, a new reason trimmed, blank lines dropped. | The form kept `alarm_sound` and `break_activities` that were set through the API. Observation 3 covers the flags it does clear. | KF-074 KF-105 KF-106 |
| R2-202 | PASS | Redirects to `?saved=1` and shows "Settings saved." | Saved an out-of-range value (observation 4). | |
| R2-203 | PASS | A `cf_` token is shown once, listed immediately, the form resets, and the secret is gone after reload. | A read-only token gets 200 on GET and 403 on POST. Creating with no scope shows an error. | KF-150 |
| R2-204 | FAIL | The styled confirm appears, OK revokes, and the token then gets 401. Cancel does nothing; only Escape closes it. `us-tokens.log.json` | | KF-146 |
| R2-205 | PASS | Scopes display correctly. Expiry Never shows "never" and 30 days shows 2026-11-15. Last used goes from "—" to 2026-10-16. | Used the token, then reloaded. | |
| R2-206 | PASS | On a fresh DB everything redirects to `/setup`. All three validation messages appear. Success goes to `/login`. | Removed the HTML constraints to test server-side validation. Also tried a spaces-only username. | |
| R2-207 | PASS | Once the admin exists, GET and POST `/setup` redirect to `/`. A second-admin POST creates no account; that login fails. | Tried the second-admin POST. | |
| R2-208 | FAIL | Valid login works. A bad password or unknown user shows "Invalid username or password." and sets no cookie. Every `/login` load logs 2 console errors (401 on `/api/timer/settings` and `/api/timer/status`). | | |
| R2-209 | PASS | Sign out clears the cookie, Back stays on `/login`, and a replayed cookie gets 401 (API) or 303 (page). The `Secure` flag is set only under X-Forwarded-Proto or Forwarded https. | Replayed the cookie after logout. Ended the session mid-use (observations 1 and 2). | KF-195 |
| R2-210 | FAIL | Before `/setup`, every request, `/api/*` with the valid env token included, gets 303 to `/setup`. After setup the env token works and still cannot mint tokens (403). | | |
| R2-211 | FAIL | The discovery files are public, identical to the repo files, and their links resolve. Their content is wrong in 6 places (see the FAILs list). | | KF-220 |
| R2-212 | FAIL | OpenAPI has 80 operations against 106 in the router. It omits GET `/api/tasks/{id}/card`, GET `/api/tasks/{id}/modal` and GET `/api/timer/settings`. `route-ops.txt`, `openapi-ops.txt` | | |
| R2-213 | PASS | Version is public and also answers HEAD. | Sent a bad Bearer token to it. | |
| R2-214 | PASS | Bad or revoked tokens get 401. A read-only token gets 403 on writes. A write-only token can read. A token cannot mint, list or revoke tokens (403), even with a valid cookie alongside it. `token-auth.log` | Sent Bearer and cookie together. The expired-token path was only checked in code and its unit test; the minimum expiry is 1 day. | KF-146 |
| R2-215 | FAIL | RFC3339, `"YYYY-MM-DD HH:MM"`, bare local datetimes and bare dates are all stored correctly. Strings like "next tuesday", "garbage" and "2026-13-45" are accepted and stored verbatim, and the card then shows no due date. `api-battery.log` | | KF-226 KF-348 KF-207 |
| R2-216 | PASS | Setting WIP to null clears it. Collapsed persists. Deleting a non-empty column returns 400. Deleting the last swimlane returns 400. | Tried both deletes on a scratch board. | KF-228 |
| R2-217 | PASS | Comments, labels, members, manual time, time entries, log filters, time-spent and statistics all behave as documented. | Deleted the same entry twice (404). Logged a negative duration (rejected). | KF-221 |
| R2-218 | PASS | The seeder ran end to end (rc 0) and produced 22 tasks, WIP 3, 2.5 h on #11 and 3 tasks in Done. | Reviewed the fields it reports as not reproducible. | KF-221 |
| R2-219 | FAIL | The accepted gap holds: column bodies stay white. Beyond it, card titles render rgb(229,231,235) on the pastel card colors (contrast 1.02 to 1.05). Column names are dark on #1f2937, the board name is light on light gray, and count badges are invisible. 36 of 69 text nodes fall below 3:1. `shots/dark-mode-01-board.png` | | KF-312 |
| R2-220 | FAIL | In the dark task modal, the title, subtasks and estimate have contrast 1.0. `shots/dark-mode-02-modal.png` | | |
| R2-221 | FAIL | The timer popup is fine. In the timer settings modal, select values are light gray on white. `shots/dark-timer-settings-01-timer-settings.png` | | |
| R2-222 | FAIL | `body.dark` is never set on any report page. The preference is only restored when `.board-wrap` exists (`BoardChrome.init`). `dark-mode.log.json` | | KF-349 |
| R2-223 | N/A | Dark mode does not reach the settings pages at all. They stay light and readable. No reference. | | |
| R2-224 | PASS | The toggle persists across reload and boards, and turning it off on POMO sticks on GM. It is stored in the browser's localStorage. | Turned it off from a different board than the one it was turned on. | KF-353 |
| R2-225 | PASS | The wordmark is hidden below 768 px. No overlap at 767, 768, 780, 800, 860 or 1024. | Tested widths just above the breakpoint. | KF-324 |
| R2-226 | PASS | At 1100 the timer log modal fills the frame, with all buttons inside and no overflow. | | KF-286 |
| R2-227 | PASS | At 500 and 400 px the board container scrolls horizontally (scroll width 1270) and columns stay 312 px. | Tested two widths. | |
| R2-228 | PASS | The task modal fits the width at 500 and 400. | | |
| R2-229 | PASS | The timer popup and the quick-add form stay inside the viewport at 500 and 400. | | KF-177 KF-208 |
| R2-230 | BLOCKED | No touch harness is available. | | |

## 3. FAILs

1. **R2-200 [HIGH] Delete board does nothing.** This is a regression of RELEASED KF-304. It throws `ReferenceError: showConfirmDialog is not defined` and no request is sent. It reproduced on two boards in two DBs. The KF-304 release note claims the function is defined at `app.js:878`, but it is private to the IIFE, and `board_delete.html` has no confirm markup.
2. **R2-219/220/221 [MEDIUM] Dark mode makes text unreadable.** Card titles, column headers, the board name, task modal text and timer settings inputs drop to contrast 1.0 to 1.05. No OPEN defect covers this. The accepted gap covers only white column bodies.
3. **R2-222 [LOW-MEDIUM] Dark mode never reaches report pages.** RELEASED KF-349 does not hold: its dark CSS never applies off the board page.
4. **R2-192 [MEDIUM] Color drag reorder is not wired.** KF-082 (no status line, recorded as fixed) reproduces.
5. **R2-188 and R2-191 [LOW] Edit color Cancel and ×, and premium Close, are dead.** The buttons use `data-close-dlg`, but app.js only wires `data-close-dialog`. This affects KF-084 and KF-085 (no status lines, recorded as fixed).
6. **R2-190 [LOW] The Copy from board no-pick error is hidden under the overlay.**
7. **R2-193 [LOW] A spaces-only or 101+ character board name gives a bare 500 page.**
8. **R2-204 [LOW] The token Revoke confirm's Cancel is dead (KF-146).** The generic close handler is not active on `/settings`.
9. **R2-208 [LOW] `/login` logs two 401 console errors on every load.**
10. **R2-210 [LOW] `CHIPFLOW_API_TOKEN` cannot be used before `/setup`.** `agent-guide.md:19` and `deploy/nixos/README.md:38` promise headless bootstrap "without touching the UI".
11. **R2-152 [LOW] The Time spent "Total:" line is invisible in light mode.** KF-292's verification listed this line as matching.
12. **R2-155 [LOW] Time spent Bookmarks is inert.** KF-091 lists Bookmarks as a KanbanFlow feature.
13. **R2-158 [LOW] Cumulative flow keeps a phantom band after a column rename.** This is a residual of RELEASED KF-350.
14. **R2-151 [LOW] Highscores invents a "0 Best day" on empty data.**
15. **R2-211 [LOW] The agent docs are wrong in 6 places:**
    - They say stopped pomodori count toward the task total. They don't (`pomodori_completed` stays 0, matching KF-200).
    - They describe a "✓ <date>" stamp when a task moves to Done. No such stamp exists.
    - They say GET `/api/tasks/{id}` lists `attachments` with a `download_url`. Neither field exists.
    - They say `grouping_date: null` clears the date. It is ignored; only `""` clears.
    - They say timestamps are Unix seconds. Comments, attachments, time entries, `completed_at` and `due_at` are RFC3339 strings.
    - They claim a 10 MiB attachment cap. Axum's default 2 MiB body limit applies, so the real raw cap is about 1.5 MB (1,500,000 bytes accepted, 1,600,000 rejected with 413).
16. **R2-212 [LOW] OpenAPI omits 3 `/api` routes.**
17. **R2-215 [LOW] Garbage `due_at` values are stored.** Same class as KF-348.

## 4. Proposed new defects

Several of these have no KanbanFlow capture. In those cases the "KanbanFlow:" line names the contract I checked against instead: ChipFlow's own docs or an earlier tracker fix.

### KF-NEW-admin-1 — Delete board button throws ReferenceError; boards cannot be deleted from the UI [HIGH]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/del-board.log.json`, `$R/results/admin/shots/single-board.log.json`
- KanbanFlow: Settings → Delete board → red confirmation page → a final "cannot be undone" dialog (KF-089), with the styled confirm from KF-304 (RELEASED).
- ChipFlow: clicking "Delete board" throws `ReferenceError: showConfirmDialog is not defined`. No dialog appears and no request is sent. The function is private to the `static/app.js` IIFE, and `templates/board_delete.html` has no `#confirm-dialog`. This is a regression of KF-304.
- Fix direction: make a styled confirm callable from the delete page, and add an end-to-end test that deletes a board through the page.

### KF-NEW-admin-2 — Dark mode makes task text, column headers, board name and timer settings unreadable [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/dark-mode-01-board.png`, `$R/results/admin/shots/dark-mode-02-modal.png`, `$R/results/admin/shots/dark-timer-settings-01-timer-settings.png`
- KanbanFlow: no dark-mode capture. KF-349 adopted WCAG AA as the bar.
- ChipFlow: in dark mode, text drops well below 3:1 contrast:
  - Card titles are rgb(229,231,235) on the pastel card colors (1.02 to 1.05:1).
  - Task modal text is 1.0:1.
  - Column names are dark on #1f2937.
  - Timer settings select values are light on white.
  
  This is separate from the accepted white-column-body gap.
- Fix direction: keep dark text on task-colored surfaces, and give headers, the board bar and settings inputs AA-contrast dark styles.

### KF-NEW-admin-3 — Dark mode never applies on report pages [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/dark-mode.log.json`
- KanbanFlow: no capture. KF-349 (RELEASED) shipped dark styles for report pages.
- ChipFlow: `BoardChrome.init` returns before `restorePrefs()` when there is no `.board-wrap`, so `body.dark` is false on `/timer/statistics`, `/timer/log` and every board report page.
- Fix direction: restore the dark preference on every page, not just the board.

### KF-NEW-admin-4 — Colors tab drag reorder is never wired [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/colors-followup.log.json`
- KanbanFlow: drag handles (⋮⋮) reorder the enabled palette (KF-082).
- ChipFlow: the inline script runs before the deferred `Sortable.min.js` loads (`base.html:9`), so no sortable instance is attached. Dragging changes nothing.
- Fix direction: initialize the sortable after page load, or move it into app.js.

### KF-NEW-admin-5 — Colors tab Edit and premium dialogs cannot be closed with their buttons [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/bs-colors.log.json`, `$R/results/admin/shots/bs-colors-03-edit-after-close-attempts.png`
- KanbanFlow: the Edit color dialog (KF-084) and the premium prompt (KF-085) close from their own buttons.
- ChipFlow: Edit's Cancel and ×, and the premium dialog's Close, do nothing. They use `data-close-dlg`, which this page only wires for the Set default and Copy dialogs, while app.js only handles `data-close-dialog`. Escape and backdrop clicks do close them.
- Fix direction: wire `data-close-dlg` for every overlay on the page, or switch to `data-close-dialog`.

### KF-NEW-admin-6 — Copy from board "pick a board" error hidden under the overlay [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/bs-colors-06-copy-nopick.png`
- KanbanFlow: no capture of this state.
- ChipFlow: the error is written into `#bc-error` on the page behind the overlay, so the click appears to do nothing.
- Fix direction: show the error inside the dialog, or disable "Copy palette" until a board is picked.

### KF-NEW-admin-7 — Whitespace-only or over-long board name returns a bare HTTP 500 page [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/bs-general-03-blank.png`
- KanbanFlow: no capture.
- ChipFlow: POST `/b/{id}/settings` with a spaces-only name, or a name over 100 characters, returns 500 with plain text "board name must be 1-100 characters".
- Fix direction: return 400 and re-render the General tab with the error message.

### KF-NEW-admin-8 — API token Revoke confirm: Cancel does nothing [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/us-tokens.log.json`
- KanbanFlow: n/a (ChipFlow-only page). The intended behavior comes from KF-146 and the template comment.
- ChipFlow: the generic `[data-close-dialog]` handler is not active on `/settings`, so Cancel does nothing.
- Fix direction: register that handler on every page.

### KF-NEW-admin-9 — Login page logs two 401 console errors on every load [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/setup-first-run.log.json`
- KanbanFlow: n/a.
- ChipFlow: app.js polls `/api/timer/settings` and `/api/timer/status` on `/login`, and both return 401.
- Fix direction: skip timer init on pages that are not logged in.

### KF-NEW-admin-10 — CHIPFLOW_API_TOKEN cannot bootstrap the API before /setup [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/server-headless.log`
- KanbanFlow: n/a. The promise comes from `agent-guide.md:19` and `deploy/nixos/README.md:38`.
- ChipFlow: the auth middleware checks `user_count()==0` first and sends everything, including token-authenticated API calls, to `/setup` with a 303.
- Fix direction: let the env token through before setup (or add an API setup endpoint), or fix the docs to say `/setup` comes first.

### KF-NEW-admin-11 — Time spent "Total:" line invisible in light mode [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/rep-spent-01-summary.png`
- KanbanFlow: the report shows the total (GM-123; KF-292 verified the line as matching).
- ChipFlow: `.spent-total{color:#f3f4f6}` (`static/style.css:3723`) on the #eef0f3 page background, contrast 1.04.
- Fix direction: use the body text color in light mode and keep the light color under `body.dark`.

### KF-NEW-admin-12 — Time spent Bookmarks is static text [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/rep-spent-05-bookmarks-click.png`
- KanbanFlow: Time spent has Bookmarks (KF-091, v1 S10).
- ChipFlow: "Bookmarks (0)" is an inert div.
- Fix direction: implement saving and recalling filter bookmarks, or remove the label.

### KF-NEW-admin-13 — Cumulative flow keeps an empty band for a renamed column's old name [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/br-cumflow-02-after-rename.png`
- KanbanFlow: n/a. This is a residual of KF-350 (RELEASED).
- ChipFlow: after a rename, the legend shows both names with "In Review: up to 0 tasks", and the status counts 6 columns on a 5-column board.
- Fix direction: attribute history by column id, or drop bands whose name matches no current column.

### KF-NEW-admin-14 — Statistics Highscores shows a "0 Best day" with no data [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/shots/single-board-01-stats-empty.png`
- KanbanFlow: empty statistics show "No data to display" (KF-092).
- ChipFlow: with no sessions in range, Highscores shows "0 Best day — Friday, Oct 16".
- Fix direction: return `best_day: null` when nothing was logged and show the empty state.

### KF-NEW-admin-15 — API attachment uploads over ~1.5 MB fail with 413 despite the documented 10 MiB cap [MEDIUM]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/api-battery.log`
- KanbanFlow: n/a. The contract is the "10 MiB cap" in agent-guide and skill.md, and `Db::MAX_ATTACHMENT_BYTES`.
- ChipFlow: no `DefaultBodyLimit` is set, so axum's 2 MiB default applies to the base64 JSON body. 1,500,000 bytes is accepted; 1,600,000 and 8,000,000 bytes get 413 "Failed to buffer the request body".
- Fix direction: raise the body limit on the attachment route to about 14 MiB (base64 of 10 MiB), or document the real cap.

### KF-NEW-admin-16 — Task API stores unparseable due_at strings [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/api-battery.log`
- KanbanFlow: n/a. Same class as KF-348 (RELEASED).
- ChipFlow: POST and PATCH accept "next tuesday", "garbage" and "2026-13-45" (201/200) and store them as-is. The card then shows no due date.
- Fix direction: reject unparseable `due_at` with 400.

### KF-NEW-admin-17 — Agent guide and skill describe API behavior that does not exist [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/api-battery.log`, `$R/results/admin/api-battery2.log`, `$R/results/admin/seed-log.out`
- KanbanFlow: n/a (ChipFlow's own docs).
- ChipFlow: the 6 mismatches listed under FAIL 15 (stopped pomodori, ✓ stamp, attachments in task GET, `grouping_date: null`, timestamp format, attachment cap).
- Fix direction: fix both docs, or change the API to match (null clears, attachments listed).

### KF-NEW-admin-18 — OpenAPI omits three /api routes [LOW]
- Status: OPEN — filed 2026-10-16 by the R2 battery (admin). Evidence: `$R/results/admin/route-ops.txt`, `$R/results/admin/openapi-ops.txt`
- KanbanFlow: n/a. agent-guide calls openapi.json the contract for every endpoint.
- ChipFlow: these three routes have no `#[utoipa::path]`:
  - GET `/api/tasks/{id}/card`
  - GET `/api/tasks/{id}/modal`
  - GET `/api/timer/settings`
  
  HTML pages are covered inconsistently: the board settings pages are in the spec, the report pages and `/setup` are not.
- Fix direction: annotate the three routes and decide whether HTML pages belong in the spec.

### Observations (no KanbanFlow reference, not filed)

1. **Sessions never expire on the server.** `created_at` is stored but never checked (`db.rs:603`). Only the cookie's 30-day Max-Age limits them.
2. **Ending a session mid-use is handled inconsistently.** A card click fails silently (the modal GET returns 401 and nothing shows). Adding a task shows "Could not add task: unauthorized". Nothing redirects to `/login`.
3. **Every `/settings` save sets `ding_enabled` and `notifications_enabled` to false** (`routes.rs:6306`). Nothing in app.js reads either flag, so there is no visible effect today.
4. **`pomodoro_minutes=0` saved through the form silently becomes 25**, and the page still says "Settings saved."
5. **Horizontal overflow at 400 px (scroll width):** `/settings` 462, board settings General 424, Colors 715, Layout 547, statistics 559, time spent 484, calendar 832, due date 435, task count 576, time estimate 576. All clean at 1100.
6. **The Board history type filter mixes labels with raw ids** such as `attachment_added`, `comment_deleted` and `watched`.
7. **Monte Carlo counts 5 completions where Cycle & lead and Throughput count 4.** One task entered Done twice.
8. **The Break activities tab shows `daily_goal` as minutes ("2m goal")**, while the docs define it as a count from 1 to 100.
9. **htmx and SortableJS load from `cdn.jsdelivr.net`**, so a self-hosted install needs internet for drag and drop and the add-task forms.
10. **Creating a column without `board_id` puts it on the first board.** The guide's example omits `board_id`. The last column on a board can be deleted, leaving zero columns.
11. **The only board can be archived**, after which `/api/boards` returns `[]`.
12. **A lowercase `bearer` auth scheme is not recognized** and falls through to cookie auth (401).
13. **`expires_in_days` of 0 or less silently creates a never-expiring token.**
14. **OpenAPI documents no 403 (insufficient scope) or 413 responses anywhere.**
15. **The timer popup's today list shows green dots for stopped pomodori.** This is a timer row, outside this batch.
16. **The printer glyph (U+1F5B6) on the Print buttons renders as an empty box** in this Chromium.
17. **The guide titles `GET /api/tasks` "List the board's tasks"**, but it returns id/name pairs across all boards. The guide never mentions the "Kanban basics" built-in template.
18. **Statistics counts stopped pomodori in its "Pomodori" total (4)**, while the task's `pomodori_completed` (2) does not.
19. **Dark mode is stored per browser in localStorage**, not as an account setting.
20. **`seed-golden-master.sh` is not tracked by git**; it exists only in the r2-main worktree. It also deletes Brown and White, where the inventory's GM profile expects them disabled.

## 5. BLOCKED

- **R2-230 (touch drag on a phone):** Playwright's touchscreen API only taps. Unblocking needs a CDP `Input.dispatchTouchEvent` start/move/end sequence that SortableJS accepts, or a real phone or emulator pointed at a non-production instance.

DONE: admin 40 PASS 20 FAIL 1 BLOCKED (plus 4 N/A)

Top findings:
1. **Delete board is broken in the UI** (HIGH, regression of RELEASED KF-304). The `showConfirmDialog` ReferenceError means no board can be deleted from the page. Archive still works. The API delete and the last-board refusal work, but the UI can't reach them.
2. **Dark mode makes text unreadable** (MEDIUM). Card titles, column headers, the board name, task modal text and timer settings inputs drop to contrast 1.0 to 1.05. That goes well beyond the accepted white-column-body gap. Dark mode also never applies on report or settings pages, so RELEASED KF-349 does not hold.
3. **Board settings Colors tab** (MEDIUM). Drag reorder is never wired. The Edit dialog's Cancel and ×, the premium dialog's Close and the token Revoke Cancel are all dead buttons.
4. **API and docs** (MEDIUM). Attachments over about 1.5 MB fail with 413 despite the documented 10 MiB cap. Arbitrary `due_at` strings are stored. The guide is wrong in 6 places. OpenAPI omits 3 `/api` routes. `CHIPFLOW_API_TOKEN` cannot bootstrap before `/setup`.
5. **Smaller defects** (LOW):
   - The Time spent "Total:" line is invisible in light mode.
   - Bookmarks is inert.
   - Cumulative flow keeps a phantom band after a rename (KF-350 residual).
   - Highscores invents a best day on empty data.
   - A spaces-only board name gives a raw 500.
   - `/login` logs 401 console errors on every load.

All three accepted gaps still hold: board history shows no deletions, column bodies stay white in dark mode, and Monte Carlo varies per run.

The test server is stopped (nothing on port 3270). The DBs are `/tmp/chipflow-r2-3270.redb` and `$R/results/admin/headless.redb`.
