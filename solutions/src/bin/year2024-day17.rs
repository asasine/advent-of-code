//! Day 17: Chronospatial Computer
//!
//! https://adventofcode.com/2024/day/17

use std::str::FromStr;

use itertools::Itertools;
use tracing::{debug, instrument};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> String {
    let mut computer: ChronospatialComputer = input.parse().unwrap();
    debug!("{:?}", computer);
    let output = computer.output();
    output.into_iter().join(",")
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> u64 {
    let computer = ChronospatialComputer::from_str(input).unwrap();
    computer.find_part2()
}

fn main() {
    solutions::main(part1, part2)
}

#[derive(Debug, Clone)]
struct ChronospatialComputer {
    registers: Registers,
    program: Vec<u8>,
}

impl FromStr for ChronospatialComputer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut parse_register = |name: &str| -> Result<u64, String> {
            lines
                .next()
                .ok_or("Expected register")?
                .strip_prefix(&format!("Register {name}: "))
                .ok_or(format!("Expected register {name}"))?
                .trim()
                .parse::<u64>()
                .map_err(|e| e.to_string())
        };

        // format: Register A: 123
        let registers = Registers {
            a: parse_register("A")?,
            b: parse_register("B")?,
            c: parse_register("C")?,
        };

        // section break (blank line) between registers and program
        lines.next();

        let program = lines
            .next()
            .and_then(|program| program.strip_prefix("Program: "))
            .ok_or("Expected program")?;

        let instructions = program
            .split(',')
            .map(|s| {
                if s.len() == 1 {
                    Ok(s.chars().next().unwrap())
                } else {
                    Err(format!("Value in instruction was too large: {s}"))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let program = instructions
            .into_iter()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as u8)
                    .ok_or(format!("Invalid digit: {c}"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { registers, program })
    }
}

impl ChronospatialComputer {
    /// Executes the program until an output is produced, returning the output.
    ///
    /// If the program halts without producing an output, `None` is returned.
    fn execute_until_output(
        &self,
        instruction_pointer: &mut usize,
        registers: &mut Registers,
    ) -> Option<u8> {
        while let Some((opcode, operand)) = self.instruction_at(*instruction_pointer) {
            match opcode {
                Opcode::Adv => registers.a >>= operand.combo(registers),
                Opcode::Bxl => registers.b ^= operand.0 as u64,
                Opcode::Bst => registers.b = operand.combo(registers) & 0b111,
                Opcode::Jnz => {
                    if registers.a != 0 {
                        *instruction_pointer = operand.0 as usize;
                        continue; // don't increment the instruction pointer
                    }
                }
                Opcode::Bxc => registers.b ^= registers.c,
                Opcode::Out => {
                    *instruction_pointer += 2;
                    return Some((operand.combo(registers) & 0b111) as u8);
                }
                Opcode::Bdv => registers.b = registers.a >> operand.combo(registers),
                Opcode::Cdv => registers.c = registers.a >> operand.combo(registers),
            }

            *instruction_pointer += 2;
        }

        None
    }

    /// Execute the program until it halts and return the output.
    fn output(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        let mut instruction_pointer = 0;
        let mut registers = self.registers.clone();
        while let Some(byte) = self.execute_until_output(&mut instruction_pointer, &mut registers) {
            output.push(byte);
        }

        self.registers = registers;
        output
    }

    fn instruction_at(&self, instruction_pointer: usize) -> Option<(Opcode, Operand)> {
        let opcode = self
            .program
            .get(instruction_pointer)
            .and_then(|&b| b.try_into().ok())?;

        let operand = self
            .program
            .get(instruction_pointer + 1)
            .and_then(|&b| b.try_into().ok())?;

        Some((opcode, operand))
    }

    fn find_part2(&self) -> u64 {
        let a = self
            .find_part2_impl_recursive(0, self.program.len() - 1)
            .expect("No solution found");

        let mut computer = self.clone();
        computer.registers.a = a;
        let output = computer.output();
        assert_eq!(output, self.program);
        a
    }

    fn find_part2_impl_recursive(&self, a: u64, byte_to_check: usize) -> Option<u64> {
        for nibble in 0..=7 {
            let a = (a << 3) | nibble;
            let byte = self.execute_until_output(&mut 0, &mut Registers { a, b: 0, c: 0 });
            if byte.is_none_or(|byte| byte != self.program[byte_to_check]) {
                continue;
            } else if byte_to_check == 0 {
                return Some(a);
            } else if let Some(a) = self.find_part2_impl_recursive(a, byte_to_check - 1) {
                return Some(a);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

struct Operand(u8);

impl TryFrom<char> for Operand {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let b = value.to_digit(10).map(|o| o as u8).ok_or(value)?;
        Self::try_from(b).map_err(|_| value)
    }
}

impl TryFrom<u8> for Operand {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 7 {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

impl Operand {
    /// The purpose of the combo operand depends on the [`Opcode`].
    ///
    /// - Combo operands `0` through `3` represent literal values `0` through `3`.
    /// - Combo operand `4` represents the value of [`Registers::a`].
    /// - Combo operand `5` represents the value of [`Registers::b`].
    /// - Combo operand `6` represents the value of [`Registers::c`].
    /// - Combo operand `7` is reserved and will not appear in valid programs.
    fn combo(&self, registers: &Registers) -> u64 {
        match self.0 {
            x @ 0..=3 => x as u64,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            7 => panic!("7 is reserved and should not appear as a combo operand in valid programs"),
            x => unreachable!("not a 3-bit operand: {}", x),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    /// Opcode `0` performs division.
    ///
    /// The numerator is the value in the `A` register.
    /// The denominator is `2` to the power of [`Operand::combo`].
    /// The result is truncated to an integer and written to the `A` register.
    Adv,

    /// Opcode `1` performs bitwise XOR of register `B` and [`Operand::0`], writing the result to the `B` register.
    Bxl,

    /// Opcode `2` calculates [`Operand::combo`] modulo `8`, writing the result to the `B` register.
    Bst,

    /// Opcode `3` does nothing if register `A` is `0`.
    /// However, if it is not zero, it sets the [`ChronospatialComputer::instruction_pointer`] to the value [`Operand::0`].
    /// Additionally, if value was not zero, the [`ChronospatialComputer::instruction_pointer`] is not incremented after this instruction.
    Jnz,

    /// Opcode `4` calculates the bitwise XOR of registers `B` and `C`, writing the result to the `B` register.
    ///
    /// This operand has an operand but ignores it.
    Bxc,

    /// Opcode `5` calculates [`Operand::combo`] modulo `8` and outputs the result as program output.
    Out,

    /// Opcode `6` works like [`Opcode::Adv`] but stores the result in the `B` register.
    ///
    /// The numerator is still read from the `A` register.
    Bdv,

    /// Opcode `7` works like [`Opcode::Adv`] but stores the result in the `C` register.
    ///
    /// The numerator is still read from the `A` register.
    Cdv,
}

impl TryFrom<u8> for Opcode {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            x => Err(x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/17/1.txt");
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/17/2.txt");
        assert_eq!(117440, part2(input));
    }

    #[test]
    fn parse() {
        let input = include_str!("../../data/examples/2024/17/1.txt");
        let computer: ChronospatialComputer = input.parse().unwrap();
        assert_eq!(computer.registers.a, 729);
        assert_eq!(computer.registers.b, 0);
        assert_eq!(computer.registers.c, 0);
        assert_eq!(computer.program, vec![0, 1, 5, 4, 3, 0])
    }

    #[test]
    fn example2() {
        let mut computer = ChronospatialComputer {
            registers: Registers { a: 0, b: 0, c: 9 },
            program: vec![2, 6],
        };

        computer.output();
        assert_eq!(computer.registers.b, 1);
    }

    #[test]
    fn example3() {
        let mut computer = ChronospatialComputer {
            registers: Registers { a: 10, b: 0, c: 0 },
            program: vec![5, 0, 5, 1, 5, 4],
        };

        let output = computer.output();
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn example4() {
        let mut computer = ChronospatialComputer {
            registers: Registers {
                a: 2024,
                b: 0,
                c: 0,
            },
            program: vec![0, 1, 5, 4, 3, 0],
        };

        let output = computer.output();
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.registers.a, 0);
    }

    #[test]
    fn example5() {
        let mut computer = ChronospatialComputer {
            registers: Registers { a: 0, b: 29, c: 0 },
            program: vec![1, 7],
        };

        computer.output();
        assert_eq!(computer.registers.b, 26);
    }

    #[test]
    fn example6() {
        let mut computer = ChronospatialComputer {
            registers: Registers {
                a: 0,
                b: 2024,
                c: 43690,
            },
            program: vec![4, 0],
        };

        computer.output();
        assert_eq!(computer.registers.b, 44354);
    }
}
