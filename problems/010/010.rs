extern crate test;
extern crate time;

use std::iter::range_step;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

static LIMIT: u64 = 2000000;
static SIZE: u64 = (LIMIT - 1) / 2;

#[inline]
fn f() -> u64 {
    let mut sieve = Vec::from_elem(SIZE as uint, false);
    let mut ans = 2u64;

    for i in range(0, SIZE) {
        if ! unsafe { *sieve.as_slice().unsafe_ref(i as uint) } {
            let p = 2 * i + 3;

            ans += p;

            for j in range_step(p * p, LIMIT, 2 * p) {
                let j = (j as uint - 3) / 2;

                unsafe {
                    *sieve.as_mut_slice().unsafe_mut_ref(j) = true
                }
            }
        }
    }

    return ans;
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
