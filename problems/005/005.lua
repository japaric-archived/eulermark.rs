posix = require "posix"

local function gcd(a, b)
    if a < b then
        local t = a
        a = b
        b = t
    end

    while b ~= 0 do
        a, b = b, a % b
    end

    return a
end

local function lcm(a, b)
    return a * b / gcd(a, b)
end

local function f()
    local n = 2

    for i = 3,20 do
        n = lcm(n, i)
    end

    return n
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
