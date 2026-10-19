-- Prepends a status line under a tracker section header
-- Usage: luajit insert_status.lua <tracker-file> <header-line> <status-line>

---Read the whole file at `path`.
---@param path string
---@return string text
local function read_all(path)
    local file = assert(io.open(path, "r"))
    local text = file:read("*a")
    file:close()
    return text
end

---Write `text` to `path`, replacing its contents.
---@param path string
---@param text string
local function write_all(path, text)
    local file = assert(io.open(path, "w"))
    file:write(text)
    file:close()
end

---Insert `status` on the line after the single whole-line match of `header`.
---Errors when the header is missing or appears more than once.
---@param text string tracker contents
---@param header string exact header line
---@param status string line to insert
---@return string text updated tracker contents
local function insert_status(text, header, status)
    local needle = "\n" .. header .. "\n"
    local _, last = text:find(needle, 1, true)
    assert(last, "header not found: " .. header)
    assert(not text:find(needle, last, true), "header not unique: " .. header)
    return text:sub(1, last) .. status .. "\n" .. text:sub(last + 1)
end

local path, header, status = arg[1], arg[2], arg[3]
if not (path and header and status) then
    io.stderr:write("usage: insert_status.lua <tracker-file> <header-line> <status-line>\n")
    os.exit(2)
end

write_all(path, insert_status(read_all(path), header, status))
