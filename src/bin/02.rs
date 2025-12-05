advent_of_code::solution!(2);

use atoi_simd::parse;
use hashbrown::HashSet;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn construct_repetitive(seed: usize, repetitions: usize, seed_len: u32) -> usize {
    let mut result = seed;
    let multiplier = 10_usize.pow(seed_len);

    for _ in 1..repetitions {
        result = result * multiplier + seed;
    }

    result
}

fn generate_invalid_numbers_part1(min: usize, max: usize) -> HashSet<usize> {
    let mut invalid_numbers = HashSet::new();

    // calculate digit lengths
    let min_len = min.ilog10() + 1;
    let max_len = max.ilog10() + 1;

    for total_len in min_len..=max_len {
        let seed_lengths: Vec<u32> = vec![total_len / 2];

        for seed_len in seed_lengths {
            let repetitions = 2;

            let start_seed = 10_usize.pow(seed_len - 1);
            let end_seed = 10_usize.pow(seed_len) - 1;

            for seed in start_seed..=end_seed {
                let candidate = construct_repetitive(seed, repetitions, seed_len);

                if candidate >= min && candidate <= max {
                    invalid_numbers.insert(candidate);
                }
            }
        }
    }

    invalid_numbers
}
fn generate_invalid_numbers_part2(min: usize, max: usize) -> HashSet<usize> {
    let mut invalid_numbers = HashSet::new();

    // calculate digit lengths
    let min_len = min.ilog10() + 1;
    let max_len = max.ilog10() + 1;

    for total_len in min_len..=max_len {
        let seed_lengths: Vec<u32> = (1..=total_len / 2)
            .filter(|&len| total_len % len == 0)
            .collect();

        for seed_len in seed_lengths {
            let repetitions = (total_len / seed_len) as usize;

            let start_seed = 10_usize.pow(seed_len - 1);
            let end_seed = 10_usize.pow(seed_len) - 1;

            for seed in start_seed..=end_seed {
                let candidate = construct_repetitive(seed, repetitions, seed_len);

                if candidate >= min && candidate <= max {
                    invalid_numbers.insert(candidate);
                }
            }
        }
    }

    invalid_numbers
}

fn solve(input: &str, part1: bool) -> Option<usize> {
    Some(
        input
            .split(',')
            .par_bridge()
            .map(|r| r.split_once('-').unwrap())
            .map(|(a, b)| {
                (
                    parse::<usize>(a.trim().as_bytes()).unwrap(),
                    parse::<usize>(b.trim().as_bytes()).unwrap(),
                )
            })
            .map(|(min, max)| {
                let invalids = if part1 {
                    generate_invalid_numbers_part1(min, max)
                } else {
                    generate_invalid_numbers_part2(min, max)
                };
                invalids.iter().sum::<usize>()
            })
            .sum::<usize>(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
