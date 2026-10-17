## verify-d4 report: KF-401, KF-404, KF-259, KF-389, KF-417 (fix/due-d @ 0342ad8)

Four of the five defects pass. KF-259 fails: the target column, due text and 7-day window now work, but the overdue card line and the modal parenthetical still differ from KanbanFlow, and the golden-board fixture never sets a target column.

All scratch paths below are under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/vd4`.

### Environment
- **Fix tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-d4`, detached at 0342ad8. Debug binary md5 7244e5feeebf7176b1fd93c999075590.
- **Baseline tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-d4-base`, at 9cfcd03. Debug binary md5 a4c378aa166d02094273000330b54b89.
- **Binaries differ:** "Any archived column" appears 4 times in the fix binary and 0 in the baseline; "Change default due date time" 2 times and 0.
- **Served assets match each tree:** fix app.js 747e0786…, baseline app.js 9979d8e9…. My first dialog capture was taken while both servers ran from the main checkout, so they served its `static/`. I threw that capture away and restarted each server from its own worktree.
- **Ports and DBs:** fix on 3327 (`/tmp/chipflow-verify-d4-3327.redb`), baseline on 3328 (`/tmp/chipflow-verify-d4-3328.redb`). Both servers ran with TZ=America/Chicago.
- **Browser:** Playwright 1.63.0 Chromium, 1905x1000, DPR 1, timezone America/Chicago, animations off, a fresh context per scenario. Driver: `$S/r2/pw/vd4ui.mjs`. Screenshots: `shots/3327/`, `shots/3328/`.
- **Diff:** touches only `agent-guide.md`, `agents/skill.md`, `src/{db,models,routes}.rs`, `static/{app.js,style.css}`, and `templates/{board,board_settings,task_card}.html`.
- **Cleanup:** both servers were killed by PID. due-d was not touched and the worktree is clean.

### Gates (fix tree)
- `cargo fmt --check` passes.
- `clippy --all-targets` finishes with 0 warnings.
- `TZ=UTC cargo test` passes: 125 tests, 1 ignored.
- The jsc `checkSyntax` check on `static/app.js` exits 0.
- **Empty environment** (`env -i`: no HOME, no TZ, `TZDIR=`): all tests pass. They also pass with `TZ=America/Chicago` and an empty TZDIR.
- `TZ=Pacific/Chatham`: `format_due_honors_seven_day_window` fails, and it fails the same way on the baseline. This is a pre-existing problem, not from this diff.
- Logs: `gates.log`, `gates2.log`.
- Every UI run had 0 console errors and 0 failed requests.

### Old database
I wrote a database with the baseline binary (seeded fixture, a done and repeating due, a board config, a feed token, date-only dues), then opened a byte-identical copy with the fix binary. Boards, columns, export-data, settings and all 27 task details are identical, apart from the new key `due_target_column_id: null` (`olddb-*.jsonl`). An old board reports `due_time` "17:00".

Stored changes are the two the builder reported. I found no other stored change. Other differences:
- export-data output now carries `due_target_column_id` for every task.
- Saving a board as a template, then applying it, drops target columns and the board's `due_time`, because neither is in the template snapshot.

### KF-401: Delete in the due dialog
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| A red Delete removes the due date | Buttons are Cancel and Add only; the date field is readonly, so a due can't be cleared | "Edit due date" shows Delete in #d9534f and Update | PASS |
| Delete clears `due_at`, `due_repeat`, `due_done` and the target | n/a | Sends `{"due_at":null,"due_repeat":null,"due_done":false}`; the server also clears the target | PASS |
| Only the open task is written | n/a | 1 of 57 task records changed | PASS |
| Delete hidden when there is no due | n/a | Hidden; the title reads "Add due date" | PASS |

Break-attempt: Delete on a repeating due that was marked done. Repeat, done and due all cleared. The card updates when the modal closes, which is existing behaviour.

### KF-404: board due time
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| A date-only due gets 5:00 PM | `2026-10-20` stored as 23:59:59, card "Tuesday 11:59 PM" | 17:00:00-05:00 | PASS |
| Due dialog defaults | Date and time empty | Today's date, time 17:00 | PASS |
| Task settings uses KanbanFlow's wording | No such setting | "Due date time" with "Edit", "5:00 PM"; dialog "Change default due date time", field "Default due date time", button "Set default" | PASS |
| A changed board time applies | n/a | Set to 08:30: label "8:30 AM", dialog default 08:30, date-only due 08:30; existing dues and other boards unchanged | PASS |

Break-attempts:
- `"24:00"` and `"9:5"` give 400; `" 17:00 "` is trimmed and stored as 17:00.
- Date-only dues on DST days at 17:00 are correct: 2026-03-08 is −05:00, 2026-11-01 is −06:00.
- With the board time at 01:30, a 2026-11-01 due resolves to the first 01:30 (−05:00).
- With the board time at 02:30, a date-only due on 2027-03-14 returns 400 "that local time is skipped by a daylight-saving change". The client only sent a date, so the message is confusing.

### KF-259: due text, target column, overdue styling
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| Target column is stored per task, limited to the task's board | Field ignored | Foreign or unknown column gives 400; null or "" clears; clearing the due clears it | PASS |
| Parenthetical is the target column name | "(Done)" only from `due_done` | "(In Review)"; follows a column rename | PASS |
| Card: "Today", weekday for later this week, "D Month" otherwise | "Friday 5:00 PM" for today, weekday for past dates | "Today 5:00 PM", "Sunday 5:00 PM" (+2 days), "11 October 5:00 PM" (5 days past) | PASS |
| 7-day window on cards | 15 November line shown | Line hidden, icon kept | PASS |
| Modal format for other dates | Weekday | "Oct 14 at 9:00 AM (Done)", same as the golden | PASS |
| Overdue colours and "!" disc | #DC2626, no disc | Text #8b0000; disc #be0000, 15px (golden ~16px) | PASS |
| Overdue weight: only the date bold | All bold | **All bold**: "Due:" and "(Done)" have font-weight 700 | **FAIL** |
| Overdue ink size, same string "Due: 30 September 11:00 PM (Done)" | n/a | **236x16 px vs golden 168x16 px** (+40%): bold plus 11.84px font vs KanbanFlow's 10px | **FAIL** |
| Modal parenthetical not bold | n/a | **"(Done)" weight 600**, plus a 📅 icon and chip border | **FAIL** |
| Dialog wording and buttons | "Add due date", Cancel/Add | "Add due date"/"Edit due date", Add or Delete+Update, close × | PASS on wording |

Dialog geometry is still off: buttons are 71x32 and 76x32 at weight 400 against KanbanFlow's 125x30 at 700, and Update is #16a34a against #49a849. These styles predate this diff.

Builder assumptions:
- **(a) The modal shows "Today 5:00 PM": not supported by the goldens.** The only modal goldens use "Oct 2 at 11:00 PM" / "Oct 14 at 9:00 AM", with "at". The "Today 5:00 PM" evidence comes only from cards. If KanbanFlow's modal uses relative words, "Today at 5:00 PM" is just as likely. It needs a live check.
- **(b) Options are the columns after the task's own column, plus "Any archived column": consistent with the goldens, not proven.** Golden modal/23 has the task in the first column and shows "Any archived column/Do today/In progress/Done". That rules out "all columns" but cannot tell "after own" from "all except own". Repeat's "Start in" list excludes the last column, which hints at an ordering rule. "Any archived column" as the default for a new due matches the golden.

Break-attempts:
- **Target column deleted:** the card falls back to no parenthetical, but GET still returns the deleted column's id, and re-sending that id gives 400.
- **Task moved to another board:** the old board's target is kept. The card hides it, and it comes back if the task moves back.
- ChipFlow has no column archiving, so "archived" cannot be tested.

### KF-389: zero-length manual time entry
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| From = To shows "0h" in Add time manually | "—" | "0h" (also 00:00–00:00 and 23:59–23:59) | PASS |
| Same in the Edit entry dialog | "—" | "0h" | PASS |
| To before From | "—" | still "—" | PASS |

Break-attempt: in the edit dialog (which has seconds), 10:00:00–10:00:30 shows "0m" while equal times show "0h". Saving a zero-length entry is still refused with "End time must be after start time", as before.

### KF-417: empty time is refused
| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| An empty time is refused with "Invalid date or time" | Silently saved at 09:00 (14:00Z), toast "Due date added." | Alert "Invalid date or time", no PATCH sent, due unchanged | PASS |
| Alert styling (golden modal/25) | n/a | Text #444 15px/22.5px; OK button background #fbfbfb, border rgba(0,0,0,.15), weight 700, focused | PASS |

Break-attempt: Enter on the focused OK closes the alert and the due dialog stays open. The alert box is 420x127 against the golden 290x124, and it does not dim the due dialog behind it as KanbanFlow dims the modal.

### Regressions checked
- **KF-409:** an unchanged save of `2026-10-20T10:00:59Z` marked done sends the exact string and stays done.
- **KF-410:** an unchanged save of `2026-11-01T06:30:00Z` (the second 01:30 in Chicago) keeps the same instant.
- **KF-414:** a lowercase `2026-10-20t15:00:00z` prefills 10:00 and is sent back unchanged.
- OpenAPI includes the new fields in TaskDetail, UpdateTaskInput, BoardConfigInput and BoardConfigView.
- No regressions found.

### Candidate new defects
- **Seed fixture has no target columns (feeds KF-259).** `.dev/golden-master-board.json` and `seed-golden-master.sh` never set one. A seeded board shows no "(Done)", while KanbanFlow shows it on every due, so the R2-065/066/067 golden-board rows will still fail.
- **No past-date confirm.** KanbanFlow asks "The selected due date has already passed. Do you still want to use it?" with Cancel and a red OK (golden modal/26). ChipFlow saves silently, and I found no tracker entry for it.
- **Dangling target ids.** A deleted column, or a column on another board after a move, stays in `due_target_column_id`.
- **History entry misleads on a target-only change.** It logs "Set the due date to …" and does not mention the target.
- **Docs are slightly off on `due_time`.** They say anything other than zero-padded HH:MM is a 400, but surrounding spaces are trimmed and `null` is silently ignored. There is also no way to reset the time to the default.
- **Stale code comment.** The comment on `save` in `static/app.js` still says "the API has no due target-column field".
- **Task settings tab layout differs from KanbanFlow.** The setting is an inline row with a Cancel button in a centred modal, against KanbanFlow's stacked layout with a popover. The wording matches.
- **Tracker claim is out of date.** KF-404 says "the golden board's dues all read 5:00 PM", but the 10-17 goldens read "11:00 PM".

### Evidence
- Golden vs fix crops: `g-sep30.png`, `f-sep30.png`, `fix-card-overdue-3x.png`, `g-card-due.png`, `g-modal-due.png`
- Fix screenshots: `shots/3327/dialog-due.png`, `shots/3327/cleartime-3327.png`, `shots/3327/settings-dialog-fix.png`, `shots/3327/modalchip-vd4_ui_overdue.png`
- Baseline screenshots: `shots/3328/`

DONE: KF-401 PASS Delete is red #d9534f, clears due, repeat, done and target on the open task only (1 of 57 records changed); baseline cannot clear a due
DONE: KF-404 PASS date-only dues get the board time (17:00 default, 08:30 applied per board, existing dues unchanged); Task settings uses KanbanFlow's wording; baseline stores 23:59:59
DONE: KF-259 FAIL target column, Today/weekday/date formats and 7-day window work, but overdue cards bold "Due:" and "(Done)" (ink 236 vs 168 px for the same string), the modal "(Done)" is bold, and the golden fixture seeds no targets
DONE: KF-389 PASS From = To shows "0h" in the add and edit dialogs (baseline "—"); To before From still "—"
DONE: KF-417 PASS an empty time shows the "Invalid date or time" alert and sends no PATCH; baseline silently saved 09:00
