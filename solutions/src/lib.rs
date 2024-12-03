//! Solutions to the Advent of Code puzzles.

/// Reads the entire stdin into a [`String`].
pub fn read_stdin() -> String {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Should have been able to read stdin to end");

    input
}
