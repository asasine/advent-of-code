//! Day 15: Warehouse Woes
//!
//! https://adventofcode.com/2024/day/15

use core::fmt;
use std::str::FromStr;

use solutions::grid::{Coordinate, Direction, Grid};
use tracing::{debug, trace};

fn part1(input: &str) -> usize {
    let mut warehouse = Warehouse::from_str(input).unwrap();
    debug!("{}", warehouse.grid);
    warehouse.simulate();
    warehouse.sum_box_gps()
}

fn part2(input: &str) -> usize {
    let mut warehouse = Warehouse::from_str(input).unwrap().into_part2();
    debug!("{}", warehouse.grid);
    warehouse.simulate();
    warehouse.sum_box_gps()
}

aoc_macro::aoc_main!();

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Box,
    LeftBox,
    RightBox,
    Wall,
    Robot,
}

impl TryFrom<char> for Cell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::Box),
            '#' => Ok(Self::Wall),
            '@' => Ok(Self::Robot),
            _ => Err(value),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Box => 'O',
            Self::LeftBox => '[',
            Self::RightBox => ']',
            Self::Wall => '#',
            Self::Robot => '@',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
struct WarehouseGrid(Grid<Cell>);

impl FromStr for WarehouseGrid {
    type Err = char;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self(grid))
    }
}

impl fmt::Display for WarehouseGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl WarehouseGrid {
    /// Checks if cell at the given `coordinate` can be moved the given `direction`.
    ///
    /// Records the moves in the `moves` hashmap.
    fn can_move(&self, coordinate: Coordinate, direction: Direction) -> bool {
        let current_cell = self.0.get(coordinate).unwrap();
        match current_cell {
            Cell::Wall => false,
            Cell::Robot => {
                // a robot can move if the next cell can [recursively] move
                coordinate
                    .try_move(direction)
                    .map_or(false, |coord| self.can_move(coord, direction))
            }
            Cell::Box => {
                // a box can move if the next cell can [recursively] move
                coordinate
                    .try_move(direction)
                    .map_or(false, |coord| self.can_move(coord, direction))
            }
            Cell::LeftBox => {
                // if moving vertically, a left box can move if the next cell can [recursively] move and the cell to the right can [recursively] move
                // otherwise, if moving horizontally, a left box can move if the next cell can [recursively] move
                if direction.is_vertical() {
                    let left = coordinate.try_move(direction);
                    let right = left.and_then(|c| c.try_move(Direction::Right));

                    left.map_or(false, |coord| self.can_move(coord, direction))
                        && right.map_or(false, |coord| self.can_move(coord, direction))
                } else {
                    coordinate
                        .try_move(direction)
                        .map_or(false, |coord| self.can_move(coord, direction))
                }
            }
            Cell::RightBox => {
                // if moving vertically, a right box can move if the next cell can [recursively] move and the cell to the left can [recursively] move
                // otherwise, if moving horizontally, a right box can move if the next cell can [recursively] move
                if direction.is_vertical() {
                    let right = coordinate.try_move(direction);
                    let left = right.and_then(|c| c.try_move(Direction::Left));
                    right.map_or(false, |coord| self.can_move(coord, direction))
                        && left.map_or(false, |coord| self.can_move(coord, direction))
                } else {
                    coordinate
                        .try_move(direction)
                        .map_or(false, |coord| self.can_move(coord, direction))
                }
            }
            Cell::Empty => {
                // an empty cell can always move
                true
            }
        }
    }

    /// Shift the cell at the given `coordinate` in the given `direction`, updating the grid with any movable boxes along the way.
    ///
    /// A cell replaces its destination cell, and the the original cell is replaced with an empty cell.
    ///
    /// This method assumes that it is valid to move the cell in the given direction.
    /// This includes any boxes that need to be moved, and their other halves if they are a multi-cell box.
    fn shift(&mut self, coordinate: Coordinate, direction: Direction) {
        let current_cell = *self.0.get(coordinate).unwrap();
        let destination = coordinate.try_move(direction).unwrap();
        let destination_cell = *self.0.get(destination).unwrap();
        *self.0.get_mut(coordinate).unwrap() = Cell::Empty;
        match destination_cell {
            Cell::Empty => {}
            Cell::Box => {
                self.shift(destination, direction);
            }
            Cell::LeftBox => {
                // if moving vertically, need to move both sides of the box,
                // otherwise, if moving horizontally, can move as if normal
                if direction.is_vertical() {
                    self.shift(destination.try_move(Direction::Right).unwrap(), direction);
                }

                self.shift(destination, direction);
            }
            Cell::RightBox => {
                // if moving vertically, need to move both sides of the box,
                // otherwise, if moving horizontally, can move as if normal
                if direction.is_vertical() {
                    self.shift(destination.try_move(Direction::Left).unwrap(), direction);
                }

                self.shift(destination, direction);
            }
            Cell::Wall => {
                unreachable!("Moving into a wall is invalid")
            }
            Cell::Robot => {
                unreachable!("Moving into a robot is invalid")
            }
        }

        *self.0.get_mut(destination).unwrap() = current_cell;
    }

