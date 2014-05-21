d = {
    1: len('one'),
    2: len('two'),
    3: len('three'),
    4: len('four'),
    5: len('five'),
    6: len('six'),
    7: len('seven'),
    8: len('eight'),
    9: len('nine'),
    10: len('ten'),
    11: len('eleven'),
    12: len('twelve'),
    13: len('thirteen'),
    15: len('fifteen'),
    18: len('eighteen'),
    20: len('twenty'),
    30: len('thirty'),
    40: len('forty'),
    50: len('fifty'),
    80: len('eighty')
}


def word_length(n):
    if n in d:
        return d[n]
    elif n < 20:
        return word_length(n % 10) + len('teen')
    elif n < 100:
        u = n % 10

        if u == 0:
            return word_length(n // 10) + len('ty')
        else:
            t = n - u
            return word_length(t) + word_length(u)
    elif n < 1000:
        if n % 100 == 0:
            tau = 0
        else:
            tau = len('and') + word_length(n % 100)

        return word_length(n // 100) + len('hundred') + tau
    elif n % 1000 == 0:
        return word_length(n // 1000) + len('thousand')


def f():
    return sum(map(word_length, range(1, 1001)))

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
