# Compare two audit_dump outputs: $a (before) and $b (after).
# Usage: jq -n --slurpfile a A.json --slurpfile b B.json -f compare.jq
# Prints counts and generalized key paths only, never values.

def expand: walk(if type == "string" and (startswith("{") or startswith("["))
                 then (try fromjson catch .) else . end);
def gen($p): $p | map(if type == "number" then "[]" else tostring end) | join(".");
def leafmap: . as $v
  | reduce (path(..) as $p
            | select($v | getpath($p) | ((type != "object" and type != "array") or length == 0))
            | $p) as $p ({}; . + {($p | tojson): ($v | getpath($p))});
def tally: group_by(.) | map({path: .[0], rows: length});

($a[0].tables) as $A | ($b[0].tables) as $B
| {
  tables_only_before: ([$A | keys[]] - [$B | keys[]]),
  tables_only_after: [($B | keys[]) as $t | select($A[$t] == null) | {table: $t, count: $B[$t].count}],
  tables: [
    $A | keys[] as $t
    | select($B[$t] != null)
    | ($A[$t].rows) as $ra | ($B[$t].rows) as $rb
    | ($ra | keys) as $ka | ($rb | keys) as $kb
    | {
        table: $t,
        kind: $A[$t].kind,
        before: $A[$t].count,
        after: $B[$t].count,
        missing_rows: ($ka - $kb | length),
        new_rows: ($kb - $ka | length),
      }
    + if ($A[$t].kind | startswith("multimap")) then
        {
          pairs_before: $A[$t].pairs,
          pairs_after: $B[$t].pairs,
          lost_pairs: ([$ka[] as $k | select($rb[$k] != null) | ($ra[$k] - $rb[$k]) | length] | add // 0),
          added_pairs: ([$ka[] as $k | select($rb[$k] != null) | ($rb[$k] - $ra[$k]) | length] | add // 0)
        }
      else
        ([$ka[] as $k | select($rb[$k] != null)
          | ($ra[$k] | expand | leafmap) as $la
          | ($rb[$k] | expand | leafmap) as $lb
          | {
              changed: [$la | keys[] as $p | select($lb | has($p)) | select($la[$p] != $lb[$p]) | gen($p | fromjson)],
              removed: [$la | keys[] as $p | select($lb | has($p) | not) | gen($p | fromjson)],
              added:   [$lb | keys[] as $p | select($la | has($p) | not) | gen($p | fromjson)]
            }]) as $rows
        | {
            rows_with_changed_bytes: ([$ka[] as $k
              | select($rb[$k] != null and $A[$t].sha != null and $B[$t].sha != null)
              | select($A[$t].sha[$k] != $B[$t].sha[$k])] | length),
            changed_leaves: ([$rows[].changed[]] | tally),
            removed_leaves: ([$rows[].removed[]] | tally),
            added_leaves: ([$rows[].added[]] | tally)
          }
      end
  ]
}
| . + {
    verdict: (if (.tables_only_before | length) == 0
                 and all(.tables[]; .missing_rows == 0
                                     and ((.lost_pairs // 0) == 0)
                                     and ((.changed_leaves // []) | length) == 0
                                     and ((.removed_leaves // []) | length) == 0)
              then "NO-LOSS" else "LOSS-OR-CHANGE" end)
  }
