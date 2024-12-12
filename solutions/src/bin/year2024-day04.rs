//! Day 4: Ceres Search
//!
//! https://adventofcode.com/2024/day/4

use itertools::Itertools;
use solutions::grid::{Coordinate, Direction, Grid};

fn part1(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().unwrap();
    let grid = WordSearch { data: grid };
    grid.count_word("XMAS")
}

struct WordSearch {
    data: Grid<char>,
}

impl WordSearch {
    /// Count the number of times the `X-MAS` pattern appears in the grid.
    ///
    /// This pattern is a cross of `MAS`, rooted at the `A`. The `MAS` can be written forwards or backwards.
    /// For example:
    ///
    /// ```text
    /// M.S
    /// .A.
    /// M.S
    /// ```
    ///
    /// The X-MAS pattern appears once in this example.
    fn count_x_mas(&self) -> usize {
        // For starters, we know that every `X-MAS` pattern must have an `A` in the middle, so we can start our searches there.
        self.data
            .enumerate()
            .filter(|(_, &c)| c == 'A')
            .filter(|(c, _)| self.is_x_mas(*c))
            .count()
    }

    /// Checks whether the `X-MAS` pattern appears at the given coordinate.
    ///
    /// The `X-MAS` pattern is a cross of `MAS`, rooted at the `A`. The `MAS` can be written forwards or backwards. The
    /// coordinate is the location of the `A`.
    fn is_x_mas(&self, coordinate: Coordinate) -> bool {
        // if the pattern would go out of bounds, it can't be an X-MAS
        if let Some((_, ne, _, se, _, sw, _, nw)) = coordinate
            .moore_within(self.data.extent())
            .into_iter()
            .filter_map(|c| c.and_then(|c| self.data.get(c)))
            .copied()
            .collect_tuple()
        {
            let a = *self.data.get(coordinate).unwrap();
            if a != 'A' {
                return false;
            }

            // check top-left to bottom-right first
            match (nw, se) {
                ('M', 'S') => {}
                ('S', 'M') => {}
                _ => return false,
            }

            // check top-right to bottom-left
            match (ne, sw) {
                ('M', 'S') => {}
                ('S', 'M') => {}
                _ => return false,
            }

            true
        } else {
            // some of the neighbors are out of bounds
            false
        }
    }

    /// Count the number of times a word appears in the grid.
    fn count_word(&self, word: &str) -> usize {
        let first_letter = match word.chars().next() {
            Some(c) => c,
            None => return 0,
        };

        self.data
            .enumerate()
            .filter(|(_, &c)| c == first_letter)
            .map(|(coordinate, _)| {
                // eprintln!("{coordinate}");
                self.count_words_at(word, coordinate)
            })
            .sum()
    }

    /// Counts the number of times a word appears rooted at a given coordinate.
    ///
    /// This checks for the word in all directions: horizontal, vertical, and diagonal, plus the reverse of each.
    fn count_words_at(&self, word: &str, coordinate: Coordinate) -> usize {
        let mut count = 0;

        if self.check_word_horizontal(word, coordinate) {
            eprintln!("Found horizontal at {coordinate}");
            count += 1;
        }

        if self.check_word_horizontal_reverse(word, coordinate) {
            eprintln!("Found horizontal reverse at {coordinate}");
            count += 1;
        }

        if self.check_word_vertical(word, coordinate) {
            eprintln!("Found vertical at {coordinate}");
            count += 1;
        }

        if self.check_word_vertical_reverse(word, coordinate) {
            eprintln!("Found vertical reverse at {coordinate}");
            count += 1;
        }

        if self.check_word_diagonal_down(word, coordinate) {
            eprintln!("Found diagonal down at {coordinate}");
            count += 1;
        }

        if self.check_word_diagonal_down_reverse(word, coordinate) {
            eprintln!("Found diagonal down reverse at {coordinate}");
            count += 1;
        }

        if self.check_word_diagonal_up(word, coordinate) {
            eprintln!("Found diagonal up at {coordinate}");
            count += 1;
        }

        if self.check_word_diagonal_up_reverse(word, coordinate) {
            eprintln!("Found diagonal up reverse at {coordinate}");
            count += 1;
        }

        count
    }

