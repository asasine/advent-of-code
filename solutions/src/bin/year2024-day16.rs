//! Day 16: Reindeer Maze
//!
//! https://adventofcode.com/2024/day/16

use core::fmt;
use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use itertools::Itertools;
use solutions::grid::{Coordinate, Direction, Grid};

fn part1(input: &str) -> usize {
    let maze = Maze::from_str(input).unwrap();
    let graph = Graph::from(&maze);
    eprintln!(
        "Graph with {} nodes and {} edges",
        graph.0.len(),
        graph.0.values().map(|edges| edges.len()).sum::<usize>()
    );

    let (cost, _) = graph.shortest_paths(maze.start(), maze.end());
    cost
}

fn part2(input: &str) -> usize {
    let maze = Maze::from_str(input).unwrap();
    let graph = Graph::from(&maze);
    let (_, paths) = graph.shortest_paths(maze.start(), maze.end());
    paths
        .iter()
        .flatten()
        .map(|path| path.coordinate)
        .unique()
        .count()
}

aoc_macro::aoc_main!();

enum Cell {
    Start,
    End,
    Wall,
    Open,
}

impl TryFrom<char> for Cell {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Open),
            _ => Err(value),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "S"),
            Self::End => write!(f, "E"),
            Self::Wall => write!(f, "#"),
            Self::Open => write!(f, "."),
        }
    }
}

struct Maze(Grid<Cell>);

impl FromStr for Maze {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self(grid))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    coordinate: Coordinate,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    source: Node,
    target: Node,
    cost: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    node: Node,
    cost: usize,
    path: Vec<Node>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // min heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&Maze> for Graph {
    fn from(maze: &Maze) -> Self {
        let mut adjacencies = HashMap::new();

        let is_cell_and_not_wall =
            |c: &Coordinate| maze.0.get(*c).is_some_and(|c| !matches!(c, Cell::Wall));

        // each coordinate gets four nodes, one for each direction, and edges for the turns
        for (coordinate, cell) in maze.0.enumerate() {
            if matches!(cell, Cell::Wall) {
                continue;
            }

            for direction in Direction::all() {
                let node = Node {
                    coordinate,
                    direction,
                };

                let neighbors = [
                    (1001, direction.turn_left()),
                    (1, direction),
                    (1001, direction.turn_right()),
                ]
                .into_iter()
                .filter_map(|(cost, direction)| {
                    coordinate
                        .try_move(direction)
                        .filter(is_cell_and_not_wall)
                        .map(|neighbor| Edge {
                            source: node,
                            target: Node {
                                coordinate: neighbor,
                                direction: direction,
                            },
                            cost: cost,
                        })
                });

                let edges = adjacencies.entry(node).or_insert_with(Vec::new);
                edges.extend(neighbors);
            }
        }

        Self(adjacencies)
    }
}

impl Maze {
    /// Get the starting node.
    fn start(&self) -> Node {
        self.0
            .enumerate()
            .find_map(|(coordinate, cell)| match cell {
                Cell::Start => Some(Node {
                    coordinate,
                    direction: Direction::Right,
                }),
                _ => None,
            })
            .unwrap()
    }

    /// Get the ending coordinate. This is not a [`Node`] because the direction is not specified for the puzzle.
    fn end(&self) -> Coordinate {
        self.0
            .enumerate()
            .find_map(|(coordinate, cell)| match cell {
                Cell::End => Some(coordinate),
                _ => None,
            })
            .unwrap()
    }
}

struct Graph(HashMap<Node, Vec<Edge>>);

impl Graph {
    fn out_edges(&self, node: Node) -> &[Edge] {
        self.0.get(&node).map_or(&[], |edges| edges.as_slice())
    }

    fn shortest_paths(&self, source: Node, destination: Coordinate) -> (usize, Vec<Vec<Node>>) {
        // Track the best paths to the destination.
        let mut paths = Vec::new();
        let mut best = std::usize::MAX;

        // Track the nodes we've visited and the cost to get there.
        let mut visited = HashMap::new();

        // Explore the nodes with lowest cost first (min heap).
        let mut frontier = BinaryHeap::new();
        frontier.push(State {
            node: source,
            cost: 0,
            path: vec![source],
        });

        while let Some(State { node, cost, path }) = frontier.pop() {
            // If we have already visited this node with a lower cost, skip it.
            if let Some(&prev_cost) = visited.get(&node) {
                if cost > prev_cost {
                    continue;
                }
            } else {
                visited.insert(node, cost);
            }

            // If we've reached the end, we've found one shortest path.
            // This is guaranteed to be a shortest path because we're using a min heap
            if node.coordinate == destination && cost <= best {
                paths.push(path.clone());
                best = cost;
            }

            for edge in self.out_edges(node) {
                frontier.push(State {
                    node: edge.target,
                    cost: cost + edge.cost,
                    path: {
                        let mut path = path.clone();
                        path.push(edge.target);
                        path
                    },
                });
            }
        }

        (best, paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("../../data/examples/2024/16/1.txt");
        assert_eq!(7036, part1(input));
    }

    #[test]
    fn part1_example2() {
        let input = include_str!("../../data/examples/2024/16/2.txt");
        assert_eq!(11048, part1(input));
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("../../data/examples/2024/16/1.txt");
        assert_eq!(45, part2(input));
    }

    #[test]
    fn part2_example2() {
        let input = include_str!("../../data/examples/2024/16/2.txt");
        assert_eq!(64, part2(input));
    }
}
