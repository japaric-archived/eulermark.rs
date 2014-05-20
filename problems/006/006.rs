extern crate test;
extern crate time;

use std::iter::AdditiveIterator;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> u64 {
    let sumOfSquares = range(1u64, 101).map(|x| x * x).sum();
    let sum = range(1u64, 101).sum();
    let squaredSum = sum * sum;

    return squaredSum - sumOfSquares;
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