    /// Checks whether a word appears in the grid rooted at the given coordinate.
    ///
    /// The `try_move` function is used to move from one coordinate to the next. It should return the next coordinate,
    /// or [`None`] if there are no more moves to make (e.g., it leaves the grid).
    fn check_word<F>(&self, word: &str, start: Coordinate, mut try_move: F) -> bool
    where
        F: FnMut(Coordinate) -> Option<Coordinate>,
    {
        let mut coordinate = Some(start);
        word.chars().all(|letter| match coordinate {
            Some(c) => match self.data.get(c) {
                Some(cell) if *cell == letter => {
                    coordinate = try_move(c);
                    true
                }
                Some(_) | None => false,
            },
            None => false,
        })
    }

    /// Checks whether the word appears horizontally at the given coordinate.
    fn check_word_horizontal(&self, word: &str, coordinate: Coordinate) -> bool {
        self.check_word(word, coordinate, |c| c.try_move(Direction::Right))
    }

    /// Checks whether the word appears horizontally in reverse at the given coordinate.
    fn check_word_horizontal_reverse(&self, word: &str, coordinate: Coordinate) -> bool {
        self.check_word(word, coordinate, |c| c.try_move(Direction::Left))
    }

    /// Checks whether the word appears vertically at the given coordinate.
    fn check_word_vertical(&self, word: &str, coordinate: Coordinate) -> bool {
        self.check_word(word, coordinate, |c| c.try_move(Direction::Down))
    }

    /// Checks whether the word appears vertically in reverse at the given coordinate.
    fn check_word_vertical_reverse(&self, word: &str, coordinate: Coordinate) -> bool {
        self.check_word(word, coordinate, |c| c.try_move(Direction::Up))
    }

    /// Checks whether the word appears diagonally downward at the given coordinate.
    fn check_word_diagonal_down(&self, word: &str, coordinate: Coordinate) -> bool {
        self.check_word(word, coordinate, |c| {
            c.try_move(Direction::Down)
                .and_then(|c| c.try_move(Direction::Right))
        })
    }

    /// Checks whether the word appeard diagonally downward in reverse at the given coordinate.
    fn check_word_diagonal_down_reverse(&self, word: &str, coordinate: Coordinate) -> bool {
        self.check_word(word, coordinate, |c| {
            c.try_move(Direction::Down)
                .and_then(|c| c.try_move(Direction::Left))
        })
    }

    /// Checks whether the word appears diagonally upward at the given coordinate.
    fn check_word_diagonal_up(&self, word: &str, coordinate: Coordinate) -> bool {
        self.check_word(word, coordinate, |c| {
            c.try_move(Direction::Up)
                .and_then(|c| c.try_move(Direction::Right))
        })
    }

    /// Checks whether the word appears diagonally upward in reverse at the given coordinate.
    fn check_word_diagonal_up_reverse(&self, word: &str, coordinate: Coordinate) -> bool {
        self.check_word(word, coordinate, |c| {
            c.try_move(Direction::Up)
                .and_then(|c| c.try_move(Direction::Left))
        })
    }
}

fn part2(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().unwrap();
    let grid = WordSearch { data: grid };
    grid.count_x_mas()
}

aoc_macro::aoc_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/04/1.txt");
        assert_eq!(18, part1(input));
    }

    #[test]
    fn grid_check_words() {
        let input = r#"S..S..S
.A.A.A.
..MMM..
SAMXMAS
..MMM..
.A.A.A.
S..S..S"#;

        let grid = WordSearch {
            data: input.parse::<Grid<char>>().unwrap(),
        };

        let c = Coordinate { x: 3, y: 3 };
        assert!(grid.check_word_horizontal("XMAS", c));
        assert!(grid.check_word_horizontal_reverse("XMAS", c));
        assert!(grid.check_word_vertical("XMAS", c));
        assert!(grid.check_word_vertical_reverse("XMAS", c));
        assert!(grid.check_word_diagonal_down("XMAS", c));
        assert!(grid.check_word_diagonal_down_reverse("XMAS", c));
        assert!(grid.check_word_diagonal_up("XMAS", c));
        assert!(grid.check_word_diagonal_up_reverse("XMAS", c));
        assert_eq!(8, grid.count_word("XMAS"));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/04/1.txt");
        assert_eq!(9, part2(input));
    }
}
