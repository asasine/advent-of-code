#!/usr/bin/env python3

import sys
from typing import Optional
from more_itertools import windowed, first_true, all_unique
from rich.console import Console

info_console = Console(stderr=True)

def read_input() -> str:
    return sys.stdin.readline().strip()

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
    info_console.print(f"Read {len(data)} characters")

    part_1_index = part_1(data)
    info_console.log(f"[bold blue]Part 1[/bold blue]: Index {part_1_index}: marker={data[part_1_index - 4 : part_1_index]}")
    print(part_1_index)

    part_2_index = part_2(data)
    info_console.log(f"[bold blue]Part 2[/bold blue]: Index {part_2_index}: marker={data[part_2_index - 14 : part_2_index]}")
    print(part_2_index)

if __name__ == "__main__":
    main()
