//! A [`Grid`] of things.

use super::{Coordinate, Rectangle};
use core::{fmt::Display, str::FromStr};

/// A grid of cells.
///
/// A grid can be created by hand or parsed from a string.
///
/// # Examples
/// ## Parsing from a string
///
/// Parsing a grid of cells from a string utilizes the [`TryFrom<char>`] trait to convert characters into cells.
/// ```
/// # use solutions::grid::{Coordinate, Grid};
/// // Each `.` represents an empty cell and each `*` represents an obstacle.
/// let s = r#"..*.
/// .*..
/// *..."#;
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum Cell {
///     Empty,
///     Obstacle,
/// }
///
/// impl TryFrom<char> for Cell {
///     type Error = char;
///     fn try_from(value: char) -> Result<Self, Self::Error> {
///         match value {
///             '.' => Ok(Cell::Empty),
///             '*' => Ok(Cell::Obstacle),
///             unknown => Err(unknown),
///         }
///     }
/// }
///
/// let grid: Grid<Cell> = s.parse().unwrap();
/// assert_eq!(Some(&Cell::Empty), grid.get(Coordinate { x: 0, y: 0 }));
/// assert_eq!(Some(&Cell::Obstacle), grid.get(Coordinate { x: 0, y: 2 }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    /// The cells in the grid.
    cells: Vec<Vec<T>>,

    /// The extent of the grid.
    extent: Rectangle,
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = T::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<T>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| T::try_from(c))
                    .collect::<Result<Vec<T>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(cells))
    }
}

impl<T> Grid<T> {
    /// Create a new grid from a vector of cells.
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        let extent = Rectangle {
            min: Coordinate { x: 0, y: 0 },
            max: Coordinate {
                x: cells[0].len() - 1,
                y: cells.len() - 1,
            },
        };

        Self { cells, extent }
    }

    /// Get a reference to the cell at the given coordinate, or [`None`] if the coordinate is out of bounds.
    pub fn get(&self, c: Coordinate) -> Option<&T> {
        self.cells.get(c.y).and_then(|row| row.get(c.x))
    }

    /// Get a mutable reference to the cell at the given coordinate, or [`None`] if the coordinate is out of bounds.
    pub fn get_mut(&mut self, c: Coordinate) -> Option<&mut T> {
        self.cells.get_mut(c.y).and_then(|row| row.get_mut(c.x))
    }

    pub fn extent(&self) -> Rectangle {
        self.extent
    }

    /// Consume the grid and return its cells.
    pub fn into_cells(self) -> Vec<Vec<T>> {
        self.cells
    }

    /// Enumerate the cells in the grid.
    pub fn enumerate(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, t)| (Coordinate { x, y }, t))
            })
            .flatten()
    }

    /// Enumerate the mutable cells in the grid.
    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> {
        self.cells
            .iter_mut()
            .enumerate()
            .map(|(y, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(x, t)| (Coordinate { x, y }, t))
            })
            .flatten()
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let cells = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new(cells);

        assert_eq!(grid.get(Coordinate { x: 0, y: 0 }), Some(&1));
        assert_eq!(grid.get(Coordinate { x: 2, y: 2 }), Some(&9));
        assert_eq!(grid.get(Coordinate { x: 3, y: 3 }), None);
    }

    #[test]
    fn get_mut() {
        let cells = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let mut grid = Grid::new(cells);
        assert_eq!(grid.get_mut(Coordinate { x: 0, y: 0 }), Some(&mut 1));
        assert_eq!(grid.get_mut(Coordinate { x: 2, y: 2 }), Some(&mut 9));
        assert_eq!(grid.get_mut(Coordinate { x: 3, y: 3 }), None);

        *grid.get_mut(Coordinate { x: 0, y: 0 }).unwrap() = 10;
        assert_eq!(grid.get(Coordinate { x: 0, y: 0 }), Some(&10));
    }

    #[test]
    fn extent() {
        let cells = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new(cells);

        assert_eq!(
            grid.extent(),
            Rectangle {
                min: Coordinate { x: 0, y: 0 },
                max: Coordinate { x: 2, y: 2 },
            }
        );
    }

    #[test]
    fn into_cells() {
        let cells = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(cells.clone());
        assert_eq!(grid.into_cells(), cells);
    }

    #[test]
    fn enumerate() {
        let cells = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::new(cells);

        let mut iter = grid.enumerate();
        assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 0 }, &1)));
        assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 0 }, &2)));
        assert_eq!(iter.next(), Some((Coordinate { x: 2, y: 0 }, &3)));
        assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 1 }, &4)));
        assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 1 }, &5)));
        assert_eq!(iter.next(), Some((Coordinate { x: 2, y: 1 }, &6)));
        assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 2 }, &7)));
        assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 2 }, &8)));
        assert_eq!(iter.next(), Some((Coordinate { x: 2, y: 2 }, &9)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // fused
    }

    #[test]
    fn enumerate_mut() {
        let cells = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut grid = Grid::new(cells);

        let mut iter = grid.enumerate_mut();
        let next = iter.next();
        assert_eq!(next, Some((Coordinate { x: 0, y: 0 }, &mut 1)));
        *next.unwrap().1 *= 10;

        assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 0 }, &mut 2)));
        assert_eq!(iter.next(), Some((Coordinate { x: 2, y: 0 }, &mut 3)));
        assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 1 }, &mut 4)));
        assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 1 }, &mut 5)));
        assert_eq!(iter.next(), Some((Coordinate { x: 2, y: 1 }, &mut 6)));
        assert_eq!(iter.next(), Some((Coordinate { x: 0, y: 2 }, &mut 7)));
        assert_eq!(iter.next(), Some((Coordinate { x: 1, y: 2 }, &mut 8)));
        assert_eq!(iter.next(), Some((Coordinate { x: 2, y: 2 }, &mut 9)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // fused

        drop(iter); // to release the mutable borrow

        let mut iter = grid.enumerate();
        assert_eq!(
            iter.next(),
            Some((Coordinate { x: 0, y: 0 }, &10)),
            "First cell should have been modified"
        );
    }

    #[test]
    fn from_str() {
        let grid = Grid::<char>::from_str(
            r#"ABC
DEF
GHI"#,
        )
        .unwrap();

        assert_eq!(grid.get(Coordinate { x: 0, y: 0 }), Some(&'A'));
        assert_eq!(grid.get(Coordinate { x: 2, y: 2 }), Some(&'I'));
        assert_eq!(grid.get(Coordinate { x: 3, y: 3 }), None);
    }

    struct Number;
    impl TryFrom<char> for Number {
        type Error = char;
        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                '0'..='9' => Ok(Number),
                _ => Err(c),
            }
        }
    }

    #[test]
    fn from_str_error() {
        let grid = Grid::<Number>::from_str("123ABC");
        match grid {
            Ok(_) => panic!("Expected error"),
            Err('A') => {}
            Err(c) => panic!("Expected error with 'A', got '{}'", c),
        }
    }
}
