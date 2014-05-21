#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define WINDOW 13

uint64_t product(uint8_t* digits) {
  uint64_t p = 1;
  int i;

  for (i = 0; i < WINDOW; i++)
    p *= digits[i];

  return p;
}

__attribute__((always_inline))
inline uint64_t f() {
  FILE *file = fopen("008.txt", "r");
  int pos = 0;
  uint64_t max = 0;
  uint8_t digits[WINDOW] = {0};

  int chr = fgetc(file);
  while (chr != EOF) {
    if (chr != '\n') {
      digits[pos] = chr - 48;
      uint64_t p = product(digits);

      if (p > max)
        max = p;

      pos = (pos + 1) % WINDOW;
    }

    chr = fgetc(file);
  }

  fclose(file);

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
