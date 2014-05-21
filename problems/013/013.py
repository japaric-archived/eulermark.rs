def f():
    with open("013.txt") as f:
        return str(sum(map(int, f.readlines())))[0:10]

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
