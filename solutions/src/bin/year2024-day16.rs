//! Day 16: Reindeer Maze
//!
//! https://adventofcode.com/2024/day/16

use core::fmt;
use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use solutions::grid::{Coordinate, Direction, Grid};

fn part1(input: &str) -> usize {
    let maze = Maze::from_str(input).unwrap();
    let graph = Graph::from(&maze);
    let start = maze.start();
    let search = graph.single_source(start);
    let end = maze.end();
    Direction::all()
        .into_iter()
        .filter_map(|direction| {
            let node = Node {
                coordinate: end,
                direction,
            };

            search.cost_to(node)
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    0
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    node: Node,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // deterministically order states by cost, then coordinate.x, then .y, then direction
        self.cost
            .cmp(&other.cost)
            .then_with(|| self.node.coordinate.x.cmp(&other.node.coordinate.x))
            .then_with(|| self.node.coordinate.y.cmp(&other.node.coordinate.y))
            .then_with(|| {
                // arbitrary, just needs to be consistent
                (self.node.direction as usize).cmp(&(other.node.direction as usize))
            })
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

                let edges = adjacencies.entry(node).or_insert_with(Vec::new);
                edges.push(Edge {
                    source: node,
                    target: Node {
                        coordinate,
                        direction: direction.turn_right(),
                    },
                    cost: 1000,
                });

                edges.push(Edge {
                    source: node,
                    target: Node {
                        coordinate,
                        direction: direction.turn_left(),
                    },
                    cost: 1000,
                });

                if let Some(neighbor) = coordinate.try_move(direction).filter(|next| {
                    maze.0
                        .get(*next)
                        .is_some_and(|cell| !matches!(cell, Cell::Wall))
                }) {
                    edges.push(Edge {
                        source: node,
                        target: Node {
                            coordinate: neighbor,
                            direction,
                        },
                        cost: 1,
                    });
                }
            }
        }

        Self(adjacencies)
    }
}

impl Maze {
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
    fn single_source(&self, source: Node) -> SingleSource {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();
        let mut predecessors = HashMap::new();
        distances.insert(source, 0);
        heap.push(State {
            node: source,
            cost: 0,
        });

        while let Some(State { cost, node }) = heap.pop() {
            if cost > distances[&node] {
                // we've already found a lower cost to this node
                continue;
            }

            for edge in &self.0[&node] {
                let next = State {
                    cost: cost + edge.cost,
                    node: edge.target,
                };

                let distance = distances.entry(next.node).or_insert(usize::MAX);
                if next.cost > *distance {
                    // not a better path
                    continue;
                }

                // this path is at least as good
                *distance = next.cost;
                heap.push(next);
                let predecessors = predecessors.entry(next.node).or_insert_with(Vec::new);
                if next.cost < *distance {
                    // this path is better, so erase all predecessors
                    predecessors.clear();
                }

                predecessors.push(*edge);
            }
        }

        SingleSource {
            graph: self,
            source,
            distances,
            predecessors,
        }
    }
}

struct SingleSource<'a> {
    graph: &'a Graph,
    source: Node,
    distances: HashMap<Node, usize>,
    predecessors: HashMap<Node, Vec<Edge>>,
}

impl<'a> SingleSource<'a> {
    /// Returns a shortest path from the source to the end.
    ///
    /// Note: there may be multiple paths with the same cost, so this function returns the first one found.
    fn shortest_path(&self, end: Node) -> Option<Path> {
        let mut path = Vec::new();
        let mut node = end;
        while node != self.source {
            let edge = self.predecessors(node).first()?;
            path.push(*edge);
            node = edge.source;
        }

        path.reverse();
        Some(Path(path))
    }

    /// Returns all shortest path from the source to the end.
    fn shortest_paths(&self, end: Node) -> Option<Vec<Path>> {
        let paths = self.shortest_paths_recursive(end, vec![]);
        Some(paths.into_iter().map(Path).collect())
    }

    /// Recursively finds all shortest paths from the source to the end.
    fn shortest_paths_recursive(&self, end: Node, path: Vec<Edge>) -> Vec<Vec<Edge>> {
        if end == self.source {
            return vec![path];
        }

        self.predecessors(end)
            .iter()
            .flat_map(|edge| {
                let mut path = path.clone();
                path.push(*edge);
                self.shortest_paths_recursive(edge.source, path)
            })
            // .filter(|path| path.len() > 0)
            .collect()
    }

    /// Gets the predecessors of a node.
    fn predecessors(&self, node: Node) -> &[Edge] {
        self.predecessors
            .get(&node)
            .map_or(&[], |edges| edges.as_slice())
    }

    fn cost_to(&self, node: Node) -> Option<usize> {
        self.distances.get(&node).copied()
    }
}

struct Path(Vec<Edge>);

