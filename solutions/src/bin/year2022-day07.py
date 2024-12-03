#!/usr/bin/env python3

import sys
from functools import total_ordering
from typing import Dict, Iterable, Optional, Set
from more_itertools import first_true
from rich.console import Console

info_console = Console(stderr=True)

@total_ordering
class File:
    def __init__(self, name: str, size: int, parent: "Directory"):
        self.name = name
        self._size = size
        self.parent = parent

    def size(self) -> int:
        return self._size

    def __hash__(self) -> int:
        return hash(self.name) ^ hash(self.parent)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, File):
            return NotImplemented

        return self.name == other.name and self.parent == other.parent

    def __lt__(self, other: object) -> bool:
        if not isinstance(other, File):
            return NotImplemented

        return self.name < other.name

    def __repr__(self):
        return f"File({self.name}, size={self.size()})"

@total_ordering
class Directory:
    def __init__(self, name: str, parent: Optional["Directory"] = None):
        self.name = name
        self.parent = parent

        self.files: Set[File] = set()
        self.directories: Dict[str, Directory] = dict()

    def walk(self) -> Iterable[File]:
        """Recursively walks all files in this directory."""
        yield from sorted(self.files)
        for directory in self.directories.values():
            yield from directory.walk()

    def get_maximal_root(self) -> "Directory":
        """Gets the absolute root of this tree of directories."""
        if self.parent is None:
            return self

        return self.parent.get_maximal_root()

    def size(self) -> int:
        return sum(child.size() for child in self.walk())

    def absolute_path(self) -> str:
        if self.parent is None:
            return "/"

        return self.parent.absolute_path() + self.name + "/"

    def __hash__(self) -> int:
        return hash(self.name) ^ hash(self.parent)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Directory):
            return NotImplemented

        return self.name == other.name and self.parent == other.parent

    def __lt__(self, other: object) -> bool:
        if not isinstance(other, Directory):
            return NotImplemented

        return self.name < other.name

    def __repr__(self):
        return f"Directory({self.absolute_path()})"


def read_input() -> Directory:
    root = Directory("/")
    for line in sys.stdin:
        if line.startswith("$ cd"):
            dir = line[5:].strip()
            if dir == "/":
                root = root.get_maximal_root()
            elif dir == "..":
                if root.parent is None:
                    raise ValueError("Cannot go up from root")

                root = root.parent
            else:
                root = root.directories[dir]

        elif line.startswith("$ ls"):
            pass
        elif line.startswith("dir "):
            name = line[4:].strip()
            root.directories[name] = Directory(name, root)
        elif line.strip() != "":
            size, name = line.strip().split(" ")
            size = int(size)
            root.files.add(File(name, size, root))

    return root.get_maximal_root()

def part_1(root: Directory, max: int = 100000) -> int:
    """Returns the sum of the size of all directories with a total size of at most max"""
    size = root.size()
    inner_size = sum(part_1(directory) for directory in root.directories.values())
    return inner_size + (size * (size <= max))

def part_2(root: Directory, total_size: int = 70000000, required: int = 30000000) -> Directory:
    """Returns the smallest directory that, when deleted, will increase the free space by at least required."""

    def get_all_directories(root: Directory) -> Iterable[Directory]:
        yield root
        for directory in root.directories.values():
            yield from get_all_directories(directory)

    size = root.size()
    free_space = total_size - size
    needed = required - free_space
    sizes = ((directory, directory.size()) for directory in get_all_directories(root))
    sizes = sorted(sizes, key=lambda x: x[1])
    sizes = first_true(sizes, default=(root, size), pred=lambda x: x[1] >= needed)
    return sizes[0]

def main():
    root = read_input()
    info_console.print(f"Read directory {root.name} (size: {root.size()})")
    print(part_1(root))

    part_2_dir = part_2(root)
    part_2_dir_size = part_2_dir.size()
    info_console.print(f"Part 2: {part_2_dir_size} (removing {part_2_dir} to free up {part_2_dir_size} bytes)")
    print(part_2_dir_size)

if __name__ == "__main__":
    main()
