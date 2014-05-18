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

__attribute__((always_inline))
inline int f() {
  int ans = 0, curr = 1, next = 2, tmp;

  while (curr < 4000000) {
    if (curr % 2 == 0)
      ans += curr;

    tmp = next;
    next += curr;
    curr = tmp;
  }

  return ans;
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
