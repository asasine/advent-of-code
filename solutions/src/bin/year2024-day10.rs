//! Day 10: Hoof It
//!
//! https://adventofcode.com/2024/day/10

use std::{collections::HashSet, str::FromStr};

use solutions::grid::{Coordinate, Grid};
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let grid = Grid::<Cell>::from_str(input).unwrap();
    let guide = LavaIslandHikingGuide { grid };
    let trailheads =
        guide.grid.enumerate().filter_map(
            move |(c, cell)| {
                if cell.is_trailhead() {
                    Some(c)
                } else {
                    None
                }
            },
        );

    trailheads.map(|c| guide.find_score(c)).sum()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let grid = Grid::<Cell>::from_str(input).unwrap();
    let guide = LavaIslandHikingGuide { grid };
    let trailheads =
        guide.grid.enumerate().filter_map(
            move |(c, cell)| {
                if cell.is_trailhead() {
                    Some(c)
                } else {
                    None
                }
            },
        );

    trailheads.map(|c| guide.find_rating(c)).sum()
}

fn main() {
    solutions::main(part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Height(u8),
    Impassable,
}

impl Cell {
    fn is_trailhead(&self) -> bool {
        matches!(self, Cell::Height(0))
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            c @ '0'..='9' => Cell::Height(c.to_digit(10).unwrap() as u8),
            '.' => Cell::Impassable,
            _ => panic!("Invalid cell: {}", c),
        }
    }
}

struct LavaIslandHikingGuide {
    grid: Grid<Cell>,
}

impl LavaIslandHikingGuide {
    /// Find a cell's score by searching for the number of distinct `9`-height cells that are reachable from the given coordinate.
    fn find_score(&self, c: Coordinate) -> usize {
        self.search(c, false)
    }

    /// Find a cell's rating by searching for the number of paths to `9`-height cells from the given coordinate.
    fn find_rating(&self, c: Coordinate) -> usize {
        self.search(c, true)
    }

    /// Search for paths to `9`-height cells from the given coordinate.
    fn search(&self, c: Coordinate, all_paths: bool) -> usize {
        let mut stack = Vec::new();
        let mut reachable = Vec::new();
        let mut visited = HashSet::new();
        stack.push(c);
        while let Some(current) = stack.pop() {
            if !all_paths && visited.contains(&current) {
                continue;
            }

            visited.insert(current);
            let current_cell = self.grid.get(current).unwrap();
            let height = match current_cell {
                Cell::Height(h) => *h,
                _ => continue,
            };

            if height == 9 {
                reachable.push(current);
            }

            let reachable_new_neighbors = current
                .von_neumann_within(self.grid.extent())
                .into_iter()
                .flatten()
                .filter(|neighbor| {
                    matches!(self.grid.get(*neighbor), Some(Cell::Height(h)) if *h == height + 1)
                });

            stack.extend(reachable_new_neighbors);
        }

        if all_paths {
            reachable.len()
        } else {
            reachable.into_iter().collect::<HashSet<_>>().len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLES: &[(usize, &str)] = &[
        (
            2,
            r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#,
        ),
        (
            4,
            r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#,
        ),
        (
            3,
            r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#,
        ),
        (
            36,
            r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
        ),
    ];

    #[test]
    fn part1_example() {
        for example in PART1_EXAMPLES {
            assert_eq!(example.0, part1(example.1));
        }
    }

    const PART2_EXAMPLES: &[(usize, &str)] = &[
        (
            3,
            r#".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."#,
        ),
        (
            13,
            r#"..90..9
...1.98
...2..7
6543456
765.987
876....
987...."#,
        ),
        (
            227,
            r#"012345
123456
234567
345678
4.6789
56789."#,
        ),
        (
            81,
            r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
        ),
    ];

    #[test]
    fn part2_example() {
        for example in PART2_EXAMPLES {
            assert_eq!(example.0, part2(example.1));
        }
    }
}
