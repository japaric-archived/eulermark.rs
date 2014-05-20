posix = require "posix"

local function f()
    local i, map, n, q = 0, {}, 10000, 1

    while true do
        q = q + 1

        p = map[q]

        if p == nil then
            map[q * q] = q

            if i == n then
                return q
            else
                i = i + 1
            end
        else
            map[q] = nil
            x = p + q

            while map[x] ~= nil do
                x = x + p
            end

            map[x] = p
        end
    end
end

function toNs(ts)
    return ts[1] * 1000000000 + ts[2]
end

if #arg == 0 then
    print(f())
elseif #arg == 1 then
    iters = tonumber(arg[1])
    start = { posix.clock_gettime("monotonic") }
    for i = 1,iters do
        f()
    end
    end_ = { posix.clock_gettime("monotonic") }
    print(toNs(end_) - toNs(start))
end
