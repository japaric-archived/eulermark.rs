package main

import "fmt"
import "os"
import "strconv"
import "time"

func f() uint64 {
    sum, sumOfSquares := uint64(0), uint64(0)

    for i := uint64(1); i < 101; i++ {
        sum += i
        sumOfSquares += i * i
    }

    squaredSum := sum * sum

    return squaredSum - sumOfSquares
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
