package main

import "fmt"
import "os"
import "strconv"
import "time"

func isPalindrome(n uint64) bool {
    reversed, tmp := uint64(0), n

    for tmp != 0 {
        reversed = 10 * reversed + (tmp % 10)
        tmp /= 10
    }

    return reversed == n
}

func f() uint64 {
    max := uint64(0)

    for i := uint64(0); i < 1000; i++ {
        for j := uint64(0); j < i; j++ {
            p := i * j

            if p > max && isPalindrome(p) {
                max = p
            }
        }
    }

    return max
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
