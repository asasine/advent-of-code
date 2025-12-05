//! Day 5: Cafeteria
//!
//! https://adventofcode.com/2025/day/5

use core::{fmt::Display, num::ParseIntError, ops::Deref, str::FromStr};
use std::collections::HashSet;

use itertools::Itertools;
use tracing::{instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let database: Database = input.parse().unwrap();
    database.count_available_fresh()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let (fresh_ranges, _) = Database::split_input(input);
    let fresh_ranges = fresh_ranges.parse::<FreshRanges>().unwrap().merge();
    fresh_ranges.count_fresh_ids()
}

fn main() {
    solutions::main(part1, part2);
}

struct Database {
    fresh_ranges: FreshRanges,
    available: Available,
}

impl Database {
    fn split_input(input: &str) -> (&str, &str) {
        let delimiter = if input.contains("\r\n") {
            "\r\n\r\n"
        } else {
            "\n\n"
        };

        input.split_once(delimiter).unwrap()
    }

    /// Count the number of available fresh items.
    fn count_available_fresh(&self) -> usize {
        self.available
            .iter()
            .filter(|item| self.fresh_ranges.iter().any(|range| range.contains(item)))
            .count()
    }
}

impl FromStr for Database {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fresh_ranges, available) = Database::split_input(s);
        Ok(Database {
            fresh_ranges: fresh_ranges.parse()?,
            available: available.parse()?,
        })
    }
}

struct FreshRanges(Vec<Range>);

impl FreshRanges {
    /// Clone and merge and overlapping ranges.
    fn merge(mut self) -> Self {
        self.0.sort_by_key(|range| *range.start());
        let ranges = self
            .clone()
            .into_iter()
            .coalesce(|previous, current| previous.merge(current))
            .collect_vec();

        Self(ranges)
    }

    /// Count the total number of fresh IDs in all ranges.
    ///
    /// This assumes that the ranges are non-overlapping. Use [`merge()`][Self::merge] first.
    fn count_fresh_ids(&self) -> usize {
        self.iter().map(Range::len).sum()
    }
}

impl Deref for FreshRanges {
    type Target = Vec<Range>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for FreshRanges {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s.lines().map(str::parse).try_collect()?;
        Ok(FreshRanges(ranges))
    }
}

struct Available(HashSet<u64>);

impl Deref for Available {
    type Target = HashSet<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Available {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.lines().map(str::parse).try_collect()?;
        Ok(Available(items))
    }
}

#[derive(Debug)]
enum ParseRangeError {
    InvalidFormat,
    InvalidNumber,
}

impl From<ParseIntError> for ParseRangeError {
    fn from(_: ParseIntError) -> Self {
        ParseRangeError::InvalidNumber
    }
}

#[derive(Clone)]
struct Range(core::ops::RangeInclusive<u64>);

impl Range {
    /// Merge this range with `other` if they overlap.
    ///
    /// Returns [`Ok`] with the merged ranges if they overlap, or [`Err`] with the two original ranges if they do not.
    fn merge(self, other: Range) -> Result<Range, (Range, Range)> {
        if self.end() >= other.start() && other.end() >= self.start() {
            let start = *self.start().min(other.start());
            let end = *self.end().max(other.end());
            let new = Range(start..=end);
            trace!(%self, %other, %new, "merged ranges");
            Ok(new)
        } else {
            Err((self, other))
        }
    }

    fn len(&self) -> usize {
        (*self.end() - *self.start() + 1) as usize
    }
}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(ParseRangeError::InvalidFormat)?;
        let start = start.trim().parse()?;
        let end = end.trim().parse()?;
        Ok(Range(start..=end))
    }
}

impl Deref for Range {
    type Target = core::ops::RangeInclusive<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start(), self.end())
    }
}

#[cfg(test)]
mod tests {
    use solutions::setup_tracing;

    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2025/05/1.txt");
        assert_eq!(3, part1(input));
    }

    #[test]
    fn part2_example() {
        setup_tracing();
        let input = include_str!("../../data/examples/2025/05/1.txt");
        assert_eq!(14, part2(input));
    }

    #[test]
    fn range_len() {
        let range: Range = "10-20".parse().unwrap();
        assert_eq!(11, range.len());
    }
}
