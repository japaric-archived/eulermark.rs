import sys
import time


def step_sum(start, end, step):
    (s, e) = (start // step, (end - 1) // step)

    return step * (e * (e + 1) / 2 + s * (s + 1) / 2)


def f():
    (s, e) = (0, 1000)

    return step_sum(s, e, 3) + step_sum(s, e, 5) - step_sum(s, e, 15)

if len(sys.argv) == 1:
    print(f())
elif len(sys.argv) == 2:
    iterations = int(sys.argv[1])

    start = time.time()
    for i in range(0, iterations):
        f()
    end = time.time()

    print(int((end - start) * 1000000000) // iterations)
