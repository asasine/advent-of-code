#!/usr/bin/env python3

import re
from dataclasses import dataclass
from pathlib import Path
from typing import FrozenSet, Iterable, Iterator, List, Optional
from more_itertools import ilen
from rich.console import Console

current_file = Path(__file__).absolute()
problem_number = current_file.stem
info_console = Console(stderr=True)


line_pattern = re.compile(r"^Card (?P<number>\d+): (?P<winning_numbers>(?:\d+ )*\d+) \| (?P<numbers>(?:\d+ )*\d+)$")
token_pattern = re.compile(r"(?P<number>\d+)|(?P<colon>\:)|(?P<pipe>\|)|(?P<end>$)")


@dataclass(frozen=True)
class Scratchcard:
    number: int
    """The number of the scratchcard."""

    winning_numbers: FrozenSet[int]
    """The winning numbers on the scratchcard."""

    numbers: FrozenSet[int]
    """The numbers on the scratchcard."""

    @property
    def number_of_winners(self) -> int:
        """The number of winners on this card."""
        return len(self.winning_numbers.intersection(self.numbers))

    @property
    def part_one_score(self) -> int:
        """
        Calculate the score of the scratchcard according to part one of the problem.

        The first number is worth 1 point, and every subsequent number doubles the score.
        """
        number_of_winners = self.number_of_winners
        if number_of_winners == 0:
            return 0

        return 2 ** (number_of_winners - 1)

    @classmethod
    def from_line(cls, line: str) -> "Scratchcard":
        """
        Create a scratchcard from a line.

        Args:
            line (str): The line to parse.

        Returns:
            Scratchcard: The scratchcard.

        Raises:
            ValueError: If the line is invalid.
        """
        match: Optional[re.Match[str]] = None

        def parse_numbers() -> Iterator[int]:
            nonlocal match
            pos = 0 if match is None else match.end()
            match = token_pattern.search(line, pos=pos)
            while match and match.group("number") is not None:
                yield int(match.group("number"))
                match = token_pattern.search(line, pos=match.end())

        card_numbers = list(parse_numbers())
        if len(card_numbers) != 1:
            raise ValueError(f"Invalid line: invalid card number: {line}")

        card_number = card_numbers[0]
        winning_numbers = frozenset(parse_numbers())
        numbers = frozenset(parse_numbers())
        return cls(card_number, winning_numbers, numbers)


def get_input(example: bool = False) -> Iterator[Scratchcard]:
    data_file = current_file.parent / "data" / "input"
    if example:
        data_file /= "example"

    data_file /= f"{problem_number}.txt"

    with data_file.open() as f:
        for line in f:
            line = line.rstrip()
            if len(line) == 0:
                continue

            yield Scratchcard.from_line(line)


def part_one(scratchcards: Iterable[Scratchcard]):
    """Calculates the sum of scores of the scratchcards."""
    info_console.print(f"Sum of scores: {sum(scratchcard.part_one_score for scratchcard in scratchcards)}")


def part_two(scratchcards: List[Scratchcard]):
    """Calculates the number of scratchcards using the rules of part two."""
    # yield items in scratchcards, repeating the next N elements in scratchcards whenever a scratchcard has N winners
    # TODO: this could be optimized by caching the results of self_and_copies but it's fast enough as-is and Iterators don't cache well
    def self_and_copies(i: int, scratchcard: Scratchcard) -> Iterator[Scratchcard]:
        yield scratchcard
        # if the scratchcard has N winners, yield the next N scratchcards recursively (copies can yield copies)
        for j in range(i + 1, i + 1 + scratchcard.number_of_winners):
            yield from self_and_copies(j, scratchcards[j])

    def all_self_and_copies() -> Iterator[Scratchcard]:
        for i, scratchcard in enumerate(scratchcards):
            yield from self_and_copies(i, scratchcard)

    info_console.print(
        f"Number of scratchcards: {ilen(all_self_and_copies())}")


def main():
    scratchcards = list(get_input())
    part_one(scratchcards)
    part_two(scratchcards)


if __name__ == "__main__":
    main()
