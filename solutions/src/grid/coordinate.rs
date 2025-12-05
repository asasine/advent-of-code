use super::{Direction, Rectangle};
use core::fmt;

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
    /// See [`Self::von_neumann`] for a version that does not stay within a rectangle.
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
    /// See [`Self::moore`] for a version that does not stay within a rectangle.
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

#[cfg(test)]
mod tests {
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
