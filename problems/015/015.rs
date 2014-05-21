extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

static SIZE: uint = 21;

#[inline]
fn f() -> u64 {
    let mut grid = Vec::from_elem(SIZE * SIZE, 1u64);
    let slice = grid.as_mut_slice();

    for i in range(1, SIZE) {
        for j in range(1, SIZE) {
            unsafe {
                *slice.unsafe_mut_ref(i * SIZE + j) =
                    *slice.unsafe_ref((i - 1) * SIZE + j) +
                    *slice.unsafe_ref(i * SIZE + j - 1);
            }
        }
    }

    *slice.last().unwrap()
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
