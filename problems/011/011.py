DIRECTIONS = [(1, 0), (0, 1), (1, 1), (1, -1)]
SIZE = 20
WINDOW = 4


def f():
    grid = []
    max_ = 0

    with open("011.txt") as f:
        for number in f.read().split():
            grid.append(int(number))

    for (row_step, col_step) in DIRECTIONS:
        for row in range(0, SIZE):
            max_row = row + WINDOW * row_step
            if max_row > SIZE or max_row < -1:
                continue

            for col in range(0, SIZE):
                max_col = col + WINDOW * col_step
                if max_col > SIZE or max_col < -1:
                    continue

                tmp = 1
                idx = row * SIZE + col

                for _ in range(0, WINDOW):
                    tmp *= grid[idx]

                    idx += row_step * SIZE + col_step

                if tmp > max_:
                    max_ = tmp

    return max_


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
