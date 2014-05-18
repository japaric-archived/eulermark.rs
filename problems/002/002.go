package main

import "fmt"
import "os"
import "strconv"
import "time"

func f() int {
    ans, curr, next := 0, 1, 2

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
        elapsed := time.Now().Sub(start)

        fmt.Println(elapsed.Nanoseconds() / iters)
    }

}
