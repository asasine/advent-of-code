#!/usr/bin/env python3

import sys
from dataclasses import dataclass
import re
from typing import List, Optional
from rich.console import Console

info_console = Console(stderr=True)

map_beginning_pattern = re.compile(r"^(?P<source>\w+)-to-(?P<destination>\w+) map:$")


@dataclass
class Range:
    destination_start: int
    source_start: int
    length: int

    def destination_range(self) -> range:
        """Gets the inclusive range of the destination category."""
        return range(self.destination_start, self.destination_start + self.length + 1)

    def source_range(self) -> range:
        """Gets the inclusive range of the source category."""
        return range(self.source_start, self.source_start + self.length + 1)

    def map(self, source_number: int) -> Optional[int]:
        """Maps a source number to a destination number."""
        if source_number in self.source_range():
            return self.destination_start + (source_number - self.source_start)

        return None


@dataclass
class CategoryMap:
    source: str
    destination: str
    ranges: List[Range]

    def source_to_destination(self, source_number: int) -> int:
        """Maps a source number to a destination number."""
        for range in self.ranges:
            destination_number = range.map(source_number)
            if destination_number is not None:
                return destination_number

        return source_number


@dataclass
class Problem:
    seeds: List[int]
    """The seeds."""

    maps: List[CategoryMap]
    """The maps from source categories to destination categories."""

    def get_location_number(self, seed: int) -> int:
        """Gets the location number for a seed."""
        maps = {map.source: map for map in self.maps}
        current_map = maps["seed"]
        while current_map.destination != "location":
            seed = current_map.source_to_destination(seed)
            current_map = maps[current_map.destination]

        return current_map.source_to_destination(seed)

    def lowest_location_number(self) -> int:
        """Gets the lowest location number from the initial seeds."""
        return min(self.get_location_number(seed) for seed in self.seeds)


def part_one(problem: Problem):
    """Solve part one of the problem."""
    lowest_location_number = min(problem.get_location_number(seed) for seed in problem.seeds)
    info_console.print(f"Lowest location number: {lowest_location_number}")
    return lowest_location_number


def part_two(problem: Problem):
    """Solve part two of the problem."""
    # need to efficiently find the minimum intersection of ranges from different categories without iterating over all
    # numbers, since part two changes the seeds to be ranges, and the problem input is too large to iterate over all
    pass


def get_input(example: bool = False):
    problem = Problem([], [])
    seeds_line = sys.stdin.readline().strip()
    for i, line in enumerate(sys.stdin):
        line = line.strip()
        if i == 0:
            seeds_line = seeds_line.lstrip("seeds: ")
            seeds = [int(seed) for seed in seeds_line.split(" ")]
            info_console.log(f"Seeds: {seeds}")
            problem.seeds = seeds
            continue

        if not line:
            continue

        match = map_beginning_pattern.match(line)
        if match:
            source = match.group("source")
            destination = match.group("destination")
            info_console.log(f"Map source {source} to destination {destination}:")
            problem.maps.append(CategoryMap(source, destination, []))
            continue
        else:
            ranges = line.split(" ")
            if len(ranges) != 3:
                raise ValueError(f"Invalid line, expected 3 numbers: {line}")

            problem.maps[-1].ranges.append(Range(destination_start=int(ranges[0]),
                                            source_start=int(ranges[1]), length=int(ranges[2])))

        info_console.log(line)

    return problem


def main():
    problem = get_input()
    info_console.print(problem)
    print(part_one(problem))
    print(part_two(problem))


if __name__ == "__main__":
    main()
