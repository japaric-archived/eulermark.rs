package main

import "fmt"
import "os"
import "strconv"
import "time"

func gcd(a uint64, b uint64) uint64 {
    if a < b {
        t := a;
        a = b;
        b = t;
    }

    for b != 0 {
        a, b = b, a % b
    }

    return a
}

func lcm(a uint64, b uint64) uint64 {
    return a * b / gcd(a, b)
}

func f() uint64 {
    n := uint64(2)

    for i := uint64(3); i < 21; i++ {
        n = lcm(n, i)
    }

    return n
}

func main() {
    if len(os.Args) == 1 {
        fmt.Println(f())
    } else if len(os.Args) == 2 {
        iters, _ := strconv.ParseInt(os.Args[1], 10, 64)

        start := time.Now()
        for i := int64(0); i < iters; i++ {
            f()
        }
        end := time.Now()

        fmt.Println(end.Sub(start).Nanoseconds())
    }
}
