# Advent of Code Runner

This crate contains a runner for my [Advent of Code solutions](../solutions/).

Each day is a separate binary, with the solutions for each part of the challenge printed to stdout, one part per line.
Status messages can be printed to stderr.
The binaries are named `year{year}-day{day:02}` (e.g., `year2024-day01`) and are placed in the [./src/bin/](src/bin/) directory.

The solutions can be run directly with `cargo run --bin year2024-day01`, through the [runner](./src/bin/runner.rs) for
a specific date with `cargo run -q -- 2024 01`, or through the runner tool for the current date with `cargo run -q`.

Any common code is contained in this crate's [library](src/lib.rs).
