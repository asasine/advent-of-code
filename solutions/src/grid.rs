//! Utilities for working with grids of things.

use core::fmt;
use std::str::FromStr;

/// A coordinate on a 2D grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    /// Try and move a coordinate in a direction.
    pub fn try_move(&self, other: Direction) -> Option<Self> {
        match other {
            Direction::Up => self.y.checked_sub(1).map(|y| Self { x: self.x, y }),
            Direction::Right => Some(Self {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::Down => Some(Self {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::Left => self.x.checked_sub(1).map(|x| Self { x, y: self.y }),
        }
    }

    /// Try and move a coordinate in a direction, staying within the rectangle.
    pub fn try_move_within(&self, other: Direction, extents: Rectangle) -> Option<Self> {
        self.try_move(other)
            .and_then(|c| if extents.contains(c) { Some(c) } else { None })
    }

    /// Get the von Neumann neighborhood of a coordinate. The returned array starts with [`Direction::Up`] and goes clockwise.
    ///
    /// The von Neumann neighborhood is the four cells adjacent to a cell along the cardinal directions.
    ///
    /// See [`Direction`] for the directions.
    pub fn von_neumann(&self) -> [Option<Coordinate>; 4] {
        [
            self.try_move(Direction::Up),
            self.try_move(Direction::Right),
            self.try_move(Direction::Down),
            self.try_move(Direction::Left),
        ]
    }

    /// Get the von Neumann neighborhood of a coordinate, staying within the rectangle.
    /// The returned array starts with [`Direction::Up`] and goes clockwise.
    ///
    /// See [`Self::von_neumann`] for a version that does not stay within a rectangular.
    pub fn von_neumann_within(&self, extents: Rectangle) -> [Option<Coordinate>; 4] {
        [
            self.try_move_within(Direction::Up, extents),
            self.try_move_within(Direction::Right, extents),
            self.try_move_within(Direction::Down, extents),
            self.try_move_within(Direction::Left, extents),
        ]
    }

    /// Get the Moore neighborhood of a coordinate. The returned array starts with [`Direction::Up`] and goes clockwise.
    ///
    /// The Moore neighborhood is the eight cells adjacent to a cell along the cardinal and diagonal directions.
    pub fn moore(&self) -> [Option<Coordinate>; 8] {
        [
            self.try_move(Direction::Up),
            self.try_move(Direction::Up)
                .and_then(|c| c.try_move(Direction::Right)),
            self.try_move(Direction::Right),
            self.try_move(Direction::Right)
                .and_then(|c| c.try_move(Direction::Down)),
            self.try_move(Direction::Down),
            self.try_move(Direction::Down)
                .and_then(|c| c.try_move(Direction::Left)),
            self.try_move(Direction::Left),
            self.try_move(Direction::Left)
                .and_then(|c| c.try_move(Direction::Up)),
        ]
    }

    /// Get the Moore neighborhood of a coordinate, staying within the rectangle.
    /// The returned array starts with [`Direction::Up`] and goes clockwise.
    ///
    /// See [`Self::moore`] for a version that does not stay within a rectangular.
    pub fn moore_within(&self, extents: Rectangle) -> [Option<Coordinate>; 8] {
        [
            self.try_move_within(Direction::Up, extents),
            self.try_move_within(Direction::Up, extents)
                .and_then(|c| c.try_move_within(Direction::Right, extents)),
            self.try_move_within(Direction::Right, extents),
            self.try_move_within(Direction::Right, extents)
                .and_then(|c| c.try_move_within(Direction::Down, extents)),
            self.try_move_within(Direction::Down, extents),
            self.try_move_within(Direction::Down, extents)
                .and_then(|c| c.try_move_within(Direction::Left, extents)),
            self.try_move_within(Direction::Left, extents),
            self.try_move_within(Direction::Left, extents)
                .and_then(|c| c.try_move_within(Direction::Up, extents)),
        ]
    }

    /// The Manhattan distance between two coordinates.
    pub fn manhattan(&self, other: Coordinate) -> usize {
        (self.x as isize - other.x as isize).unsigned_abs()
            + (self.y as isize - other.y as isize).unsigned_abs()
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A rectangle defined by two coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rectangle {
    pub min: Coordinate,
    pub max: Coordinate,
}

impl Rectangle {
    /// Check if a coordinate is within the rectangle.
    pub fn contains(&self, c: Coordinate) -> bool {
        c.x >= self.min.x && c.x <= self.max.x && c.y >= self.min.y && c.y <= self.max.y
    }

    /// The width of the rectangle.
    pub fn width(&self) -> usize {
        self.max.x - self.min.x + 1
    }

    /// The height of the rectangle.
    pub fn height(&self) -> usize {
        self.max.y - self.min.y + 1
    }
}

impl IntoIterator for Rectangle {
    type Item = Coordinate;
    type IntoIter = RectIterator;

    fn into_iter(self) -> Self::IntoIter {
        RectIterator {
            rect: self,
            current: Some(self.min),
        }
    }
}

pub struct RectIterator {
    rect: Rectangle,
    current: Option<Coordinate>,
}

impl Iterator for RectIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.current {
            None => None,
            Some(current) => current
                .try_move_within(Direction::Right, self.rect)
                .or_else(|| {
                    let next = Coordinate {
                        x: self.rect.min.x,
                        y: current.y + 1,
                    };

                    if self.rect.contains(next) {
                        Some(next)
                    } else {
                        None
                    }
                }),
        };

        std::mem::replace(&mut self.current, next)
    }
}

/// The four cardinal directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Up, or an increase in the y coordinate.
    Up,

    /// Right, or an increase in the x coordinate.
    Right,

    /// Down, or a decrease in the y coordinate.
    Down,

    /// Left, or a decrease in the x coordinate.
    Left,
}

impl Direction {
    /// Turn right from the current direction.
    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    /// Turn left from the current direction.
    pub fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    /// Reverse the current direction.
    pub fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    pub fn is_horizontal(self) -> bool {
        matches!(self, Direction::Right | Direction::Left)
    }

    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "v",
            Direction::Left => "<",
        };

        write!(f, "{}", s)
    }
}

