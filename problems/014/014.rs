extern crate num;
extern crate test;
extern crate time;

use num::Integer;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> u64 {
    let upper = 1_000_000u64;
    let mut v = Vec::from_elem(upper as uint + 1, 0u64);
    let memo = v.as_mut_slice();
    memo[1] = 1;

    range(2, upper + 1).max_by(|&n| collatz_length(n, memo)).unwrap()
}

fn collatz_length(n: u64, memo: &mut [u64]) -> u64 {
    match memo.get(n as uint) {
        Some(&length) if length != 0 => length,
        _ => {
            let length = 1 + collatz_length(if n.is_even() {
                n / 2
            } else {
                3 * n + 1
            }, memo);

            if n < memo.len() as u64 {
                unsafe {
                    *memo.unsafe_mut_ref(n as uint) = length;
                }
            }

            length
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
