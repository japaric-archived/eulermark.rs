# Eulermark

## What's this?

A [Rust](http://www.rust-lang.org/) application for benchmarking solutions to
[Project Euler problems](https://projecteuler.net/problems) written in
different programming languages.

## How does it work?

Each solution is a program that exposes the following interface:

* If called directly, the program returns the answer to the problem.
* If called with an argument `n`, the program returns the average time taken to
  solve the problem `n` times.

With this information, `eulermark` will compute the median and range time of
each solution, using the same algorithm that the Rust Bencher framework uses.

## License

Eulermark is dual licensed under the Apache 2.0 license and the MIT license.

See LICENSE-APACHE and LICENSE-MIT for more details.
