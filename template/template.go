package main

import "fmt"
import "os"
import "strconv"
import "time"

func f() int {
    // PROBLEM SOLUTION GOES HERE
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
        elapsed := time.Now().Sub(start)

        fmt.Println(elapsed.Nanoseconds() / iters)
    }

}
