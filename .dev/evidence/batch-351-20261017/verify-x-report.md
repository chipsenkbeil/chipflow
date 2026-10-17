## verify-x report: hotfix KF-420..KF-423 (branch fix/csrf-origin, head 7969f65)

KF-421, KF-422, KF-423 and the UI sweep pass. KF-420 fails on one point only, under the brief's own rule that "a GET that writes is a bypass". Every cross-origin cookie write by non-GET method is refused, including the real browser attacks and the proxy shape. But `GET /api/boards/{id}/calendar-feed` still creates and saves a feed token when hit cross-site. The impact is LOW, and production has the same behaviour today (details in the KF-420 section).

### Environment
- **Fix worktree:** `/Users/senkwich/projects/chipflow/.dev/worktrees/verify-x`, detached at 7969f65. The diff from 1194d71 touches only `AGENTS.md`, `agent-guide.md`, `agents/skill.md`, `src/csrf.rs` (new), `src/db.rs`, `src/main.rs`, `src/routes.rs`, `tests/task_move_concurrency.rs` and `tests/task_subtasks_members.rs`. That matches the four defects.
- **Baseline worktree:** `.dev/worktrees/verify-x-base` at 1194d71.
- **Debug binaries:**
  - fix md5 c3e4510fd5c13edd4c6fad563c6ea4a9
  - baseline md5 c239a0e226fbc5aeafa2d6d2868fe032
  - `rg -a -c 'cross-site request refused'` finds 3 in the fix and 0 in the baseline.
- **Servers and DBs:** fix on 3392 with `/tmp/chipflow-verify-x-3392.redb`; baseline on 3393 with `/tmp/chipflow-verify-x-3393.redb`. Both fresh, both run with `--database-path`, and both stopped by exact PID at the end. No containers are left running.
- **Playwright:** 1.63.0 Chromium, 1905x1000, DPR 1, a fresh context per scenario.
- **Evidence:**
  - my scripts and outputs are in `$S/verify-x/`;
  - browser attack scripts are in `$S/r2/pw/vx/` (`attack.mjs`, `realattack.mjs`) with outputs in `out/`;
  - the UI sweep is in `$S/r2/pw/vx/sweep/` with outputs in `out/sweep/`.

### Gates (fix tree; every cargo command used the isolated build-dir)
| Gate | Result |
|---|---|
| `cargo fmt --check` | exit 0 |
| `cargo build` | exit 0 |
| `cargo clippy --all-targets` | exit 0, 0 warnings |
| `TZ=UTC cargo test` | exit 0 (74 + 13 + 11 + 8 + 6 + 4 + 4 + 3 + 2 + 2 + 2 + 1 passed, 1 ignored) |
| jsc `checkSyntax(static/app.js)` | exit 0 |
| Empty environment on macOS (`env -i`, no HOME, no TZ, empty TZDIR) | **exit 101**: `format_due_card_weekday_near_absolute_far` and `format_due_modal_relative_and_done_marker` fail ("got Sunday 6:00 PM") |
| Linux container `rust:1.98.0`: `/etc/localtime` removed, `env -i`, no HOME, no TZ, empty TZDIR (the Nix sandbox shape) | **exit 0**, every suite passes, including all three `format_due_*` tests |

- **Why the empty-environment run fails on macOS:** chrono still reads the local zone from `/etc/localtime` (here Europe/London), whatever TZDIR says. So on a Mac this run does not reproduce the Nix sandbox.
- **Same failure on the baseline:** the baseline fails the same two tests with the local TZ and passes them with `TZ=UTC`. This is the known test bug, not caused by this diff.
- Logs: `/tmp/vx-test-*.txt`, `$S/verify-x/gates.out`, `$S/verify-x/linux-sandbox-test.out`.

### KF-420 (CSRF): FAIL, one GET bypass; every non-GET vector is refused

**Real browser, no request interception.** The attacker page was a real page on the other instance: same site 127.0.0.1, different port. The browser sent `Sec-Fetch-Site: same-site` with the cookie attached. Output: `out/realattack-339{2,3}.json`.

| Attack | Baseline 3393 | Fix 3392 |
|---|---|---|
| Form POST /settings (urlencoded) | 303; pomodoro 41→50 (written) | 403; unchanged |
| Form POST /api/tasks, urlencoded | 200, task created | 403 |
| Form POST /api/tasks, text/plain | 200, task created | 403 |
| Form POST /api/tasks, multipart | 400 (parse fails) | 403 |
| no-cors fetch POST text/plain | task created | 403 |
| Form POST /api/v1/auth/tokens | 400 (scopes cannot be urlencoded) | 403 |
| Form POST /logout | 303; victim session gets 401 | 403; session still 200 |
| fetch DELETE board / PUT archive / JSON token | blocked by the preflight | blocked by the preflight |

