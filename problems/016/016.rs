extern crate test;
extern crate time;
extern crate num;

use num::bigint::ToBigUint;
use std::char::to_digit;
use std::iter::AdditiveIterator;
use std::num::pow;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> uint {
    pow(2u.to_biguint().unwrap(), 1000).to_str().as_slice()
                                       .chars()
                                       .filter_map(|c| to_digit(c, 10))
                                       .sum()
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
