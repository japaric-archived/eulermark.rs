extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

fn is_palindrome(n: u64) -> bool {
    let (mut reversed, mut tmp) = (0, n);

    while tmp != 0 {
        reversed = 10 * reversed + tmp % 10;
        tmp /= 10;
    }

    reversed == n
}

#[inline]
fn f() -> u64 {
    let mut max = 0;

    for a in range(100u64, 1000) {
        for b in range(100, a) {
            let p = a * b;

            if p > max && is_palindrome(p) {
                max = p;
            }
        }
    }

    max
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
