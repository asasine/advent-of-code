//! Day 1: Historian Hysteria
//!
//! https://adventofcode.com/2024/day/1

use std::collections::HashMap;
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let mut lists: [Vec<i64>; 2] = [Vec::new(), Vec::new()];
    for line in input.lines() {
        for (i, n) in line.split_whitespace().enumerate() {
            lists[i].push(n.parse().expect("All numbers should be valid"));
        }
    }

    lists.iter_mut().for_each(|list| list.sort_unstable());
    (0..lists[0].len())
        .map(|i| (lists[0][i].abs_diff(lists[1][i])) as usize)
        .sum()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let mut left: Vec<usize> = Vec::new();
    let mut right: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let l = split
            .next()
            .expect("Each line should have at least one number")
            .parse()
            .expect("All lists should only contain numbers");

        let r = split
            .next()
            .expect("Each line should have at least two numbers")
            .parse()
            .expect("All lists should only contain numbers");

        left.push(l);
        right.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }

    left.iter().map(|n| n * right.get(n).unwrap_or(&0)).sum()
}

fn main() {
    solutions::main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/01/1.txt");
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/01/1.txt");
        assert_eq!(part2(input), 31);
    }
}
