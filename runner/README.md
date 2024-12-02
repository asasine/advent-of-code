# Advent of Code Runner

This crate contains a runner for my [Advent of Code solutions](../solutions/).

The runner assumes each solution is its own binary with the name `year{year}-day{day:02}` (e.g., `year2024-day01`).
Each binary should be runnable with `cargo run --bin ...`.
The runner can either run the current day's solution with `cargo run` or a specific date with `cargo run -- run 2024 01`.
