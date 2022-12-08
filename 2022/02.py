#!/usr/bin/env python3

from collections import namedtuple
from enum import Enum, auto
from pathlib import Path
from typing import Iterable, Iterator, List
from rich.console import Console

current_file = Path(__file__).absolute()

class Outcome(Enum):
    LOSE = 0
    DRAW = 3
    WIN = 6

    def score(self) -> int:
        return self.value

    @classmethod
    def from_character(cls, c: str) -> "Outcome":
        if c == "X":
            return cls.LOSE
        elif c == "Y":
            return cls.DRAW
        elif c == "Z":
            return cls.WIN
        else:
            raise ValueError(f"Invalid character: {c}")

class Shape(Enum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3

    @classmethod
    def from_character(cls, c: str) -> "Shape":
        if c == "A" or c == "X":
            return cls.ROCK
        elif c == "B" or c == "Y":
            return cls.PAPER
        elif c == "C" or c == "Z":
            return cls.SCISSORS
        else:
            raise ValueError(f"Invalid character: {c}")

    def beats(self, other: "Shape") -> Outcome:
        if self == other:
            return Outcome.DRAW
        elif self == Shape.ROCK:
            if other == Shape.SCISSORS:
                return Outcome.WIN
            elif other == Shape.PAPER:
                return Outcome.LOSE
            else:
                raise ValueError(f"Invalid other shape: {other}")
        elif self == Shape.PAPER:
            if other == Shape.ROCK:
                return Outcome.WIN
            elif other == Shape.SCISSORS:
                return Outcome.LOSE
            else:
                raise ValueError(f"Invalid other shape: {other}")
        elif self == Shape.SCISSORS:
            if other == Shape.PAPER:
                return Outcome.WIN
            elif other == Shape.ROCK:
                return Outcome.LOSE
            else:
                raise ValueError(f"Invalid other shape: {other}")
        else:
            raise ValueError(f"Invalid shape: {self}")

    def score(self, other: "Shape") -> int:
        outcome = self.beats(other)
        return self.value + outcome.score()

    def get_other_shape(self, outcome: Outcome) -> "Shape":
        """Gets the shape that would result in the given outcome against this shape."""
        if outcome == Outcome.DRAW:
            return self
        elif outcome == Outcome.WIN:
            if self == Shape.ROCK:
                return Shape.PAPER
            elif self == Shape.PAPER:
                return Shape.SCISSORS
            elif self == Shape.SCISSORS:
                return Shape.ROCK
            else:
                raise ValueError(f"Invalid shape: {self}")
        elif outcome == Outcome.LOSE:
            if self == Shape.ROCK:
                return Shape.SCISSORS
            elif self == Shape.PAPER:
                return Shape.ROCK
            elif self == Shape.SCISSORS:
                return Shape.PAPER
            else:
                raise ValueError(f"Invalid shape: {self}")
        else:
            raise ValueError(f"Invalid outcome: {outcome}")

info_console = Console(stderr=True)

Game = namedtuple("Game", ["opponent", "me_shape", "me_desired_outcome"])

def read_input() -> List[Game]:
    with (current_file.parent / "data" / "input" / "02.txt").open() as f:
        def create_game(line: str) -> Game:
            opponent, me = line.split()
            return Game(
                Shape.from_character(opponent),
                Shape.from_character(me),
                Outcome.from_character(me),
            )

        return [create_game(line) for line in f if line.strip() != ""]

def get_part_1_scores(games: Iterable[Game]) -> Iterator[int]:
    return (game.me_shape.score(game.opponent) for game in games)

def get_part_2_scores(games: Iterable[Game]) -> Iterator[int]:
    def get_score(game: Game) -> int:
        me = game.opponent.get_other_shape(game.me_desired_outcome)
        return me.score(game.opponent)

    return (get_score(game) for game in games)

def main():
    games = read_input()
    info_console.print(f"Read {len(games)} games")

    info_console.print(f"Total score (part 1): {sum(get_part_1_scores(games))}")
    info_console.print(f"Total score (part 2): {sum(get_part_2_scores(games))}")

if __name__ == "__main__":
    main()
