extern crate test;
extern crate time;

use std::iter::AdditiveIterator;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> u64 {
    range(0u64, 1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
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
