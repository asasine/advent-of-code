//! Day 18
//!
//! https://adventofcode.com/2024/day/18

use core::fmt;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use itertools::Itertools;
use solutions::grid::{Coordinate, Grid};
use tracing::{instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> u64 {
    part1_impl::<71>(input, 1024)
}

fn part1_impl<const SIZE: usize>(input: &str, steps: usize) -> u64 {
    let mut memory_space: MemorySpace<SIZE> = input.parse().unwrap();
    trace!("{}", memory_space);

    memory_space.simulate(steps);
    trace!("{}", memory_space);

    let start = Coordinate { x: 0, y: 0 };
    let end = Coordinate {
        x: SIZE - 1,
        y: SIZE - 1,
    };
    let dijkstra_result = memory_space.dijkstra(start, Some(end));

    dijkstra_result
        .cost_to(end)
        .expect("Should have found a path to the target")
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> String {
    part2_impl::<71>(input)
}

fn part2_impl<const SIZE: usize>(input: &str) -> String {
    let memory_space: MemorySpace<SIZE> = input.parse().unwrap();
    let start = Coordinate { x: 0, y: 0 };
    let end = Coordinate {
        x: SIZE - 1,
        y: SIZE - 1,
    };

    let blocking_byte = memory_space.first_blocking_falling_byte(start, end);
    format!("{},{}", blocking_byte.x, blocking_byte.y)
}

fn main() {
    solutions::main(part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Safe,
    Corrupted,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Safe => write!(f, "."),
            Cell::Corrupted => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone)]
struct MemorySpace<const SIZE: usize> {
    grid: Grid<Cell>,
    falling_bytes: Vec<Coordinate>,
    time: usize,
}

impl<const SIZE: usize> MemorySpace<SIZE> {
    /// Create a new memory space with the given falling bytes.
    ///
    /// The grid is initialized with all safe cells.
    fn new(falling_bytes: Vec<Coordinate>) -> Self {
        let grid = Grid::new(vec![vec![Cell::Safe; SIZE]; SIZE]);
        Self {
            grid,
            falling_bytes,
            time: 0,
        }
    }

    /// Simulate the falling bytes for the given number of steps.
    fn simulate(&mut self, steps: usize) {
        for &byte in self.falling_bytes.iter().skip(self.time).take(steps) {
            *self.grid.get_mut(byte).unwrap() = Cell::Corrupted;
        }

        self.time += steps;
    }

    /// Return an iterator over the adjacencies of the given coordinate.
    fn adjacencies(&self, coordinate: Coordinate) -> impl Iterator<Item = Coordinate> + '_ {
        coordinate
            .von_neumann()
            .into_iter()
            .filter_map(|c| c.filter(|&c| matches!(self.grid.get(c), Some(Cell::Safe))))
    }

    /// Find the shortest path from the start to all other coordinates.
    ///
    /// If `end` is provided, the search will stop when the end coordinate is reached.
    /// Otherwise, the search will continue until all coordinates are visited.
    fn dijkstra(&self, start: Coordinate, end: Option<Coordinate>) -> DijkstraResult {
        let mut distances = HashMap::new();
        distances.insert(start, 0);

        let mut predecessors = HashMap::new();

        let mut queue = BinaryHeap::new();
        queue.push(Reverse(State {
            cost: 0,
            coordinate: start,
        }));

        while let Some(Reverse(State {
            cost: _,
            coordinate: u,
        })) = queue.pop()
        {
            trace!("Visiting {:?}", u);
            if end == Some(u) {
                break;
            }

            let current_distance = distances[&u];
            for v in self.adjacencies(u) {
                trace!("  Adjacent: {:?}", v);
                let alt = current_distance + 1;
                let dist_v = distances.entry(v).or_insert(u64::MAX);
                if alt < *dist_v {
                    trace!("    Found a shorter distance at {}", alt);
                    predecessors.insert(v, u);
                    *dist_v = alt;
                    queue.push(Reverse(State {
                        cost: alt,
                        coordinate: v,
                    }));
                }
            }
        }

        DijkstraResult {
            distances,
            predecessors,
        }
    }

    /// Determines the first coordinate in [`Self::falling_bytes`] that prevents a path from `start` to `end`.
    fn first_blocking_falling_byte(&self, start: Coordinate, end: Coordinate) -> Coordinate {
        assert_eq!(self.time, 0, "Should not have simulated any bytes yet");
        let mut memory_space = self.clone();
        let mut dijkstra_result = memory_space.dijkstra(start, Some(end));
        if !dijkstra_result.reachable(end) {
            panic!("End coordinate is not reachable on the initial grid");
        }

        for coordinate in &self.falling_bytes {
            memory_space.simulate(1);
            trace!("Simulated {}ns with {}", memory_space.time, coordinate);
            if dijkstra_result.predecessors(end).contains(coordinate) {
                // the current best path was blocked by the falling byte, so we need to recompute
                trace!("Recomputing path after falling byte at {}", coordinate);
                dijkstra_result = memory_space.dijkstra(start, Some(end));
            }

            if !dijkstra_result.reachable(end) {
                trace!("Found a blocking byte at {}", coordinate);
                return *coordinate;
            }
        }

        unreachable!("Should have found a blocking byte");
    }
}

impl<const SIZE: usize> FromStr for MemorySpace<SIZE> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let falling_bytes = s
            .lines()
            .map(|l| {
                let mut parts = l.split(',');
                let x = parts
                    .next()
                    .expect("Should have been an x")
                    .parse()
                    .expect("x should have been an integer");

                let y = parts
                    .next()
                    .expect("Should have been a y")
                    .parse()
                    .expect("y should have been an integer");

                Coordinate { x, y }
            })
            .collect();

        Ok(Self::new(falling_bytes))
    }
}