    /// Tries to move the robot in the given direction, shifting any boxes in the direction if possible. Returns the robots new coordinate.
    fn try_move_robot(&mut self, robot_coordinate: Coordinate, direction: Direction) -> Coordinate {
        // find the first empty or wall cell from the robot in the direction of the move
        trace!("Move: {direction:?}");
        if self.can_move(robot_coordinate, direction) {
            self.shift(robot_coordinate, direction);
            trace!("{}", self);
            robot_coordinate.try_move(direction).unwrap()
        } else {
            // the robot cannot move
            robot_coordinate
        }
    }

    /// Finds the coordinate of the robot.
    fn find_robot(&self) -> Option<Coordinate> {
        self.0
            .enumerate()
            .find(|(_, cell)| matches!(cell, Cell::Robot))
            .map(|(coord, _)| coord)
    }

    /// The Goods Positioning System (GPS) coordinate of a box is 100 times its distance from the top ede of the map plus its distance from the left edge of the map.
    fn gps_coordinates(&self) -> impl Iterator<Item = (Coordinate, usize)> + '_ {
        self.0
            .enumerate()
            .filter(|(_, cell)| matches!(cell, Cell::Box | Cell::LeftBox))
            .map(|(coord, _)| {
                let gps = 100 * coord.y + coord.x;
                (coord, gps)
            })
    }
}

#[derive(Debug, Clone, Copy)]
struct WarehouseDirection(Direction);

impl TryFrom<char> for WarehouseDirection {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self(Direction::Up)),
            '>' => Ok(Self(Direction::Right)),
            'v' => Ok(Self(Direction::Down)),
            '<' => Ok(Self(Direction::Left)),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    grid: WarehouseGrid,
    moves: Vec<WarehouseDirection>,
}

impl Warehouse {
    /// Simulate all of the moves.
    fn simulate(&mut self) {
        let mut robot = self
            .grid
            .find_robot()
            .expect("There should be a robot in the grid");

        for WarehouseDirection(direction) in &self.moves {
            robot = self.grid.try_move_robot(robot, *direction);
        }
    }

    /// The sum of all GPS coordinates of all boxes in the warehouse.
    fn sum_box_gps(&self) -> usize {
        self.grid.gps_coordinates().map(|(_, gps)| gps).sum()
    }

    fn into_part2(self) -> Self {
        let cells = self
            .grid
            .0
            .into_cells()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|cell| match cell {
                        Cell::Wall => [Cell::Wall, Cell::Wall],
                        Cell::Box => [Cell::LeftBox, Cell::RightBox],
                        Cell::Empty => [Cell::Empty, Cell::Empty],
                        Cell::Robot => [Cell::Robot, Cell::Empty],
                        _ => unreachable!("Invalid cell in grid: {:?}", cell),
                    })
                    .collect()
            })
            .collect();

        let grid = WarehouseGrid(Grid::new(cells));
        Self {
            grid,
            moves: self.moves,
        }
    }
}

impl FromStr for Warehouse {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_break = if s.contains("\r\n") { "\r\n" } else { "\n" };
        let section_break = format!("{}{}", line_break, line_break);
        let (grid, moves) = s
            .split_once(section_break.as_str())
            .ok_or("Input does not contain a section break")?;

        let grid = WarehouseGrid::from_str(grid)?;
        let moves = moves
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(WarehouseDirection::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|c| format!("Invalid character in moves: {}", c))?;

