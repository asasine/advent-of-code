//! Day 6: Guard Gallivant
//!
//! https://adventofcode.com/2024/day/6

use core::fmt;
use std::collections::{HashMap, HashSet};

use solutions::grid::Direction;

fn part1(input: &str) -> usize {
    part1_impl::<130>(input)
}

fn part1_impl<const N: usize>(input: &str) -> usize {
    let grid = Grid::<N>::from_input(input);
    let (_, visited) = grid.walk_guard();
    let grid = VisitedGrid {
        grid: grid.data,
        visited,
    };

    eprintln!("{}", grid);
    grid.get_visited_coordinates().len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    /// `.`
    Empty,

    /// `#`
    Obstruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Visited {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Visited {
    fn and(&self, other: Self) -> Self {
        Self {
            up: self.up || other.up,
            down: self.down || other.down,
            left: self.left || other.left,
            right: self.right || other.right,
        }
    }
}

impl From<Direction> for Visited {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Self {
                up: true,
                down: false,
                left: false,
                right: false,
            },
            Direction::Right => Self {
                up: false,
                down: false,
                left: false,
                right: true,
            },
            Direction::Down => Self {
                up: false,
                down: true,
                left: false,
                right: false,
            },
            Direction::Left => Self {
                up: false,
                down: false,
                left: true,
                right: false,
            },
        }
    }
}

impl fmt::Display for Visited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let horizontal = self.left || self.right;
        let vertical = self.up || self.down;
        match (horizontal, vertical) {
            (true, true) => write!(f, "+"),
            (true, false) => write!(f, "-"),
            (false, true) => write!(f, "|"),
            (false, false) => unreachable!(),
        }
    }
}

/// A guard's state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    direction: Direction,
    position: Coordinate,
}

impl Guard {
    fn turn_right(self) -> Self {
        Self {
            direction: self.direction.turn_right(),
            ..self
        }
    }

    fn move_to(self, c: Coordinate) -> Self {
        Self {
            position: c,
            ..self
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Obstruction => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    /// Returns the coordinate after moving in the given direction.
    ///
    /// Returns `None` if the coordinate is out of bounds after moving.
    fn checked_add(self, rhs: Direction, max: usize) -> Option<Self> {
        match rhs {
            Direction::Up => self.y.checked_sub(1).map(|y| Coordinate { x: self.x, y }),
            Direction::Right => self.x.checked_add(1).map(|x| Coordinate { x, y: self.y }),
            Direction::Down => self.y.checked_add(1).map(|y| Coordinate { x: self.x, y }),
            Direction::Left => self.x.checked_sub(1).map(|x| Coordinate { x, y: self.y }),
        }
        .filter(|&c| c.x < max && c.y < max)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WalkResult {
    /// The guard encountered a loop.
    Loop,

    /// The guard exited the grid.
    Exit,
}

#[derive(Debug, Clone)]
struct Grid<const N: usize> {
    data: [[Cell; N]; N],
    guard: Guard,
}

impl<const N: usize> Grid<N> {
    fn from_input(input: &str) -> Self {
        let mut data = [[Cell::Empty; N]; N];
        let mut guard = None;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                debug_assert!(x < N && y < N);
                data[y][x] = match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Obstruction,
                    '^' => {
                        guard = Some(Guard {
                            direction: Direction::Up,
                            position: Coordinate { x, y },
                        });

                        Cell::Empty
                    }
                    _ => unreachable!("unexpected character: {}", c),
                };
            }
        }

        Self {
            data,
            guard: guard.expect("There should have been a guard in the input"),
        }
    }

    fn get(&self, pos: Coordinate) -> &Cell {
        &self.data[pos.y][pos.x]
    }

    fn get_mut(&mut self, pos: Coordinate) -> &mut Cell {
        &mut self.data[pos.y][pos.x]
    }

    /// Walk the guard until it leaves the grid or encounters a loop.
    fn walk_guard(&self) -> (WalkResult, HashSet<Guard>) {
        let grid = self.clone();
        let mut visited = HashSet::new();
        let mut guard = Some(grid.guard);
        while let Some(current_guard) = guard {
            if visited.contains(&current_guard) {
                return (WalkResult::Loop, visited);
            }

            visited.insert(current_guard);
            guard = current_guard
                .position
                .checked_add(current_guard.direction, N)
                .map(|next_pos| match grid.get(next_pos) {
                    Cell::Obstruction => current_guard.turn_right(),
                    _ => current_guard.move_to(next_pos),
                });
        }

        (WalkResult::Exit, visited)
    }
}

impl<const N: usize> fmt::Display for Grid<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.data.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let coordinate = Coordinate { x, y };
                if coordinate == self.guard.position {
                    write!(f, "{}", self.guard.direction)?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct VisitedGrid<const N: usize> {
    grid: [[Cell; N]; N],
    visited: HashSet<Guard>,
}

impl<const N: usize> VisitedGrid<N> {
    fn get_visitations(&self) -> HashMap<Coordinate, Visited> {
        let mut visitations: HashMap<Coordinate, Visited> = HashMap::new();
        for g in self.visited.iter() {
            visitations
                .entry(g.position)
                .and_modify(|v| *v = v.and(g.direction.into()))
                .or_insert_with(|| Visited::from(g.direction));
        }

        visitations
    }

    fn get_visited_coordinates(&self) -> HashSet<Coordinate> {
        self.get_visitations()
            .keys()
            .cloned()
            .collect::<HashSet<_>>()
    }
}

impl<const N: usize> fmt::Display for VisitedGrid<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // turn the flat set of visited Guards into Visiteds
        let visitations = self.get_visitations();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let coordinate = Coordinate { x, y };
                if let Some(visited) = visitations.get(&coordinate) {
                    write!(f, "{}", visited)?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

fn part2(input: &str) -> usize {
    part2_impl::<130>(input)
}

fn part2_impl<const N: usize>(input: &str) -> usize {
    let grid = Grid::<N>::from_input(input);
    let visited = {
        let (_, visited) = grid.walk_guard();
        let grid = VisitedGrid {
            grid: grid.data,
            visited,
        };

        grid.get_visited_coordinates()
    };

    let mut looping_obstructions = HashSet::new();
    for extra in visited {
        let mut grid = grid.clone();
        *grid.get_mut(extra) = Cell::Obstruction;
        let (walk_result, _) = grid.walk_guard();
        if walk_result == WalkResult::Loop {
            looping_obstructions.insert(extra);
        }
    }

    looping_obstructions.len()
}

aoc_macro::aoc_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/06/1.txt");
        assert_eq!(41, part1_impl::<10>(input));
    }

    #[test]
    fn coordinate_checked_add() {
        let c = Coordinate { x: 1, y: 1 };
        assert_eq!(
            Some(Coordinate { x: 2, y: 1 }),
            c.checked_add(Direction::Right, 3)
        );

        assert_eq!(
            Some(Coordinate { x: 1, y: 0 }),
            c.checked_add(Direction::Up, 3)
        );

        assert_eq!(None, c.checked_add(Direction::Left, 1));
        assert_eq!(None, c.checked_add(Direction::Down, 1));
    }

    #[test]
    fn direction_turn_right() {
        let original = Direction::Up;
        assert_eq!(
            original,
            original.turn_right().turn_right().turn_right().turn_right()
        );
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/06/1.txt");
        assert_eq!(6, part2_impl::<10>(input));
    }
}
