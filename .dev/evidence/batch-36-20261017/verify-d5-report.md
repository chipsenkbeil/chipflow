## verify-d5 report: KF-259 on fix/due-d @ 2b42928

KF-259 passes. All three points from verify-d4 are fixed and measure within 1 px of KanbanFlow. The four defects that passed before still pass, and so do the KF-409/410/414 behaviours.

Scratch root: `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-d5` (written `$D` below).

### Environment
- **Fix tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-d5`, detached at 2b42928. Debug binary md5 1d062283e3a7f347d82185681417d9d0.
- **Comparison tree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-d5-prev`, at 0342ad8. Debug binary md5 70a76dff30f6bff14a0d88719f63e73f.
- **Binaries differ:** "falls in a daylight-saving gap" appears once in the fix binary and not at all in the comparison; "tm-due-column" also 1 against 0.
- **Diff (0342ad8..2b42928):** touches only the fixture JSON, `seed-golden-master.sh`, `agent-guide.md`, `agents/skill.md`, `src/routes.rs`, `static/app.js` (a comment only), `static/style.css` and `templates/modal.html`. All are expected for this defect.
- **Servers:** 3327 (fix) and 3328 (comparison), each started from its own worktree root with TZ=America/Chicago. Fresh databases `/tmp/chipflow-verify-d5-3327.redb` and `/tmp/chipflow-verify-d5-3328.redb`.
- **Boards:**
  - Each server got a board seeded with the fix's seed script, so the CSS comparison uses identical data.
  - 3328 also got a board seeded with 0342ad8's own script, to test point (3).
  - Exact-string tasks were added for comparison with the goldens: due 30 Sep 23:00, 2 Oct 23:00 and 25 Sep 17:00, each with target Done.
- **Browsers** (fresh context per run, animations off, timezone America/Chicago):
  - macOS Playwright 1.63.0 at 1440 px wide, DPR 2, the same setup as the 10-17 goldens.
  - Linux capture container at 1919x998, DPR 1, for the 20260930 golden. Container `chipflow-verify-d5`, now removed.
- **Cleanup:** both servers were killed by PID. Both worktrees are clean and left in place. due-d was not touched.

### Gates (fix tree)
- `fmt --check` exits 0.
- `clippy --all-targets` exits 0 with 0 warnings.
- `TZ=UTC cargo test` exits 0: 127 passed, 1 ignored.
- Empty environment (`env -i`, `TZDIR=`, HOME set to a nonexistent path) exits 0 with the same counts. The two new tests (`modal_due_value_bolds_only_the_date`, `date_only_due_in_a_dst_gap_names_the_board_time`) also run and pass by name in that environment.
- jsc `checkSyntax` on `static/app.js` exits 0.
- Every UI run had 0 console errors, 0 failed requests and 0 HTTP responses of 400 or above. Server logs show no errors.
- Log: `$D/gates.log`.

### KF-259 — the three points verify-d4 failed

**Card due line**

| Claim | Comparison (0342ad8) | Fix (2b42928) | Golden | Verdict |
|---|---|---|---|---|
| Font size, all parts | 11.84px | 10px, line-height 15px | 10px / 15px | PASS |
| Weight: "Due:" / date / "(Done)" | 700 / 700 / 700 | 400 / 700 / 400 | only the date bold (PNG and styles note) | PASS |
| Overdue colours | text #8b0000 | text #8b0000; disc #be0000, 15x15 | #8b0000; disc ~16x16 | PASS |
| Normal colour | #222222 | #222222 | #222222 | PASS |
| Ink of "Due: 30 September 11:00 PM (Done)" (macOS, CSS px, 55% threshold) | 214.5 x 11 | 181.5 x 9.5 | 181 x 9 (board/02) | PASS |
| Ink of "Due: 25 September 5:00 PM (Done)" (Linux, 1919x998) | 195 x 10 | 162 x 10 | 162 x 10 (20260930 PNG) | PASS |
| Text left edge minus title left edge (Linux) | n/a | 21 px | 22 px | PASS |

My ink numbers differ from verify-d4's 168/236 because I used a different crop and threshold. I applied one method to all three images.

**Modal due value**

| Claim | Comparison (0342ad8) | Fix (2b42928) | Golden (modal/01) | Verdict |
|---|---|---|---|---|
| Weight per part | 📅 icon plus all text at 600 | date in `<strong>` at 700; "(Done)" at 400 | date bold, "(Done)" normal | PASS |
| Icon and chip | emoji; white background; 1px #fca5a5 border, radius 6 | no emoji, no img/svg, no ::before or ::after content; border none 0px; transparent background | none | PASS |
| Size and colour | 13.6px Arial, #8b0000, box 202.5x32 | 14px/28px, #8b0000, box 168.59x28 | 14px/28px, #8b0000, box 169x28 | PASS |
| Ink of "Oct 2 at 11:00 PM (Done)" | 180 x 13 | 166 x 12 | 167 x 13 | PASS |
| Colour when not overdue | n/a | #333333 | propValue #333333 | PASS |

**Seeded golden board shows "(Done)"**

| Claim | 0342ad8's seed script | Fix's seed script | Golden | Verdict |
|---|---|---|---|---|
| Dues carry the target column | all 4 targets null; no "(Done)" anywhere | all 4 target Done | — | PASS |
| Which card lines carry "(Done)" | none | the three visible lines: "Today 5:00 PM (Done)", "Sunday 5:00 PM (Done)", "11 October 5:00 PM (Done)" | all three visible dues carry it, in 20260930 and in 10-17 board/02 | PASS |
| Onboarding (+30 days) | line hidden | line hidden, due icon kept | no line | PASS |

