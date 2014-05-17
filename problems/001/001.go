package main

import "fmt"
import "os"
import "strconv"
import "time"

func step_sum(start int, end int, step int) int {
    s := start / step
    e := (end - 1) / step

    return step * (e * (e + 1) / 2 - s * (s + 1) / 2)
}

func f() int {
    s, e := 0, 1000

    return step_sum(s, e, 3) + step_sum(s, e, 5) - step_sum(s, e, 15)
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
