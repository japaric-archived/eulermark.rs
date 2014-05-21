extern crate test;
extern crate time;

use std::iter::AdditiveIterator;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

fn word_length(n: uint) -> uint {
    match n {
        0 => 0,
        1 => "one".len(),
        2 => "two".len(),
        3 => "three".len(),
        4 => "four".len(),
        5 => "five".len(),
        6 => "six".len(),
        7 => "seven".len(),
        8 => "eight".len(),
        9 => "nine".len(),
        10 => "ten".len(),
        11 => "eleven".len(),
        12 => "twelve".len(),
        13 => "thirteen".len(),
        15 => "fifteen".len(),
        18 => "eighteen".len(),
        n if n < 20 => word_length(n % 10) + "teen".len(),
        20 => "twenty".len(),
        30 => "thirty".len(),
        40 => "forty".len(),
        50 => "fifty".len(),
        80 => "eighty".len(),
        n if n < 100 => {
            let u = n % 10;

            if u == 0 {
                word_length(n / 10) + "ty".len()
            } else {
                let t = n - u;
                word_length(t) + word_length(u)
            }
        },
        n if n < 1000 => {
            word_length(n / 100) + "hundred".len() + if n % 100 == 0 {
                0
            } else {
                "and".len() + word_length(n % 100)
            }
        },
        n if n % 1000 == 0 => word_length(n / 1000) + "thousand".len(),
        n => fail!("{}", n),
    }
}

#[inline]
fn f() -> uint {
    range(1u, 1001).map(|n| word_length(n)).sum()
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
