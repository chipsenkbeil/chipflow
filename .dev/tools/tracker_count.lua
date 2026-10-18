-- Counts tracker defects by status from .dev/kf-parity-defects.md
local COUNTED = { "OPEN", "IN_PROGRESS", "BUILT", "VERIFIED", "DEPLOYED", "RELEASED" }
local EXCLUDED = { "FIXED", "CLOSED", "INVALID" }

---Split `text` into lines, keeping empty ones.
---@param text string
---@return string[]
local function lines_of(text)
    local lines = {}
    for line in (text .. "\n"):gmatch("(.-)\n") do
        lines[#lines + 1] = line
    end
    return lines
end

---Group `lines` into sections, one per `###` header, header text first.
---@param lines string[]
---@return string[][] sections
local function split_sections(lines)
    local sections = {}
    local current = nil
    for _, line in ipairs(lines) do
        local header = line:match("^###%s+(.*)$") or (line == "###" and "" or nil)
        if header then
            current = { header }
            sections[#sections + 1] = current
        elseif current then
            current[#current + 1] = line
        end
    end
    return sections
end

---Find the first status token in `section`.
---@param section string[]
---@return string? state nil when the section has no status line
local function first_status(section)
    for _, line in ipairs(section) do
        local state = line:match("^%s*%-?%s*Status:%s*([A-Z_]+)")
        if state then
            return state
        end
    end
    return nil
end

local path = arg[1]
if not path then
    io.stderr:write("usage: tracker_count.lua <path-to-kf-parity-defects.md>\n")
    os.exit(2)
end
local file = io.open(path, "r")
if not file then
    io.stderr:write("error: cannot read " .. path .. "\n")
    os.exit(2)
end
local text = file:read("*a")
file:close()

local counts, excluded, other, seen = {}, {}, {}, {}
for _, state in ipairs(COUNTED) do
    counts[state] = {}
end
for _, state in ipairs(EXCLUDED) do
    excluded[state] = 0
end
local no_status, total = 0, 0

for _, section in ipairs(split_sections(lines_of(text))) do
    local id = section[1]:match("^(KF%-%d+)%f[^%w_]")
    if id and not seen[id] then
        seen[id] = true
        total = total + 1
        local state = first_status(section)
        if not state then
            no_status = no_status + 1
        elseif counts[state] then
            table.insert(counts[state], id)
        elseif excluded[state] then
            excluded[state] = excluded[state] + 1
        else
            other[state] = (other[state] or 0) + 1
        end
    end
end

for _, state in ipairs(COUNTED) do
    local ids = counts[state]
    local suffix = #ids > 0 and (" (" .. table.concat(ids, " ") .. ")") or ""
    print(("%s: %d%s"):format(state, #ids, suffix))
end
local parts = {}
for _, state in ipairs(EXCLUDED) do
    parts[#parts + 1] = ("%s=%d"):format(state, excluded[state])
end
print("excluded: " .. table.concat(parts, " "))
local other_keys = {}
for state in pairs(other) do
    other_keys[#other_keys + 1] = state
end
if #other_keys > 0 then
    table.sort(other_keys)
    local rest = {}
    for _, state in ipairs(other_keys) do
        rest[#rest + 1] = ("%s=%d"):format(state, other[state])
    end
    print("other states: " .. table.concat(rest, " "))
end
print(("no status line: %d"):format(no_status))
print(("total KF sections (first occurrences): %d"):format(total))
