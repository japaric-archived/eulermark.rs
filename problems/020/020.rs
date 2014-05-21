extern crate num;
extern crate test;
extern crate time;

use num::bigint::{BigUint,ToBigUint};
use std::iter::{AdditiveIterator,MultiplicativeIterator,range_inclusive};
use std::num::One;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

fn factorial(n: uint) -> BigUint {
    range_inclusive(One::one(), n.to_biguint().unwrap()).product()
}

#[inline]
fn f() -> uint {
    factorial(100).to_str().chars().filter_map(|c| c.to_digit(10)).sum()
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
