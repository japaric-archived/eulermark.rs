use executable::Executable;
use file::read;
use hash::Hashes;
use solution::Solution;
use std::cmp::max;
use std::hash;
use test::stats;
use time::precise_time_ns;

static INITIAL_BENCHMARK_PERIOD: u64 = 50_000_000;
static MAX_BENCH_TIME: u64 = 10_000_000_000;
static MIN_BENCH_TIME: u64 = 2_000_000_000;
static NSAMPLES: uint = 50;
static WINSORIZE_PCT: f64 = 5.0;

#[deriving(Decodable,Encodable)]
pub struct Metric<'a> {
    language: StrBuf,
    max: f64,
    median: f64,
    min: f64,
}

impl<'l> Metric<'l> {
    pub fn language<'a>(&'a self) -> &'a str {
        self.language.as_slice()
    }

    pub fn median(&self) -> f64 {
        self.median
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

pub fn benchmark<'l, 'p>(solution: &Solution<'l, 'p>, hashes: &mut Hashes)
    -> Option<Metric<'l>>
{
    let language = solution.language();
    let problem = solution.problem();
    let file = solution.file();
    let new_hash = StrBuf::from_owned_str(hash::hash(&read(file)).to_str());
    let name = StrBuf::from_str(language.name());

    match hashes.find(&name) {
        None => {},
        Some(old_hash) => if new_hash.as_slice() == old_hash.as_slice() {
            println!("File unchanged, skipping");

            return None;
        }
    }

    let compiler_output = match language.compiler() {
        None => None,
        Some(compiler) => match compiler.compile(file) {
            None => return None,
            output => output,
        }
    };

    let executable = match compiler_output {
        None => Executable::new(file, language.interpreter()),
        Some(ref output) => {
            Executable::new(output.binary(), language.interpreter())
        },
    };

    let answer = read(problem.answer());
    match executable.test() {
        None => return None,
        Some(output) => if output != answer {
            println!("Incorrect answer");

            return None;
        }
    }

    // this block implements the algorithm used for the Rust bencher framework
    let mut n = max(INITIAL_BENCHMARK_PERIOD / max(executable.bench(1), 1), 1);
    let mut samples = [0.0, ..NSAMPLES];
    let mut summ5;
    let mut total_run = 0;
    loop {
        let loop_start = precise_time_ns();

        for sample in samples.mut_iter() {
            *sample = executable.bench(n) as f64 / n as f64;
        };

        stats::winsorize(samples, WINSORIZE_PCT);
        let summ = stats::Summary::new(samples);

        for sample in samples.mut_iter() {
            *sample = executable.bench(5 * n) as f64 / (5 * n) as f64;
        };

        stats::winsorize(samples, WINSORIZE_PCT);
        summ5 = stats::Summary::new(samples);

        let now = precise_time_ns();
        let loop_run = now - loop_start;

        if loop_run > MIN_BENCH_TIME &&
            summ.median_abs_dev_pct < 1.0 &&
            summ.median - summ5.median < summ5.median_abs_dev {
            break;
        }

        total_run += loop_run;

        if total_run > MAX_BENCH_TIME {
            break;
        }

        n *= 2;
    }
    // end of block

    let max = summ5.max;
    let median = summ5.median;
    let min = summ5.min;
    let noise = max - min;

    if median < 1.0 {
        println!("{:>9} ps/iter (+/- {})",
                 (1000.0 * median) as u64,
                 (1000.0 * noise) as u64);
    } else {
        println!("{:>9} ns/iter (+/- {})", median as u64, noise as u64);
    }

    hashes.insert(name, new_hash);

    Some(Metric {
        language: StrBuf::from_str(language.name()),
        max: max,
        median: median,
        min: min,
    })
}
