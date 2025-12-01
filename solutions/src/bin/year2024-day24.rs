//! Day 24: Crossed Wires
//!
//! https://adventofcode.com/2024/day/24

use core::fmt;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use tracing::{debug, instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> u64 {
    let wires = input.parse::<GatesAndWires>().unwrap();
    debug!("{}", wires);

    wires.output()
}

/// An output bit `z[n]` is a [single-bit adder](https://en.wikipedia.org/wiki/Adder_(electronics)) on `x[n]` and
/// `y[n]` with carry from `x[n-1]` and `y[n-1]`.
///
/// They all should have the form:
///
/// ```text
/// z[n] = ((x[n] ^ y[n]) ^ ((x[n-1] & y[n-1]) | ((x[n-1] ^ y[n-1]) & z[n-1])))
/// ```
///
/// When broken down into individual gates:
///
/// ```text
/// z[n] = a ^ b
/// a = x[n] ^ y[n]
/// b = c | d
/// c = x[n-1] & y[n-1]
/// d = e & z[n-1]
/// e = x[n-1] ^ y[n-1]
/// ```
///
/// Some properties of this:
/// 1. An `XOR` gate always has inputs of x and y, or an output of z.
/// 2. All gates that output to z should be `XOR` (edge case: except the MSB).
/// 3. The outputs of all `AND` only go to `OR`, except for the LSB's `AND`.
///
/// Any gate that violates these rules is an error.
#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> String {
    let wires = input.parse::<GatesAndWires>().unwrap();

    let max_z = wires
        .gates
        .iter()
        .map(|gate| gate.output)
        .filter(|id| id.is_z())
        .map(|id| id.gate_number())
        .max()
        .unwrap();

    let mut errors = HashSet::new();
    let mut or_inputs = HashSet::new();
    let mut and_outputs = HashSet::new();
    for gate in &wires.gates {
        // all gates that output to z should be XORs, except the MSB
        if gate.output.is_z() && gate.op != GateOperation::Xor && gate.output.gate_number() != max_z
        {
            trace!("Output XOR error: {}", gate);
            errors.insert(gate.output);
        }

        // all XORs should have two inputs that are x or y, or an output that is a z
        if gate.op == GateOperation::Xor
            && !(gate.a.is_xy() && gate.b.is_xy()) && !gate.output.is_z() {
                trace!("Invalid XOR error: {}", gate);
                errors.insert(gate.output);
            }

        // all AND outputs should go to OR inputs, except for LSB
        match gate.op {
            GateOperation::Or => {
                or_inputs.insert(gate.a);
                or_inputs.insert(gate.b);
            }
            GateOperation::And
                if !(gate.a.is_xy()
                    && gate.a.gate_number() == 0
                    && gate.b.is_xy()
                    && gate.b.gate_number() == 0) =>
            {
                and_outputs.insert(gate.output);
            }
            _ => {}
        }
    }

    trace!("Errors after XOR inspection: {:?}", errors);
    errors.extend(or_inputs.symmetric_difference(&and_outputs));
    errors.into_iter().sorted().join(",")
}

fn main() {
    solutions::main(part1, part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WireIdentifier(char, char, char);

impl FromStr for WireIdentifier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        let c = chars.next().unwrap();
        Ok(WireIdentifier(a, b, c))
    }
}

impl WireIdentifier {
    fn is_x(&self) -> bool {
        self.0 == 'x'
    }

    fn is_y(&self) -> bool {
        self.0 == 'y'
    }

    fn is_xy(&self) -> bool {
        self.is_x() || self.is_y()
    }

    fn is_z(&self) -> bool {
        self.0 == 'z'
    }

    fn gate_number(&self) -> u8 {
        self.1.to_digit(10).unwrap() as u8 * 10 + self.2.to_digit(10).unwrap() as u8
    }
}

impl fmt::Display for WireIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy)]
struct Wire {
    id: WireIdentifier,
    state: bool,
}

impl FromStr for Wire {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let id = parts.next().unwrap().parse().unwrap();
        let state = parts.next().unwrap().trim().parse::<u8>().unwrap() != 0;
        Ok(Wire { id, state })
    }
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.state as u8)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    a: WireIdentifier,
    op: GateOperation,
    b: WireIdentifier,
    output: WireIdentifier,
}

