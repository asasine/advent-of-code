//! Day 1: Trebuchet?!
//!
//! https://adventofcode.com/2023/day/1

use regex::Regex;
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // first and last digit on the line
            let mut digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap());

            let first = digits.next().expect("no digits found");
            let last = digits.next_back().unwrap_or(first);

            (first * 10 + last) as usize
        })
        .sum()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let re = Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    fn match_to_digit(m: &regex::Match) -> usize {
        match m.as_str() {
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => m.as_str().parse().unwrap(),
        }
    }

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // first and last digit on the line
            // the digit may be spelled out with letters
            // the digit may overlap with other digits if spelled out
            // there may be only a single digit
            let m = re.find(line).expect("no digits found");
            let first = match_to_digit(&m);

            // find the last match (possibly overlapping) by starting with the [1..] slice
            let last = (1..line.len())
                .filter_map(|i| {
                    let remaining = &line[i..];
                    re.find(remaining).map(|m| match_to_digit(&m))
                })
                .next_back()
                .unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

fn main() {
    solutions::main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../../data/examples/2023/01/1.txt");
        let part1 = part1(input);
        eprintln!("part1: {}", part1);
        assert_eq!(part1, 142);

        let input = include_str!("../../data/examples/2023/01/2.txt");
        let part2 = part2(input);
        eprintln!("part2: {}", part2);
        assert_eq!(part2, 281);
    }
}
