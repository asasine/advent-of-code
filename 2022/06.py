#!/usr/bin/env python3

from typing import Optional
from more_itertools import windowed, first_true, all_unique
from pathlib import Path
from rich import print

current_file = Path(__file__).absolute()

def read_input() -> str:
    with (current_file.parent / "data" / "input" / "06.txt").open() as f:
        return f.readline().strip()

def find_marker(data: str, n: int) -> Optional[int]:
    result = first_true(enumerate(windowed(data, n)), pred=lambda window: all_unique(window[1]))
    if result is None:
        return None

    return result[0] + n

def part_1(data: str) -> int:
    result = find_marker(data, 4)
    if result is None:
        raise ValueError("No result found")

    return result

def part_2(data: str) -> int:
    result = find_marker(data, 14)
    if result is None:
        raise ValueError("No result found")

    return result

def main():
    data = read_input()
    print(f"Read {len(data)} characters")

    part_1_index = part_1(data)
    print(f"[bold blue]Part 1[/bold blue]: Index {part_1_index}: marker={data[part_1_index - 4 : part_1_index]}")

    part_2_index = part_2(data)
    print(f"[bold blue]Part 2[/bold blue]: Index {part_2_index}: marker={data[part_2_index - 14 : part_2_index]}")

if __name__ == "__main__":
    main()
