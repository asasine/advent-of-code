//! Day 2: Gift Shop
//!
//! https://adventofcode.com/2025/day/2

use itertools::Itertools;
use tracing::{instrument, trace};

fn range(input: &str) -> Option<[&str; 2]> {
    input.trim().split('-').collect_array()
}

fn ranges(input: &str) -> impl Iterator<Item = [&str; 2]> + '_ {
    input.trim().split(',').map(range).flatten()
}

/// Sums the invalid IDs in the given range.
fn sum_invalid([start, end]: [&str; 2], is_invalid: impl Fn(usize) -> bool) -> usize {
    trace!("Checking range: {}-{}", start, end);
    let start = start.parse::<usize>().unwrap();
    let end = end.parse::<usize>().unwrap();

    (start..=end)
        .filter(|&id| is_invalid(id))
        .inspect(|&id| trace!("Invalid ID found: {}", id))
        .sum()
}

/// Split a number into an iterator of its digits based on a grouping size if the number of digits is evenly divisible
/// by the grouping size.
fn digitize(id: usize, n: u32) -> Option<impl Iterator<Item = usize>> {
    let digits = solutions::num::usize::digits(id);
    if digits.is_multiple_of(n) {
        let group_size = 10usize.pow(n);
        Some((0..(digits / n)).map(move |i| (id / 10usize.pow(i * n) % group_size) as usize))
    } else {
        None
    }
}

/// Split a number into its digits based on all evenly divisible groupings.
///
/// The outer iterator yields groupings of all divisible sizes of the number, with the inner iterator yielding the
/// digits in that grouping.
fn split_to_groups(id: usize) -> impl Iterator<Item = impl Iterator<Item = usize>> {
    let digits = solutions::num::usize::digits(id);

    // Skip group size equal to the number of digits, since that would just be the number itself.
    (1..digits).map(move |n| digitize(id, n)).flatten()
}

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    /// An ID is invalid if it is some sequence of digits repeated twice (e.g., "1212", "7777", "123123")
    fn is_invalid(id: usize) -> bool {
        // a number has an even number of digits if log10(n) is an odd number
        let digits = solutions::num::usize::digits(id);
        // split the number into two halves, the lower and upper digits
        let half = 10usize.pow(digits / 2);
        let lower = id % half;
        let upper = id / half;
        lower == upper
    }

    ranges(input).map(|r| sum_invalid(r, is_invalid)).sum()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    /// An ID is invalid if it is some sequence of digits repeated at least twice.
    ///
    /// - `12341234` (`1234` repeated twice)
    /// - `123123123` (`123` repeated three times)
    /// - `1212121212` (`12` repeated five times)
    /// - `1111111` (`1` repeated seven times)
    fn is_invalid(id: usize) -> bool {
        let mut splits = split_to_groups(id);
        splits.any(|mut group| group.all_equal())
    }

    ranges(input).map(|r| sum_invalid(r, is_invalid)).sum()
}

fn main() {
    solutions::main(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2025/02/1.txt");
        assert_eq!(1227775554, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/02/1.txt");
        assert_eq!(4174379265, part2(input));
    }
}
