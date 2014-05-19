#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

uint64_t gcd(uint64_t a, uint64_t b) {
  if (a < b) {
    uint64_t t = a;
    a = b;
    b = t;
  }

  uint64_t r;
  while (r = a % b) {
    a = b;
    b = r;
  }

  return b;
}

uint64_t lcm(uint64_t a, uint64_t b) {
  return a * b / gcd(a, b);
}

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t n = 2;

  uint64_t i;
  for (i = 3; i < 21; i++)
    n = lcm(n, i);

  return n;
}

int64_t to_ns(struct timespec ts) {
  return (uint64_t)ts.tv_sec * 1000000000lu + (uint64_t)ts.tv_nsec;
}

int main(int argc, char *argv[]) {
  if (argc == 1) {
    printf("%d\n", f());
  } else if (argc == 2) {
    struct timespec start = {0, 0}, end = {0, 0};
    uint64_t i = 0;
    uint64_t iters = atol(argv[1]);

    clock_gettime(CLOCK_MONOTONIC, &start);
    for (; i < iters; i++) {
      f();
      asm("");
    }
    clock_gettime(CLOCK_MONOTONIC, &end);

    printf("%lu", to_ns(end) - to_ns(start));
  } else {
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
