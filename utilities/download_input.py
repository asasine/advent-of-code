#!/usr/bin/env python3

import argparse
import datetime
from pathlib import Path
import requests
from rich.console import Console
from typing import TextIO

info_console = Console(stderr=True)

def get_default_advent_day() -> int:
    """Returns the current day of the month."""
    return datetime.date.today().day

def get_advent_year() -> int:
    """
    Returns the current advent year.

    If it is before December 1st, it returns the previous year.
    """
    today = datetime.date.today()
    if today.month == 12 and today.day >= 1:
        return today.year
    else:
        return today.year - 1

def get_default_output_file(day: int, year: int) -> TextIO:
    """
    Returns the default output file for the given day and year.
    """
    path = Path(Path(__file__).absolute().parent.parent, str(year), "data", "input", f"{day:02d}.txt")
    path.parent.mkdir(parents=True, exist_ok=True)
    return argparse.FileType("w")(str(path))

def download_input(day: int, year: int, session_key_file: TextIO, output_file: TextIO) -> None:
    """
    Downloads the input for the given day and year and writes it to the given output file.

    Args:
        day (int): Day of the Advent of Code
        year (int): Year of the Advent of Code
        session_key_file (TextIO): File containing the user's Advent of Code session key
        output_file (TextIO): Output file
    """
    info_console.print(f"Downloading input for day {day} of {year} to {output_file.name} using session key from {session_key_file.name}")
    session_key = session_key_file.readline().strip()
    print(requests.get(f"https://adventofcode.com/{year}/day/{day}/input", cookies={"session": session_key}).text, file=output_file)

def main():
    parser = argparse.ArgumentParser(formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("day", type=int, help="Day of the Advent of Code")
    parser.add_argument("year", type=int, help="Year of the Advent of Code", default=get_advent_year(), nargs="?")
    parser.add_argument("-o", "--output", type=argparse.FileType("w"), help="Output file")
    parser.add_argument("--session-key-file", type=argparse.FileType("r"),
        help="A file containing the user's Advent of Code session key",
        default=argparse.FileType("r")(Path(Path(__file__).absolute().parent.parent, "session.key").__str__()))

    parser.add_argument("-v", "--verbose", help="Verbose output", action="store_true")

    known_args, _ = parser.parse_known_args()
    parser.set_defaults(output=get_default_output_file(known_args.day, known_args.year))

    args = parser.parse_args()
    if args.verbose:
        info_console.print(args)

    download_input(args.day, args.year, args.session_key_file, args.output)

if __name__ == "__main__":
    main()
