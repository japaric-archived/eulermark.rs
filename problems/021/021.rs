extern crate test;
extern crate time;

use std::iter::AdditiveIterator;
use std::num::pow;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

fn is_amicable(n: uint, sod: &[uint]) -> bool {
    let m = sod[n];

    m != n && sod.get(m) == Some(&n)
}

// Factors(vec!((a, x), (b, y))) <-> a^x * b^y
struct Factors(Vec<(uint, uint)>);

fn factorize(mut n: uint, primes: &mut Vec<uint>) -> Factors {
    let mut factors = vec!();

    for &prime in primes.iter() {
        if n == 1 {
            break;
        }

        if prime * prime > n {
            factors.push((n, 1));
            break;
        }

        let mut i = 0;
        while n % prime == 0 {
            i += 1;
            n /= prime;
        }

        if i != 0 {
            factors.push((prime, i));
        }
    }

    if n != 1 && n > *primes.last().unwrap() {
        primes.push(n);
    }

    Factors(factors)
}

impl Factors {
    fn divisors(&self) -> Vec<uint> {
        let &Factors(ref s) = self;

        s.iter().fold(vec!(1), |xs, &(b, n)| {
            let ys: Vec<uint> = range(0, n + 1).map(|x| pow(b, x)).collect();

            combine(xs.as_slice(), ys.as_slice())
        })
    }
}

fn combine(xs: &[uint], ys: &[uint]) -> Vec<uint> {
    let mut z = Vec::with_capacity(xs.len() * ys.len());

    for &x in xs.iter() {
        for &y in ys.iter() {
            z.push(x * y)
        }
    }

    z
}

#[inline]
fn f() -> uint {
    let n = 10_000u;
    let mut primes = vec!(2);
    let mut sod = Vec::with_capacity(n + 1);
    sod.push(0);
    sod.push(0);

    for i in range(2, n + 1) {
        let factors = factorize(i, &mut primes);
        sod.push(factors.divisors().iter().fold(0, |a, &b| a + b) - i);
    }

    range(2, n + 1).filter(|&x| is_amicable(x, sod.as_slice())).sum()
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
