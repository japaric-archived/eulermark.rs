extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> u64 {
    let (mut n, mut factor) = (600851475143, 2);

    loop {
        if n % factor == 0 {
            n /= factor;
        } else {
            factor += 1;
        }

        if factor * factor > n {
            return n;
        } else if n == 1 {
            return factor;
        }
    }
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
