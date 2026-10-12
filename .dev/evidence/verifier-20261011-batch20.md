# Batch-20 verifier report — KF-346 Export (newly built feature)

**Verdict: PRESENT-AND-CORRECT.** Commit `5701a67` (worktree
`.dev/worktrees/batch-20`, base `6bb6cd1`, detached HEAD) fully implements
the KF-346 contract. Nothing returns to the build path.

## Contract checked

KF-346: "Export is a functional destination in the 15-item Reports menu
(v7-e020)." ChipFlow previously showed toast "Export report is not
available in ChipFlow yet." (tracker section, filed 2026-10-09).

## What was verified (test instance 127.0.0.1:3233, throwaway DB, PID 32300)

### 1. Both real menu paths route to the export page — no toast
- Topbar Reports menu (`#topbar-reports-btn` → `#topbar-reports-menu` →
  `[data-report="export"]`): navigated to `/b/<id>/export`, 0 toasts.
- Board menu → Reports submenu (`#board-menu-btn` → `#board-menu
  [data-bm="reports"]` hover → `#reports-submenu [data-report="export"]`):
  navigated to `/b/<id>/export`, 0 toasts.
- The string "Export report is not available" no longer exists in
  `static/app.js` or templates.
- NOTE: the `P`-shortcut `#reports-menu` placeholder contains only "Time
  spent"/"Pomodoro statistics" links and no export item — pre-existing
  placeholder (code comment: the 15-item submenu "do[es] not exist yet" for
  that path); out of KF-346's scope.

### 2. Export page renders (1600×900, screenshots reviewed)
- `/tmp/v20t/export-light.png`: h1 "Export", close ×, description naming
  the board, 4 format buttons (Excel (.xls), CSV (.csv), XML (.xml),
  JSON (.json)).
- Close button returns to `/b/<id>`.
- Board name `Export "Test" Board, <special>` is HTML-escaped in the page
  source (no raw `<special>`); no XSS.

### 3. All 4 formats download and validate (seeded task: commas, quotes,
`&`, `<b>html</b>` in name/description; due date; labels; subtask; comment)
- Filenames sanitized: `Export-Test-Board-special-export.{xls,csv,xml,json}`.
- Status line: "Exported 1 task." after each.
- **CSV** (`out-csv.csv`): 19 header columns exactly as documented;
  RFC-4180 quoting verified:
  `"Task, with ""quotes"" & <tags>","Line one, with comma. ""Quoted"" text
  & <b>html</b>",To-do,Default,Yellow,"bug, ui",,2026-10-20T17:00:00Z,yes,2.5,4,0,0,0,<ts>,,<ts>,0/1,1`
- **XML** (`out-xml.xml`): parses with `xml.dom.minidom`; `&quot;`,
  `&amp;`, `&lt;` escaping verified; board attribute escaped.
- **JSON** (`out-json.json`): valid JSON, pretty-printed; keys
  `{board, tasks}`; record has all 19 documented fields.
- **Excel** (`out-excel.xls`): HTML-table .xls, 2 rows × 19 cells,
  HTML-escaped (`&lt;b&gt;html&lt;/b&gt;`), BOM + `charset=utf-8`.

### 4. Dataset shape (`/api/boards/:id/export-data`)
- Returns `{board, columns, swimlanes, colors, users, tasks}` — all six
  keys; raw `TaskRow`s including `history` (7 events on the seeded task),
  `column_added_at`, `member_ids`, `labels`, timestamps.
- Assessment for the 10 remaining reports: sufficient for Burndown,
  Cumulative flow, Cycle & lead time, Throughput, Task count, Due date
  performance, Time estimate (raw rows + history + lookups all present).
- 404 on unknown board for both the endpoint and the page.

### 5. Break attempts survived
- **Special characters**: commas, double quotes, `&`, `<tags>` in board
  name, task name, and description — all four formats correct.
- **Empty board**: "Exported 0 tasks.", valid `{"board": "Empty",
  "tasks": []}` JSON.
- **1100px viewport**: zero horizontal overflow.
- **Dark mode**: downloads work, no page errors. NOTE (residual, not a
  failure): the `.timer-page` shell has no dark-mode variants at all —
  `.report-desc` (#4b5563) and h1 (#111827) render low-contrast on the dark
  body. Verified the pre-existing timer statistics page has the identical
  behavior (h1 #111827 on body #0f172b), so this is a shell-wide
  pre-existing gap, not a KF-346 regression. Recommend a separate LOW
  defect for `.timer-page` dark-mode support.
- **Console/network**: zero failed requests on the export page (the
  `ERR_TUNNEL_CONNECTION_FAILED` entries in the sandbox are blocked
  external font fetches, environmental).

### 6. L0 re-run on the built tree (exact)
- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — **86 passed, 0 failed** (incl. new
  `routes::tests::build_board_export_data_assembles_lookups`)
- `node --check static/app.js` — PASS

### 7. Docs
- `agents.md` + `agents/skill.md` updated with the new endpoint (per repo
  AGENTS.md rule).
- `/api/v1/openapi.json` lists `/api/boards/{id}/export-data`.

## Files produced
- `/tmp/v20t/export-light.png`, `/tmp/v20t/export-dark2.png`,
  `/tmp/v20t/stats-dark.png` (screenshots)
- `/tmp/v20t/out-{csv,xml,json,excel}.csv/.xml/.json/.xls`,
  `/tmp/v20t/out-dark.csv`, `/tmp/v20t/out-empty.json`,
  `/tmp/v20t/out-narrow.xml` (download captures)

## Cleanup
Test server (PID 32300) killed by PID, port 3233 confirmed free,
throwaway DB `/tmp/v20.db` removed. Production untouched, nothing pushed,
tracker not modified.

## Release note (for the orchestrator)
Worktree base is `6bb6cd1`; local master is behind `origin/main`'s app
tree — the release worker must cherry-pick `5701a67` onto origin/main
(batch-19 pattern), never merge.
