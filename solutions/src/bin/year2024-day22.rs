//! Day 22: Monkey Market
//!
//! https://adventofcode.com/2024/day/22

use core::fmt;
use std::str::FromStr;

use itertools::Itertools;
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .flat_map(SecretNumber::from_str)
        .map(|sn| sn.evolve_n(2000).0 as u64)
        .sum()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> u64 {
    // There are 19^4 = 130321 possible sequences of 4 changes in the range [-9, 9].
    let mut sequence_totals = vec![0; 19 * 19 * 19 * 19];
    let mut seen_sequences = vec![0; 19 * 19 * 19 * 19];
    for (buyer, line) in input.lines().enumerate() {
        let mut secret_number = line.parse::<SecretNumber>().unwrap();
        let mut prices = [0; 2000];
        let mut deltas = [0; 2000];
        for (price, delta) in prices.iter_mut().zip(deltas.iter_mut()) {
            let next = secret_number.evolve();
            *price = next.price();
            *delta = next.price() - secret_number.price();
            secret_number = next;
        }

        let base19_indices = deltas
            .iter()
            .tuple_windows()
            .map(|(&a, &b, &c, &d)| base19(&[a, b, c, d]));

        for (i, price) in base19_indices.zip(prices.iter().skip(3)) {
            let sequence_total = &mut sequence_totals[i];
            let seen_sequence = &mut seen_sequences[i];
            if *seen_sequence != buyer {
                *seen_sequence = buyer;
                *sequence_total += *price as u64;
            }
        }
    }

    *sequence_totals.iter().max().unwrap()
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

    /// Gets the secret number after evolving `n` times.
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

fn base19(changes: &[i8; 4]) -> usize {
    let mut x = 0;
    for &c in changes {
        x = x * 19 + (c + 9) as usize;
    }

    x
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
}
