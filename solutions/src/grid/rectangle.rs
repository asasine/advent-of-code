use super::{Coordinate, Direction};

/// A rectangle defined by two coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rectangle {
    pub a: Coordinate,
    pub b: Coordinate,
}

impl Rectangle {
    /// Check if a coordinate is within the rectangle.
    pub fn contains(&self, c: Coordinate) -> bool {
        c.x >= self.a.x && c.x <= self.b.x && c.y >= self.a.y && c.y <= self.b.y
    }

    /// The width of the rectangle.
    pub fn width(&self) -> usize {
        self.b.x.abs_diff(self.a.x) + 1
    }

    /// The height of the rectangle.
    pub fn height(&self) -> usize {
        self.b.y.abs_diff(self.a.y) + 1
    }

    /// The area of the rectangle.
    pub fn area(&self) -> usize {
        self.width() * self.height()
    }
}

impl IntoIterator for Rectangle {
    type Item = Coordinate;
    type IntoIter = RectIterator;

    fn into_iter(self) -> Self::IntoIter {
        RectIterator {
            rect: self,
            current: Some(self.a),
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
                        x: self.rect.a.x,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let rect = Rectangle {
            a: Coordinate { x: 0, y: 0 },
            b: Coordinate { x: 2, y: 2 },
        };

        assert!(rect.contains(Coordinate { x: 0, y: 0 }));
        assert!(rect.contains(Coordinate { x: 2, y: 2 }));
        assert!(!rect.contains(Coordinate { x: 3, y: 3 }));
    }

    #[test]
    fn test_width() {
        let rect = Rectangle {
            a: Coordinate { x: 0, y: 0 },
            b: Coordinate { x: 2, y: 2 },
        };

        assert_eq!(rect.width(), 3);
    }

    #[test]
    fn test_height() {
        let rect = Rectangle {
            a: Coordinate { x: 0, y: 0 },
            b: Coordinate { x: 2, y: 2 },
        };

        assert_eq!(rect.height(), 3);
    }

    #[test]
    fn test_into_iter() {
        let rect = Rectangle {
            a: Coordinate { x: 0, y: 0 },
            b: Coordinate { x: 1, y: 1 },
        };

        let mut iter = rect.into_iter();
        assert_eq!(iter.next(), Some(Coordinate { x: 0, y: 0 }));
        assert_eq!(iter.next(), Some(Coordinate { x: 1, y: 0 }));
        assert_eq!(iter.next(), Some(Coordinate { x: 0, y: 1 }));
        assert_eq!(iter.next(), Some(Coordinate { x: 1, y: 1 }));
        assert_eq!(iter.next(), None);
    }
}
