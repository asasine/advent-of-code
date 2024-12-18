//! Day 7: Bridge Repair
//!
//! https://adventofcode.com/2024/day/7

use core::fmt;
use std::str::FromStr;

use itertools::Itertools;
use tracing::trace;

#[derive(Debug, Clone)]
struct IncompleteEquation {
    /// The value we are trying to reach, on the left side of the colon.
    test_value: usize,

    /// The numbers we can use to reach the test value.
    numbers: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
enum EquationParseError {
    NoColon,
    NotAnInteger,
}

impl FromStr for IncompleteEquation {
    type Err = EquationParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_value, numbers) = s.split_once(": ").ok_or(EquationParseError::NoColon)?;
        let test_value = test_value
            .parse()
            .map_err(|_| EquationParseError::NotAnInteger)?;

        let numbers = numbers
            .split(" ")
            .map(|n| n.parse().map_err(|_| EquationParseError::NotAnInteger))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            test_value,
            numbers,
        })
    }
}

impl IncompleteEquation {
    /// Given an incomplete equation, find a set of operators that will make the equation true.
    fn make_true(&self, operators: &[Operator]) -> Option<CompleteEquation> {
        let operators =
            itertools::repeat_n(operators, self.numbers.len() - 1).multi_cartesian_product();

        // iterate over all possible combinations of operators
        for ops in operators {
            let equation = CompleteEquation {
                test_value: self.test_value,
                numbers: self.numbers.clone(),
                operators: ops.iter().map(|op| **op).collect(),
            };

            trace!("candidate: {equation}");
            if equation.is_valid() {
                return Some(equation);
            }
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CompleteEquation {
    /// The value we are trying to reach, on the left side of the colon.
    test_value: usize,

    /// The numbers we can use to reach the test value.
    numbers: Vec<usize>,

    /// The operators we can use to reach the test value.
    ///
    /// This will be of length `numbers.len() - 1`, since we need one less operator than numbers.
    /// The operators are stored in the order they should be applied.
    /// Operators are applied from left to right.
    operators: Vec<Operator>,
}

impl CompleteEquation {
    /// Given a complete equation, evaluate it.
    fn evaluate(&self) -> usize {
        let mut numbers = self.numbers.iter();
        let first = *numbers.next().expect("should have at least one number");
        numbers
            .zip(self.operators.iter())
            .fold(first, |acc, (x, op)| op.apply(acc, *x))
    }

    fn is_valid(&self) -> bool {
        self.evaluate() == self.test_value
    }
}

impl fmt::Display for CompleteEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut numbers = self.numbers.iter();
        let first = *numbers.next().expect("should have at least one number");
        let mut result = write!(f, "{}", first);
        for (x, op) in numbers.zip(self.operators.iter()) {
            result = result.and_then(|_| write!(f, " {} {}", op, x));
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

impl Operator {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenation => {
                let b_digits = solutions::num::usize::digits(b);
                a * 10usize.pow(b_digits) + b
            }
        }
    }

    fn part1() -> [Operator; 2] {
        [Operator::Add, Operator::Multiply]
    }

    fn part2() -> [Operator; 3] {
        [Operator::Add, Operator::Multiply, Operator::Concatenation]
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
            Operator::Concatenation => write!(f, "||"),
        }
    }
}

fn part1(input: &str) -> usize {
    let equations = input
        .lines()
        .map(|line| line.parse::<IncompleteEquation>().unwrap())
        .collect::<Vec<_>>();

    equations
        .into_iter()
        .filter_map(|eq| eq.make_true(&Operator::part1()))
        .map(|eq| eq.evaluate())
        .sum()
}

fn part2(input: &str) -> usize {
    let equations = input
        .lines()
        .map(|line| line.parse::<IncompleteEquation>().unwrap())
        .collect::<Vec<_>>();

    equations
        .into_iter()
        .filter_map(|eq| eq.make_true(&Operator::part2()))
        .map(|eq| eq.evaluate())
        .sum()
}

aoc_macro::aoc_main!();

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/07/1.txt");
        assert_eq!(3749, part1(input));
    }

    #[test]
    fn evaluate() {
        let mut equation = CompleteEquation {
            test_value: 10,
            numbers: vec![1, 2, 3, 1],
            operators: vec![Operator::Add, Operator::Multiply, Operator::Add],
        };

        assert_eq!(equation.evaluate(), 10);
        assert!(equation.is_valid());

        equation.test_value = 11;
        assert_eq!(equation.evaluate(), 10);
        assert!(!equation.is_valid());
    }

    #[test]
    fn make_true() {
        let eqs = [
            IncompleteEquation {
                test_value: 190,
                numbers: vec![10, 19],
            },
            IncompleteEquation {
                test_value: 3267,
                numbers: vec![81, 40, 27],
            },
            IncompleteEquation {
                test_value: 83,
                numbers: vec![17, 5],
            },
            IncompleteEquation {
                test_value: 156,
                numbers: vec![15, 6],
            },
            IncompleteEquation {
                test_value: 7290,
                numbers: vec![6, 8, 6, 15],
            },
            IncompleteEquation {
                test_value: 161011,
                numbers: vec![16, 10, 13],
            },
            IncompleteEquation {
                test_value: 192,
                numbers: vec![17, 8, 14],
            },
            IncompleteEquation {
                test_value: 21037,
                numbers: vec![9, 7, 18, 13],
            },
            IncompleteEquation {
                test_value: 292,
                numbers: vec![11, 6, 16, 20],
            },
        ];

        let expected = [
            Some(HashSet::from([CompleteEquation {
                test_value: 190,
                numbers: vec![10, 19],
                operators: vec![Operator::Multiply],
            }])),
            Some(HashSet::from([
                CompleteEquation {
                    test_value: 3267,
                    numbers: vec![81, 40, 27],
                    operators: vec![Operator::Multiply, Operator::Add],
                },
                CompleteEquation {
                    test_value: 3267,
                    numbers: vec![81, 40, 27],
                    operators: vec![Operator::Add, Operator::Multiply],
                },
            ])),
            None,
            Some(HashSet::from([CompleteEquation {
                test_value: 156,
                numbers: vec![15, 6],
                operators: vec![Operator::Concatenation],
            }])),
            Some(HashSet::from([CompleteEquation {
                test_value: 7290,
                numbers: vec![6, 8, 6, 15],
                operators: vec![
                    Operator::Multiply,
                    Operator::Concatenation,
                    Operator::Multiply,
                ],
            }])),
            None,
            Some(HashSet::from([CompleteEquation {
                test_value: 192,
                numbers: vec![17, 8, 14],
                operators: vec![Operator::Concatenation, Operator::Add],
            }])),
            None,
            Some(HashSet::from([CompleteEquation {
                test_value: 292,
                numbers: vec![11, 6, 16, 20],
                operators: vec![Operator::Add, Operator::Multiply, Operator::Add],
            }])),
        ];

        // sanity check
        assert_eq!(eqs.len(), expected.len());

        for (eq, expected) in eqs.iter().zip(expected.iter()) {
            let actual = eq.make_true(&Operator::part2());
            match actual {
                Some(ref actual) => {
                    assert!(actual.is_valid());
                    assert_eq!(actual.evaluate(), eq.test_value);
                }
                None => {
                    assert!(expected.is_none());
                }
            }
        }
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/07/1.txt");
        assert_eq!(11387, part2(input));
    }

    #[test]
    fn apply() {
        assert_eq!(Operator::Add.apply(1, 2), 3);
        assert_eq!(Operator::Multiply.apply(1, 2), 2);
        assert_eq!(Operator::Concatenation.apply(1, 2), 12);
        assert_eq!(Operator::Concatenation.apply(2345, 5678), 23455678);
    }
}
