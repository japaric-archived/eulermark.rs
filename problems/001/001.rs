extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

fn step_sum(end: u64, step: u64) -> u64 {
    let e = (end - 1) / step;

    step * e * (e + 1) / 2
}

#[inline]
fn f() -> u64 {
    let end = 1000;

    step_sum(end, 3) + step_sum(end, 5) - step_sum(end, 15)
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
