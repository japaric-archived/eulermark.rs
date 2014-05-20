#include<cstdint>
#include<cstdlib>
#include<ctime>
#include<iostream>
#include<unordered_map>

using namespace std;

__attribute__((always_inline))
inline uint64_t f() {
  int i = 0, n = 10000;
  uint64_t q = 1;
  unordered_map<uint64_t, uint64_t> map;

  while (true) {
    q += 1;

    uint64_t p = map[q];

    if (p == 0) {
      map[q * q] = q;

      if (i == n) {
        return q;
      } else {
        i += 1;
      }
    } else {
      uint64_t x = p + q;

      map[q] = 0;

      while (map[x] != 0) {
        x += p;
      }

      map[x] = p;
    }
  }
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
