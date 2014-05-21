#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

bool is_palindrome(uint64_t n) {
  uint64_t reversed = 0, tmp = n;

  while (tmp) {
    reversed = 10 * reversed + (tmp % 10);
    tmp /= 10;
  }

  return reversed == n;
}

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t i, j, max = 0;

  for (i = 100; i < 1000; i++)
    for (j = 100; j < i; j++) {
      uint64_t p = i * j;

      if (p > max && is_palindrome(p))
        max = p;
    }

  return max;
}

int64_t to_ns(struct timespec ts) {
  return (uint64_t)ts.tv_sec * 1000000000lu + (uint64_t)ts.tv_nsec;
}

int main(int argc, char *argv[]) {
  if (argc == 1) {
    printf("%lu\n", f());
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
