//! Day 9: Movie Theater
//!
//! https://adventofcode.com/2025/day/9

use itertools::Itertools;
use solutions::grid::Coordinate;
use tracing::{instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let coordinates = iter_coordinates(input).collect::<Option<Vec<_>>>().unwrap();

    coordinates
        .iter()
        .tuple_combinations()
        .inspect(|(a, b)| trace!(?a, ?b))
        .map(|(a, b)| area_of(a, b))
        .inspect(|area| trace!(area))
        .max()
        .unwrap_or(0)
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    0
}

fn main() {
    solutions::main(part1, part2);
}

/// Parse the input into an iterator of coordinates.
fn iter_coordinates(input: &str) -> impl Iterator<Item = Option<Coordinate>> {
    input
        .lines()
        .map(|line| line.split_once(','))
        .map(Option::unzip)
        .map(|(x, y)| {
            Some(Coordinate {
                x: x?.parse().ok()?,
                y: y?.parse().ok()?,
            })
        })
}

/// Calculate the area of a rectangle with opposite corners at the given coordinates.
fn area_of(a: &Coordinate, b: &Coordinate) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

#[cfg(test)]
mod tests {
    use solutions::setup_tracing;

    use super::*;

    #[test]
    fn part1_example() {
        setup_tracing();
        let input = include_str!("../../data/examples/2025/09/1.txt");
        assert_eq!(50, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/09/1.txt");
        assert_eq!(24, part2(input));
    }
}
