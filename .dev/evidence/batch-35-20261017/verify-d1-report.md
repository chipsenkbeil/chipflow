verify-d1 report: KF-398, KF-409, KF-410 on fix/due-followup @ 000c5ff, baseline c17372f

KF-409 and KF-410 pass. KF-398 fails. The year and separator bugs the tracker named are gone, but two due_at sentences still claim a 400 for input the binary accepts (details below).

## Environment
- Fix worktree: /Users/senkwich/projects/chipflow/.dev/worktrees/verify-d1, detached at 000c5ff. Baseline worktree: /Users/senkwich/projects/chipflow/.dev/worktrees/verify-d1-base, detached at c17372f.
- The diff from c17372f to 000c5ff touches only agent-guide.md, agents/skill.md, src/routes.rs and static/app.js. Those are the expected files.
- Debug binaries:
  - Fix md5 3d836a94c4b651558bca5434e005dd5f. Baseline md5 7413c4671e91e2cab8d27536815832ae.
  - `rg -a -c 'as are second 60'` (a string only in the fix) gives 1 on fix and 0 on baseline.
  - The served /static/app.js contains `loaded.dueAt` once on fix (3327) and not at all on baseline (3328).
- Ports: fix 3327, baseline 3328. DBs: /tmp/chipflow-verify-d1-3327.redb and /tmp/chipflow-verify-d1-3328.redb.
- Server TZ was UTC, then Europe/Dublin for the API probes, then Europe/London for the UI and regression runs.
- Viewport 1905x1000, DPR 1, a fresh context per scenario, reduced motion with transitions off.
- Both servers were killed by exact PID afterwards (96388 and 25855).
- Evidence directory: /private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/vd1/
  - API probes: probe-{fix,base}-{UTC,Dublin}.tsv
  - UI runs: ui-fix.jsonl, ui-base.jsonl
  - Screenshots: shots/
  - Scripts: /private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/r2/pw/vd1-due.mjs and /private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/r2/pw/vd1-pages.mjs

## Gates (fix tree)
- `fmt --check`: exit 0.
- `build`: ok.
- `clippy --all-targets`: 0 warnings.
- `TZ=UTC cargo test`: every suite ok (main.rs 69 passed, 1 ignored). The new `normalize_due_input_requires_four_digit_year_and_one_separator` test passes.
- jsc `checkSyntax(app.js)`: exit 0.

## KF-398: due_at docs vs binary
I sent 96 inputs through POST and PATCH under TZ=UTC and TZ=Europe/Dublin. The PATCH target always started at due 2026-01-01T00:00:00Z with due_done true.

| Claim | Baseline | Fix | Verdict |
|---|---|---|---|
| Year is four digits with no sign | `26-10-20`, `026-10-20` → 201, stored `0026-…`; `+2026-10-20` → 201; `-2026-10-20` → 201 | all 400 (POST and PATCH, both zones); `02026`/`20260` 400 | ok |
| Exactly one space or T between date and time | tab, double space, NBSP separator, `T ` → 201 | all 400; `" T"`, `"T "`, lowercase `t` (local form) 400 | ok |
| Fields other than the year may drop the leading zero | `2026-1-5`, `8:5:7`, `08:00:5` → 201 | same 201; `008:00` and a 3-digit month → 400 | ok |
| Seconds 00-59 in every form, RFC3339 included | `:60` → 400 | `:60` and `:60.5Z` → 400; `:59` → 201 | ok |
| RFC3339 stored as sent | — | `.123456789Z`, `+05:30`, `-00:00`, `t…z`, `"2026-10-20 08:00:00Z"` stored verbatim | ok |
| Bare date means 23:59:59 local | — | UTC `+00:00`; Dublin `+01:00` | ok |
| DST: fall-back takes the earlier instant; spring-forward gap is a 400 | — | Dublin: `2026-10-25 01:00`, `01:30`, `01:59:59` → `+01:00`; `02:00` → `+00:00`; `2027-03-28 01:00`..`01:59:59` → 400; `02:00` → `+01:00`. UTC: all 201 | ok |
| A rejected request changes nothing | — | PATCH 400 leaves due and due_done=true; POST 400 creates no task (0 tasks named probe-yr2, probe-tab_sep or probe-plus_year) | ok |
| "Surrounding spaces are trimmed", then "Anything else is a 400" (agent-guide.md) | — | Any Unicode whitespace is trimmed. Leading tab, trailing LF, leading NBSP and leading U+3000 → 201 in POST and PATCH, both zones. Whitespace-only `\t` or NBSP clears the due date. | **mismatch** |
| skill.md: "any other format is a 400" | — | It never mentions trimming, yet `"  2026-10-20T08:00:00Z  "` → 201 | **mismatch** |
| OpenAPI `CreateTaskInput.due_at` (UpdateTaskInput says "same formats as on create") | — | It lists only RFC3339, `"YYYY-MM-DD HH:MM[:SS]"` and `"YYYY-MM-DD"`, then says "Anything else is a 400". The binary returns 201 for `2026-10-20T08:00` (T separator), `2026-1-5`, `2026-10-20 8:5:7` and surrounding whitespace. Read from the served /api/v1/openapi.json. | **inaccurate** |
| Other tracker claims | — | 2 MiB and 9 MiB attachments → 200, so the "10 MiB" in the docs holds. `grouping_date: null` clears. The docs now state RFC3339, with Unix seconds only for the timer. | ok |

