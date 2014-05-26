extern crate num;
extern crate test;
extern crate time;

use num::bigint::BigInt;
use std::io::fs::File;
use std::iter::AdditiveIterator;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> String {
    let content = match File::open(&Path::new("013.txt")) {
        Err(_) => fail!("couldn't open input file"),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => fail!("couldn't read input file"),
            Ok(string) => string,
        }
    };

    let sum = content.as_slice().lines()
                     .filter_map(|line| from_str::<BigInt>(line.trim()))
                     .sum()
                     .to_str();

    sum.as_slice().slice(0, 10).to_owned()
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
