# advent-of-code

Implementations to solve Advent of Code puzzles https://adventofcode.com/

## Usage

### Prerequisites

1. `cargo install aoc-cli`

### Yearly

1. Get the session token from the browser's cookies and put it into the `~/.config/adventofcode.session` file.
1. `mkdir -vp ./solutions/data/{real,examples}/$(date +%Y)`

### Daily

1. `aoc d -o -I -i ./solutions/data/real/$(date +%Y)/$(date +%d).txt`
1. Create a new binary for the day's puzzle in `src/bin/year${year}-day${day:02}.rs`.
1. Add markdown descriptions for the puzzles: `aoc r -w 120 -P -m`. This may require some cleanup (multiline code blocks, links, etc.)
1. `cargo run -q`
