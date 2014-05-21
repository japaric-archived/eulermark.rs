#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define SIZE 20

__attribute__((always_inline))
inline uint64_t f() {
  int i, j;
  uint64_t grid[SIZE + 1][SIZE + 1];

  for (i = 0; i <= SIZE; i++)
    for (j = 0; j <= SIZE; j++)
      if (i == 0 || j == 0)
        grid[i][j] = 1;
      else
        grid[i][j] = grid[i - 1][j] + grid[i][j - 1];

  return grid[SIZE][SIZE];
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
