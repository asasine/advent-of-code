#!/usr/bin/env python3

import sys
import re
from dataclasses import dataclass
from typing import Iterator, Optional
from rich.console import Console

info_console = Console(stderr=True)

# https://stackoverflow.com/a/5616910/1472764
# a capturing group inside a lookahead so that the capture is technically zero-length, allowing for overlapping matches
is_digit = re.compile(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))")
to_digit = {
    "1": 1,
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

@dataclass
class CalibrationValue:
    line: str
    first_digit: int
    second_digit: int

    @property
    def value(self) -> int:
        return self.first_digit * 10 + self.second_digit

    @classmethod
    def from_line(cls, line: str) -> Optional["CalibrationValue"]:
        """
        Reads a line and returns a calibration value from the first digit and last digit in the line.

        Args:
            line (str): The line to read.

        Returns:
            CalibrationValue: The parsed calibration value.
        """
        if len(line.strip()) == 0:
            return None

        # find the first and last characters that are digits
        first_digit = None
        second_digit = None
        for match in is_digit.finditer(line):
            if first_digit is None:
                first_digit = to_digit[match[1]]

            second_digit = to_digit[match[1]]

        if first_digit is None or second_digit is None:
            raise ValueError(f"Could not find two digits in line: {line}")

        return cls(line=line, first_digit=first_digit, second_digit=second_digit)


def read_input() -> Iterator[CalibrationValue]:
    for line in sys.stdin:
        if line.strip() == "":
            continue

        calibration_value = CalibrationValue.from_line(line)
        if calibration_value is None:
            continue

        yield calibration_value


def main():
    calbiration_values = list(read_input())
    info_console.print(f"Read {len(calbiration_values)} calibration values")
    s = sum(value.value for value in calbiration_values)
    info_console.print(f"Sum of all values: {s}")
    print(s)



if __name__ == "__main__":
    main()
