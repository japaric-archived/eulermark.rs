**Note** This project has been deprecated in favor of
[`euler_criterion.rs`](https://github.com/japaric/euler_criterion.rs), which
uses [`criterion.rs`](https://github.com/japaric/criterion.rs) to perform a
more thoughtful analysis of the benchmark results. Please don't open issues or
send PRs here, because this repository won't receive further updates.

---

![](https://projecteuler.net/profile/japaric.png)
[![Build Status](https://travis-ci.org/japaric/eulermark.rs.svg?branch=master)](https://travis-ci.org/japaric/eulermark.rs)

# Eulermark

## What's this?

A [Rust](http://www.rust-lang.org/) application for benchmarking solutions to
[Project Euler problems](https://projecteuler.net/problems) written in
different programming languages.

The benchmark results are being showcased in this
[page](http://japaric.github.io/eulermark.rs).

## How does it work?

Each solution is a program that exposes the following interface:

* If called directly, the program returns the answer to the problem.
* If called with an argument `n`, the program returns the time taken (in
  nanoseconds) to solve the problem `n` times.
* The program must use a monotonic timer with nanosecond resolution under the
  hood.

Eulermark will continuously call this program with different arguments to
benchmark the solution. Eulermark uses the same statistics driven algorithm
used by the Rust Bencher framework.

## License

Eulermark is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.
