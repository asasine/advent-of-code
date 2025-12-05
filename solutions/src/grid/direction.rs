use core::fmt::Display;

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
    /// An array of all directions.
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

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

    /// Check if the direction is vertical.
    pub fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    /// Check if the direction is horizontal.
    pub fn is_horizontal(self) -> bool {
        matches!(self, Direction::Right | Direction::Left)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "v",
            Direction::Left => "<",
        };

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
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
        let all = Direction::ALL.into_iter().collect::<HashSet<_>>();
        assert_eq!(all.len(), 4);
    }
}