Break attempts:
- **Rejected, as they should be:** 5-digit year, Arabic-Indic and fullwidth digits (in the date and in the time), trailing `T`, `T` with no time, time with no date (`08:00`, `T08:00`), empty time (`2026-10-20 :`), `08:`, hour only, NBSP separator, `T 08:00`, `Z` or `+01:00` on a local form, `24:00`, minute 60, Feb 30, month 13, day 0, RFC3339 with comma fraction, `+0530`, `+05`, `+24:00`.
- **Accepted:** leading and trailing spaces, as documented. Lowercase `t` is a 400 in local forms but accepted in RFC3339, which RFC3339 itself allows.
- **Accepted, contradicting the docs:** leading tab, trailing LF, NBSP and U+3000 (the mismatch rows above).

## KF-409: unchanged save dropped the seconds
Each scenario was driven with real clicks: card title, then the `.tm-due-chip`, then `#dd-add`. Server TZ was London.

| Scenario | Baseline PATCH → stored, due_done | Fix PATCH → stored, due_done | Verdict |
|---|---|---|---|
| `10:00:59Z` + done, browser UTC, Add unchanged | `T10:00:00.000Z` → false | `2026-10-20T10:00:59Z` → true | ok |
| Same, browser America/Los_Angeles | `T10:00:00.000Z` → false | exact → true | ok |
| `10:00:59.123Z` (UTC and LA) | `T10:00:00.000Z` → false | exact → true | ok |
| `.1234567890123Z` (13 fraction digits) | dropped → false | exact → true | ok |
| `+23:59` offset; `t…z`; space-separated RFC3339 | rebuilt → false | exact → true | ok |
| Date only (`2026-10-20` → `23:59:59+01:00`), browser London and UTC | `22:59:00.000Z` → false | exact → true | ok |
| Open and save twice | false | both PATCHes exact → true | ok |
| Task A open, Cancel, then task B Add | B `07:07:00.000Z` → false | B `07:07:07Z` exact → true, A untouched (no stale state) | ok |
| Change the date and pick the original date again | → false | exact → true | ok |
| Change the time and type the original back | → false | exact → true | ok |
| Repeat checkbox only | due rebuilt → false | due exact → true, repeat "every week" | ok |
| Date only changed | `2026-10-21T10:00:00.000Z` → false | same | expected (real change) |
| Time only changed (11:15) | `11:15:00.000Z` → false | same | expected |
| Time field cleared | `09:00` → false | same | pre-existing |
| Date cleared (by script, since there is no UI path, KF-401) | null | null | ok |

