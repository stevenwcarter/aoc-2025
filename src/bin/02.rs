advent_of_code::solution!(2);

use std::ops::RangeInclusive;

use atoi_simd::parse;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn extract_digits(value: usize) -> Vec<u8> {
    let mut value = value;

    let count: usize = (value.ilog10() + 1) as usize;

    (0..count).rev().fold(vec![0; count], |mut container, i| {
        container[i] = (value % 10) as u8;
        value /= 10;
        container
    })
}

fn is_even_length(value: usize) -> bool {
    let len: usize = (value.ilog10() + 1) as usize;
    len.is_multiple_of(2)
}

fn is_valid_id(number: usize) -> bool {
    let len: usize = (number.ilog10() + 1) as usize;
    let half = len / 2;

    number / 10usize.pow(half as u32) != number % (10usize.pow(half as u32))
}

fn is_valid_id_part_2(number: usize) -> bool {
    let digits = extract_digits(number);
    let len = digits.len();
    let half = len / 2 + 1;

    'outer_loop: for repeat_size in 1..half {
        if !len.is_multiple_of(repeat_size) {
            continue;
        }
        let mut i = repeat_size;
        let check = &digits[0..repeat_size];
        while i <= len - repeat_size {
            if &digits[i..repeat_size + i] != check {
                continue 'outer_loop;
            }
            i += repeat_size;
        }
        return false;
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let ranges: Vec<&str> = input.split(',').collect();
    let ranges: Vec<RangeInclusive<usize>> = ranges
        .iter()
        .map(|r| r.split_once('-').unwrap())
        .map(|(a, b)| {
            (
                parse::<usize>(a.trim().as_bytes()).unwrap(),
                parse::<usize>(b.trim().as_bytes()).unwrap(),
            )
        })
        .map(|(a, b)| a..=b)
        .collect();
    Some(
        ranges
            .par_iter()
            .map(|r| {
                r.clone()
                    .filter(|n| is_even_length(*n))
                    .filter(|&n| !is_valid_id(n))
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let ranges: Vec<&str> = input.split(',').collect();
    let ranges: Vec<RangeInclusive<usize>> = ranges
        .iter()
        .map(|r| r.split_once('-').unwrap())
        .map(|(a, b)| {
            (
                parse::<usize>(a.trim().as_bytes()).unwrap(),
                parse::<usize>(b.trim().as_bytes()).unwrap(),
            )
        })
        .map(|(a, b)| a..=b)
        .collect();
    Some(
        ranges
            .par_iter()
            .map(|r| r.clone().filter(|&n| !is_valid_id_part_2(n)).sum::<usize>())
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
    #[test]
    fn test_is_valid_id() {
        assert!(is_valid_id(123125));
        assert!(!is_valid_id(123123));
    }
}
