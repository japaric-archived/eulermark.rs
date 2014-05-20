package main

import "fmt"
import "os"
import "strconv"
import "time"

func f() uint64 {
    ans := uint64(0)

    for i := uint64(0); i < 1000; i++ {
        if i % 3 == 0 || i % 5 == 0 {
            ans += i
        }
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
