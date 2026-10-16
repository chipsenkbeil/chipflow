# ChipFlow laptop validation runbook

How to check ChipFlow work on a laptop, from a fresh clone, with
nothing from the old build VM. Everything here was run or read from
source on 2026-10-16. Where a result can drift over time, the
expected value says what it was at handoff.

## Install

- Rust, stable, via rustup (https://rustup.rs). The repo builds with
  the stable toolchain.
- Node, LTS release. Only used here for a syntax check of the
  JavaScript.
- Python 3. The helper tools in `.dev/tools/` use the standard
  library only.
- Optional, for screenshot work: Playwright with its Chromium, and
  Pillow for pixel sampling.

```bash
python3 -m pip install pillow playwright
python3 -m playwright install chromium
```

Clone the repo and work from its root for every command below:

```bash
git clone https://github.com/chipsenkbeil/chipflow.git
cd chipflow
```

## The check suite

Run these before and after any change. Stop at the first failure.

```bash
cargo fmt --check
cargo build
cargo clippy --all-targets
cargo test
node --check static/app.js
python3 .dev/tools/tracker_count.py .dev/kf-parity-defects.md
```

Expected at handoff:

- The fmt check prints nothing and exits 0.
- The build finishes. Clippy finishes with zero warnings.
- `cargo test` reports 96 passed and 0 failed. The count can grow
  as tests are added. Failures cannot.
- The node check prints nothing and exits 0.
- The tracker counter prints `OPEN: 2 (KF-356 KF-357)`, `DEPLOYED: 0`,
  a RELEASED count of 110, and an excluded line of
  `FIXED=56 CLOSED=2 INVALID=2` (as of the KF-331 release,
  2026-10-16; the handoff value was OPEN 1, RELEASED 109).

### Caveats found on Chip's laptop (2026-10-16)

- Two due-date tests (`format_due_card_weekday_near_absolute_far`,
  `format_due_modal_relative_and_done_marker`) fail on any host whose
  local zone is not UTC: they build a local wall-clock string and
  append `Z`. Run `TZ=UTC cargo test` until the tests are fixed.
- `~/.cargo/config.toml` shares `build.build-dir` across checkouts, so
  a release build in one worktree can reuse another worktree's embedded
  `static/` files. Pass `--config 'build.build-dir="target/build"'` to
  every cargo command in a worktree whose output is compared or
  released, and confirm the binaries differ before comparing them.
- Agents may not run `python3` on this laptop. `.dev/tools/tracker_count.lua`
  (`luajit .dev/tools/tracker_count.lua .dev/kf-parity-defects.md`)
  prints the same report as the Python counter and reproduced the
  handoff counts above. In place of `node --check`,
  `/System/Library/Frameworks/JavaScriptCore.framework/Versions/Current/Helpers/jsc -e "checkSyntax('<abs path>/static/app.js')"`
  exits 0 on valid syntax and 3 on a syntax error.
- green-box's LAN address is unreachable away from home; reach it over
  the tailnet with `ssh -o HostName=100.121.59.71 greenbox`.

## A throwaway instance on port 3200

The binary takes three flags, each with a matching environment
variable (confirmed in `src/main.rs`):

- `--host` / `HOST`, default `0.0.0.0`
- `--port` / `PORT`, default `3000`
- `--database-path` / `DATABASE_PATH`, no flag default

Without `--database-path` the database lands in the platform data
directory, so always pass it for throwaway runs:

```bash
cargo build
./target/debug/chipflow --host 127.0.0.1 --port 3200 \
  --database-path /tmp/chipflow-test.redb
```

On first run the app sends you to `/setup`. Create the admin
account there, then log in and create a board.

There is also a scripted version, which the visual audit below
depends on:

```bash
cargo build --release
bash .dev/tools/visual-audit-setup.sh
```

What the setup script does: starts `target/release/chipflow` on
port 3200 with a fresh database at `/tmp/chipflow-audit-test.redb`,
posts to `/setup` to create a user named `audit` with a password it
generates on the spot, logs in, creates a board named
"Audit Board", and writes the session cookie to
`/tmp/cf-cookies2.txt` and the board id to `/tmp/cf-bid.txt`.

Two cautions, both also in the script header:

- The script first kills any process named `chipflow`. Never run it
  on a machine that serves production.
- It looks for the repo at `~/workspace/pomodoro-kanban`. Set
  `CHIPFLOW_REPO` to the clone path when it lives somewhere else.

## The four structural visual checks

With the instance from the setup script still running:

```bash
bash .dev/tools/chipflow-visual-audit.sh
```

The script fetches the audit board's page HTML with the session
cookie and greps the markup. A good run prints four lines:

```text
PASS: Sidebar hidden by default
PASS: Old board-toolbar removed
PASS: Board name bar present
PASS: Board tabs removed
```

These cover KF-172 (sidebar hidden by default), KF-173 (old toolbar
removed, board name bar present), and KF-174 (board tabs removed).
The same lines are saved to
`/tmp/chipflow-audit-<timestamp>/checks.txt`. A failing check
prints `FAIL:` with what broke. When no instance is running on
port 3200 the script exits 1 and leaves a note in that folder's
`status.txt`.

## Screenshot comparison method

The rules come from `.dev/VERIFIER_SPEC.md` sections 2 and 3:

- Capture both sides at the same viewport and at device pixel
  ratio 1.
- Wait for network idle plus a fixed settle delay, and let
  animations finish before capturing.
- A scrollbar present on one side and not the other is a real
  difference, not noise.
- Record the viewport, the device pixel ratio, the instance port,
  and the database identity alongside every verdict.
- No verdict counts without evidence attached: the commands run,
  the measurements, the screenshots.

The pixel-diff tool behind the old reports is lost, so compare by
pixel sampling instead. With Pillow, open the reference and the
capture, read the exact coordinates that matter (page background
away from content, container face, container edge), and compare
the RGB values. `.dev/evidence/kf331-gm134-recovery-20261016.md`
is a worked example of this method, down to a shadow edge profile
compared pixel by pixel.

Reference images live in `.dev/evidence/golden-masters/` (13
KanbanFlow captures at 1920x1000, with a README on provenance),
plus the board-settings capture at
`.dev/evidence/kf331-kanbanflow-settings-colors-recovered-20261016.png`.

About `.dev/screenshot.py`: it is a Playwright script written for
the old build VM. It takes a URL and an output path, opens a
1920x1080 Chromium page, waits for network idle plus 2 seconds,
and saves a full-page screenshot. Two parts are tied to that VM:
it launches the Chromium at `/opt/meta-chromium/chrome`, and it
routes traffic through that VM's proxy, built from the
`HTTPS_PROXY` environment variable with the port changed to 3130.
On a laptop, point `executable_path` at a local Chromium (or drop
it to use the Playwright download), and drop the proxy block for
local instances. A local port-3200 instance needs no proxy at all.

## Production checks (read-only)

Production is https://chipflow.chip.network. These checks only
read, and they are safe to run from the laptop at any time.

Version:

```bash
curl -s https://chipflow.chip.network/api/v1/version
```

At handoff this returned build SHA
`371133161bf32d98ed42e301a4c1e3858de27683`.

Served asset against the tree. Release builds embed `static/`, so
what production serves must match the file at the deployed commit,
byte for byte:

```bash
curl -s https://chipflow.chip.network/static/app.js | shasum -a 256
git show 3711331:static/app.js | shasum -a 256
```

Both printed
`b511b6b166be7630670e7addc2200e61649149304db7def47c4f1ddc4c26de52`
when checked on 2026-10-16. On Linux the hash command is
`sha256sum`. A mismatch between the two means production is not
running the tree you think it is, whatever the tracker says.

## What is deterministic and what is not

Fully deterministic today, from the laptop alone:

- The compiler checks and the unit tests.
- The tracker counts.
- The four structural visual checks against a local instance.
- The production version and served-asset hash comparisons.

Not possible without Chip in the loop:

- Pixel comparisons whose reference image was lost. Most of the
  original golden set is gone, and only the 13 recovered captures
  can anchor new comparisons.
- Anything on the KanbanFlow side. That board sits behind Chip's
  own login at kanbanflow.com, so fresh captures there need his
  browser.
- The final visual verdict on the whole app. That one is Chip's,
  by design.
