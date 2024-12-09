//! Day 8: Resonant Collinearity
//!
//! https://adventofcode.com/2024/day/8

use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use nalgebra::Point2;

fn part1(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    eprintln!("{}", grid);

    let antinodes = grid.get_first_antinodes();
    eprintln!("With first antinodes:");
    eprintln!("{}", antinodes);
    antinodes.distinct_in_bounds().len()
}

fn part2(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    eprintln!("{}", grid);
    let antinodes = grid.get_all_antinodes();
    eprintln!("With all antinodes:");
    eprintln!("{}", antinodes);
    antinodes.distinct_in_bounds().len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Frequency(char);

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Antenna(Frequency),
}

impl Cell {
    fn is_antenna(&self) -> bool {
        matches!(self, Cell::Antenna(_))
    }
}

impl TryFrom<char> for Cell {
    type Error = char;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Empty),
            '0'..='9' | 'A'..='Z' | 'a'..='z' => Ok(Cell::Antenna(Frequency(c))),
            _ => Err(c),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Antenna(c) => write!(f, "{}", c),
        }
    }
}

type Point2i = Point2<i64>;

/// An antinode occurs at `location` from collinear antennas `a` and `b`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antinode {
    frequency: Frequency,
    a: Point2i,
    b: Point2i,
    location: Point2i,
}

#[derive(Debug)]
struct Antinodes<'a> {
    antinodes: HashSet<Antinode>,
    grid: &'a Grid,
}

impl<'a> Antinodes<'a> {
    fn distinct_in_bounds(&self) -> HashSet<Point2i> {
        self.antinodes.iter().map(|a| a.location).collect()
    }
}

impl<'a> fmt::Display for Antinodes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let locations = self.distinct_in_bounds();
        for (y, row) in self.grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let p = Point2i::new(x as i64, y as i64);

                if locations.contains(&p) && !cell.is_antenna() {
                    write!(f, "#")?;
                } else {
                    write!(f, "{cell}")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    frequencies_to_locations: HashMap<Frequency, Vec<Point2i>>,
}

impl Grid {
    fn from_cells(cells: Vec<Vec<Cell>>) -> Self {
        let frequencies_to_locations = {
            let mut m = HashMap::new();
            for (y, row) in cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if let Cell::Antenna(frequency) = cell {
                        m.entry(*frequency)
                            .or_insert_with(Vec::new)
                            .push(Point2i::new(x as i64, y as i64));
                    }
                }
            }

            m
        };

        Grid {
            cells,
            frequencies_to_locations,
        }
    }

    fn get(&self, point: Point2i) -> Option<&Cell> {
        if point.y < 0 || point.x < 0 {
            return None;
        }

        self.cells.get(point.y as usize)?.get(point.x as usize)
    }

    fn get_first_antinodes(&self) -> Antinodes {
        let mut antinodes = HashSet::new();

        // for every pair of antennas with the same frequency, get the location of the cell that's collinear with them.
        for (frequency, locations) in self.frequencies_to_locations.iter() {
            for (a, b) in locations.iter().tuple_combinations() {
                let delta = {
                    let delta = a - b;
                    if a + delta == *b {
                        -delta
                    } else {
                        delta
                    }
                };

                let locations = [a + delta, b - delta];
                let locations = locations.iter().filter(|p| self.get(**p).is_some());
                antinodes.extend(locations.map(|l| Antinode {
                    frequency: *frequency,
                    a: *a,
                    b: *b,
                    location: *l,
                }));
            }
        }

        Antinodes {
            antinodes,
            grid: self,
        }
    }

    fn get_all_antinodes(&self) -> Antinodes {
        let mut antinodes = HashSet::new();

        // for every pair of antennas with the same frequency, get the location of the cells that are collinear with them and in the grid's bounds.
        for (frequency, locations) in self.frequencies_to_locations.iter() {
            for (a, b) in locations.iter().tuple_combinations() {
                let delta = {
                    let delta = a - b;
                    if a + delta == *b {
                        -delta
                    } else {
                        delta
                    }
                };

                let locations = std::iter::successors(Some(*a), |&p| {
                    let next = p + delta;
                    if self.get(next).is_some() {
                        Some(next)
                    } else {
                        None
                    }
                })
                .chain(std::iter::successors(Some(*b), |&p| {
                    let next = p - delta;
                    if self.get(next).is_some() {
                        Some(next)
                    } else {
                        None
                    }
                }));

                antinodes.extend(locations.map(|l| Antinode {
                    frequency: *frequency,
                    a: *a,
                    b: *b,
                    location: l,
                }));
            }
        }

        Antinodes {
            antinodes,
            grid: self,
        }
    }
}

impl FromStr for Grid {
    type Err = char;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<Cell>> = s
            .lines()
            .map(|line| line.chars().map(|c| Cell::try_from(c)).try_collect())
            .try_collect()?;

        Ok(Grid::from_cells(cells))
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.iter() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

aoc_macro::aoc_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/08/1.txt");
        assert_eq!(14, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/08/1.txt");
        assert_eq!(34, part2(input));
    }
}
