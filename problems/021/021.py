def factorize(n, primes):
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
            n //= prime

        if i != 0:
            factors.append((prime, i))

    if n != 1 and n > primes[-1]:
        primes.append(n)

    return factors


def divisors(factors):
    divs = [1]

    for (x, n) in factors:
        divs = [a * b
                for a in divs
                for b in list(map(lambda i: x ** i, range(0, n + 1)))]

    return divs


def is_amicable(n, sod):
    m = sod[n]

    return m != n and m < len(sod) and sod[m] == n


def f():
    n = 10000
    primes = [2]
    sod = [0, 0]

    for i in range(2, n + 1):
        factors = factorize(i, primes)
        sod.append(sum(divisors(factors)) - i)

    return sum(filter(lambda n: is_amicable(n, sod), range(2, n + 1)))

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
