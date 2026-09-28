# KanbanFlow Timer UX — Every-Unique-Frame Review Report

Date: 2026-09-28. Coordinator: every-unique-frame review of 4 KanbanFlow reference videos.

## Method (honest)
- 11,614 native frames exported at 30fps via ffmpeg across v1–v4; near-duplicates collapsed
  (>250 half-res px diff threshold) → **961 unique UI states** in `/tmp/kf-uniq/`.
- 11 workers each read every frame in their range as images (no sampling); coordinator spot-checked
  4 disputed frames with zoom crops.
- The ~10.6k dropped frames were near-duplicates (cursor moves, per-second countdown ticks) —
  **not** individually eyeballed. Every unique state was.
- Claims marked **VERIFIED** (seen in a frame) or **INFERRED**. Frame refs: `vN-NNNNN` = video N @ NNNNN/30s.
- Coverage: v1 444/444, v2 292/292, v3 199/199, v4 26/26 = 961/961.

## (a) "Why did you stop?" menu — NOT OBSERVED in any frame
No early stop occurred in any video: v1's pomodoro ran continuously (24:57 → 23:29, never stopped);
v2/v3 timers stayed idle; v4's two stops were both <20s and hit the discard path (toast, no reason menu).
Menu items, layout, pick-behavior remain **UNVERIFIED from video** — ChipFlow's implementation rests on
Chip's verbal description only.
Related VERIFIED facts:
- Reasons editable: Settings → Interruptions tab shows all 15 defaults with drag-reorder handles,
  inline rename (click → yellow-highlighted input, v2-03480), green "Add reason" (v2-03411–03484).
- 15 defaults VERIFIED verbatim in order: Boss interrupted, Colleague interrupted, Context switch, Dog,
  Email, Family, Finished with no new task, Food Delivery, Meeting, Other, Phone call, Restroom, Sleep,
  Web browsing, Workchat (v2-03441–03484).
- Custom reason "Task done" seen live in logs (v1-01316) — not in the 15 defaults, confirming editability.
- Stopped sessions render red `Stopped Pomodoro with reason 'X'` in logs.
- INFERRED: reason menu appears only for stops ≥20s; <20s stops go to the discard toast (v4-00002:
  "Session discarded / Session lasted less than 20 seconds").

