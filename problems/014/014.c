#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define SIZE 1000000

uint64_t collatz_length(uint64_t n, uint64_t* memo) {
  uint64_t next = n % 2 ? 3 * n + 1 : n / 2;

  if (n <= SIZE) {
    if (! memo[n])
      memo[n] = 1 + collatz_length(next, memo);

    return memo[n];

  }
  else
    return 1 + collatz_length(next, memo);
}

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t *memo = (uint64_t *)calloc(SIZE + 1, sizeof(uint64_t));
  uint64_t ans, i, max = 0;
  memo[1] = 1;

  for (i = 1; i <= SIZE; i++) {
    uint64_t tmp = collatz_length(i, memo);

    if (tmp > max) {
      max = tmp;
      ans = i;
    }
  }

  free(memo);

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
