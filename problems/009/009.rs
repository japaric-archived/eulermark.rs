extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

static PERIMETER: u64 = 1000;

#[inline]
fn f() -> u64 {
    for c in range(PERIMETER / 3 + 1, PERIMETER / 2) {
        for b in range((PERIMETER - c) / 2 + 1, c) {
            let a = PERIMETER - b - c;

            if a * a + b * b == c * c {
                return a * b * c
            }
        }
    }

    unreachable!();
}

fn main() {
    match args().as_slice() {
        [_] => {
            println!("{}", f());
        },
        [_, ref iters] => {
            let iters: u64 = from_str(iters.as_slice()).unwrap();

            let start = precise_time_ns();
            for _ in range(0, iters) {
                black_box(f());
            }
            let end = precise_time_ns();

            println!("{}", end - start);
        },
        _ => unreachable!(),
    }
}
