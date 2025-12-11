//! Day 11: Reactor
//!
//! https://adventofcode.com/2025/day/11

use core::{fmt::Display, hash::Hash, str::FromStr};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let lines: Lines = input.parse().unwrap();
    lines.count_paths(Id::YOU, Id::OUT)
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let lines: Lines = input.parse().unwrap();
    lines.count_paths_that_visit(Id::SVR, Id::OUT, &[Id::DAC, Id::FFT])
}

fn main() {
    solutions::main(part1, part2);
}

struct Lines(HashMap<Id, Outputs>);

impl Lines {
    /// Count the number of paths starting from `source` and ending at `target`.
    ///
    /// The search starts from the line whose [`Line::device`] is `source` until all lines that contain `target` in
    /// their [`Line::outputs`] have been found.
    fn count_paths(&self, source: Id, target: Id) -> usize {
        self.count_paths_impl(
            source,
            target,
            &mut HashSet::new(),
            &mut Cache::Simple(HashMap::new()),
            &RequiredMask::empty(),
        )
    }

    /// Count the number of paths starting from `source` and ending at `target` that visit all nodes in `must_visit`.
    ///
    /// The search starts from the line whose [`Line::device`] is `source` until all lines that contain `target` in
    /// their [`Line::outputs`] have been found. The returned count only includes paths that visit all nodes in
    /// `must_visit`.
    fn count_paths_that_visit(&self, source: Id, target: Id, must_visit: &[Id]) -> usize {
        self.count_paths_impl(
            source,
            target,
            &mut HashSet::new(),
            &mut Cache::WithRequired(HashMap::new()),
            &RequiredMask::from_required(must_visit),
        )
    }

    /// Count the number of paths from `source` to `target`.
    ///
    /// - `visited` keeps track of nodes already visited in the current path to avoid cycles.
    /// - `cache` stores previously computed path counts to optimize performanc
    /// - `required_mask` indicates which nodes must be visited for the path to be valid.
    ///
    /// For simple path counting (part 1), use [`Cache::Simple`] and [`RequiredMask::empty()`].
    ///
    /// For path counting with required nodes (part 2), use [`Cache::WithRequired`] and a proper [`RequiredMask`].
    fn count_paths_impl(
        &self,
        source: Id,
        target: Id,
        visited: &mut HashSet<Id>,
        cache: &mut Cache,
        required_mask: &RequiredMask,
    ) -> usize {
        if let Some(count) = cache.get(source, target, visited, required_mask) {
            // value already cached, skip this branch
            return count;
        }

        if source == target {
            // only count this path if we've visited all required nodes (or if there are none)
            let count = if required_mask.all_visited(visited) {
                1
            } else {
                0
            };

            cache.insert(source, target, visited, required_mask, count);
            return count;
        }

        visited.insert(source);

        let count = if let Some(adjacent) = self.0.get(&source) {
            let mut count = 0;
            for next in adjacent.0.iter() {
                if visited.contains(next) {
                    continue;
                }

                count += self.count_paths_impl(*next, target, visited, cache, required_mask);
            }

            count
        } else {
            0
        };

        cache.insert(source, target, visited, required_mask, count);
        visited.remove(&source);
        count
    }
}

/// A bitmask representing a set of required nodes to visit.
///
/// Maps each required node to a bit position for efficient set operations.
struct RequiredMask {
    /// Maps each required node ID to its bit position (0-7).
    id_to_bit: HashMap<Id, u8>,

    /// The bitmask representing all required nodes.
    full_mask: u8,
}

impl RequiredMask {
    /// Create a required mask from a list of required nodes.
    fn from_required(required_nodes: &[Id]) -> Self {
        let id_to_bit: HashMap<Id, u8> = required_nodes
            .iter()
            .enumerate()
            .map(|(i, &id)| (id, i as u8))
            .collect();

        let full_mask = if required_nodes.is_empty() {
            0
        } else {
            (1u8 << required_nodes.len()) - 1
        };

        RequiredMask {
            id_to_bit,
            full_mask,
        }
    }

    /// Create an empty required mask with no required nodes.
    fn empty() -> Self {
        RequiredMask {
            id_to_bit: HashMap::new(),
            full_mask: 0,
        }
    }

    /// Check if all required nodes have been visited.
    fn all_visited(&self, visited: &HashSet<Id>) -> bool {
        if self.full_mask == 0 {
            return true;
        }

        self.visited_mask(visited) == self.full_mask
    }

