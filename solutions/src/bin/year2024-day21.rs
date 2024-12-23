//! Day 21: Keypad Conundrum
//!
//! https://adventofcode.com/2024/day/21

use std::collections::HashMap;

use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let mut solver = Solver {
        paths: paths(),
        cache: HashMap::new(),
    };

    input.lines().map(|line| solver.score(line, 3)).sum()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let mut solver = Solver {
        paths: paths(),
        cache: HashMap::new(),
    };

    input.lines().map(|line| solver.score(line, 26)).sum()
}

fn main() {
    solutions::main(part1, part2);
}

/// Returns the shortest paths between all pairs of characters.
///
/// The numeric keypad is represented as a 3x4 grid with the following layout:
/// ```text
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
/// ```
///
/// The directional keypad is represented as a 2x2 grid with the following layout:
/// ```text
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
/// ```
fn paths() -> HashMap<(char, char), &'static str> {
    // I could run a BFS for every pair of characters to find the shortest path, but that would yield paths that have
    // a mixture of directions, which would yield suboptimal results. Instead, there aren't that many, so it's easier
    // to hardcode them.

    // Priority for moves is <, ^/v, >, unless it would result in zig-zags, in which case the zig-zag is avoided.
    HashMap::from([
        // numeric keypad
        // A -> [A0-9] and reverse
        (('A', 'A'), "A"),
        (('A', '0'), "<A"),
        (('0', 'A'), ">A"),
        (('A', '1'), "^<<A"),
        (('1', 'A'), ">>vA"),
        (('A', '2'), "<^A"),
        (('2', 'A'), "v>A"),
        (('A', '3'), "^A"),
        (('3', 'A'), "vA"),
        (('A', '4'), "^^<<A"),
        (('4', 'A'), ">>vvA"),
        (('A', '5'), "<^^A"),
        (('5', 'A'), "vv>A"),
        (('A', '6'), "^^A"),
        (('6', 'A'), "vvA"),
        (('A', '7'), "^^^<<A"),
        (('7', 'A'), ">>vvvA"),
        (('A', '8'), "<^^^A"),
        (('8', 'A'), "vvv>A"),
        (('A', '9'), "^^^A"),
        (('9', 'A'), "vvvA"),
        // 0 -> [0-9] and reverse
        (('0', '0'), "A"),
        (('0', '1'), "^<A"),
        (('1', '0'), ">vA"),
        (('0', '2'), "^A"),
        (('2', '0'), "vA"),
        (('0', '3'), "^>A"),
        (('3', '0'), "<vA"),
        (('0', '4'), "^<^A"),
        (('4', '0'), ">vvA"),
        (('0', '5'), "^^A"),
        (('5', '0'), "vvA"),
        (('0', '6'), "^^>A"),
        (('6', '0'), "<vvA"),
        (('0', '7'), "^^^<A"),
        (('7', '0'), ">vvvA"),
        (('0', '8'), "^^^A"),
        (('8', '0'), "vvvA"),
        (('0', '9'), "^^^>A"),
        (('9', '0'), "<vvvA"),
        // 1 -> [1-9] and reverse
        (('1', '1'), "A"),
        (('1', '2'), ">A"),
        (('2', '1'), "<A"),
        (('1', '3'), ">>A"),
        (('3', '1'), "<<A"),
        (('1', '4'), "^A"),
        (('4', '1'), "vA"),
        (('1', '5'), "^>A"),
        (('5', '1'), "<vA"),
        (('1', '6'), "^>>A"),
        (('6', '1'), "<<vA"),
        (('1', '7'), "^^A"),
        (('7', '1'), "vvA"),
        (('1', '8'), "^^>A"),
        (('8', '1'), "<vvA"),
        (('1', '9'), "^^>>A"),
        (('9', '1'), "<<vvA"),
        // 2 -> [2-9] and reverse
        (('2', '2'), "A"),
        (('2', '3'), ">A"),
        (('3', '2'), "<A"),
        (('2', '4'), "<^A"),
        (('4', '2'), "v>A"),
        (('2', '5'), "^A"),
        (('5', '2'), "vA"),
        (('2', '6'), "^>A"),
        (('6', '2'), "<vA"),
        (('2', '7'), "<^^A"),
        (('7', '2'), "vv>A"),
        (('2', '8'), "^^A"),
        (('8', '2'), "vvA"),
        (('2', '9'), "^^>A"),
        (('9', '2'), "<vvA"),
        // 3 -> [3-9] and reverse
        (('3', '3'), "A"),
        (('3', '4'), "<<^A"),
        (('4', '3'), "v>>A"),
        (('3', '5'), "<^A"),
        (('5', '3'), "v>A"),
        (('3', '6'), "^A"),
        (('6', '3'), "vA"),
        (('3', '7'), "<<^^A"),
        (('7', '3'), "vv>>A"),
        (('3', '8'), "<^^A"),
        (('8', '3'), "vv>A"),
        (('3', '9'), "^^A"),
        (('9', '3'), "vvA"),
        // 4 -> [4-9] and reverse
        (('4', '4'), "A"),
        (('4', '5'), ">A"),
        (('5', '4'), "<A"),
        (('4', '6'), ">>A"),
        (('6', '4'), "<<A"),
        (('4', '7'), "^A"),
        (('7', '4'), "vA"),
        (('4', '8'), "^>A"),
        (('8', '4'), "<vA"),
        (('4', '9'), "^>>A"),
        (('9', '4'), "<<vA"),
        // 5 -> [5-9] and reverse
        (('5', '5'), "A"),
        (('5', '6'), ">A"),
        (('6', '5'), "<A"),
        (('5', '7'), "<^A"),
        (('7', '5'), "v>A"),
        (('5', '8'), "^A"),
        (('8', '5'), "vA"),
        (('5', '9'), "^>A"),
        (('9', '5'), "<vA"),
        // 6 -> [6-9] and reverse
        (('6', '6'), "A"),
        (('6', '7'), "<<^A"),
        (('7', '6'), "v>>A"),
        (('6', '8'), "<^A"),
        (('8', '6'), "v>A"),
        (('6', '9'), "^A"),
        (('9', '6'), "vA"),
        // 7 -> [7-9] and reverse
        (('7', '7'), "A"),
        (('7', '8'), ">A"),
        (('8', '7'), "<A"),
        (('7', '9'), ">>A"),
        (('9', '7'), "<<A"),
        // 8 -> [8-9] and reverse
        (('8', '8'), "A"),
        (('8', '9'), ">A"),
        (('9', '8'), "<A"),
        // 9 -> 9
        (('9', '9'), "A"),
        // directional keypad
        // < -> [<^<vA] and reverse
        (('<', '<'), "A"),
        (('<', '^'), ">^A"),
        (('^', '<'), "v<A"),
        (('<', 'v'), ">A"),
        (('v', '<'), "<A"),
        (('<', '>'), ">>A"),
        (('>', '<'), "<<A"),
        (('<', 'A'), ">>^A"),
        (('A', '<'), "v<<A"),
        // ^ -> [^v>A] and reverse
        (('^', '^'), "A"),
        (('^', 'v'), "vA"),
        (('v', '^'), "^A"),
        (('^', '>'), "v>A"),
        (('>', '^'), "<^A"),
        (('^', 'A'), ">A"),
        (('A', '^'), "<A"),
        // v -> [v>A] and reverse
        (('v', 'v'), "A"),
        (('v', 'A'), "^>A"),
        (('A', 'v'), "<vA"),
        (('v', '>'), ">A"),
        (('>', 'v'), "<A"),
        // > -> [>A] and reverse
        (('>', '>'), "A"),
        (('>', 'A'), "^A"),
        (('A', '>'), "vA"),
        // A -> A is already covered by the numeric keypad and it's the same as the directional keypad
    ])
}

struct Solver<'a> {
    paths: HashMap<(char, char), &'a str>,
    cache: HashMap<(&'a str, usize), usize>,
}

impl<'a> Solver<'a> {
    fn get_move_count(&mut self, current: char, next: char, depth: usize) -> usize {
        if current == next {
            return 1;
        }

        let new_sequence = self.paths[&(current, next)];
        self.get_sequence_length(new_sequence, depth)
    }

    fn get_sequence_length(&mut self, target: &'a str, depth: usize) -> usize {
        if let Some(&count) = self.cache.get(&(target, depth)) {
            return count;
        }

        let mut length = 0;
        if depth == 0 {
            length = target.len();
        } else {
            let mut current = 'A';
            for next in target.chars() {
                length += self.get_move_count(current, next, depth - 1);
                current = next;
            }
        }

        self.cache.insert((target, depth), length);
        length
    }

    fn score(&mut self, sequence: &'a str, depth: usize) -> usize {
        let value = sequence[..sequence.len() - 1].parse::<usize>().unwrap();
        let length = self.get_sequence_length(sequence, depth);
        value * length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/21/1.txt");
        assert_eq!(126384, part1(input));
    }
}
