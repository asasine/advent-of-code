//! Day 19: Linen Layout
//!
//! https://adventofcode.com/2024/day/19

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
use tracing::{debug, trace};

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut patterns = lines.next().unwrap().split(", ");
    lines.next(); // section break

    let designs = lines.map(str::trim);

    let r = Regex::new(&format!("^(?-u:{})*$", patterns.join("|"))).unwrap();
    debug!("Regex: {:?}", r);

    designs
        .enumerate()
        .filter(|(i, design)| {
            trace!("Checking design {}", i + 1);
            r.is_match(design)
        })
        .count()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect_vec();
    lines.next(); // section break

    let designs = lines.map(str::trim);
    let mut cache = HashMap::new();
    let count = designs
        .map(|design| {
            trace!("Checking design: {}", design);
            dfs(design, &patterns, &mut cache)
        })
        .sum();

    count
}

fn dfs<'a>(line: &'a str, patterns: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if line.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(line) {
        return count;
    }

    let count = patterns
        .iter()
        .filter_map(|&pattern| {
            line.strip_prefix(pattern)
                .map(|suffix| dfs(suffix, patterns, cache))
        })
        .sum();

    cache.insert(line, count);
    count
}

aoc_macro::aoc_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        solutions::setup_tracing();
        let input = include_str!("../../data/examples/2024/19/1.txt");
        assert_eq!(6, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/19/1.txt");
        assert_eq!(16, part2(input));
    }
}
