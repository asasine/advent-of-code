//! Day 1: Trebuchet?!

use regex::Regex;

/// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a
/// map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations, you need to check all *fifty stars* by
/// December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second
/// puzzle is unlocked when you complete the first. Each puzzle grants *one star*. Good luck!
///
/// You try to ask why they can't just use a [weather machine][1] ("not powerful enough") and where they're even sending you
/// ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky
/// ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a
/// [trebuchet][2] ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been
/// *amended* by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are
/// having trouble reading the values on the document.
///
/// The newly-improved calibration document consists of lines of text; each line originally contained a specific
/// *calibration value* that the Elves now need to recover. On each line, the calibration value can be found by combining
/// the *first digit* and the *last digit* (in that order) to form a single *two-digit number*.
///
/// For example:
///
/// ```text
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ```
///
/// In this example, the calibration values of these four lines are `12`, `38`, `15`, and `77`. Adding these together
/// produces *`142`*.
///
/// Consider your entire calibration document. *What is the sum of all of the calibration values?*
///
/// [1]: https://adventofcode.com/2015/day/1
/// [2]: https://en.wikipedia.org/wiki/Trebuchet
pub fn part1(input: &str) -> usize {
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
            let last = digits.rev().next().unwrap_or(first);

            (first * 10 + last) as usize
        })
        .sum()
}

/// Your calculation isn't quite right. It looks like some of the digits are actually *spelled out with letters*: `one`,
/// `two`, `three`, `four`, `five`, `six`, `seven`, `eight`, and `nine` *also* count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
///
/// ```text
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ```
///
/// In this example, the calibration values are `29`, `83`, `13`, `24`, `42`, `14`, and `76`. Adding these together produces
/// `*281*`.
///
/// *What is the sum of all of the calibration values?*
pub fn part2(input: &str) -> usize {
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
                .map(|i| {
                    let remaining = &line[i..];
                    re.find(remaining).map(|m| match_to_digit(&m))
                })
                .flatten()
                .last()
                .unwrap_or(first);

            (first * 10 + last) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("inputs/examples/01/1.txt");
        let part1 = part1(input);
        eprintln!("part1: {}", part1);
        assert_eq!(part1, 142);

        let input = include_str!("inputs/examples/01/2.txt");
        let part2 = part2(input);
        eprintln!("part2: {}", part2);
        assert_eq!(part2, 281);
    }
}
