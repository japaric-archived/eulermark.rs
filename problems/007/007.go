package main

import "fmt"
import "os"
import "strconv"
import "time"

func f() uint64 {
    i, map_, n, q := 0, make(map[uint64]uint64), 10000, uint64(1)

    for {
        q += 1

        p := map_[q]

        if p == 0 {
            map_[q * q] = q

            if i == n {
                return q
            } else {
                i += 1
            }
        } else {
            map_[q] = 0
            x := p + q

            for map_[x] != 0 {
                x += p
            }

            map_[x] = p
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
