//! Day 3: Lobby
//!
//! https://adventofcode.com/2025/day/3

use core::{fmt::Display, str::FromStr};
use itertools::Itertools;
use solutions::iter::IteratorExt;
use tracing::{instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let banks: Banks = input.parse().unwrap();
    banks.total_output_joltage::<2>() as usize
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let banks: Banks = input.parse().unwrap();
    banks.total_output_joltage::<12>() as usize
}

fn main() {
    solutions::main(part1, part2);
}

/// The [joltage](https://adventofcode.com/2020/day/10) rating of a [`Bank`] is the number formed by the `N`
/// [`Battery`] that are turned on.
#[derive(Clone, Copy)]
struct Joltage<'a, const N: usize> {
    /// The bank of batteries this joltage rating belongs to.
    bank: &'a Bank,

    /// The battery indices that are turned on in [`Self::bank`].
    batteries: [usize; N],
}

impl<'a, const N: usize> Joltage<'a, N> {
    /// Find the max joltage of size `N` from the provided [`Bank`].
    ///
    /// The max joltage is the highest possible rating that can be achieved by turning on exactly `N` batteries.
    fn max_joltage(bank: &'a Bank) -> Self {
        // the joltage must be N digits long, so the first number must be before the last (N-1) digits
        // every iteration thereafter, the range starts from the index after the last selected digit, up to (N-i-1)
        let mut range = 0..(bank.0.len() - (N - 1));
        let batteries = core::array::from_fn(|_| {
            // find the highest number for the digit that's in the [range]
            let index = bank.0[range.clone()].iter().position_first_max().unwrap() + range.start;
            range = (index + 1)..(range.end + 1);

            trace!(
                index,
                ?range,
                value = %bank.0[index],
                "selected battery index",
            );

            index
        });

        Self { bank, batteries }
    }
}

impl<const N: usize> Joltage<'_, N> {
    /// This joltage rating's value is the concatenation of all of the selected batteries' digits.
    fn rating(self) -> u64 {
        self.batteries
            .into_iter()
            .map(|i| self.bank.0[i].0 as u64)
            .collect_num(10)
    }
}

impl<const N: usize> Display for Joltage<'_, N> {
    /// Display the joltage rating as an `N`-digit number.
    ///
    /// In the alternate mode, display the rating followed by the entire bank, with the selected batteries highlighted.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0width$}", self.rating(), width = N)?;
        if f.alternate() {
            write!(f, ": ")?;
            for (i, battery) in self.bank.0.iter().enumerate() {
                if self.batteries.contains(&i) {
                    // bold and underlined
                    write!(f, "\x1b[1;4m{}\x1b[0m", battery)?;
                } else {
                    write!(f, "{}", battery)?;
                }
            }
        }

        Ok(())
    }
}

struct Banks(Vec<Bank>);

impl Banks {
    /// The total output joltage is the sum of all [`Bank::max_joltage`] for `N` batteries from each bank.
    fn total_output_joltage<const N: usize>(&self) -> u64 {
        self.0
            .iter()
            .map(Bank::max_joltage::<N>)
            .inspect(|joltage| trace!(joltage = %format_args!("{:#}", joltage)))
            .map(Joltage::rating)
            .map(u64::from)
            .sum()
    }
}

impl FromStr for Banks {
    type Err = ();

    /// Parse multiple banks from the input string, one per line.
    ///
    /// See [`Bank::from_str`] for the format of each bank.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let banks = s.trim().lines().map(Bank::from_str).try_collect()?;
        Ok(Banks(banks))
    }
}

struct Bank(Vec<Battery>);

impl Bank {
    /// The max joltage is the highest possible rating that can be achieved by turning on exactly `N` batteries.
    fn max_joltage<const N: usize>(&self) -> Joltage<'_, N> {
        Joltage::max_joltage(self)
    }
}

impl Display for Bank {
    /// Display the bank as a sequence of battery ratings.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for battery in &self.0 {
            write!(f, "{}", battery)?;
        }

        Ok(())
    }
}

impl FromStr for Bank {
    type Err = ();

    /// Parse a bank from a string of digits, each digit representing a [`Battery`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s.trim().chars().map(Battery::try_from).try_collect()?;
        Ok(Bank(batteries))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Battery(u8);

impl Battery {
    /// Try to create a new [`Battery`] with the specified rating.
    ///
    /// Returns [`Some`] for valid ratings in the range `1..=9`, [`None`] otherwise.
    fn try_new(rating: u8) -> Option<Self> {
        match rating {
            1..=9 => Some(Battery(rating)),
            _ => None,
        }
    }
}

impl TryFrom<char> for Battery {
    type Error = ();

    /// Try to convert a numeric character to a [`Battery`].
    ///
    /// Each digit is a battery with a rating of `d`. See [`Battery::try_new`] for valid ratings.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        value
            .to_digit(10)
            .map(|d| d as u8)
            .and_then(Battery::try_new)
            .ok_or(())
    }
}

impl Display for Battery {
    /// Display the battery's rating.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2025/03/1.txt");
        assert_eq!(357, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/03/1.txt");
        assert_eq!(3121910778619, part2(input));
    }

    /// The max joltage uses the first max digit for tens place, not the last.
    #[test]
    fn max_joltage_uses_first_max() {
        // selecting the first 3 gives 33, selecting the last 3 gives 31
        let bank: Bank = "3231".parse().unwrap();
        let joltage = bank.max_joltage::<2>();
        let expected = 33;
        assert_eq!(
            joltage.rating(),
            expected,
            "Expected max joltage to be {} but got {:#}",
            expected,
            joltage
        );
    }
}
