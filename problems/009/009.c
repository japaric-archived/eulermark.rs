#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define PERIMETER 1000

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t a, b, c;

  for (c = PERIMETER / 3 + 1; c < PERIMETER / 2; c++)
    for (b = (PERIMETER - c) / 2 + 1; b < c; b++) {
      a = PERIMETER - b - c;

      if (a * a + b * b == c * c)
        return a * b * c;
    }
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