The baseline also lost a write when a cross-origin cookie fetch went through, which the fix refuses.

**Other browser modes** (`out/attack-*-339x.json`):
- **Cross-site** (attacker on 127.0.0.1, target on localhost) and **Origin null** (sandboxed iframe): SameSite=Lax already withholds the cookie, so the baseline gives 401 or 303 with no effect. The fix answers all of them with 403.
- **Proxy shape:** a route emulating Caddy (Host chipflow.chip.network, `X-Forwarded-Proto: https`) with the attacker at https://wallabag.chip.network. The baseline was written to (settings changed, 3 tasks created, logged out). The fix refused all 10 writes with 403.
- **Harness artifact, not a product finding:** with `context.route` enabled, Playwright answers CORS preflights itself. Under that, a cross-origin fetch DELETE really deleted a board on the baseline (a throwaway board, and earlier the Golden Master board on my own test DB). With no interception, real Chromium blocks it at the preflight. Only the interception runs show this.

**curl matrix** (`$S/verify-x/matrix-339x.txt`). Each variant was sent to six write endpoints: settings form; task create as urlencoded, text/plain and multipart; token create as JSON; column PATCH as JSON.

| Header variant | Fix result |
|---|---|
| No headers | allowed (same status as the baseline) |
| Origin same | allowed |
| `Sec-Fetch-Site: same-origin` | allowed |
| `Sec-Fetch-Site: none` | allowed |
| Upper-case `HTTP://` same origin | allowed (normalised) |
| `same-site` + other port | **403** |
| Origin other port | **403** |
| `cross-site` + localhost | **403** |
| localhost, same port | **403** |
| `Origin: null` | **403** |
| `cross-site` alone | **403** |
| Sibling https origin | **403** |
| Trailing-slash origin | **403** |
| Origin with a path | **403** |
| Empty `Sec-Fetch-Site` | **403** |
| `Same-Origin` (wrong case) | **403** |
| https origin against plain http | **403** |
| `X-Forwarded-Proto: https` + http origin | **403** |
| `X-Forwarded-Host: evil` + `Origin: evil` | **403** |
| Two Origin headers, evil first | **403** |
| PUT /api/settings cross-site | **403** |
| DELETE board cross-site | **403** |
| Custom method FOO | **403** |

The baseline accepted every one of these.

