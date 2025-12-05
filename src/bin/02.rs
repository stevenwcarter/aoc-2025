advent_of_code::solution!(2);

use std::ops::RangeInclusive;

use atoi_simd::parse;
use hashbrown::HashSet;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn construct_repetitive_update(
    start_seed: usize,
    end_seed: usize,
    repetitions: usize,
    seed_len: u32,
    range: &RangeInclusive<usize>,
    invalid_numbers: &mut HashSet<usize>,
) {
    let multiplier = 10_usize.pow(seed_len);

    for seed in start_seed..=end_seed {
        let mut result = 0;
        for _ in 0..repetitions {
            result = result * multiplier + seed;
        }

        if range.contains(&result) {
            invalid_numbers.insert(result);
        }
    }
}

fn generate_invalid_numbers_part1(min: usize, max: usize) -> usize {
    let mut invalid_numbers = HashSet::new();

    let range: RangeInclusive<_> = min..=max;
    // calculate digit lengths
    let min_len = min.ilog10() + 1;
    let max_len = max.ilog10() + 1;

    for total_len in min_len..=max_len {
        let seed_len = total_len / 2;

        let start_seed = 10_usize.pow(seed_len - 1);
        let end_seed = 10_usize.pow(seed_len) - 1;

        construct_repetitive_update(
            start_seed,
            end_seed,
            2,
            seed_len,
            &range,
            &mut invalid_numbers,
        );
    }

    invalid_numbers.iter().sum()
}
fn generate_invalid_numbers_part2(min: usize, max: usize) -> usize {
    let mut invalid_numbers = HashSet::new();

    let range: RangeInclusive<_> = min..=max;

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

            construct_repetitive_update(
                start_seed,
                end_seed,
                repetitions,
                seed_len,
                &range,
                &mut invalid_numbers,
            );
        }
    }

    invalid_numbers.iter().sum()
}

pub fn parse_to_range((a, b): (&str, &str)) -> (usize, usize) {
    (
        parse(a.trim().as_bytes()).unwrap(),
        parse(b.trim().as_bytes()).unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split(',')
            .par_bridge()
            .map(|r| r.split_once('-').unwrap())
            .map(parse_to_range)
            .map(|(min, max)| generate_invalid_numbers_part1(min, max))
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split(',')
            .par_bridge()
            .map(|r| r.split_once('-').unwrap())
            .map(parse_to_range)
            .map(|(min, max)| generate_invalid_numbers_part2(min, max))
            .sum::<usize>(),
    )
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
