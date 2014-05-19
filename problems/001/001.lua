posix = require "posix"

local function stepSum(end_, step)
    local e = math.floor((end_ - 1) / step)

    return step * e * (e + 1) / 2
end

local function f()
    local end_ = 1000

    return stepSum(end_, 3) + stepSum(end_, 5) - stepSum(end_, 15)
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
