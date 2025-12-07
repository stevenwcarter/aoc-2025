use hashbrown::{HashMap, HashSet};
use nohash::BuildNoHashHasher;
use std::mem;

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

fn find_start_index(tachyon_manifold: &[Vec<TileType>]) -> usize {
    tachyon_manifold
        .first()
        .unwrap()
        .iter()
        .position(|&t| t == TileType::Start)
        .expect("No start found in first line")
}

pub fn part_one(input: &str) -> Option<usize> {
    let tachyon_manifold = parse_lines(input);
    let start_index = find_start_index(&tachyon_manifold);

    let mut visited = 0;

    let mut current_blocks: HashSet<usize, BuildNoHashHasher<usize>> =
        HashSet::with_hasher(BuildNoHashHasher::default());
    let mut next_blocks: HashSet<usize, BuildNoHashHasher<usize>> =
        HashSet::with_hasher(BuildNoHashHasher::default());
    current_blocks.insert(start_index);

    tachyon_manifold.iter().skip(1).for_each(|current_line| {
        next_blocks.clear();

        for &index in &current_blocks {
            match current_line.get(index) {
                Some(TileType::Splitter) => {
                    visited += 1;
                    next_blocks.insert(index - 1);
                    next_blocks.insert(index + 1);
                }
                Some(TileType::Empty) => {
                    next_blocks.insert(index);
                }
                _ => unreachable!("no other tile types should be possible"),
            }
        }
        mem::swap(&mut current_blocks, &mut next_blocks);
    });

    Some(visited)
}

pub fn part_two(input: &str) -> Option<usize> {
    let tachyon_manifold = parse_lines(input);
    let start_index = find_start_index(&tachyon_manifold);

    let mut current_blocks: HashMap<usize, usize, BuildNoHashHasher<usize>> =
        HashMap::with_hasher(BuildNoHashHasher::default());
    let mut next_blocks: HashMap<usize, usize, BuildNoHashHasher<usize>> =
        HashMap::with_hasher(BuildNoHashHasher::default());
    current_blocks.insert(start_index, 1);

    tachyon_manifold.iter().skip(1).for_each(|current_line| {
        next_blocks.clear();

        for (&index, &size) in &current_blocks {
            match current_line.get(index) {
                Some(TileType::Splitter) => {
                    *next_blocks.entry(index - 1).or_insert(0) += size;
                    *next_blocks.entry(index + 1).or_insert(0) += size;
                }
                Some(TileType::Empty) => {
                    *next_blocks.entry(index).or_insert(0) += size;
                }
                _ => unreachable!("no other tile types should be possible"),
            }
        }
        mem::swap(&mut current_blocks, &mut next_blocks);
    });

    Some(current_blocks.iter().fold(0, |acc, (_, size)| acc + size))
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
