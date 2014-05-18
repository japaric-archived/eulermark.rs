extern crate num;
extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> u64 {
    let (mut ans, mut curr, mut next) = (0, 1, 2);

    while curr < 4_000_000 {
        if curr % 2 == 0 {
            ans += curr;
        }

        let tmp = next;
        next += curr;
        curr = tmp;
    }

    ans
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
