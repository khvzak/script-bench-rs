local charset = { "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f" }
local function generate_string(len)
    local data = {}
    for _ = 1, len do
        table.insert(data, charset[rand(#charset) + 1])
    end
    return table.concat(data)
end

local array = {}
for _ = 1, 100000 do
    table.insert(array, RustData.new(generate_string(8 + rand(16))))
end

local function partition(arr, lo, hi)
    local pivot = arr[math.floor((hi - lo) / 2 + lo)]
    local i = lo - 1
    local j = hi + 1
    while true do
        repeat
            i = i + 1
        until not (arr[i] < pivot)
        repeat
            j = j - 1
        until arr[j] <= pivot
        if i >= j then
            return j
        end
        arr[i], arr[j] = arr[j], arr[i]
    end
end

local function quicksort(arr, lo, hi)
    while lo >= 1 and hi >= 1 and lo < hi do
        local p = partition(arr, lo, hi)
        quicksort(arr, lo, p)
        -- Tail recursion
        lo = p + 1
    end
end

quicksort(array, 1, #array)
