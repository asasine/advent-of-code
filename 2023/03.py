#!/usr/bin/env python3

import math
import operator
import re
from dataclasses import dataclass
from functools import lru_cache
from more_itertools import windowed
from itertools import chain
from pathlib import Path
from typing import Dict, Iterator, List, Set, Tuple
from rich.console import Console

current_file = Path(__file__).absolute()
info_console = Console(stderr=True)

pattern = re.compile(r"(?P<number>\d+)|(?P<symbol>[^.])")


@dataclass(frozen=True)
class Coordinate:
    row: int
    """The row number."""

    column: int
    """The column number."""


@dataclass(frozen=True)
class Symbol:
    coordinate: Coordinate
    """The coordinate of the symbol."""

    symbol: str
    """The symbol."""


@dataclass(frozen=True)
class PartNumber:
    number: int
    """The part number."""

    start_coordinate: Coordinate
    """The coordinate for the start of the number."""

    @property
    def end_coordinate(self) -> Coordinate:
        """The coordinate for the end of the number."""
        return Coordinate(self.start_coordinate.row, self.start_coordinate.column + len(str(self.number)) - 1)

    def is_adjacent(self, coordinate: Coordinate) -> bool:
        """
        Returns whether the given coordinate is adjacent to this part number, including diagonally.

        Args:
            coordinate (Coordinate): The coordinate to check.

        Returns:
            bool: True if the coordinate is adjacent to this part number.
        """
        # the coordinate can be adjacent to any of the columns in the part number, including diagonally
        return (
            coordinate.row in range(self.start_coordinate.row - 1, self.start_coordinate.row + 2)
            and coordinate.column in range(self.start_coordinate.column - 1, self.end_coordinate.column + 2)
        )


@lru_cache(maxsize=3)
def get_symbols(line: str, row: int) -> Set[Symbol]:
    """
    Returns the indices of all symbols in the given line.

    Args:
        line (str): The line to search.
        row (int): The row number of the line.

    Returns:
        List[Symbol]: The symbols in the given line.
    """
    return {Symbol(Coordinate(row, match.start()), match.group("symbol")) for match in pattern.finditer(line) if match.group("symbol") is not None}


@dataclass
class Adjacencies:
    symbol_to_part_numbers: Dict[Symbol, Set[PartNumber]]
    """A mapping of symbols to the part numbers adjacent to them."""

    part_number_to_symbols: Dict[PartNumber, Set[Symbol]]
    """A mapping of part numbers to the symbols adjacent to them."""

    @property
    def symbols(self) -> Set[Symbol]:
        """The set of all symbols."""
        return set(self.symbol_to_part_numbers.keys())

    @property
    def part_numbers(self) -> Set[PartNumber]:
        """The set of all part numbers."""
        return set(self.part_number_to_symbols.keys())

    def gear_ratios(self) -> Iterator[int]:
        """
        Returns the gear ratios for all gears.

        A gear is any "*" symbol adjacent to exactly two part numbers.

        Returns:
            Iterator[int]: The gear ratios.
        """
        for symbol, part_numbers in self.symbol_to_part_numbers.items():
            if symbol.symbol != "*":
                continue

            if len(part_numbers) != 2:
                continue

            part_number_1, part_number_2 = part_numbers
            yield part_number_1.number * part_number_2.number


def get_input(example: bool = False) -> Adjacencies:
    data_file = current_file.parent / "data" / "input"
    if example:
        data_file /= "example"

    data_file /= "03.txt"

    symbol_to_part_numbers: Dict[Symbol, Set[PartNumber]] = {}
    part_number_to_symbols: Dict[PartNumber, Set[Symbol]] = {}
    with open(data_file, "r") as file:
        # read the file in windows of 3 lines, where the middle line is the one we're inspecting for part numbers
        # and all three lines are searched for symbols
        for row, window in enumerate(windowed(chain([""], file, [""]), 3)):
            line_0, line_1, line_2 = window
            if line_0 is None or line_1 is None or line_2 is None:
                raise ValueError(f"Invalid window: line_0={line_0}, line_1={line_1}, line_2={line_2}")

            line_0 = line_0.rstrip()
            line_1 = line_1.rstrip()
            line_2 = line_2.rstrip()

            symbols = get_symbols(line_0, row - 1) | get_symbols(line_1, row) | get_symbols(line_2, row + 1)
            for match in pattern.finditer(line_1):
                if match.group("number") is None:
                    continue

                number = int(match.group(0))
                part_number = PartNumber(number, Coordinate(row, match.start()))
                for symbol in symbols:
                    if part_number.is_adjacent(symbol.coordinate):
                        symbol_to_part_numbers.setdefault(symbol, set()).add(part_number)
                        part_number_to_symbols.setdefault(part_number, set()).add(symbol)

    return Adjacencies(symbol_to_part_numbers, part_number_to_symbols)


def part_one(adjacencies: Adjacencies) -> None:
    """Prints the sum of all part numbers."""
    part_numbers = adjacencies.part_numbers
    info_console.print(f"Sum of all part numbers: {sum(part_number.number for part_number in part_numbers)}")


def part_two(adjacency: Adjacencies) -> None:
    """
    Prints the sum of all gear ratios. A gear ratio is the product of part numbers adjacent to a gear. A gear is any
    "*" symbol adjacent to exactly two part numbers.
    """
    info_console.print(f"Sum of all gear ratios: {sum(adjacency.gear_ratios())}")


def main():
    adjacencies = get_input()
    part_one(adjacencies)
    part_two(adjacencies)


if __name__ == "__main__":
    main()
