//! Day 14: Restroom Redoubt
//!
//! https://adventofcode.com/2024/day/14

use core::fmt;
use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use nalgebra::Vector2;

fn part1(input: &str) -> usize {
    part1_impl::<101, 103>(input)
}

fn part1_impl<const WIDTH: usize, const HEIGHT: usize>(input: &str) -> usize {
    let mut grid: WrappingGrid<WIDTH, HEIGHT> = input.parse().unwrap();
    grid.step_n(100);
    eprintln!("{}", grid);
    grid.safety_factor()
}

fn part2(input: &str) -> usize {
    let mut original_grid: WrappingGrid<101, 103> = input.parse().unwrap();
    let mut grid = original_grid.clone();
    let tree_probably = (0usize..(101 * 103))
        .map(|i| {
            grid.step();
            // eprintln!("{}:\n{}\n", i, grid);
            (i + 1, grid.safety_factor())
        })
        .sorted_by_key(|(_, safety_factor)| *safety_factor)
        .next()
        .unwrap()
        .0;

    original_grid.step_n(tree_probably);
    eprintln!("{}", original_grid);
    tree_probably
}

aoc_macro::aoc_main!();

type Vector2i = Vector2<i64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(Vector2i);

impl FromStr for Position {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // form: p=0,4
        s.split('=') // ["p", "0,4"]
            .nth(1) // "0,4"
            .and_then(|s| {
                let mut parts = s.split(',');
                let x = parts.next()?.parse().ok()?;
                let y = parts.next()?.parse().ok()?;
                Some(Position(Vector2i::new(x, y)))
            })
            .ok_or(())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0.x, self.0.y)
    }
}

impl Position {
    /// Wrap the position around a grid, handling if it goes off the edge.
    ///
    /// This includes if the position is negative.
    fn wrap(&mut self, width: i64, height: i64) {
        self.0.x = self.0.x.rem_euclid(width);
        self.0.y = self.0.y.rem_euclid(height);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Velocity(Vector2i);

impl FromStr for Velocity {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // form: v=3,-3
        s.split('=') // ["v", "3,-3"]
            .nth(1) // "3,-3"
            .and_then(|s| {
                let mut parts = s.split(',');
                let x = parts.next()?.parse().ok()?;
                let y = parts.next()?.parse().ok()?;
                Some(Velocity(Vector2i::new(x, y)))
            })
            .ok_or(())
    }
}

#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl FromStr for Robot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // form: p=0,4 v=3,-3
        let mut parts = s.split(' ');
        let position = parts.next().ok_or(())?.parse()?;
        let velocity = parts.next().ok_or(())?.parse()?;
        Ok(Robot { position, velocity })
    }
}

#[derive(Debug, Clone)]
struct WrappingGrid<const WIDTH: usize, const HEIGHT: usize> {
    robots: Vec<Robot>,
}

impl<const WIDTH: usize, const HEIGHT: usize> FromStr for WrappingGrid<WIDTH, HEIGHT> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let robots = s.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(WrappingGrid { robots })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> WrappingGrid<WIDTH, HEIGHT> {
    fn step(&mut self) {
        for robot in &mut self.robots {
            robot.position.0 += robot.velocity.0;
            robot.position.wrap(WIDTH as i64, HEIGHT as i64);
        }
    }

    fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// The number of robots at each position.
    fn locations(&self) -> HashMap<Position, usize> {
        let mut locations = HashMap::new();
        for robot in &self.robots {
            *locations.entry(robot.position).or_default() += 1;
        }

        locations
    }

    /// The product of the number of robots in each quadrant.
    fn safety_factor(&self) -> usize {
        self.locations()
            .into_iter()
            .into_grouping_map_by(|(position, _)| Quadrant::from_position(*position, WIDTH, HEIGHT))
            .fold(0, |acc, _, (_, count)| acc + count)
            .into_iter()
            .filter_map(|(quadrant, count)| quadrant.map(|_| count))
            .product()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for WrappingGrid<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locations = self.locations();
        // let single_biggest = locations.values().max().copied().unwrap_or(0);
        // let digits = solutions::num::usize::digits(single_biggest);
        // let blank_format_args = format_args!("{:..{}}", digits);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let position = Position(Vector2i::new(x as i64, y as i64));
                let count = locations.get(&position).copied().unwrap_or(0);
                if count >= 10 {
                    write!(f, "+")?;
                } else if count > 0 {
                    write!(f, "{}", count)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    /// Determine the quadrant of a position.
    ///
    /// If width or height is odd, positions in the middle are not considered to be in any quadrant.
    fn from_position(position: Position, width: usize, height: usize) -> Option<Self> {
        let x = position.0.x;
        let y = position.0.y;
        let left_width_cutoff = width as i64 / 2;
        let right_min_width = (width as i64 + 1) / 2;
        let top_height_cutoff = height as i64 / 2;
        let bottom_min_height = (height as i64 + 1) / 2;
        if x < left_width_cutoff && y < top_height_cutoff {
            Some(Quadrant::TopLeft)
        } else if x >= right_min_width && y < top_height_cutoff {
            Some(Quadrant::TopRight)
        } else if x < left_width_cutoff && y >= bottom_min_height {
            Some(Quadrant::BottomLeft)
        } else if x >= right_min_width && y >= bottom_min_height {
            Some(Quadrant::BottomRight)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

        assert_eq!(12, part1_impl::<11, 7>(input));
    }

    // #[test]
    // fn part2_example() {
    //     let input = include_str!("../../data/examples/2024/14/1.txt");
    //     assert_eq!(0, part2(input));
    // }

    #[test]
    fn wrap() {
        let mut position = Position(Vector2i::new(0, 0));
        position.wrap(3, 3);
        assert_eq!(Position(Vector2i::new(0, 0)), position);

        let mut position = Position(Vector2i::new(3, 3));
        position.wrap(3, 3);
        assert_eq!(Position(Vector2i::new(0, 0)), position);

        let mut position = Position(Vector2i::new(-1, -1));
        position.wrap(3, 3);
        assert_eq!(Position(Vector2i::new(2, 2)), position);
    }

    #[test]
    fn from_position() {
        assert_eq!(
            Some(Quadrant::TopLeft),
            Quadrant::from_position(Position(Vector2i::new(0, 0)), 3, 3)
        );
        assert_eq!(
            Some(Quadrant::TopRight),
            Quadrant::from_position(Position(Vector2i::new(2, 0)), 3, 3)
        );
        assert_eq!(
            Some(Quadrant::BottomLeft),
            Quadrant::from_position(Position(Vector2i::new(0, 2)), 3, 3)
        );
        assert_eq!(
            Some(Quadrant::BottomRight),
            Quadrant::from_position(Position(Vector2i::new(2, 2)), 3, 3)
        );
        assert_eq!(
            None,
            Quadrant::from_position(Position(Vector2i::new(0, 1)), 3, 3)
        );
        assert_eq!(
            None,
            Quadrant::from_position(Position(Vector2i::new(1, 1)), 3, 3)
        );
        assert_eq!(
            None,
            Quadrant::from_position(Position(Vector2i::new(2, 1)), 3, 3)
        );
        assert_eq!(
            None,
            Quadrant::from_position(Position(Vector2i::new(1, 0)), 3, 3)
        );
        assert_eq!(
            None,
            Quadrant::from_position(Position(Vector2i::new(1, 2)), 3, 3)
        );
    }
}
