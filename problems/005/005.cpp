#include<cstdint>
#include<cstdlib>
#include<ctime>
#include<iostream>

using namespace std;

uint64_t gcd(uint64_t a, uint64_t b) {
  if (a < b) {
    uint64_t t = a;
    a = b;
    b = t;
  }

  uint64_t r;
  while (r = a % b) {
    a = b;
    b = r;
  }

  return b;
}

uint64_t lcm(uint64_t a, uint64_t b) {
  return a * b / gcd(a, b);
}

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t n = 2;

  uint64_t i;
  for (i = 3; i < 21; i++)
    n = lcm(n, i);

  return n;
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