    /// Create a bitmask of which required nodes have been visited.
    fn visited_mask(&self, visited: &HashSet<Id>) -> u8 {
        self.id_to_bit
            .iter()
            .filter_map(|(&id, &bit)| visited.contains(&id).then_some(1u8 << bit))
            .fold(0, |acc, mask| acc | mask)
    }
}

/// Cache for storing previously computed path counts.
enum Cache {
    /// Cache for part 1, where no required nodes need to be visited.
    ///
    /// Since the path only depends on source and target, we can use a simple key.
    Simple(HashMap<(Id, Id), usize>),

    /// Cache for part 2, where certain nodes need to be visited.
    ///
    /// Since the path depends on source, target, and which required nodes have been visited, we use a bitmask
    /// to efficiently represent the set of visited required nodes.
    WithRequired(HashMap<(Id, Id, u8), usize>),
}

impl Cache {
    /// Retrieve a cached value if it exists.
    fn get(
        &self,
        source: Id,
        target: Id,
        visited: &HashSet<Id>,
        required_mask: &RequiredMask,
    ) -> Option<usize> {
        match self {
            Cache::Simple(map) => map.get(&(source, target)).copied(),
            Cache::WithRequired(map) => {
                let visited_mask = required_mask.visited_mask(visited);
                map.get(&(source, target, visited_mask)).copied()
            }
        }
    }

    /// Insert a value into the cache.
    ///
    /// `visited` and `required_mask` are used to determine the correct key for the cache in the [`Cache::WithRequired`]
    /// case.
    fn insert(
        &mut self,
        source: Id,
        target: Id,
        visited: &HashSet<Id>,
        required_mask: &RequiredMask,
        count: usize,
    ) {
        match self {
            Cache::Simple(map) => {
                map.insert((source, target), count);
            }
            Cache::WithRequired(map) => {
                let visited_mask = required_mask.visited_mask(visited);
                map.insert((source, target, visited_mask), count);
            }
        }
    }
}

impl FromStr for Lines {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for line in s.lines() {
            let Line { device, outputs } = line.parse()?;
            map.insert(device, outputs);
        }

        Ok(Lines(map))
    }
}

struct Line {
    device: Id,
    outputs: Outputs,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.device)?;
        write!(f, "{}", self.outputs)
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (device, outputs) = s.split_once(": ").ok_or(())?;
        Ok(Line {
            device: device.parse()?,
            outputs: outputs.parse()?,
        })
    }
}

struct Outputs(Vec<Id>);

impl FromStr for Outputs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outputs = s.split(' ').map(str::parse).try_collect()?;
        Ok(Outputs(outputs))
    }
}

impl Display for Outputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, output) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }

            write!(f, "{}", output)?;
        }
        Ok(())
    }
}

/// An identifier consisting of three lowercase ASCII letters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Id(u16);

impl Id {
    pub const YOU: Self = Id::new(*b"you");
    pub const OUT: Self = Id::new(*b"out");
    pub const DAC: Self = Id::new(*b"dac");
    pub const FFT: Self = Id::new(*b"fft");
    pub const SVR: Self = Id::new(*b"svr");

    /// Creates a new identifier from an array of three bytes.
    const fn new(bytes: [u8; 3]) -> Self {
        let [a, b, c] = bytes;
        let first = (a - b'a') as u16;
        let second = (b - b'a') as u16;
        let third = (c - b'a') as u16;
        Id(first * (26 * 26) + second * 26 + third)
    }

    /// Returns the identifier as an array of three characters.
    fn id(&self) -> [char; 3] {
        let first = ((self.0 / 676) % 26) as u8 + b'a';
        let second = ((self.0 / 26) % 26) as u8 + b'a';
        let third = (self.0 % 26) as u8 + b'a';
        [first as char, second as char, third as char]
    }
}

impl TryFrom<[u8; 3]> for Id {
    type Error = ();

    fn try_from(value: [u8; 3]) -> Result<Self, Self::Error> {
        if !value.iter().all(|b| b.is_ascii_lowercase()) {
            return Err(());
        }

        Ok(Id::new(value))
    }
}

impl FromStr for Id {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (bytes, _) = s.as_bytes().split_first_chunk().ok_or(())?;
        Self::try_from(*bytes)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [a, b, c] = self.id();
        write!(f, "{}{}{}", a, b, c)
    }
}

#[cfg(test)]
mod tests {
    use solutions::setup_tracing;

    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2025/11/1.txt");
        assert_eq!(5, part1(input));
    }

    #[test]
    fn part2_example() {
        setup_tracing();
        let input = include_str!("../../data/examples/2025/11/2.txt");
        assert_eq!(2, part2(input));
    }
}
