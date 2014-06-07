#![feature(default_type_params)]

extern crate test;
extern crate time;

use std::collections::HashMap;
use std::hash::sip::SipHasher;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

struct Eratostenes {
    count: u64,
    map: HashMap<u64, u64, SipHasher>,
}

impl Iterator<u64> for Eratostenes {
    fn next(&mut self) -> Option<u64> {
        loop {
            self.count += 1;
            let q = self.count;

            match self.map.pop(&q) {
                None => {
                    self.map.insert(q * q, q);

                    return Some(q);
                },
                Some(p) => {
                    let mut x = p + q;

                    while self.map.contains_key(&x) {
                        x += p;
                    }

                    self.map.insert(x, p);
                }
            }
        }
    }
}

fn eratostenes(capacity: uint) -> Eratostenes {
    Eratostenes {
        count: 1,
        map: HashMap::with_capacity(capacity),
    }
}

#[inline]
fn f() -> u64 {
    let target = 10000;
    eratostenes(target).nth(target).unwrap()
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
