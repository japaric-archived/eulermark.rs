#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define LIMIT 2000000
#define SIZE (LIMIT - 1) / 2

__attribute__((always_inline))
inline uint64_t f() {
  bool *sieve = (bool *)calloc(SIZE, sizeof(bool));
  uint64_t ans = 2, i, j, p;

  for (i = 0; i < SIZE; i++) {
    if (!sieve[i]) {
      p = 2 * i + 3;

      ans += p;

      for (j = p * p; j < LIMIT; j += 2 * p)
        sieve[(j - 3) / 2] = true;
    }
  }

  free(sieve);

  return ans;
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
