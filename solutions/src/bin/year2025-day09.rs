//! Day 9: Movie Theater
//!
//! https://adventofcode.com/2025/day/9

use itertools::Itertools;
use solutions::grid::{Coordinate, Line, Rectangle};
use tracing::{instrument, trace, warn};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let coordinates = iter_coordinates(input).collect::<Option<Vec<_>>>().unwrap();

    coordinates
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| Rectangle { a, b })
        .inspect(|rectangle| trace!(?rectangle))
        .map(|r| r.area())
        .inspect(|area| trace!(area))
        .max()
        .unwrap_or(0)
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    // I checked and no two line segments intersect in the input data, so we don't have to worry about holes, just concavity.
    let coordinates = iter_coordinates(input).collect::<Option<Vec<_>>>().unwrap();
    let all_line_segments = coordinates
        .iter()
        .copied()
        .circular_tuple_windows()
        .map(|(a, b)| Line(a, b))
        .collect_vec();

    let all_rectangles = coordinates
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(a, b)| Rectangle { a, b })
        .inspect(|rectangle| trace!(?rectangle))
        .sorted_unstable_by_key(|r| r.area())
        .rev()
        .collect_vec();

    /// Check if an axis-aligned rectangle is valid, i.e. does not intersect with any of the line segments.
    fn is_valid(rectangle: &Rectangle, line_segments: &[Line]) -> bool {
        line_segments
            .iter()
            .all(|line| !is_intersecting(rectangle, line))
    }

    /// Check if an axis-aligned line segment intersects with an axis-aligned rectangle.
    fn is_intersecting(rectangle: &Rectangle, line: &Line) -> bool {
        let xmin = rectangle.a.x.min(rectangle.b.x);
        let ymin = rectangle.a.y.min(rectangle.b.y);
        let xmax = rectangle.a.x.max(rectangle.b.x);
        let ymax = rectangle.a.y.max(rectangle.b.y);

        let lxmin = line.0.x.min(line.1.x);
        let lymin = line.0.y.min(line.1.y);
        let lxmax = line.0.x.max(line.1.x);
        let lymax = line.0.y.max(line.1.y);

        if lxmin == lxmax {
            // line is vertical
            let lx = lxmin;
            if xmin < lx && lx < xmax {
                // line is strictly between the vertical edges of the rectangle
                if (lymin <= ymin && lymax > ymin) || (lymin < ymax && lymax >= ymax) {
                    // line intersects the top or bottom edge of the rectangle: invalid
                    return true;
                }
            }
        } else {
            // lymin == lymax
            // line is horizontal
            let ly = lymin;
            if ymin < ly && ly < ymax {
                // line is strictly between the horizontal edges of the rectangle
                if (lxmin <= xmin && lxmax > xmin) || (lxmin < xmax && lxmax >= xmax) {
                    // line intersects the left or right edge of the rectangle: invalid
                    return true;
                }
            }
        }

        false
    }

    // all_rectangles is sorted descending, and we want the largest, so the first valid one will do
    all_rectangles
        .into_iter()
        .find(|r| is_valid(r, &all_line_segments))
        .map(|r| r.area())
        .unwrap_or(0)
}

fn main() {
    solutions::main(part1, part2);
}

/// Parse the input into an iterator of coordinates.
fn iter_coordinates(input: &str) -> impl Iterator<Item = Option<Coordinate>> {
    input
        .lines()
        .map(|line| line.split_once(','))
        .map(Option::unzip)
        .map(|(x, y)| {
            Some(Coordinate {
                x: x?.parse().ok()?,
                y: y?.parse().ok()?,
            })
        })
}

#[cfg(test)]
mod tests {
    use solutions::setup_tracing;

    use super::*;

    #[test]
    fn part1_example() {
        setup_tracing();
        let input = include_str!("../../data/examples/2025/09/1.txt");
        assert_eq!(50, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/09/1.txt");
        assert_eq!(24, part2(input));
    }
}
