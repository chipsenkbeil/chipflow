# Golden-master references — recovered captures

## Read this first: the original set is lost

The original golden-master capture set (the `GM-NNN` images this directory
was created for) and the `.dev/golden-master-verification-standard.md`
document were **never committed to this repo and are lost**. They existed
only on Dale's VM / in working notes during the Sep 30 – Oct 5 capture
passes.

What survives is:

- `.dev/golden-master-2026-09-30.md` and `.dev/golden-master-board.json`
  (the fixture board spec) — tracked on `main`.
- The captures in this directory, recovered on 2026-10-16 from Dale's
  media library (`~/workspace/user/media_library/` on that VM) and
  verified by eye to show **KanbanFlow** (the reference app), not
  ChipFlow. These are the surviving visual references.
- The KF-331 board-settings reference, already tracked at
  `.dev/evidence/kf331-kanbanflow-settings-colors-recovered-20261016.png`
  (KanbanFlow board settings, Colors tab, captured 2026-09-29; byte
  identical to media-library file
  `browser_screenshots/a8/a84a34fe…7901.png`, sha256
  `a84a34feb48214dbac691c3187d470b2eabb47eb50674a27fed6a8a271eb7901`).
  It is referenced here rather than duplicated.

"Capture date" below is the source file's modification time (UTC). All
images are 1920×1000 unless noted.

## Index

| File | Captured | What it shows | Original path (Dale's VM media library) | sha256 |
|---|---|---|---|---|
| `kanbanflow-delete-board-dialog-20260929.png` | 2026-09-29 | KanbanFlow "Delete board" confirmation dialog ("Dale fidelity test") on the pale-blue settings background | `browser_screenshots/5d/5dc9db75….png` | `5dc9db750ab2838819e0d9d8a37fe4410b21b91469e2646f4005e21aac478769` |
| `kanbanflow-golden-master-board-20260930.png` | 2026-09-30 | The "ChipFlow Golden Master" board on KanbanFlow, all five columns expanded | `browser_screenshots/9f/9f3b5e83….png` | `9f3b5e83d5968d99d919f9c68fbdec8b8729e15603458513465a42dc6579bae5` |
| `kanbanflow-board-todo-collapsed-20260930.png` | 2026-09-30 | Golden Master board with the To Do column collapsed to a vertical strip | `browser_screenshots/ed/edc82595….png` | `edc82595baeb8ccb68477881beece9e90969cdae9c721d2b672de04896e8817e` |
| `kanbanflow-board-todo-collapsed-2-20260930.png` | 2026-09-30 | Same collapsed-To-Do board state, second capture (near-duplicate) | `browser_screenshots/86/867a21e4….png` | `867a21e4e368104329981dccd42f07a7a75e0ab6c3e2fe084f86ef9367fd5a39` |
| `kanbanflow-board-columns-collapsed-20260930.png` | 2026-09-30 | Golden Master board with To Do and In Progress both collapsed | `browser_screenshots/b1/b18c4179….png` | `b18c41796a7b696bf84be4aef33520fd2f0c07e245f95958f5dd76ea1204d024` |
| `kanbanflow-task-modal-color-dropdown-20260930.png` | 2026-09-30 | Task editor modal with the color dropdown open (plain color names, no pomodoro labels) | `browser_screenshots/d8/d89ffdcc….png` | `d89ffdcc454bbbf711714fbbb6d9bb7d066b2e8c007c74289830e3d43ead1e79` |
| `kanbanflow-column-info-popover-20260930.png` | 2026-09-30 | Column header popover: "Column: Backlog / Task count: 5" | `browser_screenshots/f2/f28abffb….png` | `f28abffba3cd7d2168ae7ec53d0d9519524dd07a6622be9d2b13ee6eba1e8f4e` |
| `kanbanflow-board-menu-open-legend-20260930.png` | 2026-09-30 | Golden Master board, board Menu panel open (toggles as plain switches), color-name legend strip along the bottom | `browser_screenshots/b1/b1767acc….png` | `b1767acc68636a3e5d234fe3ab4f4336dd69d5bcb5c2c5ac6cf969260df4e3eb` |
| `kanbanflow-general-board-done-groups-20260930.png` | 2026-09-30 | Chip's real "General" board: Done column grouped by date ("Today", "Friday, 10 July", …) and the 4-segment pomodoro legend strip at the bottom | `browser_screenshots/30/3085c577….png` | `3085c577dddf1d8709bdc43868d14f0625e921226ec8c49e87b7ee5049cb818f` |
| `kanbanflow-general-board-menu-open-20260930.png` | 2026-09-30 | "General" board with the board Menu panel open (Color legend toggle ON) | `browser_screenshots/f3/f3f79e9c….png` | `f3f79e9caeaf83f991e8af4528c5caf632fb43e4c0dd0973d79cdcad72ffd2d3` |
| `kanbanflow-pomodoro-panel-20261005.png` | 2026-10-05 | Pomodoro panel open on the Golden Master board ("Time until break / Take break", today's log) | `browser_screenshots/25/252db4ed….png` | `252db4edd9e892cdd25ade3ec58156b8fe62898805c3f15e0c086b357e82cc9c` |
| `kanbanflow-board-menu-open-20261012.png` | 2026-10-12 | Chip's own screenshot: Golden Master board with Menu open — plain toggle switches, light-grey panel, no icons (the KF-353 reference) | `image/92/9234e05c….png` | `9234e05c70aa49b52889df6c1efb0a23d0298b6db3cae2ea3c0b385545688772` |
| `kanbanflow-board-20261012.png` | 2026-10-12 | Chip's own screenshot: Golden Master board with the full 8-color legend strip (Yellow…Cyan) along the bottom (the KF-354 reference) | `image/47/474e30c8….png` | `474e30c864c8adcf602f09d2eda90d0eab85b7136135392dbe3cd2619368e3e9` |

Original paths are relative to `~/workspace/user/media_library/` on Dale's
VM; full original filenames are the sha256 with the shown directory prefix
(the media library names files by content hash).