impl FromStr for Gate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let a = parts.next().unwrap().parse().unwrap();
        let op = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();
        parts.next(); // ->
        let output = parts.next().unwrap().parse().unwrap();
        Ok(Gate { a, op, b, output })
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} -> {}", self.a, self.op, self.b, self.output)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GateOperation {
    And,
    Or,
    Xor,
}

impl FromStr for GateOperation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateOperation::And),
            "OR" => Ok(GateOperation::Or),
            "XOR" => Ok(GateOperation::Xor),
            _ => Err(()),
        }
    }
}

impl fmt::Display for GateOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GateOperation::And => write!(f, "AND"),
            GateOperation::Or => write!(f, "OR"),
            GateOperation::Xor => write!(f, "XOR"),
        }
    }
}

#[derive(Debug, Clone)]
struct Wires(Vec<Wire>);

impl FromStr for Wires {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Wires(s.lines().map(str::parse).collect::<Result<_, _>>()?))
    }
}

impl fmt::Display for Wires {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // sort by id first
            for wire in self.0.iter().sorted_by_key(|w| w.id) {
                writeln!(f, "{}", wire)?;
            }
        } else {
            for wire in &self.0 {
                writeln!(f, "{}", wire)?;
            }
        }

        Ok(())
    }
}

struct GatesAndWires {
    initials: Wires,
    gates: Vec<Gate>,
}

impl GatesAndWires {
    /// Calculates the output of the circuit.
    ///
    /// The output is the value of the `z` wires concatenated together in order.
    fn output(&self) -> u64 {
        let mut solver = GatesAndWiresSolver {
            wires: self
                .initials
                .0
                .iter()
                .map(|wire| (wire.id, *wire))
                .collect(),
            gate_by_output: self
                .gates
                .iter()
                .map(|gate| (gate.output, gate.clone()))
                .collect(),
        };

        self.gates
            .iter()
            .map(|gate| gate.output)
            .filter(|id| id.is_z())
            .map(|id| (solver.evaluate(id) as u64) << id.gate_number())
            .fold(0, |acc, x| acc | x)
    }
}

impl FromStr for GatesAndWires {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let section_break = if s.contains("\r\n") {
            "\r\n\r\n"
        } else {
            "\n\n"
        };

        let mut parts = s.split(section_break);
        let initials = parts.next().unwrap().parse()?;
        let gates = parts
            .next()
            .unwrap()
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(GatesAndWires { initials, gates })
    }
}

impl fmt::Display for GatesAndWires {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Initials:")?;
        write!(f, "{}", self.initials)?;
        writeln!(f)?;
        for gate in &self.gates {
            writeln!(f, "{}", gate)?;
        }

        Ok(())
    }
}

struct GatesAndWiresSolver {
    wires: HashMap<WireIdentifier, Wire>,
    gate_by_output: HashMap<WireIdentifier, Gate>,
}

impl GatesAndWiresSolver {
    fn evaluate(&mut self, id: WireIdentifier) -> bool {
        match self.wires.entry(id) {
            Entry::Occupied(e) => e.get().state,
            Entry::Vacant(_) => self.evaluate_impl(id),
        }
    }

    #[instrument(skip(self), level = "trace")]
    fn evaluate_impl(&mut self, id: WireIdentifier) -> bool {
        let Gate { a, b, op, .. } = *self.gate_by_output.get(&id).unwrap();
        let a = self.evaluate(a);
        let b = self.evaluate(b);
        let state = match op {
            GateOperation::And => a & b,
            GateOperation::Or => a | b,
            GateOperation::Xor => a ^ b,
        };

        self.wires.insert(id, Wire { id, state });
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("../../data/examples/2024/24/1.txt");
        assert_eq!(4, part1(input));
    }

    #[test]
    fn part1_example2() {
        let input = include_str!("../../data/examples/2024/24/2.txt");
        assert_eq!(2024, part1(input));
    }

    #[test]
    #[ignore = "example 3 is not a full adder circuit so the solution is incorrect."]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/24/3.txt");
        assert_eq!("z00,z01,z02,z05", part2(input));
    }
}
