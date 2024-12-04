//! Day 4

use core::fmt;

use itertools::Itertools;

/// "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it.
/// After a brief flash, you recognize the interior of the [Ceres monitoring station][1]!
///
/// As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if
/// you could help her with her *word search* (your puzzle input). She only has to find one word: `XMAS`.
///
/// This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words.
/// It's a little unusual, though, as you don't merely need to find one instance of `XMAS` - you need to find *all of them*.
/// Here are a few ways `XMAS` might appear, where irrelevant characters have been replaced with `.`:
///
///
/// ```text
/// ..X...
/// .SAMX.
/// .A..A.
/// XMAS.S
/// .X....
/// ```
///
/// The actual word search will be full of letters instead. For example:
///
/// ```text
/// MMMSXXMASM
/// MSAMXMSMSA
/// AMXSXMAAMM
/// MSAMASMSMX
/// XMASAMXAMM
/// XXAMMXXAMA
/// SMSMSASXSS
/// SAXAMASAAA
/// MAMMMXMMMM
/// MXMXAXMASX
/// ```
///
/// In this word search, `XMAS` occurs a total of `*18*` times; here's the same word search again, but where letters not
/// involved in any `XMAS` have been replaced with `.`:
///
/// ```text
/// ....XXMAS.
/// .SAMXMS...
/// ...S..A...
/// ..A.A.MS.X
/// XMASAMX.MM
/// X.....XA.A
/// S.S.S.S.SS
/// .A.A.A.A.A
/// ..M.M.M.MM
/// .X.X.XMASX
/// ```
///
/// Take a look at the little Elf's word search. *How many times does `XMAS` appear?*
///
/// [1]: https://adventofcode.com/2019/day/10
fn part1(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.count_word("XMAS")
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coordinate {
    /// Returns an iterator over all coordinates in a grid whose bottom-right corner is at the given coordinate, starting at the top-left corner.
    ///
    /// This iterates in row-major order. That is, it starts at the top-left corner and moves right, then down, then right, then down, and so on.
    fn grid_iter(&self) -> impl Iterator<Item = Coordinate> {
        (0..self.y)
            .cartesian_product(0..self.x)
            .map(move |(y, x)| Coordinate { x, y })
    }
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let data = input.lines().map(|line| line.chars().collect()).collect();
        Self { data }
    }
}

