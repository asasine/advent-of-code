# advent-of-code

Implementations to solve Advent of Code puzzles https://adventofcode.com/

## Usage

### Prerequisites

1. `cargo install aoc-cli`
1. Linux: `sudo apt install clang` needed to build the z3 crate.

### Yearly

1. Get the session token from the browser's cookies and put it into a session file
   - Linux: `~/.config/adventofcode.session`
   - Windows: `%APPDATA%\adventofcode.session`

### Daily

1. `cargo run download`
1. `cargo run daily`
1. `cargo run -q`
