import sys
import time

def f():
    return sum([x for x in range(0, 1000) if x % 3 == 0 or x % 5 == 0])

if len(sys.argv) == 1:
    print(f())
elif len(sys.argv) == 2:
    iterations = int(sys.argv[1])

    start = time.time()
    for i in range(0, iterations):
        f()
    end = time.time()

    print(int((end - start) * 1000000000) // iterations)
