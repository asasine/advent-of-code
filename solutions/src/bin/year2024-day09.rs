//! Day 9: Disk Fragmenter
//!
//! https://adventofcode.com/2024/day/9

use core::fmt;
use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
use tracing::trace;

fn part1(input: &str) -> usize {
    let dense = DenseDiskMap::from_str(input).unwrap();
    let defragmented = dense.sparse_defragment();
    defragmented.checksum()
}

fn part2(input: &str) -> usize {
    let dense = DenseDiskMap::from_str(input).unwrap();
    let defragmented = dense.dense_defragment();
    defragmented.expand().checksum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DenseBlock {
    File { id: usize, size: usize },
    FreeSpace(usize),
}

impl IntoIterator for DenseBlock {
    type Item = SparseBlock;
    type IntoIter = std::iter::RepeatN<SparseBlock>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            DenseBlock::File { id, size } => std::iter::repeat_n(SparseBlock::File { id }, size),
            DenseBlock::FreeSpace(size) => std::iter::repeat_n(SparseBlock::FreeSpace, size),
        }
    }
}

impl fmt::Display for DenseBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            DenseBlock::File { id, .. } => format!("{{{id}}}"),
            DenseBlock::FreeSpace(_) => ".".to_string(),
        };

        let size = match self {
            DenseBlock::File { size, .. } => *size,
            DenseBlock::FreeSpace(size) => *size,
        };

        for _ in 0..size {
            write!(f, "{}", char)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SparseBlock {
    File { id: usize },
    FreeSpace,
}

impl fmt::Display for SparseBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SparseBlock::File { id } => write!(f, "{id}"),
            SparseBlock::FreeSpace => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SparseDiskMap(Vec<SparseBlock>);

impl SparseDiskMap {
    fn checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(i, block)| match block {
                SparseBlock::File { id } => i * id,
                SparseBlock::FreeSpace => 0,
            })
            .sum()
    }
}

impl fmt::Display for SparseDiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in &self.0 {
            write!(f, "{}", element)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DenseDiskMap(Vec<DenseBlock>);

impl DenseDiskMap {
    fn expand(&self) -> SparseDiskMap {
        let elements = self
            .0
            .iter()
            .flat_map(|block| block.into_iter())
            .collect_vec();

        SparseDiskMap(elements)
    }

    /// Defragment by moving sparse blocks to the beginning of the disk, fragmenting files.
    fn sparse_defragment(&self) -> SparseDiskMap {
        let mut expanded = self.expand();
        let mut start = 0;
        let mut end = expanded.0.len() - 1;
        while start < end {
            let start_block = expanded.0[start];
            if start_block != SparseBlock::FreeSpace {
                start += 1;
                continue;
            }

            let end_block = expanded.0[end];
            if end_block == SparseBlock::FreeSpace {
                end -= 1;
                continue;
            }

            expanded.0.swap(start, end);
        }

        expanded
    }

    /// Defragment by moving dense blocks to the beginning of the disk if there's contiguous free space for them to fit.
    fn dense_defragment(&self) -> DenseDiskMap {
        let mut defragmented = self.clone();
        let mut end = self.0.len() - 1;

        let mut moved = HashSet::new();

        // move end backward until a file block is found
        // find the first free space block that can fit it (if any)
        // if the free block is larger than the file block, split it before swapping
        while end > 0 {
            let end_block = &defragmented.0[end];
            let block_size = match end_block {
                DenseBlock::File { size, .. } => *size,
                b @ DenseBlock::FreeSpace(_) => {
                    trace!("Skipping free space block {b:?} at end {end}");
                    end -= 1;
                    continue;
                }
            };

            // end points to a file block

            // find the first free space block that can fit the file block (if any)
            let mut valid_blocks = defragmented.0.iter().enumerate().filter(|(i, _)| *i <= end);
            let (free_block_index, (_, free_block)) =
                match valid_blocks.find_position(|(_, block)| match block {
                    DenseBlock::FreeSpace(size) => *size >= block_size,
                    _ => false,
                }) {
                    Some(x) => x,
                    None => {
                        trace!("No free space block found that can fit {end_block:?} at {end}");
                        end -= 1;
                        continue;
                    }
                };

            if free_block_index > end {
                // can only move leftward
                continue;
            }

            // split the start block into {block_size, remaining} and swap the first with the end block
            trace!("Inserting {end_block:?} from {end} into {free_block:?} at {free_block_index}");

            let free_size = match free_block {
                DenseBlock::FreeSpace(size) => size,
                _ => unreachable!("The previous find_position predicate should have filtered this"),
            };

            if moved.contains(end_block) {
                // already moved this block
                end -= 1;
                continue;
            }

            moved.insert(*end_block);

            let remaining = free_size - block_size;
            if remaining == 0 {
                // same size, no need to split
                defragmented.0.swap(free_block_index, end);

                // end now points to the free space block that was just swapped, so decrement to check the next block
                end -= 1;
            } else {
                trace!(
                    "Splitting {free_block:?} block at {free_block_index} into {block_size} and {remaining}"
                );

                defragmented.0[free_block_index] = defragmented.0[end];
                defragmented.0[end] = DenseBlock::FreeSpace(block_size);
                defragmented
                    .0
                    .insert(free_block_index + 1, DenseBlock::FreeSpace(remaining));

                // inserting shifts everything rightward, so end now points to the next block to check
                // no need to further decrement as that would skip the next block
            }
        }

        // combine continguous free space blocks
        defragmented.combine();
        defragmented
    }

