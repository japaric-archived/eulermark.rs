extern crate test;
extern crate time;

use std::iter::count;
use std::mem::replace;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[inline]
fn f() -> u64 {
    let mut primes = vec!(2);
    let mut next = Factors(vec!((2, 1)));

    for n in count(3u64, 1) {
        let curr = replace(&mut next, factorize(n, &mut primes));
        let triangle = next * curr * Factors(vec!((2, -1)));

        if triangle.number_of_divisors() > 500 {
            return n * (n - 1) / 2;
        }
    }

    unreachable!();
}

// Factors(vec!((a, x), (b, y))) <-> a^x * b^y
struct Factors(Vec<(u64, int)>);

fn factorize(mut n: u64, primes: &mut Vec<u64>) -> Factors {
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
    fn number_of_divisors(&self) -> int {
        let &Factors(ref s) = self;

        s.iter().fold(1, |n, &(_, e)| n * (e + 1))
    }
}

impl Mul<Factors, Factors> for Factors {
    fn mul(&self, rhs: &Factors) -> Factors {
        let (&Factors(ref s), &Factors(ref r)) = (self, rhs);

        let (m, n) = (s.len(), r.len());
        let (mut i, mut j) = (0 ,0);

        let mut o = vec!();
        while i != m || j != n {
            if i == m {
                o.push(*r.get(j));
                j += 1;
            } else if j == n {
                o.push(*s.get(i));
                i += 1;
            } else {
                let (&(a, x), &(b, y)) = (s.get(i), r.get(j));

                if a > b {
                    o.push((b, y));
                    j += 1;
                } else if a < b {
                    o.push((a, x));
                    i += 1;
                } else {
                    if x + y > 0 {
                        o.push((a, x + y));
                    }

                    i += 1;
                    j += 1;
                }
            }
        }

        Factors(o)
    }
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
