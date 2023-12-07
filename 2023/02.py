#!/usr/bin/env python3

import operator
import re
from dataclasses import dataclass
from functools import partial, reduce
from pathlib import Path
from typing import Dict, Iterator, List
from rich.console import Console

current_file = Path(__file__).absolute()
info_console = Console(stderr=True)

game_pattern = re.compile(r"^Game (\d+):")
cube_subset_pattern = re.compile(r"(?:(?P<num>\d+) (?P<color>\w+))(?P<sep>(?:, )|;|$)")


@dataclass
class Game:
    number: int
    """The game number."""

    cubes: List[Dict[str, int]]
    """The subsets of cubes in a game, mapping each color to the number drawn in that subset."""

    @classmethod
    def from_line(cls, str):
        match = game_pattern.match(str)
        if not match:
            raise ValueError(f"Invalid game line: {str}")

        number = int(match.group(1))
        subsets = [{}]
        for match in cube_subset_pattern.finditer(str):
            num = match.group("num")
            color = match.group("color")
            sep = match.group("sep")

            subsets[-1][color] = int(num)

            if sep == ";":
                # next match is a new subset
                subsets.append({})

        return cls(number, subsets)

    def has_lte_cubes(self, color: str, num: int) -> bool:
        """
        Returns whether all subsets have less than or equal to the given number of cubes of the given color.

        Args:
            color (str): The color of the cubes.
            num (int): The maximum number of cubes.

        Returns:
            bool: True if all subsets have less than or equal to the given number of cubes of the given color.
        """
        def subset_has_lte_cubes(subset: Dict[str, int]) -> bool:
            return color not in subset or subset[color] <= num

        return all(map(subset_has_lte_cubes, self.cubes))

    def is_possible(self, maximums: Dict[str, int]) -> bool:
        """
        Checks whether this game is possible.

        A game is possible if none of the subsets have more than the given maximum number of cubes of each color.

        Args:
            maximums (Dict[str, int]): The maximum number of cubes of each color.

        Returns:
            bool: True if the game is possible, false otherwise.
        """
        return all(map(lambda color: self.has_lte_cubes(color, maximums[color]), maximums))

    def fewest(self) -> Dict[str, int]:
        """
        Returns the fewest number of cubes of each color in this game in order for the game to be possible.

        This is the maximum value of each color across all subsets, which represents the minimum number of cubes of
        that color that must be present for the game to be possible.

        Returns:
            Dict[str, int]: The minimum number of cubes of each color in this game to be possible.
        """
        fewest: Dict[str, int] = {}
        for subset in self.cubes:
            for color, num in subset.items():
                if color not in fewest or num > fewest[color]:
                    fewest[color] = num

        return fewest

    def power(self) -> int:
        """
        Returns the power of this game.

        The power of a game is the product of the fewest number of cubes of each color.

        Returns:
            int: The power of this game.
        """
        return reduce(operator.mul, self.fewest().values())


def get_input() -> Iterator[Game]:
    data_file = current_file.parent / "data" / "input" / "02.txt"
    with open(data_file, "r") as file:
        for line in file:
            if len(line.strip()) == 0:
                continue

            yield Game.from_line(line)


def part_one(games: List[Game]):
    is_possible = partial(Game.is_possible, maximums={"red": 12, "green": 13, "blue": 14})
    possible_games = filter(is_possible, games)
    sum_of_game_numbers = sum(game.number for game in possible_games)
    info_console.print(f"Sum of possible game numbers: {sum_of_game_numbers}")


def part_two(games: List[Game]):
    powers = map(Game.power, games)
    sum_of_powers = sum(powers)
    info_console.print(f"Sum of powers: {sum_of_powers}")


def main():
    games = list(get_input())
    part_one(games)
    part_two(games)


if __name__ == "__main__":
    main()
