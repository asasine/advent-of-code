//! Day 12: Christmas Tree Farm
//!
//! https://adventofcode.com/2025/day/12

use solutions::grid::Grid;
use tracing::{instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let input: Input = input.parse().unwrap();
    trace!(num_trees = input.trees.0.len(), %input, "Parsed input");

    input
        .trees
        .0
        .iter()
        .filter(|tree| {
            let area = tree.area() as u32;
            let min_area_needed = tree.min_area_needed(&input.present);
            trace!(
                ?tree.dimensions,
                tree_area = area,
                min_area_needed,
                "Checking tree"
            );

            area >= min_area_needed
        })
        .count()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    0
}

fn main() {
    solutions::main(part1, part2);
}

struct Input {
    present: Presents,
    trees: Trees,
}

struct Trees(Vec<Tree>);

struct Tree {
    /// The dimensions of the region available below this tree.
    dimensions: (u8, u8),

    /// The required number of presents for this tree.
    counts: Vec<u8>,
}

impl Tree {
    /// The amount of area available below this tree.
    fn area(&self) -> u16 {
        (self.dimensions.0 as u16) * (self.dimensions.1 as u16)
    }

    /// The minimum area needed to satisfy this tree's present counts.
    ///
    /// This value assumes that all presents are optimally packed with no unoccupied cells.
    fn min_area_needed(&self, presents: &Presents) -> u32 {
        self.counts
            .iter()
            .enumerate()
            .map(|(i, &count)| {
                let present = &presents.0[i];
                let occupied = present.count_occupied() as u32;
                occupied * (count as u32)
            })
            .sum()
    }
}

struct Presents(Vec<Present>);

struct Present {
    index: usize,
    grid: Grid<Cell>,
}

impl Present {
    /// Count the number of occupied cells in this present's grid.
    fn count_occupied(&self) -> usize {
        self.grid
            .enumerate()
            .filter(|(_, cell)| **cell == Cell::Occupied)
            .count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    /// `.`
    Empty,

    /// `#`
    Occupied,
}

mod parse {
    //! Implementations of parsing traits for this day's problem.

    #![expect(dead_code, reason = "derive(Debug) errors used for parsing")]

    use super::{Cell, Input, Present, Presents, Tree, Trees};
    use core::{num::ParseIntError, str::FromStr};
    use itertools::Itertools;
    use solutions::iter::IteratorExt;
    use tracing::trace;

    #[derive(Debug)]
    pub enum InputParseError {
        MissingDelimiter,
        PresentsParseError(PresentsParseError),
        TreesParseError(TreesParseError),
    }

    impl FromStr for Input {
        type Err = InputParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let first_index_of_x = s.find('x').ok_or(InputParseError::MissingDelimiter)?;
            // search backwards for the begining of this line
            let first_index_of_x = s[..first_index_of_x]
                .rfind('\n')
                .map(|i| i + 1)
                .unwrap_or(0);

            trace!(
                "Splitting input at index {}: {}",
                first_index_of_x,
                s.chars().nth(first_index_of_x).unwrap()
            );

            let (presents, trees) = s.split_at(first_index_of_x);

            let present = presents
                .trim()
                .parse()
                .map_err(InputParseError::PresentsParseError)?;

            let trees = trees
                .trim()
                .parse()
                .map_err(InputParseError::TreesParseError)?;

            Ok(Input { present, trees })
        }
    }

    #[derive(Debug)]
    pub struct TreesParseError {
        inner: TreeParseError,
        index: usize,
    }

    impl FromStr for Trees {
        type Err = TreesParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Trees(
                s.lines()
                    .enumerate()
                    .map(|(i, line)| {
                        line.parse()
                            .map_err(|e| TreesParseError { inner: e, index: i })
                    })
                    .try_collect()?,
            ))
        }
    }

    #[derive(Debug)]
    pub enum TreeParseError {
        CannotSplitDimensionsAndCounts,
        CannotSplitWidthAndHeight,
        InvalidWidth,
        InvalidHeight,
    }

    impl FromStr for Tree {
        type Err = TreeParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (dimensions, counts) = s
                .split_once(": ")
                .ok_or(TreeParseError::CannotSplitDimensionsAndCounts)?;

            let (width, height) = dimensions
                .split_once('x')
                .ok_or(TreeParseError::CannotSplitWidthAndHeight)?;

            let width = width.parse().map_err(|_| TreeParseError::InvalidWidth)?;
            let height = height.parse().map_err(|_| TreeParseError::InvalidHeight)?;
            let dimensions = (width, height);

            let counts = counts
                .split(' ')
                .map(|num| {
                    num.chars()
                        .map(|c| c.to_digit(10).unwrap_or(0) as u8)
                        .collect_num(10)
                })
                .collect();

            Ok(Tree { dimensions, counts })
        }
    }

    #[derive(Debug)]
    pub struct PresentsParseError {
        inner: PresentParseError,
        index: usize,
    }

    impl FromStr for Presents {
        type Err = PresentsParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let delimiter = if s.contains("\r\n") {
                "\r\n\r\n"
            } else {
                "\n\n"
            };

            let presents = s
                .split(delimiter)
                .enumerate()
                .map(|(i, s)| {
                    s.parse()
                        .map_err(|e| PresentsParseError { inner: e, index: i })
                })
                .try_collect()?;

            Ok(Presents(presents))
        }
    }

    #[derive(Debug)]
    pub enum PresentParseError {
        CannotFindGrid,
        CannotParseIndex(ParseIntError),
        InvalidCell(char),
    }

    impl FromStr for Present {
        type Err = PresentParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // each present begins with a line like "1:" that we can ignore so start parsing at the first grid char
            let (index, grid) = s.split_at(
                s.find(['.', '#'])
                    .ok_or(PresentParseError::CannotFindGrid)?,
            );

            let index = index
                .trim()
                .trim_end_matches(':')
                .parse()
                .map_err(PresentParseError::CannotParseIndex)?;

            let grid = grid.parse().map_err(PresentParseError::InvalidCell)?;
            Ok(Present { index, grid })
        }
    }

    impl TryFrom<char> for Cell {
        type Error = char;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '.' => Ok(Cell::Empty),
                '#' => Ok(Cell::Occupied),
                other => Err(other),
            }
        }
    }
}

mod display {
    //! Implementations of [`core::fmt::Display`] for this day's problem.

    use super::{Cell, Input, Present, Presents, Tree, Trees};
    use core::fmt::{self, Display};

    impl Display for Input {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "{}", self.present)?;
            write!(f, "{}", self.trees)
        }
    }

    impl Display for Trees {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (i, tree) in self.0.iter().enumerate() {
                if i > 0 {
                    writeln!(f)?;
                }

                write!(f, "{}", tree)?;
            }

            Ok(())
        }
    }

    impl Display for Tree {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}x{}: ", self.dimensions.0, self.dimensions.1)?;
            for (i, count) in self.counts.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }

                write!(f, "{}", count)?;
            }

            Ok(())
        }
    }

    impl Display for Presents {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (i, present) in self.0.iter().enumerate() {
                if i > 0 {
                    writeln!(f)?;
                }

                write!(f, "{}", present)?;
            }

            Ok(())
        }
    }

    impl Display for Present {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "{}:", self.index)?;
            write!(f, "{}", self.grid)
        }
    }

    impl Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Cell::Empty => write!(f, "."),
                Cell::Occupied => write!(f, "#"),
            }
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
        let input = include_str!("../../data/examples/2025/12/1.txt");
        assert_eq!(2, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/12/1.txt");
        assert_eq!(0, part2(input));
    }
}
