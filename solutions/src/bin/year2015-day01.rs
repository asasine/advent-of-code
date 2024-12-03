//! Day 1: Not Quite LisP

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

/// Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh
/// out! To save Christmas, he needs you to collect *fifty stars* by December 25th.
///
/// Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar; the
/// second puzzle is unlocked when you complete the first. Each puzzle grants *one star*. Good luck!
///
/// Here's an easy puzzle to warm you up.
///
/// Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he
/// got are a little confusing. He starts on the ground floor (floor `0`) and then follows the instructions one character at
/// a time.
///
/// An opening parenthesis, `(`, means he should go up one floor, and a closing parenthesis, `)`, means he should go down
/// one floor.
///
/// The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.
///
/// For example:
///
/// * `(())` and `()()` both result in floor `0`.
/// * `(((` and `(()(()(` both result in floor `3`.
/// * `))(((((` also results in floor `3`.
/// * `())` and `))(` both result in floor `-1` (the first basement level).
/// * `)))` and `)())())` both result in floor `-3`.
///
/// To *what floor* do the instructions take Santa?
fn part1(input: &str) -> isize {
    input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => floor,
    })
}

/// Now, given the same instructions, find the *position* of the first character that causes him to enter the basement
/// (floor `-1`). The first character in the instructions has position `1`, the second character has position `2`, and so
/// on.
///
/// For example:
///
/// * `)` causes him to enter the basement at character position `1`.
/// * `()())` causes him to enter the basement at character position `5`.
///
/// What is the *position* of the character that causes Santa to first enter the basement?
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

struct Acc {
    floor: isize,
    pos: usize,
}

impl Default for Acc {
    fn default() -> Self {
        Acc { floor: 0, pos: 0 }
    }
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

aoc_macro::aoc_main!();

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
