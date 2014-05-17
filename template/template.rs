extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> int {
    // PROBLEM SOLUTION GOES HERE
    0
}

fn main() {
    match args().as_slice() {
        [_] => {
            println!("{}", f());
        },
        [_, ref iterations] => {
            let iterations: u64 = from_str(iterations.as_slice()).unwrap();

            let start = precise_time_ns();
            for _ in range(0, iterations) {
                black_box(f());
            }
            let end = precise_time_ns();

            println!("{}", (end - start) / iterations);
        },
        _ => unreachable!(),
    }
}
