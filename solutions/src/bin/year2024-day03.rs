//! Day 3: Mull It Over
//!
//! https://adventofcode.com/2024/day/3

use regex::Regex;
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let x = cap
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            let y = cap
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            x * y
        })
        .sum::<usize>()
}

fn parse_number(cap: &regex::Captures, name: &str) -> usize {
    cap.name(name)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0)
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let re =
        Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|(?<mul>mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))")
            .unwrap();

    let mut enabled = true;
    re.captures_iter(input)
        .map(|cap| {
            let instruction = if cap.name("do").is_some() {
                Instruction::Do
            } else if cap.name("dont").is_some() {
                Instruction::Dont
            } else {
                Instruction::Mul(parse_number(&cap, "x"), parse_number(&cap, "y"))
            };

            match instruction {
                Instruction::Do => {
                    enabled = true;
                    0
                }
                Instruction::Dont => {
                    enabled = false;
                    0
                }
                Instruction::Mul(x, y) if enabled => x * y,
                _ => 0,
            }
        })
        .sum::<usize>()
}

enum Instruction {
    Do,
    Dont,
    Mul(usize, usize),
}

fn main() {
    solutions::main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/03/1.txt");
        assert_eq!(161, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/03/2.txt");
        assert_eq!(48, part2(input));
    }
}
