# advent-of-code

Implementations to solve Advent of Code puzzles https://adventofcode.com/

## Usage

### Prerequisites

1. `cargo install aoc-cli`

### Yearly

1. Get the session token from the browser's cookies and put it into the `~/.config/adventofcode.session` file.

### Daily

1. `aoc d -o -I -i ../advent-of-code/solutions/src/year{year}/inputs/{day}.txt`
1. Create a new module for the day's puzzle in `src/lib/year{year}/day{day}.rs`
1. Print the solutions to stdout.
