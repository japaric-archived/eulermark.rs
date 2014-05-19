#![feature(default_type_params)]

extern crate collections;
extern crate serialize;
extern crate test;
extern crate time;

use benchmark::{Metric,benchmark};
use json::{load_hashes,save_hashes,update_metrics};
use language::supported_languages;
use problem::Problem;
use solution::Solution;
use std::io::stdio;

mod benchmark;
mod compiler;
mod executable;
mod file;
mod hash;
mod interpreter;
mod json;
mod language;
mod problem;
mod solution;

fn main() {
    let languages = supported_languages();

    for problem in range(1u, 1000).filter_map(|i| Problem::new_opt(i)) {
        let pid = problem.id();
        let mut hashes = load_hashes(pid);

        println!("{}", pid);
        let metrics: Vec<Metric> = languages.iter().filter_map(|language| {
            Solution::new_opt(language, &problem).and_then(|solution| {
                print!("> {:<13}", solution.language().name());
                stdio::flush();

                benchmark(&solution, &mut hashes)
            })
        }).collect();

        save_hashes(pid, &hashes);
        update_metrics(pid, metrics);
    }
}