impl Path {
    fn cost(&self) -> usize {
        self.0.iter().map(|edge| edge.cost).sum()
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

    #[test]
    fn cost() {
        let path = Path(vec![
            Edge {
                source: Node {
                    coordinate: Coordinate { x: 0, y: 0 },
                    direction: Direction::Right,
                },
                target: Node {
                    coordinate: Coordinate { x: 1, y: 0 },
                    direction: Direction::Right,
                },
                cost: 1,
            },
            Edge {
                source: Node {
                    coordinate: Coordinate { x: 1, y: 0 },
                    direction: Direction::Right,
                },
                target: Node {
                    coordinate: Coordinate { x: 1, y: 0 },
                    direction: Direction::Down,
                },
                cost: 1000,
            },
            Edge {
                source: Node {
                    coordinate: Coordinate { x: 1, y: 0 },
                    direction: Direction::Down,
                },
                target: Node {
                    coordinate: Coordinate { x: 1, y: 1 },
                    direction: Direction::Down,
                },
                cost: 1,
            },
        ]);

        assert_eq!(1002, path.cost());
    }

    fn test_graph() -> Graph {
        let maze = r#"####
#S.#
#.E#
####"#;

        let maze = Maze::from_str(maze).unwrap();
        Graph::from(&maze)
    }

    #[test]
    fn cost_to() {
        let graph = test_graph();
        let search = graph.single_source(Node {
            coordinate: Coordinate { x: 1, y: 1 },
            direction: Direction::Right,
        });

        let actual = search.cost_to(Node {
            coordinate: Coordinate { x: 1, y: 2 },
            direction: Direction::Down,
        });

        assert_eq!(Some(1001), actual);
    }

    #[test]
    fn shortest_path() {
        let graph = test_graph();
        let search = graph.single_source(Node {
            coordinate: Coordinate { x: 1, y: 1 },
            direction: Direction::Right,
        });

        let path = search.shortest_path(Node {
            coordinate: Coordinate { x: 1, y: 2 },
            direction: Direction::Down,
        });

        assert_eq!(
            Some(vec![
                Edge {
                    source: Node {
                        coordinate: Coordinate { x: 1, y: 1 },
                        direction: Direction::Right,
                    },
                    target: Node {
                        coordinate: Coordinate { x: 1, y: 1 },
                        direction: Direction::Down,
                    },
                    cost: 1000,
                },
                Edge {
                    source: Node {
                        coordinate: Coordinate { x: 1, y: 1 },
                        direction: Direction::Down,
                    },
                    target: Node {
                        coordinate: Coordinate { x: 1, y: 2 },
                        direction: Direction::Down,
                    },
                    cost: 1,
                }
            ]),
            path.map(|path| path.0)
        );
    }

    #[test]
    fn shortest_paths() {
        let graph = test_graph();
        let search = graph.single_source(Node {
            coordinate: Coordinate { x: 1, y: 1 },
            direction: Direction::Right,
        });

        let paths = search.shortest_paths(Node {
            coordinate: Coordinate { x: 2, y: 2 },
            direction: Direction::Down,
        });

        assert_eq!(
            Some(vec![
                vec![
                    Edge {
                        source: Node {
                            coordinate: Coordinate { x: 1, y: 1 },
                            direction: Direction::Right,
                        },
                        target: Node {
                            coordinate: Coordinate { x: 1, y: 1 },
                            direction: Direction::Down,
                        },
                        cost: 1000,
                    },
                    Edge {
                        source: Node {
                            coordinate: Coordinate { x: 1, y: 1 },
                            direction: Direction::Down,
                        },
                        target: Node {
                            coordinate: Coordinate { x: 1, y: 2 },
                            direction: Direction::Down,
                        },
                        cost: 1,
                    },
                    Edge {
                        source: Node {
                            coordinate: Coordinate { x: 1, y: 2 },
                            direction: Direction::Down,
                        },
                        target: Node {
                            coordinate: Coordinate { x: 1, y: 2 },
                            direction: Direction::Right,
                        },
                        cost: 1000,
                    },
                    Edge {
                        source: Node {
                            coordinate: Coordinate { x: 1, y: 2 },
                            direction: Direction::Right,
                        },
                        target: Node {
                            coordinate: Coordinate { x: 2, y: 2 },
                            direction: Direction::Right,
                        },
                        cost: 1,
                    },
                ],
                vec![
                    Edge {
                        source: Node {
                            coordinate: Coordinate { x: 1, y: 1 },
                            direction: Direction::Right,
                        },
                        target: Node {
                            coordinate: Coordinate { x: 2, y: 1 },
                            direction: Direction::Right,
                        },
                        cost: 1,
                    },
                    Edge {
                        source: Node {
                            coordinate: Coordinate { x: 2, y: 1 },
                            direction: Direction::Right,
                        },
                        target: Node {
                            coordinate: Coordinate { x: 2, y: 1 },
                            direction: Direction::Down,
                        },
                        cost: 1000,
                    },
                    Edge {
                        source: Node {
                            coordinate: Coordinate { x: 2, y: 1 },
                            direction: Direction::Down,
                        },
                        target: Node {
                            coordinate: Coordinate { x: 2, y: 2 },
                            direction: Direction::Down,
                        },
                        cost: 1,
                    },
                    Edge {
                        source: Node {
                            coordinate: Coordinate { x: 2, y: 2 },
                            direction: Direction::Down,
                        },
                        target: Node {
                            coordinate: Coordinate { x: 2, y: 2 },
                            direction: Direction::Right,
                        },
                        cost: 1000,
                    },
                ]
            ]),
            paths.map(|paths| paths.into_iter().map(|path| path.0).collect())
        );
    }
}
