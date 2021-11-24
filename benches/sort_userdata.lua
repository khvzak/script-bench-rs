local charset = {"0","1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"}
local function generate_string(len)
    local data = {}
    for i = 1,len do
        table.insert(data, charset[rand(#charset) + 1])
    end
    return table.concat(data)
end

local array = {}
for i = 1, 100000 do
    table.insert(array, RustString.new(generate_string(rand(16))))
end

table.sort(array)
