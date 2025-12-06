//! Day 6: Trash Compactor
//!
//! https://adventofcode.com/2025/day/6

use core::str::FromStr;
use itertools::Itertools;
use solutions::iter::IteratorExt;
use tracing::{instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let worksheet: WorksheetPart1 = input.parse().unwrap();

    // the result is the operation applied to the numbers in the columns
    worksheet
        .symbols
        .iter()
        .enumerate()
        .map(|(col, op)| {
            let iter = worksheet.numbers.iter().map(|row| &row[col]);
            match op {
                Op::Add => iter.sum::<usize>(),
                Op::Mul => iter.product(),
            }
        })
        .sum()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let worksheet: WorksheetPart2 = input.parse().unwrap();

    // for each symbol, the numbers to operate on are columnar, with the most significant digit at the top
    let mut col = 0;
    let mut sum = 0;
    while col < worksheet.symbols.len() {
        let op: Op = worksheet.symbols[col].try_into().unwrap();

        let mut result = op.identity();

        // col points to the first number of this operation
        // form the number by iterating the rows from the top
        // when we encounter whitespace, we've reached the end of the numbers for this op
        let mut first = true;
        while first
            || worksheet
                .symbols
                .get(col)
                .is_some_and(|c| c.is_whitespace())
        {
            trace!(col, "parsing number from column");
            first = false;
            let number = match worksheet.get_number(col) {
                Some(n) => n as usize,
                None => {
                    // the number was empty, so we've reached the end of the numbers for this op
                    trace!(col, "reached end of numbers for operation");
                    break;
                }
            };

            trace!(number, col, "parsed number from column");

            result = match op {
                Op::Add => result + number,
                Op::Mul => result * number,
            };

            col += 1;
        }

        sum += result;

        // skip a column of just whitespace between operations
        col += 1;
    }

    sum
}

fn main() {
    solutions::main(part1, part2);
}

struct WorksheetPart1 {
    numbers: Vec<Vec<usize>>,
    symbols: Vec<Op>,
}

impl FromStr for WorksheetPart1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // everything but the last line are whitespace-delimited integers
        // the last line are symbols
        let numbers = s
            .lines()
            .take_while(|line| {
                line.trim_start()
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_ascii_digit())
            }) // stop at the first line that doesn't start with a number
            .map(|line| line.split_whitespace().map(str::parse).try_collect())
            .try_collect()
            .map_err(|_| ())?;

        let symbols = s
            .trim()
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .map(Op::try_from)
            .try_collect()?;

        Ok(Self { numbers, symbols })
    }
}

struct WorksheetPart2 {
    /// The numbers with their whitespace preserved.
    numbers: Vec<Vec<Number>>,

    /// The symbols with their whitespace preserved.
    symbols: Vec<char>,
}

impl WorksheetPart2 {
    fn get_number(&self, col: usize) -> Option<u16> {
        let n = self
            .numbers
            .iter()
            .flat_map(|row| row.get(col))
            .filter_map(|c| c.digit())
            .map(|n| n as u16)
            .collect_num(10);

        if n == 0 { None } else { Some(n) }
    }
}

impl FromStr for WorksheetPart2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // everything but the last line are whitespace-delimited integers
        // the last line are symbols
        let numbers = s
            .lines()
            .take_while(|line| {
                line.trim_start()
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_ascii_digit())
            }) // stop at the first line that doesn't start with a number
            .map(|line| line.chars().map(Number::try_from).try_collect())
            .try_collect()?;

        let symbols = s.lines().last().unwrap().chars().collect();

        Ok(Self { numbers, symbols })
    }
}

/// A parsed character that is either a digit or whitespace.
#[derive(Debug, Clone, Copy)]
enum Number {
    Digit(u8),
    Whitespace,
}

impl Number {
    /// [`Some`] if the character is a digit, otherwise [`None`].
    fn digit(&self) -> Option<u8> {
        match self {
            Self::Digit(n) => Some(*n),
            Self::Whitespace => None,
        }
    }
}

impl TryFrom<char> for Number {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_digit(10) {
            Some(n) => Ok(Self::Digit(n as u8)),
            None => {
                if value.is_whitespace() {
                    Ok(Self::Whitespace)
                } else {
                    Err(())
                }
            }
        }
    }
}

enum Op {
    Add,
    Mul,
}

impl Op {
    /// The identity element of the operation.
    ///
    /// Performing the operation on this value and any other value should return the other value.
    fn identity(&self) -> usize {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }
}

impl TryFrom<char> for Op {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use solutions::setup_tracing;

    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2025/06/1.txt");
        assert_eq!(4277556, part1(input));
    }

    #[test]
    fn part2_example() {
        setup_tracing();
        let input = include_str!("../../data/examples/2025/06/1.txt");
        assert_eq!(3263827, part2(input));
    }
}
