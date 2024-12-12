//! Day 12: Garden Groups
//!
//! https://adventofcode.com/2024/day/12

use core::{fmt, hash};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use solutions::grid::{Coordinate, Grid};

fn part1(input: &str) -> usize {
    let grid = Grid::<Plot>::from_str(input).unwrap();
    let garden = Garden { grid };
    let fencing = garden.fencing();
    fencing.values().map(|f| f.price()).sum()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<Plot>::from_str(input).unwrap();
    let garden = Garden { grid };
    let fencing = garden.fencing();
    fencing.values().map(|f| f.bulk_price()).sum()
}

aoc_macro::aoc_main!();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Plot(char);

impl From<char> for Plot {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A region is a contiguous group of plots.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    plot: Plot,
    plots: HashSet<Coordinate>,
}

impl hash::Hash for Region {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.plot.0.hash(state);
        for c in self.plots.iter() {
            c.hash(state);
        }
    }
}

impl Region {
    /// Calculates the area of the region, or the number of plots it contains.
    fn area(&self) -> usize {
        self.plots.len()
    }

    /// Calculates the perimeter of the region, or the number of plots on the edge of the region.
    fn perimeter(&self) -> usize {
        self.neighbors().map(|(_, n)| 4 - n).sum()
    }

    /// Calculates the number of sides of the region.
    fn sides(&self) -> usize {
        // The number of sides is equal to the number of corners in the region, concave and convex.
        self.plots
            .iter()
            .map(|c| {
                let (n, ne, e, se, s, sw, w, nw) = c
                    .moore()
                    .into_iter()
                    .map(|c| c.and_then(|c| self.plots.get(&c)))
                    .collect_tuple()
                    .expect("Moore neighborhood is always 8 elements");

                // 8 cases for 8 corners (concave and convex)
                let mut corners = 0;
                corners += (n.is_some() && w.is_some() && nw.is_none()) as usize;
                corners += (n.is_some() && e.is_some() && ne.is_none()) as usize;
                corners += (n.is_none() && w.is_none()) as usize;
                corners += (n.is_none() && e.is_none()) as usize;
                corners += (s.is_some() && e.is_some() && se.is_none()) as usize;
                corners += (s.is_some() && w.is_some() && sw.is_none()) as usize;
                corners += (s.is_none() && e.is_none()) as usize;
                corners += (s.is_none() && w.is_none()) as usize;
                corners
            })
            .sum()
    }

    fn fencing(&self) -> Fencing {
        Fencing {
            area: self.area(),
            perimeter: self.perimeter(),
            sides: self.sides(),
        }
    }

    /// Returns an iterator over coordinates in this region and the number of neighbors each has.
    fn neighbors(&self) -> impl Iterator<Item = (Coordinate, usize)> + '_ {
        self.plots.iter().map(|c| {
            let neighbors = c
                .von_neumann()
                .into_iter()
                .flatten()
                .filter(|c| self.plots.contains(c))
                .count();

            (*c, neighbors)
        })
    }
}

struct Garden {
    grid: Grid<Plot>,
}

impl Garden {
    /// Finds the contiguous region of plots containing the given coordinate.
    fn find_region(&self, coordinate: Coordinate) -> Option<Region> {
        let plot = match self.grid.get(coordinate) {
            Some(plot) => *plot,
            None => return None,
        };

        let mut region = Region {
            plot,
            plots: HashSet::new(),
        };

        let mut stack = vec![coordinate];
        while let Some(c) = stack.pop() {
            if region.plots.contains(&c) {
                continue;
            }

            region.plots.insert(c);

            let neighbors = c
                .von_neumann_within(self.grid.extent())
                .into_iter()
                .flatten()
                .filter_map(|c| self.grid.get(c).map(|p| (c, *p)));

            for (c, plot) in neighbors {
                if plot == region.plot {
                    stack.push(c);
                }
            }
        }

        Some(region)
    }

    fn find_regions(&self) -> HashSet<Region> {
        let mut regions = HashSet::new();
        let mut visited = HashSet::<Coordinate>::new();
        let mut remaining = self.grid.extent().into_iter().collect::<HashSet<_>>();
        while !remaining.is_empty() {
            let c = *remaining.iter().next().unwrap();
            if let Some(region) = self.find_region(c) {
                visited.extend(&region.plots);
                remaining = &remaining - &region.plots;
                regions.insert(region);
            }
        }

        regions
    }

    fn fencing(&self) -> HashMap<Region, Fencing> {
        self.find_regions()
            .into_iter()
            .map(|region| {
                let fencing = region.fencing();
                (region, fencing)
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fencing {
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Fencing {
    fn price(&self) -> usize {
        self.area * self.perimeter
    }

    fn bulk_price(&self) -> usize {
        self.area * self.sides
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            140,
            part1(
                r#"AAAA
BBCD
BBCC
EEEC"#
            )
        );

        assert_eq!(
            772,
            part1(
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#
            )
        );

        assert_eq!(
            1930,
            part1(
                r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#
            )
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            80,
            part2(
                r#"AAAA
BBCD
BBCC
EEEC"#
            )
        );

        assert_eq!(
            436,
            part2(
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#
            )
        );

        assert_eq!(
            236,
            part2(
                r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#
            )
        );

        assert_eq!(
            368,
            part2(
                r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#
            )
        );

        assert_eq!(
            1206,
            part2(
                r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#
            )
        );
    }

    #[test]
    fn find_region() {
        let garden = Garden {
            grid: Grid::<Plot>::from_str(
                r#"AAAA
BBCD
BBCC
EEEC"#,
            )
            .unwrap(),
        };

        let region = garden.find_region(Coordinate { x: 2, y: 1 }).unwrap(); // C
        assert_eq!(4, region.area());
        assert_eq!(10, region.perimeter());
        assert_eq!(8, region.sides());
    }
}
