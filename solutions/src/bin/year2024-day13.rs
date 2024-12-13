//! Day 13
//!
//! https://adventofcode.com/2024/day/13

use std::str::FromStr;

fn part1(input: &str) -> usize {
    MachinesParser::new(input)
        .filter_map(|m| m.solve())
        .map(|s| s.cost() as usize)
        .sum()
}

fn part2(input: &str) -> usize {
    MachinesParser::new(input)
        .filter_map(|m| m.solve_with_offset(10_000_000_000_000))
        .map(|s| s.cost() as usize)
        .sum()
}

aoc_macro::aoc_main!();

#[derive(Debug, PartialEq, Eq)]
struct Button {
    x: i64,
    y: i64,
}

impl FromStr for Button {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // form: Button A: X+94, Y+34
        let comma = s.find(',').ok_or(())?;
        let x = s[12..comma].parse().map_err(|_| ())?;
        let y = s.trim_end()[comma + 4..].parse().map_err(|_| ())?;
        Ok(Button { x, y })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Prize {
    x: i64,
    y: i64,
}

impl FromStr for Prize {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // form: Prize: X=8400, Y=5400
        let comma = s.find(',').ok_or(())?;
        let x = s[9..comma].parse().map_err(|_| ())?;
        let y = s.trim_end()[comma + 4..].parse().map_err(|_| ())?;
        Ok(Prize { x, y })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Solution {
    a: i64,
    b: i64,
}

impl Solution {
    fn cost(&self) -> i64 {
        self.a * 3 + self.b
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
}

impl FromStr for Machine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a = lines.next().ok_or(())?.parse().map_err(|_| ())?;
        let b = lines.next().ok_or(())?.parse().map_err(|_| ())?;
        let prize = lines.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Machine { a, b, prize })
    }
}

impl Machine {
    fn solve(&self) -> Option<Solution> {
        self.solve_with_offset(0)
    }

    fn solve_with_offset(&self, offset: i64) -> Option<Solution> {
        // two variable system of linear equation
        let prize = Prize {
            x: self.prize.x + offset,
            y: self.prize.y + offset,
        };

        let denominator = self.a.x * self.b.y - self.a.y * self.b.x;
        if denominator == 0 {
            return None;
        }

        let a = (prize.x * self.b.y - prize.y * self.b.x) / denominator;
        let b = (self.a.x * prize.y - self.a.y * prize.x) / denominator;
        if self.a.x * a + self.b.x * b == prize.x && self.a.y * a + self.b.y * b == prize.y {
            Some(Solution { a, b })
        } else {
            None
        }
    }
}

struct MachinesParser<'a> {
    split: std::str::Split<'a, &'static str>,
}

impl<'a> MachinesParser<'a> {
    fn new(input: &'a str) -> Self {
        let section_break = if input.contains("\r\n") {
            "\r\n\r\n"
        } else {
            "\n\n"
        };

        Self {
            split: input.split(section_break),
        }
    }
}

impl<'a> Iterator for MachinesParser<'a> {
    type Item = Machine;
    fn next(&mut self) -> Option<Self::Item> {
        self.split.next().and_then(|s| s.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/13/1.txt");
        assert_eq!(480, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/13/1.txt");
        assert_eq!(875318608908, part2(input));
    }

    #[test]
    fn machines_parser() {
        let input = include_str!("../../data/examples/2024/13/1.txt");
        let expected = vec![
            Machine {
                a: Button { x: 94, y: 34 },
                b: Button { x: 22, y: 67 },
                prize: Prize { x: 8400, y: 5400 },
            },
            Machine {
                a: Button { x: 26, y: 66 },
                b: Button { x: 67, y: 21 },
                prize: Prize { x: 12748, y: 12176 },
            },
            Machine {
                a: Button { x: 17, y: 86 },
                b: Button { x: 84, y: 37 },
                prize: Prize { x: 7870, y: 6450 },
            },
            Machine {
                a: Button { x: 69, y: 23 },
                b: Button { x: 27, y: 71 },
                prize: Prize { x: 18641, y: 10279 },
            },
        ];

        assert_eq!(expected, MachinesParser::new(input).collect::<Vec<_>>());
    }

    #[test]
    fn from_str() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"#;

        assert_eq!(
            Machine {
                a: Button { x: 94, y: 34 },
                b: Button { x: 22, y: 67 },
                prize: Prize { x: 8400, y: 5400 },
            },
            input.parse().unwrap()
        );
    }

    #[test]
    fn cost() {
        assert_eq!(280, Solution { a: 80, b: 40 }.cost());
        assert_eq!(200, Solution { a: 38, b: 86 }.cost());
    }

    #[test]
    fn solve() {
        assert_eq!(
            Some(Solution { a: 80, b: 40 }),
            Machine {
                a: Button { x: 94, y: 34 },
                b: Button { x: 22, y: 67 },
                prize: Prize { x: 8400, y: 5400 },
            }
            .solve()
        );

        assert_eq!(
            None,
            Machine {
                a: Button { x: 26, y: 66 },
                b: Button { x: 67, y: 21 },
                prize: Prize { x: 12748, y: 12176 },
            }
            .solve()
        );

        assert_eq!(
            Some(Solution { a: 38, b: 86 }),
            Machine {
                a: Button { x: 17, y: 86 },
                b: Button { x: 84, y: 37 },
                prize: Prize { x: 7870, y: 6450 },
            }
            .solve()
        );

        assert_eq!(
            None,
            Machine {
                a: Button { x: 69, y: 23 },
                b: Button { x: 27, y: 71 },
                prize: Prize { x: 18641, y: 10279 },
            }
            .solve()
        );
    }
}
