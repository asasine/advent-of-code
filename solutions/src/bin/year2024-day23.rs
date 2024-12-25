//! Day 23: LAN Party
//!
//! https://adventofcode.com/2024/day/23

use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use tracing::{debug, instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let connections: Connections = input.parse().unwrap();
    debug!(
        "{} computers, {} connections",
        connections.computers.len(),
        connections.connections.len()
    );

    let cliques = connections.cliques_of_three();
    debug!("{} cliques of three", cliques.len());
    trace!("cliques: {:?}", cliques.iter().sorted());

    let cliques_with_t = connections.cliques_of_three_with_t_computers();
    debug!(
        "{} cliques of three with 't' computer",
        cliques_with_t.len(),
    );

    trace!(
        "cliques with 't' computer: {:?}",
        cliques_with_t.iter().sorted()
    );

    cliques_with_t.len()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> String {
    let connections: Connections = input.parse().unwrap();
    debug!(
        "{} computers, {} connections",
        connections.computers.len(),
        connections.connections.len()
    );

    let maximal_clique = {
        let maximal_clique = connections.maximal_clique();
        maximal_clique.into_iter().sorted().collect_vec()
    };

    debug!("maximal clique has {} computers", maximal_clique.len());
    trace!("maximal clique: {:?}", maximal_clique);

    maximal_clique
        .into_iter()
        .map(|computer| computer.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    solutions::main(part1, part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Computer(char, char);

impl FromStr for Computer {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let a = chars.next().ok_or(())?;
        let b = chars.next().ok_or(())?;
        Ok(Computer(a, b))
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connection(Computer, Computer);

impl FromStr for Connection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut computers = s.split('-');
        let a = computers.next().ok_or(())?.parse().map_err(|_| ())?;
        let b = computers.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Connection(a, b))
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Clique(Computer, Computer, Computer);

impl fmt::Display for Clique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.0, self.1, self.2)
    }
}

impl FromIterator<Computer> for Clique {
    fn from_iter<T: IntoIterator<Item = Computer>>(iter: T) -> Self {
        let mut iter = iter.into_iter().sorted();
        let a = iter.next().unwrap();
        let b = iter.find(|&c| c != a).unwrap();
        let c = iter.find(|&c| c != a && c != b).unwrap();

        Clique(a, b, c)
    }
}

#[derive(Debug)]
struct Connections {
    connections: Vec<Connection>,
    computers: HashSet<Computer>,
    connections_by_computer: HashMap<Computer, HashSet<Computer>>,
}

impl Connections {
    fn new(connections: Vec<Connection>) -> Self {
        let computers = connections
            .iter()
            .flat_map(|Connection(a, b)| vec![*a, *b])
            .collect();

        let mut connections_by_computer = HashMap::new();
        for Connection(a, b) in &connections {
            connections_by_computer
                .entry(*a)
                .or_insert_with(HashSet::new)
                .insert(*b);

            connections_by_computer
                .entry(*b)
                .or_insert_with(HashSet::new)
                .insert(*a);
        }

        Self {
            connections,
            computers,
            connections_by_computer,
        }
    }

    fn cliques_of_three(&self) -> HashSet<Clique> {
        self.maximal_cliques()
            .iter()
            .filter(|clique| clique.len() >= 3)
            .flat_map(|clique| {
                clique
                    .iter()
                    .tuple_combinations()
                    .map(|(&a, &b, &c)| Clique::from_iter([a, b, c]))
            })
            .collect()
    }

    fn cliques_of_three_with_t_computers(&self) -> HashSet<Clique> {
        self.cliques_of_three()
            .into_iter()
            .filter(|clique| clique.0 .0 == 't' || clique.1 .0 == 't' || clique.2 .0 == 't')
            .collect()
    }

    /// Finds the largest maximal clique in the graph.
    fn maximal_clique(&self) -> HashSet<Computer> {
        let cliques = self.maximal_cliques();
        cliques
            .into_iter()
            .max_by_key(|clique| clique.len())
            .unwrap()
    }

    /// Finds maximal cliques in a graph using the [Bron-Kerbosch algorithm]([Bron-Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm).
    fn maximal_cliques(&self) -> Vec<HashSet<Computer>> {
        let mut p = self.computers.clone();
        let mut x = HashSet::new();
        let mut cliques = Vec::new();

        // BronKerbosch3: order by degeneracy
        let r = self
            .computers
            .iter()
            .sorted_by_key(|c| self.connections_by_computer[c].len())
            .rev();

        for v in r {
            let v_neighbors = &self.connections_by_computer[v];
            self.maximal_cliques_impl(
                HashSet::from([*v]),
                &p & v_neighbors,
                &x & v_neighbors,
                &mut cliques,
            );

            p.remove(v);
            x.insert(*v);
        }

        debug!("{} maximal cliques", cliques.len());
        trace!("maximal cliques: {:?}", cliques);
        cliques
    }

    /// Finds maximal cliques in a graph using the [Bron-Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm).
    fn maximal_cliques_impl(
        &self,
        r: HashSet<Computer>,
        mut p: HashSet<Computer>,
        mut x: HashSet<Computer>,
        cliques: &mut Vec<HashSet<Computer>>,
    ) {
        if p.is_empty() && x.is_empty() {
            cliques.push(r);
            return;
        }

        // BronKerbosch2: use a pivot to reduce the number of recursive calls
        let pivot = p.union(&x).next().unwrap();
        for v in &p - &self.connections_by_computer[pivot] {
            let v_neighbors = &self.connections_by_computer[&v];
            self.maximal_cliques_impl(
                &r | &HashSet::from([v]),
                &p & v_neighbors,
                &x & v_neighbors,
                cliques,
            );

            p.remove(&v);
            x.insert(v);
        }
    }
}

impl FromStr for Connections {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let connections = s
            .lines()
            .map(|line| line.parse().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Connections::new(connections))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/23/1.txt");
        assert_eq!(7, part1(input));
    }

    #[test]
    fn part2_example() {
        solutions::setup_tracing();
        let input = include_str!("../../data/examples/2024/23/1.txt");
        assert_eq!("co,de,ka,ta", part2(input));
    }
}
