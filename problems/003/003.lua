posix = require "posix"

local function f()
    local n, factor = 600851475143, 2

    while true do
        if n % factor == 0 then
            n = n / factor
        else
            factor = factor + 1
        end

        if factor * factor > n then
            return n
        elseif n == 1 then
            return factor
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
