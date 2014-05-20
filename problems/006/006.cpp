#include<cstdint>
#include<cstdlib>
#include<ctime>
#include<iostream>

using namespace std;

__attribute__((always_inline))
inline uint64_t f() {
  uint64_t sum = 0, sumOfSquares = 0;

  for (uint64_t i = 1; i < 101; i++) {
    sum += i;
    sumOfSquares += i * i;
  }

  uint64_t squaredSum = sum * sum;

  return squaredSum - sumOfSquares;
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
