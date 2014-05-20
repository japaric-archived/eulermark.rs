#include<cstdint>
#include<cstdlib>
#include<ctime>
#include<iostream>

using namespace std;

uint64_t stepSum(uint64_t end, uint64_t step) {
  uint64_t e = (end - 1) / step;

  return step * e * (e + 1) / 2;
}

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t end = 1000;

  return stepSum(end, 3) + stepSum(end, 5) - stepSum(end, 15);
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
