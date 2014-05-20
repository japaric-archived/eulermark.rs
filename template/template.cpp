#include<cstdint>
#include<cstdlib>
#include<ctime>
#include<iostream>

using namespace std;

__attribute__((always_inline))
inline uint64_t f() {
  // PROBLEM SOLUTION GOES HERE
  return 0;
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