    /// Combine adjacent free space blocks.
    fn combine(&mut self) {
        let mut i = self.0.len() - 1;
        while i >= 1 {
            let prev = &self.0[i - 1];
            let current = &self.0[i];
            if let (DenseBlock::FreeSpace(a), DenseBlock::FreeSpace(b)) = (prev, current) {
                self.0[i - 1] = DenseBlock::FreeSpace(a + b);
                self.0.remove(i);
            }

            i -= 1;
        }
    }
}

impl FromStr for DenseDiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The input is alternating characters of the number of files in a block and the number of free blocks.
        let mut elements = Vec::new();
        let numbers = s.trim().chars().map(|c| c.to_digit(10).unwrap() as usize);
        for (file_id, mut chunk) in numbers.chunks(2).into_iter().enumerate() {
            let size = chunk.next().unwrap();
            let file = DenseBlock::File { id: file_id, size };
            elements.push(file);

            // the last chunk isn't followed by free space
            match chunk.next() {
                Some(free_space) if free_space > 0 => {
                    elements.push(DenseBlock::FreeSpace(free_space))
                }
                _ => {}
            }
        }

        Ok(Self(elements))
    }
}

impl fmt::Display for DenseDiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in &self.0 {
            write!(f, "{}", element)?;
        }

        Ok(())
    }
}

aoc_macro::aoc_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/09/1.txt");
        assert_eq!(1928, part1(input));
    }

    #[test]
    fn dense_block_file_into_iter() {
        let id = 42;
        let size = 10;
        let block = DenseBlock::File { id, size };
        let expected = std::iter::repeat_n(SparseBlock::File { id }, 10);
        assert_eq!(expected.collect_vec(), block.into_iter().collect_vec());
    }

    #[test]
    fn dense_block_free_space_into_iter() {
        let size = 10;
        let block = DenseBlock::FreeSpace(size);
        let expected = std::iter::repeat_n(SparseBlock::FreeSpace, 10);
        assert_eq!(expected.collect_vec(), block.into_iter().collect_vec());
    }

    #[test]
    fn expand() {
        let dense = DenseDiskMap::from_str("12345").unwrap();
        let expanded = dense.expand();
        let expected = SparseDiskMap(vec![
            SparseBlock::File { id: 0 },
            SparseBlock::FreeSpace,
            SparseBlock::FreeSpace,
            SparseBlock::File { id: 1 },
            SparseBlock::File { id: 1 },
            SparseBlock::File { id: 1 },
            SparseBlock::FreeSpace,
            SparseBlock::FreeSpace,
            SparseBlock::FreeSpace,
            SparseBlock::FreeSpace,
            SparseBlock::File { id: 2 },
            SparseBlock::File { id: 2 },
            SparseBlock::File { id: 2 },
            SparseBlock::File { id: 2 },
            SparseBlock::File { id: 2 },
        ]);

        assert_eq!(expected, expanded);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/09/1.txt");
        assert_eq!(2858, part2(input));
    }

    #[test]
    fn dense_defragment() {
        let dense = DenseDiskMap::from_str("2333133121414131402").unwrap();
        // sanity check
        let expected = DenseDiskMap(vec![
            DenseBlock::File { id: 0, size: 2 },
            DenseBlock::FreeSpace(3),
            DenseBlock::File { id: 1, size: 3 },
            DenseBlock::FreeSpace(3),
            DenseBlock::File { id: 2, size: 1 },
            DenseBlock::FreeSpace(3),
            DenseBlock::File { id: 3, size: 3 },
            DenseBlock::FreeSpace(1),
            DenseBlock::File { id: 4, size: 2 },
            DenseBlock::FreeSpace(1),
            DenseBlock::File { id: 5, size: 4 },
            DenseBlock::FreeSpace(1),
            DenseBlock::File { id: 6, size: 4 },
            DenseBlock::FreeSpace(1),
            DenseBlock::File { id: 7, size: 3 },
            DenseBlock::FreeSpace(1),
            DenseBlock::File { id: 8, size: 4 },
            DenseBlock::File { id: 9, size: 2 },
        ]);

        assert_eq!(expected, dense);

        eprintln!("{}", dense);
        let defragmented = dense.dense_defragment();
        let expected = DenseDiskMap(vec![
            DenseBlock::File { id: 0, size: 2 },
            DenseBlock::File { id: 9, size: 2 },
            DenseBlock::File { id: 2, size: 1 },
            DenseBlock::File { id: 1, size: 3 },
            DenseBlock::File { id: 7, size: 3 },
            DenseBlock::FreeSpace(1),
            DenseBlock::File { id: 4, size: 2 },
            DenseBlock::FreeSpace(1),
            DenseBlock::File { id: 3, size: 3 },
            DenseBlock::FreeSpace(4),
            DenseBlock::File { id: 5, size: 4 },
            DenseBlock::FreeSpace(1),
            DenseBlock::File { id: 6, size: 4 },
            DenseBlock::FreeSpace(5),
            DenseBlock::File { id: 8, size: 4 },
            DenseBlock::FreeSpace(2),
        ]);

        assert_eq!(expected, defragmented);
    }

    #[test]
    fn combine() {
        let mut actual = DenseDiskMap(vec![
            DenseBlock::FreeSpace(1),
            DenseBlock::FreeSpace(2),
            DenseBlock::FreeSpace(3),
            DenseBlock::FreeSpace(4),
            DenseBlock::File { id: 0, size: 1 },
            DenseBlock::FreeSpace(5),
            DenseBlock::FreeSpace(6),
            DenseBlock::File { id: 1, size: 1 },
            DenseBlock::FreeSpace(7),
        ]);

        actual.combine();

        let expected = DenseDiskMap(vec![
            DenseBlock::FreeSpace(10),
            DenseBlock::File { id: 0, size: 1 },
            DenseBlock::FreeSpace(11),
            DenseBlock::File { id: 1, size: 1 },
            DenseBlock::FreeSpace(7),
        ]);

        assert_eq!(expected, actual);
    }
}
