//! Day 8: Playground
//!
//! https://adventofcode.com/2025/day/8

use core::str::FromStr;
use itertools::Itertools;
use nalgebra::Point3;
use std::collections::{HashMap, HashSet};
use tracing::{instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    run(input, Until::N(1000)).product
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let result = run(input, Until::OneCircuit);
    result.last_two[0].0.x as usize * result.last_two[1].0.x as usize
}

fn main() {
    solutions::main(part1, part2);
}

#[derive(Debug)]
enum Until {
    N(usize),
    OneCircuit,
}

struct RunResult {
    /// The product of the sizes of the three largest circuits.
    product: usize,

    /// The last two circuits that were combined.
    last_two: [JunctionBox; 2],
}

#[instrument(skip(input), level = "debug")]
fn run(input: &str, until: Until) -> RunResult {
    let boxes: Boxes = input.parse().unwrap();

    let mut distances = HashMap::new();
    for (i, a) in boxes.0.iter().enumerate() {
        for b in boxes.0.iter().skip(i + 1) {
            let dist_sq = (a.0.x - b.0.x).pow(2) + (a.0.y - b.0.y).pow(2) + (a.0.z - b.0.z).pow(2);
            distances.insert((a, b), dist_sq);
        }
    }

    // the value is the index of the circuit that box belongs to
    let mut box_to_circuit = HashMap::new();

    // the index is the circuit id, the value is the set of boxes in that circuit
    let mut circuits = core::iter::repeat_n(HashSet::new(), boxes.0.len()).collect_vec();

    for (i, b) in boxes.0.iter().enumerate() {
        circuits[i].insert(i);
        box_to_circuit.insert(*b, i);
    }

    let by_distance = distances
        .iter()
        .sorted_by_key(|(_, dist_sq)| **dist_sq)
        .collect_vec();

    let iter: &mut dyn Iterator<Item = _> = match until {
        Until::N(n) => &mut by_distance.iter().take(n),
        Until::OneCircuit => &mut by_distance.iter(),
    };

    let mut last_two = [JunctionBox(Point3::new(0, 0, 0)); 2];
    let mut biggest_circuit = 1;
    for ((a, b), dist_sq) in iter {
        // combine the circuits

        let a_circuit = box_to_circuit[a];
        let b_circuit = box_to_circuit[b];
        if a_circuit == b_circuit {
            trace!(
                ?a,
                ?b,
                dist_sq,
                a_circuit,
                b_circuit,
                "already in same circuit"
            );

            continue;
        }

        let new_circuit = a_circuit.min(b_circuit);
        let old_circuit = a_circuit.max(b_circuit);
        trace!(
            ?a,
            ?b,
            dist_sq,
            a_circuit,
            b_circuit,
            new_circuit,
            old_circuit,
            "merging pair"
        );

        let combined: HashSet<usize> = circuits[new_circuit]
            .union(&circuits[old_circuit])
            .copied()
            .collect();

        trace!(?combined, "combined circuit");

        // Update all boxes in both circuits to point to the new circuit
        for &box_idx in &combined {
            let box_ptr = &boxes.0[box_idx];
            box_to_circuit.insert(*box_ptr, new_circuit);
        }

        biggest_circuit = biggest_circuit.max(combined.len());
        circuits[new_circuit] = combined;
        circuits[old_circuit].clear();
        last_two = [**a, **b];

        if matches!(until, Until::OneCircuit) && biggest_circuit == boxes.0.len() {
            // we've combined all the boxes into one circuit, we're done
            break;
        }
    }

    let product = circuits
        .iter()
        .enumerate()
        .inspect(|(i, circuit)| {
            if !circuit.is_empty() {
                trace!(i, len = circuit.len(), ?circuit, "final circuit")
            }
        })
        .sorted_unstable_by(|a, b| a.1.len().cmp(&b.1.len()).reverse())
        .take(3)
        .inspect(|(i, circuit)| trace!(i, len = circuit.len(), "final circuit"))
        .map(|(_, circuit)| circuit.len())
        .product();

    RunResult { product, last_two }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JunctionBox(Point3<i64>);

impl FromStr for JunctionBox {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(',')
            .flat_map(str::parse)
            .collect_array()
            .ok_or(())?;

        let [x, y, z] = v;
        Ok(JunctionBox(Point3::new(x, y, z)))
    }
}

struct Boxes(Vec<JunctionBox>);

impl FromStr for Boxes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boxes = s.lines().map(str::parse).try_collect()?;
        Ok(Boxes(boxes))
    }
}

#[cfg(test)]
mod tests {
    use solutions::setup_tracing;

    use super::*;

    #[test]
    fn part1_example() {
        setup_tracing();
        let input = include_str!("../../data/examples/2025/08/1.txt");
        assert_eq!(40, run(input, Until::N(10)).product);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/08/1.txt");
        assert_eq!(25272, part2(input));
    }
}
