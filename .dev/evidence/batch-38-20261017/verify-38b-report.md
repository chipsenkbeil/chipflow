verify-38b report: batch-38 UI half (candidate integ/batch-38 e9a829d vs baseline d2a466a)

DONE: BATCH-38-B PASS
DONE: KF-514 PASS
DONE: KF-512 PASS
DONE: KF-513 PASS
DONE: KF-526 PASS
DONE: KF-516 PASS
DONE: KF-525 PASS (docs-only fix; baseline already met the cap)
DONE: KF-511 PASS
DONE: KF-515 PASS
DONE: KF-524 PASS (see observations 3 and 4 on pre-existing deviations)
DONE: KF-505 PASS (unchanged by the batch)
DONE: KF-520 PASS
DONE: KF-517 FAIL (the 4 acceptance paths pass; the layout-view column and lane drag still raises requestfailed, see observation 1)
DONE: KF-499 PASS (UI path)
DONE: REGRESSION
DONE: UI-SWEEP

The BATCH-38-B PASS is conditional on the KF-517 call. The KF-517 FAIL is a single low-severity leftover in the layout-view drag. The four paths named in the tracker's acceptance line are clean. Treat it as a candidate defect, not a blocker, unless you want the broader reading of "a lane move".

## Environment
- Candidate: worktree `.dev/worktrees/verify-38b` at e9a829d. Baseline: `.dev/worktrees/verify-38b-base` at d2a466a.
- Release builds, both from `target/build`, built one cargo invocation at a time.
- Binary md5: candidate 7695c07217e7d2194f8670124c7e0e05, baseline 8cb1d0f1523c02a0974aa676a5f41c4a. The two differ.
- `rg -a` finds the KF-524 JS string in the candidate binary and not in the baseline.
- Ports: candidate 3388, baseline 3389. Databases are `db-3388.redb` and `db-3389.redb` in the scratchpad `verify-38b/` folder, always passed with `--database-path`.
- Both servers ran under libfaketime, frozen at 2026-09-30 10:00 America/Chicago (the ft file has no trailing newline). The seed used `--as-of 2026-09-30`.
- Viewport 1440x612, DPR 1, light and dark. Playwright 1.63.0 from `r2/pw`, driven by node only.
- Both servers were stopped by exact PID. Nothing is still listening on 3388 or 3389.
- Gates on the candidate, all clean. Logs are `gate-fmt.log`, `gate-clippy.log`, `gate-test.log` in `verify-38b/`:
  - fmt --check exit 0.
  - clippy --all-targets exit 0 with no warnings.
  - `TZ=UTC cargo test` exit 0, every test result "ok" (the largest binary: 247 passed, 1 ignored).
  - jsc `checkSyntax` on `static/app.js` exit 0.
- Evidence: `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-38b/`. It holds screenshots, `snap-3388/` and `snap-3389/` (with `geo.json` and `errs.json`), `dnd-*.out`, and `i-3388.out`. The scripts are `r2/pw/b38-*.mjs`.

## Per-defect results (baseline reproduces the bug, candidate fixed)

| KF | Claim | Baseline | Candidate | Verdict |
|---|---|---|---|---|
| 514 | name-to-badge gap ≥ 4 px, 60-char name | 0 px (count and hours badges) | 4 px at light and dark, widths 1920/1440/1120/900 | PASS |
| 512 | overlay and head colours | rgba(17,24,39,.55) and transparent | rgba(0,0,0,0.3) and rgb(244,244,244), light and dark | PASS |
| 513 | hover then click leaves `#reports-submenu` visible | hover opens it, click closes it | stays open after hover and click | PASS |
| 526 | opens on "This + Last week" and fetches that range | "This month", `from=2026-10-01&to=2026-10-31` | "Period: This + Last week", `/api/timer/log?from=2026-10-05&to=2026-10-18` | PASS |
| 516 | stopwatch stop has no reason menu, entry logged | menu with 17 buttons, timer still running | no menu, timer stopped, log entry kind stopwatch (90 s, "1m") | PASS |
| 525 | card live part matches the logged cap | pomodoro "+ 1h 25m", stopwatch "+ 12h" | identical, and `agent-guide.md` and `agents/skill.md` now state the cap | PASS |
| 511 | no `.done-group-label` in an empty archiving column | "Today" | none; one task done today gives "Today" | PASS |
| 515 | `scrollWidth` equals viewport width on /timer/log, /timer/statistics, /boards/new | /timer/statistics 1530 | 1440 on all three | PASS |
| 520 | `maxlength` or message at /boards/new | no `maxlength`, 400 plus "Could not create the board." | `maxlength=100`; with it bypassed the message is "Board name must be 1-100 characters." and no request is sent | PASS |
| 517 | no `requestfailed` on card-menu delete, modal delete, lane delete, lane move | ERR_ABORTED on the lane move and lane delete | 0 events on all four | see FAIL note |
| 499 | stale tab's stop leaves a same-second newer session running | pomodoro: session B stopped | stopwatch and pomodoro: B kept running (same-second starts confirmed) | PASS |

