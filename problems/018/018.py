def f():
    with open("018.txt") as f:
        content = f.read()

    prev = [0]
    for line in content.split('\n'):
        row = list(map(int, line.split()))
        n = len(row)

        if not n:
            continue

        costs = [None] * n

        for i in range(0, n):
            if not i:
                costs[i] = row[i] + prev[i]
            elif i == n - 1:
                costs[i] = row[i] + prev[i - 1]
            else:
                costs[i] = row[i] + max(prev[i - 1], prev[i])

        prev = costs

    return(max(costs))


import ctypes
import sys

CLOCK_MONOTONIC = 1


class timespec(ctypes.Structure):
    _fields_ = [
        ('tv_sec', ctypes.c_long),
        ('tv_nsec', ctypes.c_long)
    ]

librt = ctypes.CDLL('librt.so.1')
clock_gettime = librt.clock_gettime
clock_gettime.argtypes = [ctypes.c_int, ctypes.POINTER(timespec)]


def to_ns(ts):
    return ts.tv_sec * int(1e9) + ts.tv_nsec


if len(sys.argv) == 1:
    print(f())
elif len(sys.argv) == 2:
    start, end = timespec(), timespec()
    iters = int(sys.argv[1])

    clock_gettime(CLOCK_MONOTONIC, ctypes.pointer(start))
    for _ in range(0, iters):
        f()
    clock_gettime(CLOCK_MONOTONIC, ctypes.pointer(end))

    print(to_ns(end) - to_ns(start))
