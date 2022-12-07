#!/usr/bin/env python3

from collections import namedtuple
from typing import List
from pathlib import Path
from rich.console import Console

current_file = Path(__file__).absolute()

info_console = Console(stderr=True)

Elf = namedtuple("Elf", ("i", "food", "total_calories"))

def read_input() -> List[List[int]]:
    """Reads the input file and returns a list of elves and their calorie stores."""

    elves = []
    def add_new_elf():
        elves.append({"index": len(elves), "food": [], "total_calories": 0})

    add_new_elf()

    with (current_file.parent / "data" / "input" / "01.txt").open("r") as f:
        for line in f:
            if line.strip() == "":
                add_new_elf()
            else:
                calories = int(line.strip())
                elves[-1]["food"].append(calories)
                elves[-1]["total_calories"] += calories

    return elves

def main():
    elves = read_input()
    info_console.print(f"Read {len(elves)} elves")

    elves.sort(key=lambda elf: elf["total_calories"], reverse=True)

    for elf in elves[:3]:
        info_console.print(f"Elf {elf['index']} is carrying {elf['total_calories']} of food")

    top_three_calories = sum(elf["total_calories"] for elf in elves[:3])
    info_console.print(f"The top three elves are carrying {top_three_calories} of food")


if __name__ == "__main__":
    main()
