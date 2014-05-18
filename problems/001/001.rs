extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

fn step_sum(start: int, end: int, step: int) -> int {
    let (s, e) = ((start - 1) / step + 1, (end - 1) / step);

    step * (e * (e + 1) / 2 - s * (s + 1) / 2)
}

#[inline]
fn f() -> int {
    let (s, e) = (0, 1000);

    step_sum(s, e, 3) + step_sum(s, e, 5) - step_sum(s, e, 15)
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
