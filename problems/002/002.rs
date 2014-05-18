extern crate num;
extern crate test;
extern crate time;

use num::Integer;
use std::iter::AdditiveIterator;
use std::mem::replace;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

struct Fibonacci {
    curr: int,
    next: int,
}

impl Iterator<int> for Fibonacci {
    fn next(&mut self) -> Option<int> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 2 }
}

#[inline]
fn f() -> int {
    fibonacci().take_while(|&x| x < 4_000_000).filter(|x| x.is_even()).sum()
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
