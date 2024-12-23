//! Day 22: Monkey Market
//!
//! https://adventofcode.com/2024/day/22

use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use tracing::{debug, instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> u64 {
    input
        .parse::<SecretNumbers>()
        .unwrap()
        .numbers
        .into_iter()
        .map(|n| n.evolve_n(2000).0 as u64)
        .sum()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> u64 {
    let secret_numbers: SecretNumbers = input.parse().unwrap();
    secret_numbers.find_best_price_change(2000)
}

fn main() {
    solutions::main(part1, part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SecretNumber(u32);

impl SecretNumber {
    fn evolve(self) -> Self {
        // mix: XOR with the secret number
        // prune: modulo by 16777216 (2^24) (same as bitwise AND with 0xFFFFFF but rustc does this automatically)
        // multiply by 64, mix, prune
        let mut x = self.0 as u64;
        x = (x ^ (x * 64)) % 16777216;

        // divide by 32, mix, prune
        x = (x ^ (x / 32)) % 16777216;

        // multiply by 2048, mix, prune
        x = (x ^ (x * 2048)) % 16777216;
        Self(x as u32)
    }

    /// Gets the secret number after evolving `snteps` times.
    fn evolve_n(self, n: usize) -> Self {
        self.evolutions(n).last().unwrap()
    }

    /// Gets the first `n` secret numbers, including the original.
    fn numbers(self, n: usize) -> impl Iterator<Item = SecretNumber> {
        std::iter::successors(Some(self), |&x| Some(x.evolve())).take(n)
    }

    /// Gets the first `n` evolutions of the secret number. This does not include the original number.
    fn evolutions(&self, n: usize) -> impl Iterator<Item = SecretNumber> {
        self.numbers(n + 1).skip(1)
    }

    /// Gets the ones digit of the secret number.
    fn price(&self) -> i8 {
        (self.0 % 10) as i8
    }

    /// Gets the price change sequences of the secret number evolutions, along with the number of bananas.
    fn price_changes(&self, n: usize) -> impl Iterator<Item = ([i8; 4], i8)> {
        self.numbers(n).tuple_windows().map(|(a, b, c, d, e)| {
            let changes = [
                (b.price() - a.price()),
                (c.price() - b.price()),
                (d.price() - c.price()),
                (e.price() - d.price()),
            ];

            (changes, e.price())
        })
    }

    /// Evaluates a sequence of price changes and returns the number of bananas that would be sold with it, or [`None`]
    /// if no bananas would be sold.
    fn evaluate(&self, n: usize, price_changes: &[i8; 4]) -> Option<i8> {
        self.price_changes(n)
            .find(|(changes, _)| changes == price_changes)
            .map(|(_, price)| price)
    }
}

impl FromStr for SecretNumber {
    type Err = <u32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl fmt::Display for SecretNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.price())
    }
}

struct SecretNumbers {
    numbers: Vec<SecretNumber>,
}

impl SecretNumbers {
    /// Finds the best price change sequence that would result in the most bananas sold across all secret numbers.
    /// Returns the number of bananas sold.
    fn find_best_price_change(&self, n: usize) -> u64 {
        let price_changes = self
            .numbers
            .iter()
            .map(|&sn| sn.price_changes(n))
            .flatten()
            .map(|(changes, _)| changes)
            .collect::<HashSet<_>>();

        price_changes
            .iter()
            .enumerate()
            .map(|(i, pc)| {
                trace!(
                    "Evaluating price change sequence {} of {}",
                    i + 1,
                    price_changes.len()
                );

                self.evaluate(n, pc)
            })
            .max()
            .unwrap_or_default()
    }

    fn evaluate(&self, n: usize, price_changes: &[i8; 4]) -> u64 {
        self.numbers
            .iter()
            .filter_map(|&sn| sn.evaluate(n, price_changes))
            .map(|x| x as u64)
            .sum()
    }
}

impl FromStr for SecretNumbers {
    type Err = <u32 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .lines()
                .map(SecretNumber::from_str)
                .map(Result::unwrap)
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/22/1.txt");
        assert_eq!(37327623, part1(input));
    }

    #[test]
    fn part2_example() {
        solutions::setup_tracing();
        let input = include_str!("../../data/examples/2024/22/2.txt");
        assert_eq!(23, part2(input));
    }

    #[test]
    fn evolve() {
        assert_eq!(8685429, SecretNumber(1).evolve_n(2000).0);
        assert_eq!(4700978, SecretNumber(10).evolve_n(2000).0);
        assert_eq!(15273692, SecretNumber(100).evolve_n(2000).0);
        assert_eq!(8667524, SecretNumber(2024).evolve_n(2000).0);
    }

    #[test]
    fn price() {
        assert_eq!(3, SecretNumber(123).price());
        assert_eq!(0, SecretNumber(15887950).price());
        assert_eq!(6, SecretNumber(16495136).price());
    }

    #[test]
    fn evaluate() {
        assert_eq!(Some(6), SecretNumber(123).evaluate(10, &[-1, -1, 0, 2]));
        assert_eq!(Some(7), SecretNumber(1).evaluate(2000, &[-2, 1, -1, 3]));
        assert_eq!(Some(7), SecretNumber(2).evaluate(2000, &[-2, 1, -1, 3]));
        assert_eq!(None, SecretNumber(3).evaluate(2000, &[-2, 1, -1, 3]));
        assert_eq!(Some(9), SecretNumber(2024).evaluate(2000, &[-2, 1, -1, 3]));
    }
}
