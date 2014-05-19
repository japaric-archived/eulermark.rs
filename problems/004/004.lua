posix = require "posix"

local function isPalindrome(n)
    local reversed, tmp = 0, n

    while tmp ~= 0 do
        reversed = 10 * reversed + (tmp % 10)
        tmp = math.floor(tmp / 10)
    end

    return reversed == n
end

local function f()
    local max = 0

    for i = 100,1000 do
        for j = 100,i do
            local p = i * j

            if p > max and isPalindrome(p) then
                max = p
            end
        end
    end

    return max
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
