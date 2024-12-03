#!/usr/bin/env python3

import sys
from more_itertools import quantify
from typing import Iterable, List
from rich.console import Console

info_console = Console(stderr=True)

class Assignment:
    def __init__(self, low: int, high: int):
        self.low = low
        self.high = high

    def is_superset(self, other: "Assignment") -> bool:
        """Returns true if this assignment is a superset of the other assignment"""
        return self.low <= other.low and self.high >= other.high

    def overlaps(self, other: "Assignment") -> bool:
        """Returns true if this assignment overlaps with the other assignment"""
        return self.low <= other.high and self.high >= other.low

class Pair:
    def __init__(self, a: Assignment, b: Assignment):
        self.a = a
        self.b = b

    def one_contains_another(self) -> bool:
        """Returns true if one of the assignments is a superset of the other"""
        return self.a.is_superset(self.b) or self.b.is_superset(self.a)

    def one_overlaps_another(self) -> bool:
        """Returns true if one of the assignments overlaps with the other"""
        return self.a.overlaps(self.b) or self.b.overlaps(self.a)

def read_input() -> List[Pair]:
    def factory(line: str) -> Pair:
        a, b = line.split(",")
        a_low, a_high = map(int, a.split("-"))
        b_low, b_high = map(int, b.split("-"))
        return Pair(Assignment(a_low, a_high), Assignment(b_low, b_high))

    return list(map(factory, filter(None, map(str.strip, sys.stdin.readlines()))))

def part_1(pairs: Iterable[Pair]) -> int:
    """Returns the number of pairs where one assignment is a superset of the other"""
    return quantify(pairs, Pair.one_contains_another)

def part_2(pairs: Iterable[Pair]) -> int:
    """Returns the number of pairs where the assignments overlap"""
    return quantify(pairs, Pair.one_overlaps_another)

def main():
    pairs = read_input()
    info_console.print(f"Read {len(pairs)} pairs")
    print(part_1(pairs))
    print(part_2(pairs))

if __name__ == "__main__":
    main()
