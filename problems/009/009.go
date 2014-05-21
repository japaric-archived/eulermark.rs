package main

import "fmt"
import "os"
import "strconv"
import "time"

func f() uint64 {
    perimeter := uint64(1000)

    for c := perimeter / 3 + 1; c < perimeter / 2; c++ {
        for b := (perimeter - c) / 2 + 1; b < c; b++ {
            a := perimeter - b - c

            if a * a + b * b == c * c {
                return a * b * c
            }
        }
    }

    return 0
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
