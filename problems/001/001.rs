extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

fn step_sum(end: uint, step: uint) -> uint {
    let e = (end - 1) / step;

    step * e * (e + 1) / 2
}

#[inline]
fn f() -> uint {
    let end = 1000;

    step_sum(end, 3) + step_sum(end, 5) - step_sum(end, 15)
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
