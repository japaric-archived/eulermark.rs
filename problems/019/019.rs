extern crate test;
extern crate time;

use std::os::args;
use test::black_box;
use time::precise_time_ns;

#[deriving(FromPrimitive,PartialEq,Eq)]
enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday,
}

impl Day {
    fn advance(&self, days: uint) -> Day {
        FromPrimitive::from_uint((*self as uint + days) % 7).unwrap()
    }
}

enum Month {
    January, February, March, April, May, June, July, August, September,
    October, November, December,
}

static MONTHS: [Month, ..12] = [January, February, March, April, May, June,
                                July, August, September, October, November,
                                December];

impl Month {
    fn days(&self, year: Year) -> uint {
        match *self {
            January | March | May | July | August | October | December => 31,
            April | June | September | November => 30,
            February => if year.is_leap() { 29 } else { 28 },
        }
    }
}

type Year = uint;

trait Leap {
    fn is_leap(&self) -> bool;
}

impl Leap for Year {
    fn is_leap(&self) -> bool {
        if *self % 400 == 0 {
            true
        } else if *self % 100 == 0 {
            false
        } else if *self % 4 == 0 {
            true
        } else {
            false
        }
    }
}

#[inline]
fn f() -> u64 {
    let mut count = 0;
    let mut today = Monday;

    for year in range(1900 as Year, 2001) {
        for month in MONTHS.iter() {
            today = today.advance(month.days(year));

            if year != 1900 && today == Sunday {
                count += 1;
            }
        }
    }

    count
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
