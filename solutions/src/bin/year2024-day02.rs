//! Day 2: Red-Nosed Reports
//!
//! https://adventofcode.com/2024/day/2

use itertools::Itertools;
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|levels| is_safe(levels.iter()))
        .count()
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_safe<'a>(levels: impl Iterator<Item = &'a usize>) -> bool {
    // check if all levels are increasing or decreasing
    let mut direction_cmp = None;
    levels
        .tuple_windows()
        .map(|(l, r)| {
            let direction = if l < r {
                Direction::Increasing
            } else {
                Direction::Decreasing
            };

            let delta = r.abs_diff(*l);
            (direction, delta)
        })
        .all(|(direction, delta)| {
            *direction_cmp.get_or_insert(direction) == direction && (1..=3).contains(&delta)
        })
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|levels| is_safe_problem_dampener_naive(levels))
        .count()
}

fn is_safe_problem_dampener_naive(levels: &[usize]) -> bool {
    // naive solution: remove each element in turn and check if the remainder is safe
    if is_safe(levels.iter()) {
        return true;
    }

    for i in 0..levels.len() {
        let remainder =
            levels
                .iter()
                .enumerate()
                .filter_map(|(j, level)| if i == j { None } else { Some(level) });

        if is_safe(remainder) {
            return true;
        }
    }

    false
}

fn main() {
    solutions::main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/02/1.txt");
        assert_eq!(2, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/02/1.txt");
        assert_eq!(4, part2(input));
    }
}