Notes on individual defects:
- **KF-511:** a non-empty archiving column whose only task is done on a past date shows "Today" followed by the dated header. A task dragged into an empty Done column gets "Today" live and after reload. The column goes back to no header once its last task is deleted.
- **KF-525:** the cap code did not change. Over 60 s, 25 min, 40 min, 85 min, 2 h, 13 h and 25 h, the pomodoro card tops out at "+ 1h 25m" (the logged entry is 85 min) and the stopwatch at "+ 12h" (logged 720 min). The pill shows the full elapsed time.
- **KF-524, against golden modal/27 (the long-title "GM long title: Refactor the authentication ..." task):**
  - The title is a textarea and wraps to 3 lines with the same line breaks as the golden.
  - The "Due date" label and chip start at x 525.5 against the golden's 524 (±2); the baseline's chip was at 550.5.
  - Against modal/01 the due chip is at 555.2 (golden 555) and Time estimate wraps to row 2 at x 445.5.
  - Enter saves, blurs the field and keeps the modal open. Blur saves. Escape reverts without saving (KF-388). Pasted newlines become spaces, and Shift+Enter inserts no newline.
  - An empty title is rejected and reverts. A 260-character title is capped at 200.
  - The "1 Pomodoro" estimate fallback is gone, and Time spent is hidden at 0m. On a task with no estimate and no time spent the cell row is only Color, Labels and Due.
  - The colour picker opens below the grid at y 228.8 (light) and its second dot saves a new `color_id`.
  - Dark mode behaves the same.
- **KF-505:** the due popover is at 442.5,172, 280x440, which is modal-left + 27. The inner geometry is identical on candidate and baseline, for both the payment task and the onboarding task.
- **KF-499:** the stale tab's status poll was blocked during the test, so it stayed stale. Without the block the poll resyncs the tab and the stop hits the newer session either way.
- **KF-517:** passes for card-menu delete, modal delete (Delete key), lane move and lane delete. It fails the broader reading because of observation 1.

## Regression: differential against the batch-37 baseline
I took 78 screenshots on each server at frozen time, each with a geometry dump. The states were:
- Board at 1920x900, 1440x612, 1120x900 and 900x900, in light and dark.
- Board menu, Reports submenu, filter panel, quick-add, Members popover, help dialog, shortcuts dialog and idle timer panel.
- Card context menu.
- 19 pages: /boards/new, /boards/archived, /settings, /timer/log, /timer/statistics, /, task-count, throughput, burndown, cumulative-flow, cycle-lead, due-date, board-history, calendar, monte-carlo, time-estimate, export, settings/colors and settings/delete. Each in light and dark.

Result:
- Pixel difference (`compare -metric AE`) is 0 everywhere except these states:
  - The timer log (overlay and period change).
  - The task-modal states (the KF-524 changes): modal-pay, modal-due, modal-add, modal-plain and modal-narrow at 900.
  - Two chart pages with sub-pixel noise (AE 0.01 and 0.17).
- Zero console errors, page errors, failed requests and HTTP errors on every surface and every probe on both servers. The 400 on the baseline's 101-character board name is the baseline bug itself.
- Absolute numbers on the candidate:
  - Idle timer panel 992,40,260x310.
  - Filter panel 1190,34,250x578.
  - Top-bar buttons at 1258/1298/1350, y 40.
  - Menu button 80x34.
  - Timer log dialog 660x400 at y 25, against golden timer/20.
