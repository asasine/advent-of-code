//! Day 1: Secret Entrance
//!
//! https://adventofcode.com/2025/day/1

use core::str::FromStr;
use itertools::Itertools;
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let rotations: Rotations = input.parse().unwrap();
    let mut dial = Dial::new();
    let mut count_zero = 0;
    for rotation in rotations.0 {
        dial.rotate(rotation);
        if dial.0 == 0 {
            count_zero += 1;
        }
    }

    count_zero
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let rotations: Rotations = input.parse().unwrap();
    let mut dial = Dial::new();
    let mut count_zero = 0;
    for rotation in rotations.0 {
        count_zero += dial.rotate(rotation);
    }

    count_zero
}

fn main() {
    solutions::main(part1, part2);
}

struct Dial(u32);

enum Rotation {
    Right(u32),
    Left(u32),
}

impl Rotation {
    const fn amount(&self) -> u32 {
        match self {
            Rotation::Right(amount) => *amount,
            Rotation::Left(amount) => *amount,
        }
    }
}

#[derive(Debug)]
enum RotationParseError {
    TooShort,
    InvalidCount,
    InvalidDirection,
}

impl FromStr for Rotation {
    type Err = RotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_at_checked(1).ok_or(RotationParseError::TooShort)?;
        let amount = amount
            .parse::<u32>()
            .map_err(|_| RotationParseError::InvalidCount)?;

        match direction {
            "R" => Ok(Rotation::Right(amount)),
            "L" => Ok(Rotation::Left(amount)),
            _ => Err(RotationParseError::InvalidDirection),
        }
    }
}

impl core::fmt::Display for Rotation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Rotation::Right(amount) => write!(f, "R{}", amount),
            Rotation::Left(amount) => write!(f, "L{}", amount),
        }
    }
}

struct Rotations(Vec<Rotation>);

impl FromStr for Rotations {
    type Err = RotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rotations(s.lines().map(str::parse).try_collect()?))
    }
}

impl Dial {
    const fn new() -> Self {
        Self(50)
    }

    /// Rotate the dial with the given `rotation`.
    ///
    /// Returns the number of times the dial passed zero during the rotation.
    const fn rotate(&mut self, rotation: Rotation) -> usize {
        let mut count_zero_passes = 0;
        let mut amount = rotation.amount();
        while amount > 0 {
            self.0 = match rotation {
                Rotation::Right(_) => {
                    if self.0 == 99 {
                        count_zero_passes += 1;
                        0
                    } else {
                        self.0 + 1
                    }
                }
                Rotation::Left(_) => {
                    if self.0 == 0 {
                        99
                    } else {
                        if self.0 == 1 {
                            count_zero_passes += 1;
                        }

                        self.0 - 1
                    }
                }
            };

            amount -= 1;
        }

        count_zero_passes as usize
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2025/01/1.txt");
        assert_eq!(3, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/01/1.txt");
        assert_eq!(6, part2(input));
    }

    macro_rules! assert_dial {
        ($dial:expr, $rotation:expr, $expected_value:expr, $expected_zero_passes:expr) => {
            assert_eq!(
                $dial.rotate($rotation),
                $expected_zero_passes,
                "Rotation of {} should have passed zero {} times",
                $rotation,
                $expected_zero_passes
            );
            assert_eq!($dial.0, $expected_value);
        };
    }

    #[test]
    fn rotate() {
        let mut dial = Dial::new();
        assert_dial!(dial, Rotation::Left(68), 82, 1);
        assert_dial!(dial, Rotation::Left(30), 52, 0);
        assert_dial!(dial, Rotation::Right(48), 0, 1);
        assert_dial!(dial, Rotation::Left(5), 95, 0);
        assert_dial!(dial, Rotation::Right(60), 55, 1);
        assert_dial!(dial, Rotation::Left(55), 0, 1);
        assert_dial!(dial, Rotation::Left(1), 99, 0);
        assert_dial!(dial, Rotation::Left(99), 0, 1);
        assert_dial!(dial, Rotation::Right(14), 14, 0);
        assert_dial!(dial, Rotation::Left(82), 32, 1);
    }
}
