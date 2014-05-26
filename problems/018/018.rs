extern crate test;
extern crate time;

use std::cmp::max;
use std::io::fs::File;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> uint {
    let content = match File::open(&Path::new("018.txt")) {
        Err(_) => fail!("couldn't open input file"),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => fail!("couldn't read input file"),
            Ok(string) => string,
        }
    };

    let costs =
        content.as_slice().lines()
               .map(|l| l.words().filter_map(from_str).collect::<Vec<uint>>())
               .fold(vec!(0), |a, b| {
                   let a = a.as_slice();
                   let b = b .as_slice();
                   let n = b.len();
                   let mut c = Vec::with_capacity(n);

                   for i in range(0, n) {
                       if i == 0 {
                           c.push(unsafe {
                               b.unsafe_ref(i) + *a.unsafe_ref(i)
                           });
                       } else if i == n - 1 {
                           c.push(unsafe {
                               b.unsafe_ref(i) + *a.unsafe_ref(i - 1)
                           });
                       } else {
                           c.push(unsafe {
                               b.unsafe_ref(i) + *max(a.unsafe_ref(i - 1),
                                                      a.unsafe_ref(i))
                           });
                       }
                   }

                   c
               });

    *costs.iter().max().unwrap()
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