- Because every non-modal surface is pixel-identical to the baseline, the batch-37 checks (board grid F, H and W1; KF-457 widths and legend; board-only dark; V top bar; W2 timer fixes; quick-add; help; due dialog) cannot have regressed.
- Card drag and drop gives identical end states on candidate and baseline:
  - Reorder within a column.
  - Cross-column move.
  - Out of the Done "Today" group into Backlog.
  - Into Done.
  - Across swimlanes (cell to cell).

Not driven: dragging a column header or swimlane header on the board. Playwright's mouse and `dragTo` hang on those native drags, on both servers.

## Break-attempts named
- KF-499: blocked the stale tab's status polling so it really stayed stale, then stopped through both the stopwatch path and the pomodoro reason-menu path.
- KF-516: stopwatch under 20 s still gives the "Session discarded" toast with no menu, and a pomodoro stop still shows the menu.
- KF-514: tested at four viewport widths, with a WIP-limited column (no sum badge; the "n / m" gap is 8 px) and with an hours badge.
- KF-515: also checked the timer log modal with a 100-character board name; there is no overflow.
- KF-524: newline paste, Shift+Enter, empty title, over-length title, Escape revert, colour picker, dark mode, 900 px viewport.
- KF-517: also tried the layout-view drags and the column collapse and delete paths (see observation 1).
- KF-511: past-date-only column, undated task, delete back to empty, live drag into an empty archiving column.

## Observations (candidate defects, each with a one-line acceptance check)
1. **Layout-view drag still aborts (KF-517 leftover, low).**
   - On the candidate, dragging a column or swimlane in the layout view ("Edit board layout", `#layout-columns-list` and `#layout-swimlanes-list`) raises `POST /api/columns/<id>/move net::ERR_ABORTED` and `POST /api/swimlanes/<id>/move net::ERR_ABORTED`.
   - Cause: the Sortable `onEnd` handlers in `static/app.js` (around lines 279-300) reload on `res.ok` without draining the body. The baseline behaves the same.
   - Acceptance: in the layout view, `Locator.dragTo` on a column handle and on a lane handle raises 0 `requestfailed` events. Reproduced by `r2/pw/b38-i2.mjs`.
2. **Standalone /timer/log defaults to "Last 30 days" (low).**
   - The page is pixel-identical to the baseline, so KF-526 changed only the board's timer log dialog.
   - Acceptance: `/timer/log` selects "This + Last week", or the tracker says KF-526 covers only the dialog.
3. **Modal head sits higher than the golden (low, pre-existing, unchanged by this batch).**
   - Against golden modal/27 (modal top at y 25) the title's first-line centre is about 15 px higher (y 54 against 69) and the subline about 14 px higher. The Color label is about 3 px lower.
   - The head has a separator line the golden lacks.
   - The × button centre is at y 51.6 against the golden's 45. The baseline's was y 61.
   - Acceptance: the first title line centre is at y 69 ±2 at 1440x612, as in golden modal/27.
4. **Property-grid row pitch and Time spent x differ from modal/01 (low, pre-existing).**
   - Row pitch is 66 px against the golden's 58; it was also 66 on the baseline.
   - Time spent label is at x 806.6 against 819. The baseline's was 811.5, so the labels cell is narrower than the golden's, and the golden has a "+" add button the candidate lacks.
   - Acceptance: for the golden payment task, row-2 label y minus row-1 label y is 58 ±2, and the Time spent label x is 819 ±2.
5. **Stopwatch entry's interrupted flag (info only).** A stopwatch stopped without the menu logs `interrupted:true` with a null reason. The badge and statistics logic only counts pomodoro entries, so this is probably harmless. Acceptance if wanted: a stopwatch entry logged through `/api/timer/stop` has `interrupted:false`.
6. **A stale stop returns 404 and shows as a console error in the browser (low, pre-existing, both builds).** Acceptance: `POST /api/timer/stop` for an already-ended session returns 200 with `already_stopped:true`.
7. **The Reports submenu cannot be closed by clicking the Reports item again.** It closes on Escape or an outside click. That is the intended KF-513 behaviour, noted for awareness.
