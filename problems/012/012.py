primes = [2]


def factorize(n):
    factors = []

    for prime in primes:
        if n == 1:
            break

        if prime * prime > n:
            factors.append((n, 1))
            break

        i = 0
        while n % prime == 0:
            i += 1
            n /= prime

        if i != 0:
            factors.append((prime, i))

    if n != 1 and n > primes[-1]:
        primes.append(n)

    return factors


def numbers_of_divisors(factors):
    nod = 1
    for (_, n) in factors:
        nod *= n + 1

    return nod


def triangle(left_factors, right_factors):
    (m, n) = (len(left_factors), len(right_factors))
    (i, j) = (0, 0)

    out = []

    while i != m or j != n:
        if i == m:
            out.append(right_factors[j])
            j += 1
        elif j == n:
            out.append(left_factors[i])
            i += 1
        else:
            ((a, x), (b, y)) = (left_factors[i], right_factors[j])

            if a == 2:
                x -= 1
            elif b == 2:
                y -= 1

            if a > b:
                out.append((b, y))
                j += 1
            elif b > a:
                out.append((a, x))
                i += 1
            else:
                if x + y > 0:
                    out.append((a, x + y))

                i += 1
                j += 1

    return out


def f():
    n = 2
    nod = 0
    nex = [(2, 1)]
    while nod < 500:
        n += 1
        (cur, nex) = (nex, factorize(n))

        nod = numbers_of_divisors(triangle(cur, nex))

    return n * (n - 1) // 2

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
