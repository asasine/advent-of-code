# advent-of-code

Implementations to solve Advent of Code puzzles https://adventofcode.com/

## Usage

### Prerequisites

1. `cargo install aoc-cli`

### Yearly

1. Get the session token from the browser's cookies and put it into the `~/.config/adventofcode.session` file.
1. Create a new module for the year in `src/lib/year{year}/mod.rs`
1. Publicly export the year's modules in `src/lib.rs`

### Daily

1. `aoc d -o -I -i ./solutions/src/year{year}/inputs/{day}.txt`
1. Create a new module for the day's puzzle in `src/lib/year{year}/day{day}.rs`
1. Add public functions `part1` and `part2` to the module.
1. Publicly export the functions in the `year{year}` module.
1. Change `src/main.rs` to run the new puzzle.
1. Print the solutions to stdout.
