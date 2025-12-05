//! Day 4: Printing Department
//!
//! https://adventofcode.com/2025/day/4

use core::fmt::Display;

use itertools::Itertools;
use solutions::grid::{Coordinate, Grid};
use tracing::{debug, instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let grid = input.parse::<Grid<Cell>>().unwrap();
    trace!("parsed grid:\n{}", grid);

    accessible_roles(&grid).count()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    // remove rolls of paper and repeat until no more can be removed
    let mut grid = input.parse::<Grid<Cell>>().unwrap();
    trace!("parsed grid:\n{}", grid);

    let mut total_removed = 0;
    loop {
        let cells_to_remove = accessible_roles(&grid).collect_vec();
        total_removed += cells_to_remove.len();
        if cells_to_remove.is_empty() {
            debug!("no more accessible rolls to remove");
            break;
        }

        debug!(
            "removing {} accessible rolls of paper",
            cells_to_remove.len()
        );

        for coordinate in cells_to_remove {
            grid.get_mut(coordinate).map(|cell| {
                *cell = Cell::Empty;
            });
        }
    }

    total_removed
}

fn main() {
    solutions::main(part1, part2);
}

/// Get an iterator over all coordinates of [`Cell::RollOfPaper`] that are accessible.
fn accessible_roles(grid: &Grid<Cell>) -> impl Iterator<Item = Coordinate> + '_ {
    grid.enumerate()
        .filter(|(_, cell)| matches!(cell, Cell::RollOfPaper))
        .filter(move |(coordinate, _)| count_neighborhood_rolls(&grid, coordinate) < 4)
        .inspect(move |(coordinate, _)| {
            trace!(
                ?coordinate,
                rolls_in_neighborhood = %count_neighborhood_rolls(&grid, coordinate),
                "valid coordinate",
            )
        })
        .map(|(coordinate, _)| coordinate)
}

/// Count the number of rolls of paper in the Moore neighborhood of the given coordinate.
fn count_neighborhood_rolls(grid: &Grid<Cell>, coordinate: &Coordinate) -> usize {
    let neighbors = coordinate.moore();
    neighbors
        .into_iter()
        .flatten()
        .filter_map(|neighbor| grid.get(neighbor))
        .filter(|cell| matches!(cell, Cell::RollOfPaper))
        .count()
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    RollOfPaper,
}

impl Display for Cell {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::RollOfPaper => write!(f, "@"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::RollOfPaper),
            c => Err(format!("invalid cell character: {}", c)),
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
        let input = include_str!("../../data/examples/2025/04/1.txt");
        assert_eq!(13, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/04/1.txt");
        assert_eq!(0, part2(input));
    }
}