### Other checks
| Claim | Comparison | Fix | Verdict |
|---|---|---|---|
| Date-only due when the board time falls in a DST gap (board time 02:30, `2027-03-14`) | "that local time is skipped by a daylight-saving change" | "the board's due date time (02:30) falls in a daylight-saving gap on that date; send a time" (400) | PASS |
| Same date with an explicit time (`2027-03-14 02:30`, `…T02:30`) | old message | old "skipped" message, unchanged | PASS |
| `due_time` docs | — | `agent-guide.md` and `skill.md` describe trimming and "null leaves it unchanged". Measured: `" 09:15 "` and `"\t10:45\n"` are trimmed; null and `{}` leave the time unchanged; `9:05`, `24:00`, `09:15:00`, `""` and `"  "` return 400 | PASS |
| Tracker formats | — | "Today", a weekday for later this week, "D Month" for past dates, 7-day window, column name in the parentheses: all hold. The column name follows the target, e.g. "(In Review)" and the hostile name below | PASS |

**Regressions (one probe each, on the fix)**
- **KF-401:** Delete (#d9534f) sends `{"due_at":null,"due_repeat":null,"due_done":false}`. Due, done, repeat and target are all cleared. 1 record changed across the database (`$D/pre-del.jsonl` against `$D/post-del.jsonl`).
- **KF-404:** a date-only `2026-10-22` is stored as 17:00:00-05:00. The dialog defaults to 17:00. Task settings shows "Due date time / 5:00 PM", and its dialog is "Change default due date time".
- **KF-389:** From = To shows "0h", including 00:00 and 23:59.
- **KF-417:** an empty time shows the "Invalid date or time" alert and sends no PATCH; the due is unchanged.
- **KF-409:** an unchanged save of `2026-10-20T10:00:59Z` marked done sends the exact string and stays done.
- **KF-410:** an unchanged save of `2026-11-01T06:30:00Z` keeps the same instant.
- **KF-414:** a lowercase `2026-10-20t15:00:00z` prefills 10:00 and is sent back byte-identical.

### Break-attempts
1. **Hostile, long target column:** a column named `<img src=x onerror=alert(1)> … & "sign-off"`. It is escaped on both card and modal. The modal value wraps cleanly to two lines with the "· every week" repeat appended. No alert fired and there were no console errors.
2. **Dark mode:** on the fix, the modal due value is #333 on the task-colour modal and readable. On the comparison it was #e5e7eb on a white chip, nearly invisible, so this diff happens to fix it. The overdue card line stays #8b0000 in dark mode, as in the dark golden.
3. **DST boundaries** (board time set per probe):
   - 02:00 is in the gap and gets the new message.
   - 01:59 and 03:00 are accepted.
   - A whitespace-wrapped `" 2027-03-14 "` gets the new message.
   - POST create with a date-only due in the gap gets the new message.
   - An explicit `-05:00` offset is accepted.
   - The fall-back date 2026-11-01 is accepted.
4. **Font parity at the reference's own size:** the Linux render at 1919x998 matches the 20260930 golden exactly (162x10).

### Board due times: 11:00 PM or 5:00 PM
- The 10-17 goldens (board/02, modal/01) read **11:00 PM**.
- The fixture stores **17:00**, and the 20260930 golden and the 10-12 capture show **5:00 PM**.
- The 10-17 shift is a uniform +6 h across all dues. In the same capture, the test task entered at 9:00 AM still shows 9:00 AM. So that capture most likely displayed the board in a UTC+1 zone, relative to the CDT zone the board was built in. Edited dues would also explain it; the images cannot tell the two apart.
- My recommendation is not to change the fixture to 23:00. If you want string-exact comparisons against the 10-17 set, make the fixture dues zone-explicit and render ChipFlow in the capture's zone, after checking KanbanFlow's account timezone live.
- This is not a FAIL for KF-259.

### Observations (not KF-259 failures)
- **Modal value position:** (550.5, 198.9) against the golden's (555, 195). This comes from the property-grid layout, not the text, and predates this diff.
- **Overdue disc:** 15x15 against the golden's 16x16, offset 1 px. verify-d4 accepted it.
- **OpenAPI:** the `BoardConfigInput.due_time` description still lacks the trim/null wording that the agent docs now have.
- **Repeat normalisation:** a `due_repeat` of "weekly" set through the API becomes "every week" after an unchanged UI save. This predates the diff.
- **KF-389 probe, unconfirmed:** with To cleared, the readout showed "0h". My fill may not have cleared the time input, and this diff changes no time code.
- **Dark mode:** the modal title and subtasks are unreadable. This predates the diff and belongs to the dark-mode work.

### Evidence (under `$D`)
- **Golden / fix / comparison crops:** `pair-sep30.png` (macOS), `pair0930.png` (Linux), `pair-modal.png`.
- **Measurements:** `m-fix-2x.json`, `m-cmp-2x.json`, `m-fix-oct2.json`, `m-cmp-oct2.json`, `m-lin-fix.json`, `m-lin-cmp.json`, `m-dark-3327.json`, `m-dark-3328.json`, `m-evil.json`.
- **Screenshots:** `shots/`, `lin/out/`.
- **UI probe results:** `r-delete.json`, `r-dialog.json`, `r-settings.json`, `r-manual.json`, `r-clear.json`, `r-save-*.json`.
- **Gates and DST:** `gates.log`, `dst.sh`.

DONE: KF-259 PASS overdue card line is 10px with only the date bold (ink 181.5 vs golden 181 px, Linux 162 vs 162); modal "(Done)" is weight 400 with no icon or chip border (ink 166 vs 167); the seeded board now shows "(Done)" on the same dues as KanbanFlow; DST message and docs correct; KF-401/404/389/417/409/410/414 hold
