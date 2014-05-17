#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

__attribute__((always_inline))
inline int64_t precise_time_ns() {
  struct timespec start = {0, 0};
  clock_gettime(CLOCK_MONOTONIC, &start);

  return (uint64_t)start.tv_sec * 1000000000 + (uint64_t)start.tv_nsec;
}

int benchmark(int iterations, int (*func)(void)) {
  uint64_t start = precise_time_ns();

  int i;
  for (i = 0; i < iterations; i++) {
    func();
  }

  uint64_t end = precise_time_ns();

  return (end - start) / iterations;
}

int step_sum(int start, int end, int step) {
  int s = start / step, e = (end - 1) / step;

  return step * (e * (e + 1) / 2 - s * (s + 1) / 2);
}

__attribute__((always_inline))
inline int f() {
  int s = 0, e = 1000;

  return step_sum(s, e, 3) + step_sum(s, e, 5) - step_sum(s, e, 15);
}

int main(int argc, char *argv[]) {
  if (argc == 1) {
    printf("%d\n", f());
  } else if (argc == 2) {
    int iterations = atoi(argv[1]);
    printf("%d\n", benchmark(iterations, f));
  } else {
    return EXIT_FAILURE;
  }

  return EXIT_SUCCESS;
}
