# Golden-master board spec — "ChipFlow Golden Master"

Captured 2026-09-30 ~15:22 CDT from the live KanbanFlow board created in Chip's
account. This is the canonical generic-KanbanFlow reference for parity work.

- KanbanFlow board URL: https://kanbanflow.com/board/JYv2f3u
- KanbanFlow board ID: JYv2f3u
- Machine-readable fixture: `.dev/golden-master-board.json`
- KanbanFlow limitations found while building: swimlanes are premium-only
  (board is on the Free plan — no trial started); no priorities feature exists
  in KanbanFlow's task UI; 12 of 22 tasks carry basic attributes only.

## Structure

Columns (in order): Backlog | To Do | In Progress (WIP limit 3) | In Review | Done

- In Progress holds 5 tasks vs limit 3 → header shows "5 / 3" in red with
  "*LIMIT EXCEEDED* Task count: 5 Task limit: 3".
- To Do shows a "1 overdue task" indicator (task 7, due Sep 25).

Palette (8 colors, all used): Yellow, Green, Blue, Red, Orange, Purple, Magenta, Cyan.

Labels (8): backend, chore, research, design, feature, bug, docs, frontend.

## Tasks (22)

Enriched tasks (10): #1 (subtasks+estimate), #4 (due date), #6 (assignee,
subtasks, due today, estimate), #7 (assignee, overdue), #9 (estimate),
#10 (estimate), #11 (assignee, subtasks, due this week, estimate, logged
2h30m, 2 comments). Basic tasks (12): #2, #3, #5, #8, #12, #13, #14, #15,
#16, #17, #18, #19, #20, #21, #22 — name/color/column/labels/assignees only.

See `.dev/golden-master-board.json` for the full per-task data.