**Bearer requests:**
- A garbage Bearer token sent with the cookie gets 401 on both builds. Auth never falls back to the cookie when a Bearer header is present.
- A valid Bearer token is let through even with cross-site headers (the builder's unit test covers this).

**Requests the fix allows that no browser can send:**
- `Host: evil` together with `Origin: evil` and an explicit cookie: allowed. Behind Caddy the Host header must be chipflow.chip.network or the request is never routed, and the session cookie would not be sent to another host.
- Two Origin headers with the same-origin one first: allowed. Browsers send only one.
- OPTIONS has no CORS response headers (401 or 405), so preflighted cross-origin requests stay blocked.

**GET writes found** (the brief counts these as bypasses):
1. **`GET` and `HEAD /api/boards/{id}/calendar-feed`** (`ensure_feed_token`). A cross-site GET created a token and saved it: a later same-origin GET returned the same token, and the public `.ics` URL answered 200. This happens on both builds.
   - **Impact: LOW.** The attacker cannot read the response (no CORS), needs the board UUID, and only causes a secret feed URL to exist.
   - The same behaviour is in production today.
2. **`GET /api/timer/status`** clears a timer whose task was deleted. On the baseline the DB file's md5 changed on a cross-site GET after timer start and task delete. This only cleans up orphaned state, so I put it as an observation rather than a bypass.

The rest of the GET routes have no writes: I scanned every GET handler in `routes.rs` for calls to helpers that write to the database. `auth.rs` only writes for login and session creation, and for Bearer token touch.

**Production proxy.** `modules/proxy.nix` defines `chipflow = simple "127.0.0.1:3000"`, so Caddy passes the client's Host through and sets `X-Forwarded-Proto`. `request_is_https` is true when the first comma-separated value of `X-Forwarded-Proto` is https (case-insensitive), or when `Forwarded` contains `proto=https`. curl in the Caddy shape (Host chipflow.chip.network, `X-Forwarded-For`, `X-Forwarded-Host`, `Via`), fix results:

| Request | Fix result |
|---|---|
| `X-Forwarded-Proto: https` + Origin https://chipflow + `Sec-Fetch-Site: same-origin` | 200 |
| Same with no `Sec-Fetch-Site` (old Safari) | 200 |
| Origin with `:443` | 200 |
| Upper-case host | 200 |
| `Forwarded: proto=https` | 200 |
| `X-Forwarded-Proto: "https, http"` | 200 |
| Login POST, same-origin | 303 |
| Origin wallabag | **403** |
| Origin wallabag + `same-site` | **403** |
| http origin through the https proxy | **403** |
| Login POST from wallabag | **403** |
| **No `X-Forwarded-Proto` + https Origin + no `Sec-Fetch-Site`** | **403** |

The last row is fail-closed: if Caddy ever stopped sending `X-Forwarded-Proto`, old browsers without `Sec-Fetch-Site` would be refused. Current browsers decide on `Sec-Fetch-Site` alone.

**Owner's check: does any `*.chip.network` page POST to ChipFlow?** No. `rg -i chipflow` over the homelab repo finds only `modules/chipflow.nix`, the Caddy route line in `proxy.nix` and `hosts/green-box/default.nix: chipflow.enable = true`. There is no dashboard tile, no Gatus check and no form anywhere, and the dotfiles have no references either.

### KF-421 (lost writes): PASS
`$S/verify-x/race.sh` ran nine concurrent loops. Against task A: comments, subtasks and PATCH of the description. Against its cell: 3 neighbour movers (reorder at varying index, reorder, cross-column move), 50 copies (task creates into the cell), column PATCH of the name, and 200 column moves.

| Claim | Baseline (N=200) | Fix (N=200, then 300 twice) |
|---|---|---|
| Comments kept | 55/200 (145 lost) | 200/200, 300/300, 300/300 |
| Subtasks kept | 44/200 (156 lost) | 200/200, 300/300, 300/300 |
| Final description = last 200 | d199 (reverted) | match |
| Cell positions dense, task count right | yes / 55 | yes / 55, 80, 80 |

**Break-attempt `race2.sh`:** comments on T1 and T3 raced against 20 palette copy-from calls (which rewrite tasks' color ids), 100 cross-board moves of a neighbour, 20 task deletes and color reorders.
- Baseline: 23 and 26 comments lost; T1 and T3 left with stale color ids.
- Fix: 0 lost and 0 resurrected deletes. One dangling color id remains, on the task moved across boards. A sequential cross-board move does the same, so it is existing behaviour, not a race.

**Audit of `db.rs`:**
- The builder's list is confirmed: `move_task`, `renumber_cell`, `delete_task`, `delete_board`, `copy_colors`, `move_column`, `move_swimlane` and `move_color` now read inside the write transaction. `delete_column` and `delete_swimlane` changed too (they now error if the column or lane holds tasks).
- No remaining function writes a stale copy of a row it read outside the transaction.
- Remaining check-then-act races, none of which overwrite rows (candidate LOW defects):
  - `delete_color` counts tasks using the color outside the transaction, so a concurrent assignment leaves a dangling color id;
  - `update_color` and `set_board_default_color` read the id list outside it, so two colors could end up default;
  - `create_task` reads the max position outside it, so positions can be duplicated;
  - `create_entry_full` and `create_entry_at` check the task exists outside it, so an entry can attach to a task just deleted;
  - `copy_colors` reads the source palette outside it;
  - `ensure_default_swimlane`;
  - `apply_template` is not atomic.

### KF-422 (column PATCH): PASS
`$S/verify-x/kf422.sh` compared a snapshot after each request: the column's `<th>` data attributes (name, description, WIP, done, collapsed, config) plus the API JSON. A second check took the md5 of the whole `.redb` file.

| Request | Baseline | Fix |
|---|---|---|
| `{"name":"RENAMED","is_done":true,"wip_limit":0}` → 400 | partly written; DB file changed | identical; DB file unchanged |
| 9 more invalid bodies → 400: WIP -5, blank or empty name, bad `config_json`, wrong JSON types, mixed, form-encoded WIP 0 | (changed) | every one identical; DB file unchanged |
| Valid full PATCH | applied | applies all 6 fields, trimmed |
| Unknown column | 404 | 404 |

### KF-423 (404 subtask): PASS
`$S/verify-x/kf423.sh` took the md5 of the DB file and the task row from export-data around each request. A read-only control GET left both unchanged.

| Request (each → 404) | Baseline | Fix |
|---|---|---|
| PATCH unknown subtask | DB file changed | unchanged |
| DELETE unknown subtask | DB file changed | unchanged |
| PATCH subtask on unknown task | DB file changed | unchanged |
| DELETE subtask on unknown task | DB file changed | unchanged |
| DELETE the same subtask a second time | DB file changed | unchanged |

On both builds the real PATCH and DELETE still write (controls). Break-attempt: the unknown-task variants, which the old `mutate` committed even when the row was missing.

### UI sweep: PASS
A helper ran the sweep through real clicks, drags and menus, with no interception. I checked its JSON totals myself. Scripts are in `$S/r2/pw/vx/sweep/sweep.mjs`; outputs are `out/sweep/fix-run1.json` and `out/sweep/base-run1.json`.

| | Fix | Baseline |
|---|---|---|
| Responses | 797 | 799 |
| Writes | 101: 200×88, 303×13 | 101 |
| `Sec-Fetch-Site` on writes | all same-origin | — |
| 403 responses / `cross-site request refused` | 0 / 0 | 0 / 0 |
| Persistence assertions | 94/94 | 94/94 |
| Page errors | 0 | 0 |

- **Covered:**
  - first-run setup in the UI on a fresh DB (`out/fix-setup-login.json`), then login and logout;
  - tasks: create, edit, rename, drag-move, every Move path in the menu, copy and delete;
  - in the task modal: comments, subtasks (including reorder), labels, members, color, due date set, and manual time add, edit and delete;
  - timer start and stop;
  - board settings: General saved; every Colors action, including copy-from; columns added, edited, moved and deleted; swimlanes added, renamed, moved and deleted;
  - boards: create (including from a template), archive, and delete through the styled confirm;
  - API token create and revoke;
  - templates: save, use and delete;
  - filters;
  - a save of the global `/settings` form.
- **Not exercised, because there is no UI for it on either build:**
  - attachments (Add → Attachment is disabled, and there is no file input);
  - clearing a due date (`#dd-date` is readonly, with no clear button);
  - the column Done flag;
  - saving the Layout, Task settings, Advanced, API & Webhooks and Add task from email tabs (nothing to save; visited, all 200).
- **Pre-existing errors, identical on both builds:**
  - 22 console errors: the `/login` page's `/api/timer/settings` and `/api/timer/status` polling returning 401;
  - 22 `net::ERR_ABORTED`: writes that had already returned 200 before the page reloaded or navigated (this is the "aborted DELETE");
  - nothing appears only on the fix.

### Observations (candidate new defects; all on both builds, none caused by this diff)
- **Calendar-feed GET writes** (KF-420 residual, LOW). It needs a separate fix, such as creating the token only on an explicit POST.
- **Timer-status GET writes** (LOW). `delete_task` should probably clear its own timer.
- **Check-then-act races** in `db.rs`, listed under KF-421.
- **A task moved to another board keeps the source board's color id.**
- **Found by the sweep helper:**
  1. Color drag-reorder on the Colors tab never binds: the inline script checks `window.Sortable` before the deferred script has loaded.
  2. The color Edit dialog's Cancel/× and the Premium dialog's Close/× don't close them.
  3. The lane menu on a folded swimlane strip is dead.
  4. Lane Move up/down is a no-op when only one lane has a label row.
  5. A manual time entry is matched to a task by name across all boards, so it can land on the wrong task. Evidence: `sweep/namecollide.mjs`.
  6. The User filter ignores member assignment.
  7. From reading the code only: `new_board.html` calls `showError` from the wrong scope on the template-delete failure path.
  8. Dragging a column header natively crashes headless Chromium. This may be a harness limit.

DONE: KF-420 FAIL every cross-origin cookie write by POST/PUT/PATCH/DELETE is refused (real browser same-site, cross-site, null origin, Caddy proxy shape, curl matrix), and same-origin, no-header, Bearer and real Caddy requests pass; but GET /api/boards/{id}/calendar-feed still creates and saves a feed token on a cross-site GET (LOW impact, same in production) — the brief's GET-bypass rule
DONE: KF-421 PASS 0 lost writes in 200/300/300-iteration races and the copy-from/cross-board/delete race on the fix; baseline lost 145/200 comments and 156/200 subtasks; no remaining stale-copy write in db.rs (only check-then-act races, listed)
DONE: KF-422 PASS 10 invalid column PATCHes leave the column and the whole .redb file byte-identical on the fix; baseline partly wrote; a valid PATCH applies all 6 fields
DONE: KF-423 PASS 404 subtask PATCH/DELETE (unknown subtask or unknown task) leave the .redb md5 unchanged on the fix; baseline committed every time
DONE: UI-SWEEP PASS 797 requests, 101 writes, 0 guard 403s, 94/94 persistence checks on the fix; console errors (22 login-page timer 401s) and 22 aborted already-200 writes are identical on the baseline; attachments, due-date clear and the column Done flag have no UI to test
