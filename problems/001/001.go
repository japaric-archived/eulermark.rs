package main

import "fmt"
import "os"
import "strconv"
import "time"

func step_sum(end uint, step uint) uint {
    e := (end - 1) / step

    return step * e * (e + 1) / 2
}

func f() uint {
    end := uint(1000)

    return step_sum(end, 3) + step_sum(end, 5) - step_sum(end, 15)
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
        elapsed := time.Now().Sub(start)

        fmt.Println(elapsed.Nanoseconds() / iters)
    }

}
