//! Day 7: Laboratories
//!
//! https://adventofcode.com/2025/day/7

use std::collections::{HashMap, HashSet, VecDeque};

use solutions::grid::{Coordinate, Direction, Grid};
use tracing::{instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let grid: Grid<Cell> = input.parse().unwrap();
    let starting_point = grid
        .enumerate()
        .find(|(_, c)| **c == Cell::Start)
        .unwrap()
        .0;

    let mut splitters = HashSet::new();
    let mut coordinate_to_branches = HashMap::new();
    simulate(
        &grid,
        starting_point,
        &mut splitters,
        &mut coordinate_to_branches,
    );

    splitters.len()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let grid: Grid<Cell> = input.parse().unwrap();
    let starting_point = grid
        .enumerate()
        .find(|(_, c)| **c == Cell::Start)
        .unwrap()
        .0;

    let mut splitters = HashSet::new();
    let mut coordinate_to_branches = HashMap::new();
    simulate(
        &grid,
        starting_point,
        &mut splitters,
        &mut coordinate_to_branches,
    );

    *coordinate_to_branches.get(&starting_point).unwrap()
}

fn main() {
    solutions::main(part1, part2);
}

/// Simulate a tachyon beam recursively from `start`.
///
/// Returns the number of beams that will exist by a beam traveling from `start` to the bottom edge of the grid.
/// A beam that encounters a splitter is considered a new beam. A beam that does not encounter a splitter before
/// reaching the bottom edge of the grid is considered a single beam.
///
/// `splitters` is a set of all of the splitters that were hit by a beam.
fn simulate(
    grid: &Grid<Cell>,
    start: Coordinate,
    splitters: &mut HashSet<Coordinate>,
    coordinate_to_branches: &mut HashMap<Coordinate, usize>,
) -> usize {
    if let Some(&split_count) = coordinate_to_branches.get(&start) {
        trace!(%start, %split_count, "already visited");
        return split_count;
    }

    let next = start.try_move(Direction::Down).unwrap();
    match grid.get(next) {
        Some(Cell::Splitter) => {
            trace!(%next, "moved into splitter");
            splitters.insert(next);

            // simulate the left and right branches
            let left = next.try_move(Direction::Left).unwrap();
            trace!(%left, "split");
            simulate(grid, left, splitters, coordinate_to_branches);

            let right = next.try_move(Direction::Right).unwrap();
            trace!(%right, "split");
            simulate(grid, right, splitters, coordinate_to_branches);
            let combined = coordinate_to_branches[&left] + coordinate_to_branches[&right];
            coordinate_to_branches.insert(start, combined);
            return combined;
        }
        Some(Cell::Empty) => {
            trace!(%next, "moved into empty cell");
            let count = simulate(grid, next, splitters, coordinate_to_branches);
            coordinate_to_branches.insert(start, count);
            return count;
        }
        None => {
            trace!(%next, "moved down off the grid");
            coordinate_to_branches.insert(start, 1);
            return 1;
        }
        other => {
            panic!("unexpected cell: {:?}", other);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    Splitter,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            'S' => Ok(Self::Start),
            '^' => Ok(Self::Splitter),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use solutions::setup_tracing;

    use super::*;

    #[test]
    fn part1_example() {
        setup_tracing();
        let input = include_str!("../../data/examples/2025/07/1.txt");
        assert_eq!(21, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/07/1.txt");
        assert_eq!(40, part2(input));
    }
}
