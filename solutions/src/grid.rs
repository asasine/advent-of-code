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
            Direction::Up => self.y.checked_sub(1).map(|y| Self { x: self.x, y: y }),
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

/// A grid of cells.
pub struct Grid<T> {
    /// The cells in the grid.
    cells: Vec<Vec<T>>,

    /// The extent of the grid.
    extent: Rectangle,
}

impl<T> FromStr for Grid<T>
where
    T: From<char>,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|line| line.chars().map(|c| T::from(c)).collect::<Vec<T>>())
            .collect::<Vec<Vec<T>>>();

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
}
