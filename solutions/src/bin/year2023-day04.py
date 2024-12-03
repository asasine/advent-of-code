#!/usr/bin/env python3

import sys
from functools import lru_cache
import re
from dataclasses import dataclass
from typing import FrozenSet, Iterable, Iterator, List, Optional
from rich.console import Console

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
    for line in sys.stdin:
        line = line.rstrip()
        if len(line) == 0:
            continue

        yield Scratchcard.from_line(line)


def part_one(scratchcards: Iterable[Scratchcard]) -> int:
    """Calculates the sum of scores of the scratchcards."""
    result = sum(scratchcard.part_one_score for scratchcard in scratchcards)
    info_console.print(f"Sum of scores: {result}")
    return result


def part_two(scratchcards: List[Scratchcard]) -> int:
    """
    Calculates the number of scratchcards using the rules of part two.

    If a scratchcard has N winners, the next N scratchcards are copied, recursively. This process repeats until the end
    of the list of scratchcards is reached, and cards never make a copiy past the end of the list.
    """
    # yield items in scratchcards, repeating the next N elements in scratchcards whenever a scratchcard has N winners
    @lru_cache(maxsize=len(scratchcards))
    def count_self_and_copies(i: int) -> int:
        scratchcard = scratchcards[i]
        # the last scratchcard is always 1, because there's nothing after it to copy
        if i == len(scratchcards) - 1:
            return 1

        start_j = i + 1

        # don't go past the end of the list
        end_j = min(len(scratchcards), start_j + scratchcard.number_of_winners)
        return 1 + sum(count_self_and_copies(j) for j in range(start_j, end_j))

    count = sum(count_self_and_copies(i) for i in range(len(scratchcards)))
    info_console.print(f"Number of scratchcards: {count}")
    info_console.print(f"Cache statistics: {count_self_and_copies.cache_info()}")
    return count


def main():
    scratchcards = list(get_input())
    print(part_one(scratchcards))
    print(part_two(scratchcards))


if __name__ == "__main__":
    main()
