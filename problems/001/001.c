#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

uint64_t step_sum(uint64_t end, uint64_t step) {
  uint64_t e = (end - 1) / step;

  return step * e * (e + 1) / 2;
}

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t end = 1000;

  return step_sum(end, 3) + step_sum(end, 5) - step_sum(end, 15);
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