/// A grid of cells.
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

    /// Get a refrerence to the cell at the given coordinate, or [`None`] if the coordinate is out of bounds.
    pub fn get(&self, c: Coordinate) -> Option<&T> {
        self.cells.get(c.y).and_then(|row| row.get(c.x))
    }

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
    pub fn enumerate(&self) -> GridEnumerateIterator<T> {
        GridEnumerateIterator {
            grid: self,
            rect_iter: self.extent.into_iter(),
        }
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> fmt::Debug for Grid<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("cells", &self.cells)
            .field("extent", &self.extent)
            .finish()
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            cells: self.cells.clone(),
            extent: self.extent,
        }
    }
}

pub struct GridEnumerateIterator<'a, T> {
    grid: &'a Grid<T>,
    rect_iter: RectIterator,
}

impl<'a, T> Iterator for GridEnumerateIterator<'a, T> {
    type Item = (Coordinate, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.rect_iter
            .next()
            .and_then(|c| self.grid.get(c).map(|t| (c, t)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod coordinate {
        use super::*;

        #[test]
        fn test_try_move() {
            let c = Coordinate { x: 0, y: 0 };

            assert_eq!(c.try_move(Direction::Up), None);
            assert_eq!(
                c.try_move(Direction::Right),
                Some(Coordinate { x: 1, y: 0 })
            );
            assert_eq!(c.try_move(Direction::Down), Some(Coordinate { x: 0, y: 1 }));
            assert_eq!(c.try_move(Direction::Left), None);
        }

        #[test]
        fn test_try_move_within() {
            let rect = Rectangle {
                min: Coordinate { x: 0, y: 0 },
                max: Coordinate { x: 2, y: 2 },
            };

            let c = Coordinate { x: 2, y: 2 };
            assert_eq!(
                c.try_move_within(Direction::Up, rect),
                Some(Coordinate { x: 2, y: 1 })
            );
            assert_eq!(c.try_move_within(Direction::Right, rect), None);
            assert_eq!(c.try_move_within(Direction::Down, rect), None);
            assert_eq!(
                c.try_move_within(Direction::Left, rect),
                Some(Coordinate { x: 1, y: 2 })
            );
        }

        #[test]
        fn von_neumann() {
            let c = Coordinate { x: 1, y: 1 };

            let neighbors = c.von_neumann();
            assert_eq!(neighbors[0], Some(Coordinate { x: 1, y: 0 }));
            assert_eq!(neighbors[1], Some(Coordinate { x: 2, y: 1 }));
            assert_eq!(neighbors[2], Some(Coordinate { x: 1, y: 2 }));
            assert_eq!(neighbors[3], Some(Coordinate { x: 0, y: 1 }));
        }

        #[test]
        fn von_neumann_within() {
            let rect = Rectangle {
                min: Coordinate { x: 0, y: 0 },
                max: Coordinate { x: 2, y: 2 },
            };

            let c = Coordinate { x: 2, y: 2 };
            let neighbors = c.von_neumann_within(rect);
            assert_eq!(neighbors[0], Some(Coordinate { x: 2, y: 1 }));
            assert_eq!(neighbors[1], None);
            assert_eq!(neighbors[2], None);
            assert_eq!(neighbors[3], Some(Coordinate { x: 1, y: 2 }));
        }

        #[test]
        fn moore() {
            let c = Coordinate { x: 1, y: 1 };

            let neighbors = c.moore();
            assert_eq!(neighbors[0], Some(Coordinate { x: 1, y: 0 }));
            assert_eq!(neighbors[1], Some(Coordinate { x: 2, y: 0 }));
            assert_eq!(neighbors[2], Some(Coordinate { x: 2, y: 1 }));
            assert_eq!(neighbors[3], Some(Coordinate { x: 2, y: 2 }));
            assert_eq!(neighbors[4], Some(Coordinate { x: 1, y: 2 }));
            assert_eq!(neighbors[5], Some(Coordinate { x: 0, y: 2 }));
            assert_eq!(neighbors[6], Some(Coordinate { x: 0, y: 1 }));
            assert_eq!(neighbors[7], Some(Coordinate { x: 0, y: 0 }));
        }

        #[test]
        fn moore_within() {
            let rect = Rectangle {
                min: Coordinate { x: 0, y: 0 },
                max: Coordinate { x: 2, y: 2 },
            };

            let c = Coordinate { x: 2, y: 2 };
            let neighbors = c.moore_within(rect);
            assert_eq!(neighbors[0], Some(Coordinate { x: 2, y: 1 }));
            assert_eq!(neighbors[1], None);
            assert_eq!(neighbors[2], None);
            assert_eq!(neighbors[3], None);
            assert_eq!(neighbors[4], None);
            assert_eq!(neighbors[5], None);
            assert_eq!(neighbors[6], Some(Coordinate { x: 1, y: 2 }));
            assert_eq!(neighbors[7], Some(Coordinate { x: 1, y: 1 }));
        }

        #[test]
        fn manhattan() {
            let c1 = Coordinate { x: 0, y: 0 };
            let c2 = Coordinate { x: 3, y: 4 };

            assert_eq!(c1.manhattan(c2), 7);
            assert_eq!(c2.manhattan(c1), 7);
        }
    }

    mod rectangle {
        use super::*;

        #[test]
        fn test_contains() {
            let rect = Rectangle {
                min: Coordinate { x: 0, y: 0 },
                max: Coordinate { x: 2, y: 2 },
            };

            assert!(rect.contains(Coordinate { x: 0, y: 0 }));
            assert!(rect.contains(Coordinate { x: 2, y: 2 }));
            assert!(!rect.contains(Coordinate { x: 3, y: 3 }));
        }

        #[test]
        fn test_width() {
            let rect = Rectangle {
                min: Coordinate { x: 0, y: 0 },
                max: Coordinate { x: 2, y: 2 },
            };

            assert_eq!(rect.width(), 3);
        }

        #[test]
        fn test_height() {
            let rect = Rectangle {
                min: Coordinate { x: 0, y: 0 },
                max: Coordinate { x: 2, y: 2 },
            };

            assert_eq!(rect.height(), 3);
        }

        #[test]
        fn test_into_iter() {
            let rect = Rectangle {
                min: Coordinate { x: 0, y: 0 },
                max: Coordinate { x: 1, y: 1 },
            };

            let mut iter = rect.into_iter();
            assert_eq!(iter.next(), Some(Coordinate { x: 0, y: 0 }));
            assert_eq!(iter.next(), Some(Coordinate { x: 1, y: 0 }));
            assert_eq!(iter.next(), Some(Coordinate { x: 0, y: 1 }));
            assert_eq!(iter.next(), Some(Coordinate { x: 1, y: 1 }));
            assert_eq!(iter.next(), None);
        }
    }

    mod direction {
        use std::collections::HashSet;

        use super::*;

        #[test]
        fn turn_right() {
            fn four_times(d: Direction) -> Direction {
                d.turn_right().turn_right().turn_right().turn_right()
            }

            assert_eq!(Direction::Up, four_times(Direction::Up));
            assert_eq!(Direction::Right, four_times(Direction::Right));
            assert_eq!(Direction::Down, four_times(Direction::Down));
            assert_eq!(Direction::Left, four_times(Direction::Left));
        }

        #[test]
        fn turn_left() {
            fn four_times(d: Direction) -> Direction {
                d.turn_left().turn_left().turn_left().turn_left()
            }

            assert_eq!(Direction::Up, four_times(Direction::Up));
            assert_eq!(Direction::Right, four_times(Direction::Right));
            assert_eq!(Direction::Down, four_times(Direction::Down));
            assert_eq!(Direction::Left, four_times(Direction::Left));
        }

        #[test]
        fn reverse() {
            fn two_times(d: Direction) -> Direction {
                d.reverse().reverse()
            }

            assert_eq!(Direction::Up, two_times(Direction::Up));
            assert_eq!(Direction::Right, two_times(Direction::Right));
            assert_eq!(Direction::Down, two_times(Direction::Down));
            assert_eq!(Direction::Left, two_times(Direction::Left));
        }

        #[test]
        fn is_vertical() {
            assert!(Direction::Up.is_vertical());
            assert!(Direction::Down.is_vertical());
            assert!(!Direction::Right.is_vertical());
            assert!(!Direction::Left.is_vertical());
        }

        #[test]
        fn is_horizontal() {
            assert!(!Direction::Up.is_horizontal());
            assert!(!Direction::Down.is_horizontal());
            assert!(Direction::Right.is_horizontal());
            assert!(Direction::Left.is_horizontal());
        }

        #[test]
        fn all() {
            let all = Direction::all().into_iter().collect::<HashSet<_>>();
            assert_eq!(all.len(), 4);
        }
    }

    mod grid {
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
}
