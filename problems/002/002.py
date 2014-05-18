import sys
import time


def f():
    ans, curr, next = 0, 1, 2

    while curr < 4000000:
        if curr % 2 == 0:
            ans += curr

        tmp = next
        next += curr
        curr = tmp

    return ans

if len(sys.argv) == 1:
    print(f())
elif len(sys.argv) == 2:
    iterations = int(sys.argv[1])

    start = time.time()
    for i in range(0, iterations):
        f()
    end = time.time()

    print(int((end - start) * 1000000000) // iterations)
