posix = require "posix"

local function f()
    local ans, curr, next_ = 0, 1, 2

    while curr < 4000000 do
        if curr % 2 == 0 then
            ans = ans + curr
        end

        local tmp = next_
        next_ = next_ + curr
        curr = tmp
    end

    return ans
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
