extern crate test;
extern crate time;

use std::char::to_digit;
use std::io::File;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

static WINDOW: uint = 13;

#[inline]
fn f() -> u64 {
    let (mut factors, mut max, mut pos) = ([0, ..WINDOW], 0, 0);

    let contents = match File::open(&Path::new("008.txt")) {
        Err(_) => fail!("couldn't find input file"),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => fail!("couldn't read input file"),
            Ok(string) => string,
        }
    };

    for digit in contents.chars().filter_map(|chr| to_digit(chr, 10)) {
        factors[pos] = digit as u8;

        let prod = factors.iter().fold(1, |p, &f| p * f as u64);

        if prod > max {
            max = prod;
        }

        pos = (pos + 1) % WINDOW;
    }

    max
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
