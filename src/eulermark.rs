extern crate serialize;
extern crate test;
extern crate time;

use benchmark::{Metric,benchmark};
use json::save_metrics;
use language::supported_languages;
use problem::Problem;
use solution::Solution;

mod benchmark;
mod compiler;
mod executable;
mod file;
mod interpreter;
mod json;
mod language;
mod problem;
mod solution;

fn main() {
    let languages = supported_languages();

    for problem in range(1u, 1000).filter_map(|i| Problem::new_opt(i)) {
        println!("{}", problem.id());
        let mut metrics: Vec<Metric> = languages.iter().filter_map(|language| {
            Solution::new_opt(language, &problem).and_then(|solution| {
                print!("> {:<12}", solution.language().name());

                benchmark(&solution)
            })
        }).collect();

        metrics.sort_by(|&a, &b| {
            let a = a.median() as u64;
            let b = b.median() as u64;

            a.cmp(&b)
        });

        save_metrics(problem.id(), &metrics);
    }
}