## (b) Break transition — NOT OBSERVED in any frame
No pomodoro ever completed (v1's never reached 0:00; others idle). "Take a break" button, break countdown
appearance, post-break play button are **UNVERIFIED from video** — ChipFlow's break flow is provisional,
from Chip's verbal description only.
Related VERIFIED: break settings — Work 25 / Short break 5 / Long break "Every 4th break" 15 /
Picture-in-Picture ON (v2-03302).
**New unmodeled feature**: "Break activities" system — info text with examples (Stretch, Drink water,
Take a short walk, Switch sitting/standing), green "Add activity" button, "Add break activity" dialog
(Name, Description, Daily goal = 1, Daily limit = "No limit") (v2-03512, v2-03611).

## (c) Popup "Add time" dialog — VERIFIED extensively
- "Add time manually" dialog (v3-00001, v2-01176): Task field (green, editable), Date text + calendar
  icon ("2026-09-28"), From/To time fields, Duration **auto-computed** From→To ("0h" default), collapsed
  "Add comment" and "Add labels" toggles, green "Add" button.
- Zero duration renders **"0h"**, not "0m" (v3-02777).
- Future-time validation: modal error **"You can not enter a time in the future"** + OK button
  (v3-00768–00820); dialog state preserved after dismissal (v3-00821).
- Date picker: September 2026 grid, Sun–Sat, selected day blue, hover highlight; picking updates Date field
  (v3-01268–01319).
- **Manual entries add to Time spent but NOT to Pomodori** (v3-01323: "2 Pomodori, Time spent 42m"
  after a 40m manual add).
- Badge/tooltip wording: "M" tooltip = "Manually added" (v3-01594) but board log sub-line =
  **"Manually added time"** (v3-02891) — wording differs by surface.
- Entry points: popup bottom nav "Add time", per-task Time log "+ ADD ENTRY", task-modal Time-spent hover
  menu ("Add time entry" / "Open time log", v3-01449), Add menu "Manual time entry" (shortcut Y).
- Sibling: "Add time estimate" (Add menu) = single input with ⏱ icon accepting "0h" format + green "Add"
  (v2-00465–00539).

## (d) Running stopwatch — VERIFIED (v4)
- Popup: "Stopwatch" title, **"Session time"** label, counts **UP** from "00:00", red **■ Stop**,
  **no progress fill** (v4-00001). Task row + "Change task"; TODAY running entry "11:36 AM — pending".
- Header pill is **mode-aware**: stopwatch idle = dark pill, red ■ + "00:00" + ⌄ (v4-00142) — not 25:00.
- <20s stop → toast **"Session discarded / Session lasted less than 20 seconds"**, entry removed,
  TODAY unchanged (v4-00002).
- Card badge **live-increments** during the run ("42m + 1m") and reverts on discard (v4-00001–00002).
- Bottom-nav first tab names the **other** mode: "Pomodoro" in stopwatch mode, "Stopwatch" in pomodoro mode
  (v4-00002, v4-00037).

## (e) Contradictions / nuances vs the earlier 1fps review
1. "Idle pill always shows configured duration" — qualified. Pill ABSENT in layout-edit view
   (v1-02504–02516, zoom-verified). Pill ABSENT while timer popup panel is open/docked (v2 1-98, inferred).
   Stopwatch-mode pill shows "00:00" (v4-00142). "Always" does not hold.
2. "Red progress fill" on running pill — partially verified. One zoom-verified sighting (v1-00654); two
   workers saw red digits + red icon on dark pill but could not resolve a fill gradient at 854×474.
   Likely present but subtle — not refuted.
3. "Time until break" subtitle appears in the IDLE popup too (v2-01808, v3-00001), not only when running.
4. Task-row link conditional: "Change task" normally (v1-00654, v4-00001), but **"Select open task"**
   when a different task's modal is open (v1-02073, zoom-verified). INFERRED: clicking binds the open task.
5. Alarm sound is **"Grandpa clock"**, not "Grandfather clock" (v2-03850).
6. Task-modal Timer submenu RESOLVED (zoom crops): **"Start Pomodoro / Start Stopwatch / Time log"**
   (v2-01877). "Time log / Print / History / Time in column" belongs to the **Reports** button (v2-02784).
   Print and Time in column are premium-gated ("This feature is only available in the premium version",
   v2-02835, v2-02894).
7. Card context-menu Timer submenu (different surface) = **"Start timer" / "Select in timer"**
   (v1-00306, v1-00354).

## Other verified findings (condensed)
- Full-page Timer log: filters All boards / Period ("This + Last week", "This week", "Last week",
  "This month", "Last month", "Custom (absolute)", "Custom (relative)") / Entry type; day groups
  "Friday, 10 July — 2h 26m — 5 Pomodoros"; green "Successful Pomodoro" vs red
  "Stopped Pomodoro with reason 'X'"; orange dots = stopped; day "Pomodoros" count = successful only
  (INFERRED); durations minute-truncated (INFERRED: "1:25 PM - 1:55 PM" shown as "29m");
  Excel/CSV export icons (v1-01690); empty state "No entries exist for the given filter" (v1-00995).
- Time spent report: Filter/Print/Export, Period/User/Color/Label filters, Bookmarks, Reload,
  Group by Date (descending), Summary/Detailed views (v1-01800–02035, v3-03078).
- Pomodoro Statistics: tabs Pomodoros / Interruptions / **Break activities** / Highscores; Period presets
  incl. Last 7/14/30 days; Group by Day; Export; animated bar chart with weekday tooltips
  ("Friday 10 July / Pomodoros: 5"); "No data to display" / "Loading chart..." states; custom-absolute
  dual-calendar dialog (v1-03421–03846). **Interruptions tab contents never opened** — analytics UI unverified.
- Task modal: "🍅 1 Pomodoro" / "Time spent: 17m" counters; Time-spent hover menu ("Add time entry" /
  "Open time log"); per-task Time log with "P"/"M" badges + tooltips ("Pomodoro / Stopped with reason
  'Other'", "Manually added"); **Edit Pomodoro entry dialog** — Date with green ✓ validation, Task
  autocomplete (reassignment possible), From/To with seconds, Duration, "+ Add comment"/"+ Add labels",
  green "Update" → "Updating…" state; date edits re-bucket the entry (v3-01841–02540).
  **Stopped sessions count as Pomodori** (2 stopped = "2 Pomodori", v3-01594); 31s session kept
  (consistent with <20s discard).
- Card badges: play-icon + accumulated time ("34m", "1h6m", "42m"); tomato counter "🍅 1m"
  (v1-00102, v1-00391, v1-02173).
- Keyboard shortcuts: Y manual time entry, T timer menu, P reports menu, E time estimate
  (v2-02558, v2-02705).
- "Timer users" board filter (cards with active timer) + Color filter
  "1 Pomodoro / 2 Pomodori / 3 Pomodori / >3 Pomodori" (v1-02947).
- Settings: tabs General / Interruptions / Break activities / Sounds; Ticking mode Always / Timer start /
  **Never** (Chip's = Never); 9 alarm sounds (Bell ✓, Chime, Beeps, Blip, Glass, Microwave, Egg timer,
  **Grandpa clock**, Melodic); Alarm 70%, Points 70%, Sounds ON, PiP ON (v2-03302–03913).
- Idle pill: dark, "25:00", no fill (v1-00001); pomodoro-idle header pill "▶ 25:00 ▾" with green play
  triangle (v3-03212).

## Prioritized ChipFlow refinements

### P1 — correctness vs verified reference
1. Stopwatch mode: no progress fill; pill shows red ■ + "00:00" + ⌄ (not 25:00); bottom-nav first tab
   toggles to the other mode's name.
2. Discard toast exact text: "Session discarded" / "Session lasted less than 20 seconds".
3. Task-row link: "Change task" default; "Select open task" when another task's modal is open.
4. Manual time adds to Time spent, NOT to Pomodori count.
5. Durations minute-truncated, not wall-clock rounded.
6. "Time until break" subtitle in idle popup too.
7. Two Timer submenus: card context menu = "Start timer" / "Select in timer"; task-modal =
   "Start Pomodoro" / "Start Stopwatch" / "Time log".
8. Alarm sound "Grandpa clock".
9. Header pill hidden in layout-edit view and (inferred) while timer panel open.

### P2 — missing features
10. Full-page Timer log with period filters, day totals, green/red entries, Excel/CSV export.
11. "Time spent" report (filters, Group by, Summary/Detailed).
12. Pomodoro Statistics page — Pomodoros chart + Interruptions tab (Chip's interruption analytics).
13. Card badges (play-icon + accumulated time; live "+Nm" while running).
14. Task-modal Time-spent hover menu; per-task Time log with P/M badges + tooltips; Edit Pomodoro entry dialog.
15. "Timer users" board filter.
16. Add-time validation "You can not enter a time in the future"; "0h" zero-duration rendering.
17. Keyboard shortcuts Y / T / P / E.
18. Break activities system (optional — flag for Chip).

### P3 — open questions / explicitly unverified
19. "Why did you stop?" menu and full break transition never appeared in any frame — ChipFlow's
    implementation rests on Chip's verbal description only. Recommend live KanbanFlow verification.
20. Interruptions-tab contents never opened — design analytics without reference or capture later.
21. Blue-dot log entries INFERRED as manual — low confidence, don't build on it.
22. Red progress fill on running pill: one zoom-verified sighting; keep current implementation.
