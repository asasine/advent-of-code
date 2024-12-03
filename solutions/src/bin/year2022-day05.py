#!/usr/bin/env python3

import sys
from itertools import takewhile
from more_itertools import windowed
import re
from rich.console import Console
import rich.repr
from typing import Iterable, List, Optional, Tuple

info_console = Console(stderr=True)

@rich.repr.auto
class Stack:
    def __init__(self, id: int, crates: Iterable[str]):
        self.id = id
        self.crates = list(crates)

    def copy(self):
        return Stack(self.id, self.crates.copy())

    def move_individually(self, n: int, to: "Stack"):
        """Move n crates from the top of this stack to the given stack one create at a time."""
        to.crates += reversed(self.crates[-n:])
        self.crates = self.crates[:-n]

    def move_block(self, n: int, to: "Stack"):
        """Move n crates from the top of this stack to the given stack in one go."""
        to.crates += self.crates[-n:]
        self.crates = self.crates[:-n]

@rich.repr.auto
class Command:
    def __init__(self, source_id: int, destination_id: int, n: int):
        self.source_id = source_id
        self.destination_id = destination_id
        self.n = n

    @classmethod
    def _get_stack_with_id(cls, stacks: List[Stack], id: int) -> Stack:
        return next(stack for stack in stacks if stack.id == id)

    def execute_with_cratemover_9000(self, stacks: List[Stack]):
        source = Command._get_stack_with_id(stacks, self.source_id)
        destination = Command._get_stack_with_id(stacks, self.destination_id)
        source.move_individually(self.n, destination)

    def execute_with_cratemover_9001(self, stacks: List[Stack]):
        source = Command._get_stack_with_id(stacks, self.source_id)
        destination = Command._get_stack_with_id(stacks, self.destination_id)
        source.move_block(self.n, destination)


def read_input() -> Tuple[List[Stack], List[Command]]:
    def is_stack_id_line(line: str) -> bool:
        line = line.strip()
        return len(line) > 0 and line[0].isdigit()

    lines = sys.stdin.readlines()
    stacks = []
    stacks_rows = takewhile(lambda line: not is_stack_id_line(line), lines)
    stacks_rows = list(stacks_rows)
    for line in reversed(stacks_rows):
        row = line.rstrip()
        for i, crate in enumerate(windowed(row, 3, step=4)):
            while len(stacks) < i + 1:
                stacks.append([])

            if crate[1] != " ":
                stacks[i].append(crate[1])

    stacks = list(map(lambda p: Stack(p[0] + 1, p[1]), enumerate(stacks)))

    move_pattern = re.compile(r"move (?P<n>\d+) from (?P<source>\d+) to (?P<destination>\d+)")
    def create_command(line: str) -> Optional[Command]:
        m = move_pattern.match(line)
        if m is None:
            return None

        source_id = int(m.group("source"))
        destination_id = int(m.group("destination"))
        n = int(m.group("n"))
        return Command(source_id=source_id, destination_id=destination_id, n=n)

    commands = list(filter(None, map(create_command, filter(None, lines))))
    return (stacks, commands)

def top_of_stacks(stacks: List[Stack]) -> str:
    return "".join(map(lambda stack: stack.crates[-1], stacks))

def part_1(stacks: List[Stack], commands: List[Command]) -> str:
    for command in commands:
        command.execute_with_cratemover_9000(stacks)

    return top_of_stacks(stacks)

def part_2(stacks: List[Stack], commands: List[Command]) -> str:
    for command in commands:
        command.execute_with_cratemover_9001(stacks)

    return top_of_stacks(stacks)

def main():
    stacks, commands = read_input()

    info_console.print("Initial stacks:")
    for stack in stacks:
        info_console.print(stack)

    info_console.print()
    info_console.print(f"Loaded {len(commands)} commands")

    print(part_1(list(map(Stack.copy, stacks)), commands))
    print(part_2(list(map(Stack.copy, stacks)), commands))

if __name__ == "__main__":
    main()