        Ok(Self { grid, moves })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = include_str!("../../data/examples/2024/15/1.txt");
        assert_eq!(10092, part1(input));
    }

    #[test]
    fn part1_example2() {
        let input = include_str!("../../data/examples/2024/15/2.txt");
        assert_eq!(2028, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/15/1.txt");
        assert_eq!(9021, part2(input));
    }

    #[test]
    fn example1() {
        let input = include_str!("../../data/examples/2024/15/1.txt");
        let mut warehouse = Warehouse::from_str(input).unwrap();
        warehouse.simulate();
        assert_eq!(
            r#"##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
"#,
            format!("{}", warehouse.grid)
        );
    }

    #[test]
    fn part1_example2_manual() {
        let input = include_str!("../../data/examples/2024/15/2.txt");
        let mut warehouse = Warehouse::from_str(input).unwrap();
        let grid = &mut warehouse.grid;

        // Initial state:
        let mut robot = grid
            .find_robot()
            .expect("There should be a robot in the grid");

        assert_eq!(Coordinate { x: 2, y: 2 }, robot);
        assert_eq!(
            r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
"#,
            format!("{}", grid)
        );

        // Move <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 2, y: 2 }, robot);
        assert_eq!(
            r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
"#,
            format!("{}", grid)
        );

        // Move ^:
        robot = grid.try_move_robot(robot, Direction::Up);
        assert_eq!(Coordinate { x: 2, y: 1 }, robot);
        assert_eq!(
            r#"########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
"#,
            format!("{}", grid)
        );

        // Move ^:
        robot = grid.try_move_robot(robot, Direction::Up);
        assert_eq!(Coordinate { x: 2, y: 1 }, robot);
        assert_eq!(
            r#"########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
"#,
            format!("{}", grid)
        );

        // Move >:
        robot = grid.try_move_robot(robot, Direction::Right);
        assert_eq!(Coordinate { x: 3, y: 1 }, robot);
        assert_eq!(
            r#"########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
"#,
            format!("{}", grid)
        );

        // Move >:
        robot = grid.try_move_robot(robot, Direction::Right);
        assert_eq!(Coordinate { x: 4, y: 1 }, robot);
        assert_eq!(
            r#"########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
"#,
            format!("{}", grid)
        );

        // Move >:
        robot = grid.try_move_robot(robot, Direction::Right);
        assert_eq!(Coordinate { x: 4, y: 1 }, robot);
        assert_eq!(
            r#"########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########
"#,
            format!("{}", grid)
        );

        // Move: v:
        robot = grid.try_move_robot(robot, Direction::Down);
        assert_eq!(Coordinate { x: 4, y: 2 }, robot);
        assert_eq!(
            r#"########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );

        // Move: v:
        robot = grid.try_move_robot(robot, Direction::Down);
        assert_eq!(Coordinate { x: 4, y: 2 }, robot);
        assert_eq!(
            r#"########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );

        // Move: <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 3, y: 2 }, robot);
        assert_eq!(
            r#"########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );

        // Move: v:
        robot = grid.try_move_robot(robot, Direction::Down);
        assert_eq!(Coordinate { x: 3, y: 3 }, robot);
        assert_eq!(
            r#"########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );

        // Move: >:
        robot = grid.try_move_robot(robot, Direction::Right);
        assert_eq!(Coordinate { x: 4, y: 3 }, robot);
        assert_eq!(
            r#"########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );

        // Move: >:
        robot = grid.try_move_robot(robot, Direction::Right);
        assert_eq!(Coordinate { x: 5, y: 3 }, robot);
        assert_eq!(
            r#"########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );

        // Move: v:
        robot = grid.try_move_robot(robot, Direction::Down);
        assert_eq!(Coordinate { x: 5, y: 4 }, robot);
        assert_eq!(
            r#"########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );

        // Move: <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 4, y: 4 }, robot);
        assert_eq!(
            r#"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );

        // Move: <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 4, y: 4 }, robot);
        assert_eq!(
            r#"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
"#,
            format!("{}", grid)
        );
    }

    #[test]
    fn part2_example3_manual() {
        let input = include_str!("../../data/examples/2024/15/3.txt");
        let mut warehouse = Warehouse::from_str(input).unwrap().into_part2();
        let grid = &mut warehouse.grid;

        // Initial state:
        let mut robot = grid
            .find_robot()
            .expect("There should be a robot in the grid");

        assert_eq!(Coordinate { x: 10, y: 3 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############
"#,
            format!("{}", grid)
        );

        // Move: <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 9, y: 3 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############
"#,
            format!("{}", grid)
        );

        // Move: v:
        robot = grid.try_move_robot(robot, Direction::Down);
        assert_eq!(Coordinate { x: 9, y: 4 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############
"#,
            format!("{}", grid)
        );

        // Move: v:
        robot = grid.try_move_robot(robot, Direction::Down);
        assert_eq!(Coordinate { x: 9, y: 5 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############
"#,
            format!("{}", grid)
        );

        // Move: <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 8, y: 5 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############
"#,
            format!("{}", grid)
        );

        // Move: <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 7, y: 5 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############
"#,
            format!("{}", grid)
        );

        // Move: ^:
        robot = grid.try_move_robot(robot, Direction::Up);
        assert_eq!(Coordinate { x: 7, y: 4 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############
"#,
            format!("{}", grid)
        );

        // Move: ^:
        robot = grid.try_move_robot(robot, Direction::Up);
        assert_eq!(Coordinate { x: 7, y: 4 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############
"#,
            format!("{}", grid)
        );

        // Move: <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 6, y: 4 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##...[][]...##
##....[]....##
##....@.....##
##..........##
##############
"#,
            format!("{}", grid)
        );

        // Move: <:
        robot = grid.try_move_robot(robot, Direction::Left);
        assert_eq!(Coordinate { x: 5, y: 4 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##...[][]...##
##....[]....##
##...@......##
##..........##
##############
"#,
            format!("{}", grid)
        );

        // Move: ^:
        robot = grid.try_move_robot(robot, Direction::Up);
        assert_eq!(Coordinate { x: 5, y: 3 }, robot);
        assert_eq!(
            r#"##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############
"#,
            format!("{}", grid)
        );

        // Move: ^:
        robot = grid.try_move_robot(robot, Direction::Up);
        assert_eq!(Coordinate { x: 5, y: 2 }, robot);
        assert_eq!(
            r#"##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############
"#,
            format!("{}", grid)
        );
    }
}
