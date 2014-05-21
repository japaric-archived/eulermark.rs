extern crate test;
extern crate time;

use std::io::fs::File;
use std::os::args;
use test::black_box;
use time::precise_time_ns;

static DIRECTIONS: [(int, int), ..4] = [(0, 1), (1, 0), (1, 1), (-1, 1)];
static SIZE: int = 20;
static WINDOW: int = 4;

#[inline]
fn f() -> u64 {
    let contents = match File::open(&Path::new("011.txt")) {
        Err(_) => fail!("couldn't find input file"),
        Ok(mut file) => match file.read_to_str() {
            Err(_) => fail!("couldn't read input file"),
            Ok(string) => string,
        }
    };

    let mut grid: Vec<u64> = Vec::with_capacity((SIZE * SIZE) as uint);

    for word in contents.words() {
        match from_str(word) {
            None => {},
            Some(number) => grid.push(number),
        }
    }

    let mut max = 0;
    for &(row_step, col_step) in DIRECTIONS.iter() {
        for row in range(0, SIZE) {
            let max_row = row + WINDOW * row_step;
            if max_row > SIZE || max_row < -1 { continue }

            for col in range(0, SIZE) {
                let max_col = col + WINDOW * col_step;
                if max_col > SIZE || max_col < -1 { continue }

                let mut tmp = 1;
                let mut idx = row * SIZE + col;
                for _ in range(0, WINDOW) {
                    tmp *= unsafe {
                        *grid.as_slice().unsafe_ref(idx as uint)
                    };

                    idx += row_step * SIZE + col_step;
                }

                if tmp > max {
                    max = tmp;
                }
            }
        }
    }

    max
}

fn main()  {
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
