//! Day 20: Race Condition
//!
//! https://adventofcode.com/2024/day/20

use std::{collections::HashMap, str::FromStr};

use solutions::grid::{Coordinate, Grid};
use tracing::{debug, instrument, trace};

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    part1_impl(input, 100)
}

fn part1_impl(input: &str, min_savings: usize) -> usize {
    count_cheats(
        input,
        Cheat {
            picoseconds: 2,
            min_savings,
        },
    )
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    part2_impl(input, 100)
}

fn part2_impl(input: &str, min_savings: usize) -> usize {
    count_cheats(
        input,
        Cheat {
            picoseconds: 20,
            min_savings,
        },
    )
}

fn count_cheats(input: &str, cheat: Cheat) -> usize {
    let grid = input.parse::<Racetrack>().unwrap();
    grid.count_cheat_edges(&cheat)
}

fn main() {
    solutions::main(part1, part2)
}

struct Racetrack {
    /// The path from the start to the end.
    track: Vec<Coordinate>,
    end: Coordinate,
    distances_to_end: HashMap<Coordinate, usize>,
}

impl FromStr for Racetrack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.parse::<Grid<Cell>>()?;
        debug!(
            "Grid is {}x{}",
            grid.extent().width(),
            grid.extent().height()
        );

        let start = grid
            .enumerate()
            .find_map(|(coord, cell)| {
                if let Cell::Track(TrackType::Start) = cell {
                    Some(coord)
                } else {
                    None
                }
            })
            .ok_or("No start")?;

        let end = grid
            .enumerate()
            .find_map(|(coord, cell)| {
                if let Cell::Track(TrackType::End) = cell {
                    Some(coord)
                } else {
                    None
                }
            })
            .ok_or("No end")?;

        let track = Self::find_track(grid, start, end);
        debug!("Track has {} cells", track.len());

        let distances_to_end = track
            .iter()
            .enumerate()
            .map(|(index, &coord)| {
                let distance = track.len() - index - 1;
                (coord, distance)
            })
            .collect();

        Ok(Self {
            track,
            end,
            distances_to_end,
        })
    }
}

impl Racetrack {
    #[instrument(skip(grid), level = "debug")]
    fn find_track(grid: Grid<Cell>, start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
        trace!("Finding track from {:?} to {:?}", start, end);
        let mut path = vec![];
        let mut current = start;
        let mut previous = start;
        trace!("Start at {:?}", current);
        while current != end {
            path.push(current);

            // find the next cell that isn't the previous cell
            let next = current
                .von_neumann()
                .into_iter()
                .find_map(|coordinate| {
                    coordinate.filter(|&coordinate| {
                        coordinate != previous
                            && grid.get(coordinate).is_some_and(|cell| cell.is_track())
                    })
                })
                .expect("An adjacent cell should be track");

            previous = current;
            current = next;
            trace!("Move to {:?}", current);
        }

        trace!("End at {:?}", current);
        path.push(end);

        trace!("Found path with {} cells", path.len());
        path
    }

    /// Counts pairs of edges that can be used to cheat.
    #[instrument(skip_all, level = "debug")]
    fn count_cheat_edges(&self, cheat: &Cheat) -> usize {
        let mut count = 0;
        for (i, &u) in self.track.iter().enumerate() {
            if i + 2 >= self.track.len() {
                break;
            }

            for &v in &self.track[i + 2..] {
                let manhattan = u.manhattan(v);
                let valid = manhattan <= cheat.picoseconds;
                if valid {
                    trace!("Checking {} -> {}", u, v);
                    let u_to_end = self.distances_to_end[&u];
                    let v_to_end = self.distances_to_end[&v];

                    trace!("Distances to end: {} -> {} = {}", u, self.end, u_to_end);
                    trace!("Distances to end: {} -> {} = {}", v, self.end, v_to_end);

                    if v_to_end + manhattan + cheat.min_savings <= u_to_end {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

enum TrackType {
    Start,
    End,
    Track,
}

enum Cell {
    Track(TrackType),
    Wall,
}

impl Cell {
    fn is_track(&self) -> bool {
        matches!(self, Cell::Track(_))
    }
}

impl TryFrom<char> for Cell {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Track(TrackType::Track)),
            'S' => Ok(Cell::Track(TrackType::Start)),
            'E' => Ok(Cell::Track(TrackType::End)),
            '#' => Ok(Cell::Wall),
            _ => Err(value),
        }
    }
}

struct Cheat {
    picoseconds: usize,
    min_savings: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        solutions::setup_tracing();
        let input = include_str!("../../data/examples/2024/20/1.txt");
        assert_eq!(10, part1_impl(input, 10));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/20/1.txt");
        assert_eq!(285, part2_impl(input, 50));
    }
}
