//! Day 2: 1202 Program Alarm
//!
//! https://adventofcode.com/2019/day/2

use std::{num::ParseIntError, ops::ControlFlow, str::FromStr};

use itertools::Itertools;

fn part1(input: &str) -> usize {
    let mut program = Program::from_str(input).unwrap();
    program.part1_setup();
    program.execute();
    program.output()
}

fn part2(input: &str) -> usize {
    let program = Program::from_str(input).unwrap();
    for noun in 0..=99 {
        for verb in 0..=99 {
            eprintln!("Trying noun={} verb={}", noun, verb);
            let mut program = program.clone();
            program.set_noun(noun);
            program.set_verb(verb);
            match program.execute() {
                Ok(output) => {
                    if output == 19690720 {
                        return 100 * noun + verb;
                    }
                }
                Err(index) => {
                    eprintln!("Error at index {}", index);
                    continue;
                }
            }
        }
    }

    panic!("No solution found");
}

aoc_macro::aoc_main!();

enum Intcode {
    // 1: Adds two numbers from two positions and stores in a third position
    Add {
        a: usize,
        b: usize,
        destination: usize,
    },

    // 2: Multiplies two numbers from two positions and stores in a third position
    Multiply {
        a: usize,
        b: usize,
        destination: usize,
    },

    // 99
    Exit,
}

impl Intcode {
    /// Executes the instruction and returns the number of values in the instruction
    fn handle(&self, memory: &mut Program) -> ControlFlow<()> {
        match self {
            Intcode::Add { a, b, destination } => {
                let a = match memory.get(*a) {
                    Some(a) => *a,
                    None => return ControlFlow::Break(()),
                };

                let b = match memory.get(*b) {
                    Some(b) => *b,
                    None => return ControlFlow::Break(()),
                };

                let destination = match memory.get_mut(*destination) {
                    Some(destination) => destination,
                    None => return ControlFlow::Break(()),
                };

                *destination = a + b;
            }
            Intcode::Multiply { a, b, destination } => {
                let a = match memory.get(*a) {
                    Some(a) => *a,
                    None => return ControlFlow::Break(()),
                };

                let b = match memory.get(*b) {
                    Some(b) => *b,
                    None => return ControlFlow::Break(()),
                };

                let destination = match memory.get_mut(*destination) {
                    Some(destination) => destination,
                    None => return ControlFlow::Break(()),
                };

                *destination = a * b;
            }
            Intcode::Exit => return ControlFlow::Break(()),
        }

        ControlFlow::Continue(())
    }

    fn num_instructions(&self) -> usize {
        match self {
            Intcode::Add { .. } | Intcode::Multiply { .. } => 4,
            Intcode::Exit => 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    memory: Vec<usize>,
}

impl Program {
    fn part1_setup(&mut self) {
        self.set_noun(12);
        self.set_verb(2);
    }

    fn get(&self, index: usize) -> Option<&usize> {
        self.memory.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut usize> {
        self.memory.get_mut(index)
    }

    fn set_noun(&mut self, noun: usize) {
        self.memory[1] = noun;
    }

    fn set_verb(&mut self, verb: usize) {
        self.memory[2] = verb;
    }

    fn output(&self) -> usize {
        self.memory[0]
    }

    fn execute(&mut self) -> Result<usize, usize> {
        let mut i = 0; // instruction pointer
        loop {
            let code = *self.get(i).ok_or(i)?;
            let code = match code {
                // Add
                1 => {
                    let a = *self.get(i + 1).ok_or(i + 1)?;
                    let b = *self.get(i + 2).ok_or(i + 2)?;
                    let destination = *self.get(i + 3).ok_or(i + 3)?;
                    Intcode::Add { a, b, destination }
                }
                // Multiply
                2 => {
                    let a = *self.get(i + 1).ok_or(i + 1)?;
                    let b = *self.get(i + 2).ok_or(i + 2)?;
                    let destination = *self.get(i + 3).ok_or(i + 3)?;
                    Intcode::Multiply { a, b, destination }
                }
                // Exit
                99 => Intcode::Exit,
                _ => panic!("Invalid opcode: {code}"),
            };

            match code.handle(self) {
                ControlFlow::Continue(_) => i += code.num_instructions(),
                ControlFlow::Break(_) => break,
            }
        }

        Ok(self.output())
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tape = s
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>())
            .try_collect()?;

        Ok(Self { memory: tape })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    #[test]
    fn part1_example() {
        let mut program = Program::from_str(INPUT).unwrap();
        let output = program.execute();
        assert!(matches!(output, Ok(3500)));
    }

    #[test]
    fn part2_example() {
        let mut program = Program::from_str(INPUT).unwrap();
        program.set_noun(12);
        program.set_verb(2);
        assert_eq!(1202, part2(INPUT));
    }
}
