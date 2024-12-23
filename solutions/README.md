# Advent of Code Solutions

This crate contains my solutions to the [Advent of Code](https://adventofcode.com/) challenges.

Each day is a separate binary, with the solutions for each part of the challenge printed to stdout, one part per line.
Status messages can be printed to stderr.
The binaries are named `year{year}-day{day:02}` (e.g., `year2024-day01`) and are placed in the [./src/bin/](src/bin/) directory.

The solutions can be run directly with `cargo run --bin year2024-day01`, though running through the
[runner](../runner/) is recommended.

Any common code is contained in this crate's [library](src/lib.rs).
