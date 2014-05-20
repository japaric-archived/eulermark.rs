#include<cstdbool>
#include<cstdint>
#include<cstdlib>
#include<ctime>
#include<iostream>

using namespace std;

bool isPalindrome(uint64_t n) {
  uint64_t reversed = 0, tmp = n;

  while (tmp) {
    reversed = 10 * reversed + (tmp % 10);
    tmp /= 10;
  }

  return reversed == n;
}

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t max = 0;

  for (uint64_t i = 100; i < 1000; i++) {
    for (uint64_t j = 100; j < i; j++) {
      uint64_t p = i * j;

      if (p > max && isPalindrome(p))
        max = p;
    }
  }

  return max;
}

uint64_t toNs(timespec &ts) {
  return (uint64_t)ts.tv_sec * 1000000000lu + (uint64_t)ts.tv_nsec;
}

int main(int argc, char *argv[]) {
  if (argc == 1) {
    cout << f() << endl;
  } else if (argc == 2) {
    timespec start, end;
    uint64_t iters = atol(argv[1]);

    clock_gettime(CLOCK_MONOTONIC, &start);
    for (uint64_t i = 0; i < iters; i++) {
      asm("");
      f();
    }
    clock_gettime(CLOCK_MONOTONIC, &end);

    cout << toNs(end) - toNs(start) << endl;
  }
}