impl<const SIZE: usize> fmt::Display for MemorySpace<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid at {}ns:", self.time)?;
        writeln!(f, "{}", self.grid)?;
        writeln!(f, "Remaining bytes to fall, in order:")?;
        for fb in self.falling_bytes.iter().skip(self.time) {
            writeln!(f, "{}", fb)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: u64,
    coordinate: Coordinate,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct DijkstraResult {
    /// The distance from the start to each coordinate.
    distances: HashMap<Coordinate, u64>,

    /// The previous coordinate in the shortest path.
    predecessors: HashMap<Coordinate, Coordinate>,
}

impl DijkstraResult {
    /// Return whether the given end coordinate is reachable from the start.
    fn reachable(&self, end: Coordinate) -> bool {
        self.cost_to(end).is_some()
    }

    /// Return the cost to the given end coordinate, or `None` if it is unreachable.
    fn cost_to(&self, end: Coordinate) -> Option<u64> {
        self.distances.get(&end).copied()
    }

    /// Return the predecessors of the given end coordinate to the start.
    fn predecessors(&self, end: Coordinate) -> impl Iterator<Item = Coordinate> + '_ {
        std::iter::successors(Some(end), |current| self.predecessors.get(current).copied())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/18/1.txt");
        assert_eq!(22, part1_impl::<7>(input, 12));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/18/1.txt");
        assert_eq!("6,1", part2_impl::<7>(input));
    }

    #[test]
    fn adjacencies() {
        let input = include_str!("../../data/examples/2024/18/1.txt");
        let mut memory_space: MemorySpace<7> = input.parse().unwrap();
        memory_space.simulate(12);
        let actual = memory_space
            .adjacencies(Coordinate { x: 3, y: 1 })
            .collect::<HashSet<_>>();

        let expected = HashSet::from_iter([Coordinate { x: 3, y: 2 }, Coordinate { x: 4, y: 1 }]);
        assert_eq!(expected, actual);
    }
}
