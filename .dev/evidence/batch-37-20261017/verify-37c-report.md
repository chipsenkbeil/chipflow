Environment: candidate worktree `.dev/worktrees/verify-37c` at 1e1c1f8. Baseline worktree `.dev/worktrees/verify-37c-base` at 313c7a0. Both are release builds. Candidate binary md5 bdc7d83b..., baseline md5 5fefc8c2.... `rg -a -c 'membersPop.contains'` gives 1 on the candidate and 0 on the baseline. The diff is static/app.js only (a 3-line change in `closeMembers`). Ports were 3390 (candidate) and 3391 (baseline). Databases are in `$S/verify-37c/` as cand.redb and base.redb, seeded with the golden-master board. Both server PIDs were killed by exact PID afterwards. Browser: Playwright via node, light theme, 1440x435 and 1440x612, animations frozen.

Evidence is under `/private/tmp/claude-501/-Users-senkwich-projects-chipflow/c29fc99b-2474-4af8-b8d8-9e151ce77b7e/scratchpad/verify-37c/`:
- `cand.res.json` and `base.res.json` (results), `t.mjs` (test script)
- `cand.assign` and `base.assign` (API task state)
- `fmt.log`, `clippy.log`, `test.log` (gates)
- screenshots `*-c1.png` and `*-c5.png`

KF-481-ESC results, identical at both heights:

| Check | Baseline | Candidate |
|---|---|---|
| (1) Members, Escape x2 with no click | popover closes on Esc 1; focus drops to body; overlay still visible after Esc 2 (bug reproduced) | popover closes on Esc 1; focus returns to the name input; overlay hidden after Esc 2 |
| (2) popover open, one Escape | popover closed, overlay still open | same |
| (3) Members, close with ×, then Escape | overlay hidden | overlay hidden |
| (4) Enter in Members search | nothing happens: overlay and popover stay open, no card added | same |
| (4) pick member, Ctrl+Enter | task saved with `member_ids` = [the audit user's id] | same, verified via `/api/boards/{id}/tasks` |
| (5) plain Enter | dialog closes | dialog closes |
| (5) Ctrl+Shift+Enter | dialog stays open and the name is cleared | same |
| (5) Shift+Enter | dialog closes and the task modal opens | same |

The roster has one member (`rows` = 1). The only console error was a 404 from my own wrong API probe URL (`/api/v1/boards/.../tasks`). It is not an app error. The page itself logged 0 console errors and 0 failed requests in the scenarios.

Gates on the candidate:
- fmt `--check`: exit 0.
- clippy `--all-targets`: exit 0, no warnings.
- `TZ=UTC` plain libtest through `/Users/senkwich/.cargo/bin/cargo test`: exit 0, every suite ok (31, 236 with 1 ignored, and the rest).
- jsc `checkSyntax`: exit 0.

Break-attempts: I did not try a second member, and I did not try Escape pressed with focus on a roster checkbox or the Members button.

Minor note: my first server start used a zsh word-splitting slip. It failed to launch, so no bare run happened and no database was written to the owner's directory. I redid it with a bash script.

DONE: KF-481-ESC PASS (baseline needs a third Escape; the candidate closes the overlay on the second, at both 1440x435 and 1440x612)
DONE: REGRESSION PASS (checks 2-5 unchanged versus baseline, 0 app console errors, 0 failed requests)
DONE: GATE PASS (fmt, clippy 0 warnings, TZ=UTC tests, jsc all exit 0)
