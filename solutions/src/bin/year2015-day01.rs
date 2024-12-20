//! Day 1: Not Quite LisP
//!
//! https://adventofcode.com/2015/day/1

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> isize {
    input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => floor,
    })
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    input
        .chars()
        .fold_while(Acc::default(), |acc, c| {
            Continue(match c {
                '(' => acc.up(),
                ')' => {
                    let acc = acc.down();
                    if acc.floor < 0 {
                        return Done(acc);
                    } else {
                        acc
                    }
                }
                _ => acc.noop(),
            })
        })
        .into_inner()
        .pos
}

#[derive(Debug, Default)]
struct Acc {
    floor: isize,
    pos: usize,
}

impl Acc {
    fn up(self) -> Self {
        Acc {
            floor: self.floor + 1,
            pos: self.pos + 1,
        }
    }

    fn down(self) -> Self {
        Acc {
            floor: self.floor - 1,
            pos: self.pos + 1,
        }
    }

    fn noop(self) -> Self {
        Acc {
            floor: self.floor,
            pos: self.pos + 1,
        }
    }
}

fn main() {
    solutions::main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(0, part1("(())"));
        assert_eq!(0, part1("()()"));
        assert_eq!(3, part1("((("));
        assert_eq!(3, part1("(()(()("));
        assert_eq!(3, part1("))((((("));
        assert_eq!(-1, part1("())"));
        assert_eq!(-1, part1("))("));
        assert_eq!(-3, part1(")))"));
        assert_eq!(-3, part1(")())())"));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(1, part2(")"));
        assert_eq!(5, part2("()())"));
    }
}
