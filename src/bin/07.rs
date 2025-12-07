use std::mem;
use std::{collections::BTreeMap, sync::Arc};

use cached::proc_macro::cached;
use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum TileType {
    Start,
    Empty,
    Splitter,
}

fn parse_lines(input: &str) -> Vec<Vec<TileType>> {
    input
        .lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|&b| match b {
                    b'.' => TileType::Empty,
                    b'S' => TileType::Start,
                    b'^' => TileType::Splitter,
                    _ => unreachable!("invalid input"),
                })
                .collect()
        })
        .collect()
}

#[cached]
fn recurse_part2(
    index: usize,
    current_line: usize,
    acc: usize,
    tachyon_manifold: Arc<Vec<Vec<TileType>>>,
) -> Option<usize> {
    let next_line = tachyon_manifold.get(current_line);

    if let Some(line) = next_line {
        match line.get(index) {
            Some(TileType::Splitter) => {
                let left = recurse_part2(index - 1, current_line + 1, 1, tachyon_manifold.clone())
                    .unwrap_or(0);
                let right = recurse_part2(index + 1, current_line + 1, 1, tachyon_manifold.clone())
                    .unwrap_or(0);
                Some(left + right)
            }
            Some(TileType::Empty) => {
                recurse_part2(index, current_line + 1, acc, tachyon_manifold.clone())
            }
            _ => Some(acc),
        }
    } else {
        Some(acc)
    }
}

fn recurse_part1(
    index: usize,
    current_line: usize,
    visited: &mut HashSet<(usize, usize)>,
    tachyon_manifold: &[Vec<TileType>],
) {
    let next_line = tachyon_manifold.get(current_line);

    if let Some(line) = next_line {
        match line.get(index) {
            Some(TileType::Splitter) => {
                if visited.insert((current_line, index)) {
                    recurse_part1(index - 1, current_line + 1, visited, tachyon_manifold);
                    recurse_part1(index + 1, current_line + 1, visited, tachyon_manifold);
                };
            }
            Some(TileType::Empty) => {
                recurse_part1(index, current_line + 1, visited, tachyon_manifold)
            }
            _ => {}
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let tachyon_manifold = parse_lines(input);
    let start_index = tachyon_manifold
        .first()
        .unwrap()
        .iter()
        .position(|&t| t == TileType::Start)
        .expect("No start found in first line");

    let mut visited = HashSet::new();

    let mut current_blocks: HashSet<usize> = HashSet::new();
    let mut next_blocks: HashSet<usize> = HashSet::new();
    current_blocks.insert(start_index);

    tachyon_manifold
        .iter()
        .enumerate()
        .skip(1)
        .for_each(|(line_index, current_line)| {
            next_blocks.clear();

            for &index in &current_blocks {
                match current_line.get(index) {
                    Some(TileType::Splitter) => {
                        visited.insert((line_index, index));
                        next_blocks.insert(index - 1);
                        next_blocks.insert(index + 1);
                    }
                    Some(TileType::Empty) => {
                        next_blocks.insert(index);
                    }
                    _ => {}
                }
            }
            mem::swap(&mut current_blocks, &mut next_blocks);
        });

    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let tachyon_manifold = parse_lines(input);
    let start_index = tachyon_manifold
        .first()
        .unwrap()
        .iter()
        .position(|&t| t == TileType::Start)
        .expect("No start found in first line");

    let mut blocks: HashMap<usize, usize> = HashMap::new();
    blocks.insert(start_index, 1);

    (1..tachyon_manifold.len()).for_each(|line_index| {
        let current_blocks = blocks.clone();
        blocks.clear();
        let current_line = tachyon_manifold.get(line_index);
        if current_line.is_none() {
            return;
        }
        let current_line = current_line.unwrap();

        for (index, size) in current_blocks {
            match current_line.get(index) {
                Some(TileType::Splitter) => {
                    *blocks.entry(index - 1).or_insert(0) += size;
                    *blocks.entry(index + 1).or_insert(0) += size;
                }
                Some(TileType::Empty) => {
                    *blocks.entry(index).or_insert(0) += size;
                }
                _ => {}
            }
        }
    });

    Some(blocks.iter().fold(0, |acc, (_, size)| acc + size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
