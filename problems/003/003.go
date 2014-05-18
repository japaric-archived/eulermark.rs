package main

import "fmt"
import "os"
import "strconv"
import "time"

func f() uint64 {
    n, factor := uint64(600851475143), uint64(2)

    for {
        if n % factor == 0 {
            n /= factor
        } else {
            factor += 1
        }

        if factor * factor > n {
            return n
        } else if n == 1 {
            return factor
        }
    }
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