impl Grid {
    fn extent(&self) -> Coordinate {
        Coordinate {
            x: self.data[0].len(),
            y: self.data.len(),
        }
    }

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
        self.extent()
            .grid_iter()
            .filter(|&c| self.data[c.y][c.x] == 'A')
            .filter(|&c| self.is_x_mas(c))
            .count()
    }

    /// Checks whether the `X-MAS` pattern appears at the given coordinate.
    ///
    /// The `X-MAS` pattern is a cross of `MAS`, rooted at the `A`. The `MAS` can be written forwards or backwards. The
    /// coordinate is the location of the `A`.
    fn is_x_mas(&self, coordinate: Coordinate) -> bool {
        // if the pattern would go out of bounds, it can't be an X-MAS
        if coordinate.y < 1
            || coordinate.y + 1 >= self.data.len()
            || coordinate.x < 1
            || coordinate.x + 1 >= self.data[coordinate.y].len()
        {
            return false;
        }

        let a = self.data[coordinate.y][coordinate.x];
        if a != 'A' {
            return false;
        }

        // check top-left to bottom-right first
        let tl = self.data[coordinate.y - 1][coordinate.x - 1];
        let br = self.data[coordinate.y + 1][coordinate.x + 1];
        match (tl, br) {
            ('M', 'S') => {}
            ('S', 'M') => {}
            _ => return false,
        }

        // check top-right to bottom-left
        let tr = self.data[coordinate.y - 1][coordinate.x + 1];
        let bl = self.data[coordinate.y + 1][coordinate.x - 1];
        match (tr, bl) {
            ('M', 'S') => {}
            ('S', 'M') => {}
            _ => return false,
        }

        true
    }

    /// Count the number of times a word appears in the grid.
    fn count_word(&self, word: &str) -> usize {
        let first_letter = match word.chars().next() {
            Some(c) => c,
            None => return 0,
        };

        self.extent()
            .grid_iter()
            .filter(|&c| self.data[c.y][c.x] == first_letter)
            .map(|coordinate| {
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

    /// Checks whether the word appears horizontally at the given coordinate.
    fn check_word_horizontal(&self, word: &str, coordinate: Coordinate) -> bool {
        if coordinate.y >= self.data.len() {
            return false;
        }

        for (offset, letter) in word.chars().enumerate() {
            let x = offset + coordinate.x;
            if x >= self.data[coordinate.y].len() {
                return false;
            }

            if self.data[coordinate.y][x] != letter {
                return false;
            }
        }

        true
    }

    /// Checks whether the word appears horizontally in reverse at the given coordinate.
    fn check_word_horizontal_reverse(&self, word: &str, coordinate: Coordinate) -> bool {
        if coordinate.y >= self.data.len() {
            return false;
        }

        for (offset, letter) in word.chars().enumerate() {
            let x = match coordinate.x.checked_sub(offset) {
                Some(x) => x,
                None => return false,
            };

            if x >= self.data[coordinate.y].len() {
                return false;
            }

            if self.data[coordinate.y][x] != letter {
                return false;
            }
        }

        true
    }

    /// Checks whether the word appears vertically at the given coordinate.
    fn check_word_vertical(&self, word: &str, coordinate: Coordinate) -> bool {
        if coordinate.y >= self.data.len() {
            return false;
        }

        if coordinate.x >= self.data[coordinate.y].len() {
            return false;
        }

        for (offset, letter) in word.chars().enumerate() {
            let y = offset + coordinate.y;
            if y >= self.data.len() {
                return false;
            }

            if self.data[y][coordinate.x] != letter {
                return false;
            }
        }

        true
    }

    /// Checks whether the word appears vertically in reverse at the given coordinate.
    fn check_word_vertical_reverse(&self, word: &str, coordinate: Coordinate) -> bool {
        if coordinate.y >= self.data.len() {
            return false;
        }

        if coordinate.x >= self.data[coordinate.y].len() {
            return false;
        }

        for (offset, letter) in word.chars().enumerate() {
            let y = match coordinate.y.checked_sub(offset) {
                Some(y) => y,
                None => return false,
            };

            if self.data[y][coordinate.x] != letter {
                return false;
            }
        }

        true
    }

    /// Checks whether the word appears diagonally downward at the given coordinate.
    fn check_word_diagonal_down(&self, word: &str, coordinate: Coordinate) -> bool {
        if coordinate.y >= self.data.len() {
            return false;
        }

        if coordinate.x >= self.data[coordinate.y].len() {
            return false;
        }

        for (offset, letter) in word.chars().enumerate() {
            let x = offset + coordinate.x;
            let y = offset + coordinate.y;
            if y >= self.data.len() || x >= self.data[y].len() {
                return false;
            }

            if self.data[y][x] != letter {
                return false;
            }
        }

        true
    }

    /// Checks whether the word appeard diagonally downward in reverse at the given coordinate.
    fn check_word_diagonal_down_reverse(&self, word: &str, coordinate: Coordinate) -> bool {
        if coordinate.y >= self.data.len() {
            return false;
        }

        if coordinate.x >= self.data[coordinate.y].len() {
            return false;
        }

        for (offset, letter) in word.chars().enumerate() {
            let x = match coordinate.x.checked_sub(offset) {
                Some(x) => x,
                None => return false,
            };

            let y = coordinate.y + offset;
            if y >= self.data.len() {
                return false;
            }

            if x >= self.data[y].len() {
                return false;
            }

            if self.data[y][x] != letter {
                return false;
            }
        }

        true
    }

    /// Checks whether the word appears diagonally upward at the given coordinate.
    fn check_word_diagonal_up(&self, word: &str, coordinate: Coordinate) -> bool {
        if coordinate.y >= self.data.len() {
            return false;
        }

        if coordinate.x >= self.data[coordinate.y].len() {
            return false;
        }

        for (offset, letter) in word.chars().enumerate() {
            let x = offset + coordinate.x;
            let y = match coordinate.y.checked_sub(offset) {
                Some(y) => y,
                None => return false,
            };

            if y >= self.data.len() {
                return false;
            }

            if x >= self.data[y].len() {
                return false;
            }

            if self.data[y][x] != letter {
                return false;
            }
        }

        true
    }

    /// Checks whether the word appears diagonally upward in reverse at the given coordinate.
    fn check_word_diagonal_up_reverse(&self, word: &str, coordinate: Coordinate) -> bool {
        if coordinate.y >= self.data.len() {
            return false;
        }

        if coordinate.x >= self.data[coordinate.y].len() {
            return false;
        }

        for (offset, letter) in word.chars().enumerate() {
            let x = match coordinate.x.checked_sub(offset) {
                Some(x) => x,
                None => return false,
            };

            let y = match coordinate.y.checked_sub(offset) {
                Some(y) => y,
                None => return false,
            };

            if y >= self.data.len() {
                return false;
            }

            if self.data[y][x] != letter {
                return false;
            }
        }

        true
    }
}

/// The Elf looks quizzically at you. Did you misunderstand the assignment?
///
/// Looking for the instructions, you flip over the word search to find that this isn't actually an `*XMAS*` puzzle; it's an
/// `*X-MAS*` puzzle in which you're supposed to find two `MAS` in the shape of an `X`. One way to achieve that is like
/// this:
///
/// ```text
/// M.S
/// .A.
/// M.S
/// ```
///
/// Irrelevant characters have again been replaced with `.` in the above diagram. Within the `X`, each `MAS` can be written
/// forwards or backwards.
///
/// Here's the same example from before, but this time all of the `X-MAS`es have been kept instead:
///
/// ```text
/// .M.S......
/// ..A..MSMS.
/// .M.S.MAA..
/// ..A.ASMSM.
/// .M.S.M....
/// ..........
/// S.S.S.S.S.
/// .A.A.A.A..
/// M.M.M.M.M.
/// ..........
/// ```
///
/// In this example, an `X-MAS` appears `*9*` times.
///
/// Flip the word search from the instructions back over to the word search side and try again. *How many times does an
/// `X-MAS` appear?*
fn part2(input: &str) -> usize {
    let grid = Grid::from(input);
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

        let grid = Grid::from(input);
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
