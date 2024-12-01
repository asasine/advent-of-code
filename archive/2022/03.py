#!/usr/bin/env python3

from more_itertools import ichunked
from pathlib import Path
from typing import Iterable, List, Set
from rich.console import Console

info_console = Console(stderr=True)
current_file = Path(__file__).absolute()

class Rucksack:
    def __init__(self, contents: str):
        if len(contents) % 2 != 0:
            raise ValueError(f"contents must have an even length: {contents}")

        middle = len(contents) // 2
        self.compartment_1 = set(contents[0 : middle])
        self.compartment_2 = set(contents[middle :])
        self.contents = self.compartment_1 | self.compartment_2

    @classmethod
    def get_item_priority(cls, item: str) -> int:
        """
        Returns the priority of an item.
        
        Args:
            item: a single character representing an item

        Returns: the priority of the item, with a-z having priority 1-26 and A-Z having priority 27-52.
        """

        if len(item) != 1:
            raise ValueError(f"item must be a single character, got {item}")

        if item.islower():
            return ord(item) - 96
        elif item.isupper():
            return ord(item) - 38
        else:
            raise ValueError(f"item must be a letter, got {item}")

    def part_1(self):
        return sum(Rucksack.get_item_priority(item) for item in self.compartment_1 & self.compartment_2)

def read_input() -> List[Rucksack]:
    with (current_file.parent / "data" / "input" / "03.txt").open() as f:
        return list(map(Rucksack, filter(None, map(str.strip, f.readlines()))))

def part_1(rucksacks: Iterable[Rucksack]) -> int:
    """Returns the sum of the item priorities of the items shared between the two compartments of each rucksack."""
    def get_rucksack_score(rucksack: Rucksack) -> int:
        shared_element = rucksack.compartment_1 & rucksack.compartment_2
        if len(shared_element) != 1:
            raise ValueError(f"Expected exactly one shared element, got {shared_element}")

        return Rucksack.get_item_priority(shared_element.pop())
    
    return sum(map(get_rucksack_score, rucksacks))

def part_2(rucksacks: Iterable[Rucksack]) -> int:
    def get_chunk_score(group: Iterable[Rucksack]) -> int:
        """Returns the item priority of the singularly-shared items in the group of rucksacks."""
        shared_element = set.intersection(*(r.contents for r in group))
        if len(shared_element) != 1:
            raise ValueError(f"Expected exactly one shared element, got {shared_element}")

        return Rucksack.get_item_priority(shared_element.pop())

    return sum(map(get_chunk_score, ichunked(rucksacks, 3)))

def main():
    rucksacks = read_input()
    info_console.print(f"Read {len(rucksacks)} rucksacks")

    info_console.print(f"Part 1: {part_1(rucksacks)}")
    info_console.print(f"Part 2: {part_2(rucksacks)}")

if __name__ == "__main__":
    main()
