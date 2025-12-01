//! Day 25: Code Chronicle
//!
//! https://adventofcode.com/2024/day/25

use std::str::FromStr;

use itertools::Itertools;
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let keys_and_locks: KeysAndLocks = input.parse().unwrap();
    keys_and_locks
        .keys
        .iter()
        .cartesian_product(keys_and_locks.locks.iter())
        .filter(|(k, l)| k.fits(l))
        .count()
}

#[instrument(level = "debug")]
fn part2(_: &str) -> usize {
    0
}

fn main() {
    solutions::main(part1, part2);
}

struct Heights([u8; 5]);

impl FromStr for Heights {
    type Err = ();

    /// Parse a string of `#` and `.` into a `Heights` struct.
    ///
    /// The string is expected to be 6 lines of 5 characters each.
    ///
    /// The `#` character is considered occupied, and the height is the number of `#` characters in the column.
    ///
    /// # Example
    /// ```txt
    /// #####
    /// .####
    /// .####
    /// .####
    /// .#.#.
    /// .#...
    /// .....
    /// ```
    ///
    /// The above example would be parsed into a `Heights` struct with the following values:
    /// ```rust
    /// Heights([0, 5, 3, 4, 3])
    /// ```
    ///
    /// ```txt
    /// .....
    /// #....
    /// #....
    /// #...#
    /// #.#.#
    /// #.###
    /// #####
    /// ```
    ///
    /// The above example would be parsed into a `Heights` struct with the following values:
    /// ```rust
    /// Heights([5, 0, 2, 1, 3])
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = [0; 5];
        for line in s.lines() {
            for (c, height) in line.chars().zip(heights.iter_mut()) {
                if c == '#' {
                    *height += 1;
                }
            }
        }

        for height in heights.iter_mut() {
            *height -= 1;
        }

        Ok(Self(heights))
    }
}

struct Key {
    heights: Heights,
}

impl Key {
    fn fits(&self, lock: &Lock) -> bool {
        self.heights
            .0
            .iter()
            .zip(lock.heights.0.iter())
            .all(|(k, l)| k + l <= 5)
    }
}

struct Lock {
    heights: Heights,
}

struct KeysAndLocks {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

impl FromStr for KeysAndLocks {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let section_break = if s.contains("\r\n") {
            "\r\n\r\n"
        } else {
            "\n\n"
        };

        let mut keys = vec![];
        let mut locks = vec![];
        for section in s.split(section_break) {
            let heights = section.parse().unwrap();
            if section.starts_with("#") {
                locks.push(Lock { heights });
            } else {
                keys.push(Key { heights });
            }
        }

        Ok(Self { keys, locks })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/25/1.txt");
        assert_eq!(3, part1(input));
    }
}
