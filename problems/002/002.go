package main

import "fmt"
import "os"
import "strconv"
import "time"

func f() uint64 {
    ans, curr, next := uint64(0), uint64(1), uint64(2)

    for curr < 4000000 {
        if curr % 2 == 0 {
            ans += curr
        }

        tmp := next
        next += curr
        curr = tmp
    }

    return ans
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
