posix = require "posix"

local function f()
    local sum_of_squares, sum = 0, 0;

    for i = 1,100 do
        sum = sum + i;
        sum_of_squares = sum_of_squares + i * i;
    end

    local squared_sum = sum * sum;

    return squared_sum - sum_of_squares;
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
