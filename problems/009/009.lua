posix = require "posix"

local function f()
    local perimeter = 1000

    for c = math.floor(perimeter / 3) + 1, math.floor(perimeter / 2) do
        for b = math.floor((perimeter - c) / 2) + 1, c do
            a = perimeter - b -c

            if a * a + b * b == c * c then
                return a * b * c
            end
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
