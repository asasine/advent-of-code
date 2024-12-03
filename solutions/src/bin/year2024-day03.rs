//! Day 3: Mull It Over

use regex::Regex;

/// "Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check
/// the warehouse, though," says the mildly flustered shopkeeper at the [North Pole Toboggan Rental Shop][1]. The Historians
/// head out to take a look.
///
/// The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"
///
/// The computer appears to be trying to run a program, but its memory (your puzzle input) is *corrupted*. All of the
/// instructions have been jumbled up!
///
/// It seems like the goal of the program is just to *multiply some numbers*. It does that with instructions like
/// `mul(X,Y)`, where `X` and `Y` are each 1-3 digit numbers. For instance, `mul(44,46)` multiplies `44` by `46` to get a
/// result of `2024`. Similarly, `mul(123,4)` would multiply `123` by `4`.
///
/// However, because the program's memory has been corrupted, there are also many invalid characters that should be
/// *ignored*, even if they look like part of a `mul` instruction. Sequences like `mul(4*`, `mul(6,9!`, `?(12,34)`, or `mul
/// ( 2 , 4 )` do *nothing*.
///
/// For example, consider the following section of corrupted memory:
///
/// `x*mul(2,4)*%&mul[3,7]!@^do_not_*mul(5,5)*+mul(32,64]then(*mul(11,8)mul(8,5)*)`
///
/// Only the four highlighted sections are real `mul` instructions. Adding up the result of each instruction produces
/// `*161*` (`2*4 + 5*5 + 11*8 + 8*5`).
///
/// Scan the corrupted memory for uncorrupted `mul` instructions. *What do you get if you add up all of the results of the
/// multiplications?*
///
/// [1]: https://adventofcode.com/2020/day/2
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

/// As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If
/// you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate
/// result.
///
/// There are two new instructions you'll need to handle:
///
/// * The `do()` instruction *enables* future `mul` instructions.
/// * The `don't()` instruction *disables* future `mul` instructions.
///
/// Only the *most recent* `do()` or `don't()` instruction applies. At the beginning of the program, `mul` instructions are
/// *enabled*.
///
/// For example:
///
/// `x*mul(2,4)*&mul[3,7]!^*don't()*_mul(5,5)+mul(32,64](mul(11,8)un*do()*?*mul(8,5)*)`
///
/// This corrupted memory is similar to the example from before, but this time the `mul(5,5)` and `mul(11,8)` instructions
/// are *disabled* because there is a `don't()` instruction before them. The other `mul` instructions function normally,
/// including the one at the end that gets re-*enabled* by a `do()` instruction.
///
/// This time, the sum of the results is `*48*` (`2*4 + 8*5`).
///
/// Handle the new instructions; *what do you get if you add up all of the results of just the enabled multiplications?*
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

aoc_macro::aoc_main!();

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
