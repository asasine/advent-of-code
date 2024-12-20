//! Day 11: Plutonian Pebbles
//!
//! https://adventofcode.com/2024/day/11

use std::{collections::HashMap, hash::Hash, str::FromStr};

use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let mut stones = Stones::from_str(input).unwrap();
    stones.blink_for(25)
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let mut stones = Stones::from_str(input).unwrap();
    stones.blink_for(75)
}

fn main() {
    solutions::main(part1, part2)
}

/// Returns the two parts of a number split in the middle if it has an even number of digits, or [`None`] if it has odd digits.
///
/// The first part is the left half, the second part is the right half.
/// Any leading zeros in the second part are stripped.
fn split_number(n: u64) -> Option<(u64, u64)> {
    let d = solutions::num::u64::digits(n);
    if d % 2 == 0 {
        let b = 10u64.pow(d / 2);
        Some((n / b, n % b))
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone(u64);

impl Stone {
    /// Blink, causing the stone to change according to the first applicable rule:
    ///
    /// 1. `0` becomes `1`.
    /// 2. An even number of digits is split into two parts.
    /// 3. The number is multiplied with `2024`.
    fn blink(&self) -> (Stone, Option<Stone>) {
        if self.0 == 0 {
            (Stone(1), None)
        } else if let Some((a, b)) = split_number(self.0) {
            (Stone(a), Some(Stone(b)))
        } else {
            (Stone(self.0 * 2024), None)
        }
    }

    /// Blink `n` times, returning the number of stones after `n` blinks.
    ///
    /// The result is memoized to avoid recomputing the same state.
    fn blink_for(&self, n: usize, memo: &mut HashMap<(Stone, usize), usize>) -> usize {
        if n == 0 {
            return 1;
        }

        if let Some(count) = memo.get(&(*self, n)) {
            *count
        } else {
            let (new_stone, new_split) = self.blink();
            let count = new_stone.blink_for(n - 1, memo)
                + new_split.map_or(0, |s| s.blink_for(n - 1, memo));

            memo.insert((*self, n), count);
            count
        }
    }
}

impl From<&u64> for Stone {
    fn from(value: &u64) -> Self {
        Self(*value)
    }
}

struct Stones {
    stones: Vec<Stone>,
}

impl Stones {
    fn blink_for(&mut self, n: usize) -> usize {
        let mut cache = HashMap::new();
        self.stones
            .iter()
            .map(|s| s.blink_for(n, &mut cache))
            .sum::<usize>()
    }
}

impl FromStr for Stones {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            stones: s
                .split_whitespace()
                .map(|s| Stone(s.parse().unwrap()))
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(55312, part1("125 17"));
    }

    #[test]
    fn part2_example() {
        assert_eq!(65601038650482, part2("125 17"));
    }

    #[test]
    fn split_number() {
        assert_eq!(None, super::split_number(0));
        assert_eq!(None, super::split_number(1));
        assert_eq!(None, super::split_number(01));
        assert_eq!(Some((1, 0)), super::split_number(10));
        assert_eq!(Some((12, 34)), super::split_number(1234));
        assert_eq!(None, super::split_number(12345));
        assert_eq!(Some((123, 456)), super::split_number(123456));
        assert_eq!(Some((11, 0)), super::split_number(1100));
    }

    #[test]
    fn blink() {
        // initial: 125 17
        // after 1 blink: 253000 1 7
        assert_eq!((Stone(253000), None), Stone(125).blink());
        assert_eq!((Stone(1), Some(Stone(7))), Stone(17).blink());

        // after 2 blinks: 253 0 2024 14168
        assert_eq!((Stone(253), Some(Stone(0))), Stone(253000).blink());
        assert_eq!((Stone(2024), None), Stone(1).blink());
        assert_eq!((Stone(14168), None), Stone(7).blink());

        // after 3 blinks: 512072 1 20 24 28676032
        assert_eq!((Stone(512072), None), Stone(253).blink());
        assert_eq!((Stone(1), None), Stone(0).blink());
        assert_eq!((Stone(20), Some(Stone(24))), Stone(2024).blink());
        assert_eq!((Stone(28676032), None), Stone(14168).blink());

        // after 4 blinks: 512 72 2024 2 0 2 4 2867 6032
        assert_eq!((Stone(512), Some(Stone(72))), Stone(512072).blink());
        assert_eq!((Stone(2024), None), Stone(1).blink());
        assert_eq!((Stone(2), Some(Stone(0))), Stone(20).blink());
        assert_eq!((Stone(2), Some(Stone(4))), Stone(24).blink());
        assert_eq!((Stone(2867), Some(Stone(6032))), Stone(28676032).blink());

        // after 5 blinks: 1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32
        assert_eq!((Stone(1036288), None), Stone(512).blink());
        assert_eq!((Stone(7), Some(Stone(2))), Stone(72).blink());
        assert_eq!((Stone(20), Some(Stone(24))), Stone(2024).blink());
        assert_eq!((Stone(4048), None), Stone(2).blink());
        assert_eq!((Stone(1), None), Stone(0).blink());
        assert_eq!((Stone(4048), None), Stone(2).blink());
        assert_eq!((Stone(8096), None), Stone(4).blink());
        assert_eq!((Stone(28), Some(Stone(67))), Stone(2867).blink());
        assert_eq!((Stone(60), Some(Stone(32))), Stone(6032).blink());

        // after 6 blinks: 2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
        assert_eq!((Stone(2097446912), None), Stone(1036288).blink());
        assert_eq!((Stone(14168), None), Stone(7).blink());
        assert_eq!((Stone(4048), None), Stone(2).blink());
        assert_eq!((Stone(2), Some(Stone(0))), Stone(20).blink());
        assert_eq!((Stone(2), Some(Stone(4))), Stone(24).blink());
        assert_eq!((Stone(40), Some(Stone(48))), Stone(4048).blink());
        assert_eq!((Stone(2024), None), Stone(1).blink());
        assert_eq!((Stone(40), Some(Stone(48))), Stone(4048).blink());
        assert_eq!((Stone(80), Some(Stone(96))), Stone(8096).blink());
        assert_eq!((Stone(2), Some(Stone(8))), Stone(28).blink());
        assert_eq!((Stone(6), Some(Stone(7))), Stone(67).blink());
        assert_eq!((Stone(6), Some(Stone(0))), Stone(60).blink());
        assert_eq!((Stone(3), Some(Stone(2))), Stone(32).blink());
    }
}
