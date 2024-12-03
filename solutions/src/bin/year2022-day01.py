#!/usr/bin/env python3

import sys

from typing import List
from rich.console import Console

info_console = Console(stderr=True)

class Elf:
    def __init__(self, i: int):
        self.i = i
        self.food = []
        self._total_calories = 0

    @property
    def total_calories(self) -> int:
        return self._total_calories

    def add_food(self, calories: int):
        self.food.append(calories)
        self._total_calories += calories

def read_input() -> List[Elf]:
    """Reads the input file and returns a list of elves and their calorie stores."""

    elves = []
    def add_new_elf():
        elves.append(Elf(len(elves)))

    add_new_elf()

    for line in sys.stdin:
        if line.strip() == "":
            add_new_elf()
        else:
            calories = int(line.strip())
            elves[-1].add_food(calories)

    return elves

def main():
    elves = read_input()
    info_console.print(f"Read {len(elves)} elves")

    elves.sort(key=lambda elf: elf.total_calories, reverse=True)

    for elf in elves[:3]:
        info_console.print(f"Elf {elf.i} is carrying {elf.total_calories} of food")

    top_three_calories = sum(elf.total_calories for elf in elves[:3])
    info_console.print(f"The top three elves are carrying {top_three_calories} of food")

    # part 1: top elf
    print(elves[0].total_calories)

    # part 2: top three elves
    print(top_three_calories)


if __name__ == "__main__":
    main()
