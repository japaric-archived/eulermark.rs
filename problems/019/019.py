def is_leap(year):
    if year % 400 == 0:
        return True
    elif year % 100 == 0:
        return False
    elif year % 4 == 0:
        return True
    else:
        return False


def days(month, year):
    if month in [0, 2, 4, 6, 7, 9, 11]:
        return 31
    elif month in [3, 5, 8, 10]:
        return 30
    elif is_leap(year):
        return 29
    else:
        return 28


def f():
    count = 0
    today = 0

    for year in range(1900, 2001):
        for month in range(0, 12):
            today = (today + days(month, year)) % 7

            if year != 1900 and today == 6:
                count += 1

    return count

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
