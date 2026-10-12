# Builder evidence — batch-20: KF-346 Export report

## What was built

KF-346 (Export) was an unimplemented Reports-menu destination: the menu item
rendered, but `BoardReports.reportAction`'s `default:` branch showed the
"Export report is not available in ChipFlow yet." toast. Export is now a
functional full-page report.

KanbanFlow reference (from the repo's own evidence + KanbanFlow's public
docs):
- The 15-item Reports menu ends with "Export" (catalog v1 S15, e023).
- Full-page reports carry an Export button (v1.md:61 "Time spent", v1.md:93
  "Pomodoro Statistics").
- KanbanFlow's integrations page documents the export targets: tasks export
  to **Excel, CSV, XML and JSON**.

## Implementation

**Server (`src/routes.rs`):**
- `GET /api/boards/:id/export-data` → `BoardExportData` JSON:
  `{board:{id,name}, columns:[{id,name}], swimlanes:[{id,name}],
  colors:[{id,value,label}], users:[{id,username}], tasks:[TaskRow…]}`.
  This is the shared data shape for the board reports — Burndown,
  Cumulative flow, Cycle & lead time, etc. can consume the same endpoint
  and resolve display names client-side from the lookups. 404 on unknown
  board. Registered in the utoipa `paths(...)` list so `/api/v1/openapi.json`
  documents it.
- `GET /b/:board_id/export` → `ExportTemplate` full-page report (404 on
  unknown board). Not in the OpenAPI paths, matching the timer report pages.
- `build_board_export_data` is a pure assembly function (unit-tested).
- `ToSchema` added to `TaskRow` (`src/models.rs`) so the OpenAPI schema is
  accurate.

**Client (`static/app.js`):**
- `reportAction` gains `case 'export':` → navigates to `/b/:id/export`
  (both the board-menu Reports submenu and the topbar reports menu route
  through `reportAction`, so both are covered).
- New `ExportPage` module (exposed as `window.ExportPage`): fetches the
  dataset, resolves names via the lookup tables, and downloads the file
  client-side — same pattern as the timer-log exports (KF-080/KF-294):
  Excel as an HTML-table `.xls` Excel opens natively (no library, pure-Rust
  rule intact), CSV with RFC-4180 quoting, well-formed XML, pretty JSON.
  Filenames: `<board-name>-export.<ext>`, sanitized.

**Template (`templates/export.html`):** full-page report pattern (title +
close × back to the board), description, four format buttons, status line.

**Styles (`static/style.css`):** `.report-desc`, `.export-formats`.

**Docs:** `agents.md` and `agents/skill.md` document the new endpoint (repo
AGENTS.md requires this when API behavior changes).

## Export columns (tabular formats)

Task, Description, Column, Swimlane, Color, Labels, Members, Due date,
Due done, Estimate (h), Size (pomodori), Time spent (min), Pomodoros,
Interruptions, Created at, Completed at, Added to column at, Subtasks
(done/total), Comments.

## L0 (exact)

- `cargo fmt --check` — PASS
- `cargo build` — PASS
- `cargo clippy --all-targets` — 0 warnings
- `cargo test` — 86 passed, 0 failed (incl. new
  `build_board_export_data_assembles_lookups`)
- `node --check static/app.js` — PASS

## Local confirmation (test instance 127.0.0.1:3233, throwaway DB)

- Seeded board "Export Test Board" + task with comma/quotes in the
  description, due date, labels, estimate, subtask.
- `GET /api/boards/:id/export-data` returns all six top-level keys;
  404 on unknown board; `/b/nope/export` 404s; endpoint present in
  `/api/v1/openapi.json`.
- Playwright (authed): all four format buttons download —
  `Export-Test-Board-export.xls` (922 B), `.csv` (421 B), `.xml` (843 B),
  `.json` (698 B); status line reads "Exported 1 task."
- CSV quoting verified: `"Line one, with comma. ""Quoted"" text."`;
  XML entities escaped; JSON pretty-printed with board name.
- Board page → topbar Reports menu → Export navigates to `/b/:id/export`.
- Dark mode: all four buttons visible, no degradation.

Test server killed by port PID, port confirmed free, throwaway DB removed.
Production untouched, nothing pushed, tracker not modified.

## Notes for follow-on report builders

- `GET /api/boards/:id/export-data` is the intended shared dataset for all
  board reports. It returns raw `TaskRow`s (created_at/completed_at/
  column history intact) plus lookup tables — enough for Burndown,
  Cumulative flow, Cycle & lead time, Throughput, Task count, Due date
  performance, and Time estimate without new endpoints.
- Base for this worktree is `6bb6cd1` (local master); the release worker
  must cherry-pick onto `origin/main`'s app tree as in batch-19 — local
  master is behind origin/main.
