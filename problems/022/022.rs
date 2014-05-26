extern crate test;
extern crate time;

use std::io::fs::File;
use std::iter::AdditiveIterator;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

fn name_value(name: &str) -> u64 {
    let offset = 'A' as u64 - 1;

    name.chars().map(|c| c as u64 - offset).sum()
}

#[inline]
fn f() -> u64 {
    let content = match File::open(&Path::new("022.txt")) {
        Err(_) => fail!("couldn't open input file"),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => fail!("couldn't read input file"),
            Ok(string) => string,
        }
    };

    let mut names: Vec<&str> = content.as_slice().split(',')
                                      .map(|s| s.trim_chars('"'))
                                      .collect();

    names.sort();

    names.iter()
         .enumerate()
         .map(|(n, &s)| (n + 1) as u64 * name_value(s))
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
