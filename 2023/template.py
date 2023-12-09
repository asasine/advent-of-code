#!/usr/bin/env python3

from dataclasses import dataclass
from pathlib import Path
from rich.console import Console

current_file = Path(__file__).absolute()
problem_number = current_file.stem
info_console = Console(stderr=True)


def get_input(example: bool = False):
    data_file = current_file.parent / "data" / "input"
    if example:
        data_file /= "example"

    data_file /= f"{problem_number}.txt"


def main():
    pass


if __name__ == "__main__":
    main()