Changing a field and changing it back re-sends the stored instant, which is the right outcome: the fields match what was loaded. Every fix run had 0 console errors, 0 failed requests and 0 responses ≥400. A crop of the due chip (baseline above fix) is at /private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/vd1/shots/crop-k409u-chip-bs-over-fx.png: baseline shows "Tuesday 11:00 AM", fix shows "Tuesday 11:00 AM (Done)".

## KF-410: unchanged save moved the repeated hour
| Scenario | Baseline | Fix | Verdict |
|---|---|---|---|
| `2026-10-25T01:30:00+00:00` + done, browser London, Add unchanged | `00:30:00.000Z`, done false | `01:30:00+00:00` exact, done true | ok |
| First occurrence `00:30+00:00`, unchanged | `00:30Z` (same instant, done kept) | exact | ok |
| Change time and type it back; change date and pick it back | `00:30Z` | exact | ok |
| A (01:30Z) Cancel, then B (01:30Z) Add | B → `00:30Z` | B exact, done true | ok |
| Browser America/New_York or Asia/Tokyo, server London | exact instant | exact string | ok |
| Date changed to 26th; time changed to 11:15 | `10-26T01:30Z`; `11:15Z` | same | expected |

## Regressions (fix, server London)
- KF-400:
  - `2026-10-25 02:00` resolves to `2026-10-25T02:00:00+00:00` (02:00Z).
  - `01:00` and `01:59:59` resolve to `+01:00`.
  - `2027-03-28 01:00` and `01:59:59` are 400; `02:00` resolves to `+01:00`.
  - `:60` is a 400 in both local and RFC3339 forms.
  - Baseline is identical.
- KF-402: re-sending `.000Z`, `09:00:00+01:00` or `2026-10-20 09:00` over a stored `08:00:00Z` keeps the stored string and due_done=true.
- Pages: the board, `/calendar`, `/calendar?month=2026-10` and `/due-date` all return 200 on both trees with 0 console errors, 0 failed requests and 0 responses ≥400. The calendar and due report list every test task except the one whose date was cleared.

## Observations (candidate defects, outside these three items)
1. On the fall-back day, editing the time to another repeated hour moves it to the first occurrence: 01:30Z edited to 01:45 is sent as `00:45:00.000Z`. Both trees do this; the dialog has no way to express the second occurrence. Low.
2. Clearing the Time field saves 09:00 silently, while the field is shown empty. Both trees. Low.
3. Editing only the date keeps the dialog's minute precision, so stored seconds are dropped (`10:00:59` becomes `10-21T10:00:00Z`). Probably acceptable, since it is an explicit edit.
4. With server TZ Dublin, `0026-10-20` is stored as `0026-10-20T23:59:59-00:25`. The LMT offset of -00:25:21 is truncated in the RFC3339 output, so the stored instant is 21 s off. Same on baseline; cosmetic.
5. For RFC3339, the binary also accepts lowercase `t`/`z` and a space separator, and stores them as sent. Chrome parses all of them. The docs' "exactly one space or T" sentence reads as if it covers RFC3339 too; it is only true of the local forms.

Fix direction for KF-398: either trim only ASCII spaces, or say "surrounding whitespace" in both docs. In the OpenAPI text, either list the T separator, the unpadded fields and the trimming, or drop "Anything else is a 400" and point to /agents.md.

DONE: KF-398 FAIL year/separator bugs fixed (baseline 201 → fix 400), but the OpenAPI due_at text still calls `2026-10-20T08:00`, `2026-1-5` and padded input a 400 when the binary returns 201, and the docs say "spaces are trimmed" while tab, LF, NBSP and U+3000 are also trimmed and accepted
DONE: KF-409 PASS baseline unchanged Add sent T10:00:00.000Z and cleared due_done; fix re-sends 2026-10-20T10:00:59Z exactly with due_done true across 12 break scenarios, 0 console errors
DONE: KF-410 PASS baseline unchanged Add in London sent 00:30Z; fix keeps 2026-10-25T01:30:00+00:00 exactly with due_done true, including change-and-revert, cancel-then-other-task and other browser zones
