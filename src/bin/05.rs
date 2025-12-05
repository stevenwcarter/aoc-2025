use advent_of_code::condense_ranges;
use atoi_simd::parse;
use itertools::Itertools;

advent_of_code::solution!(5);

fn find_fresh_totals(ranges: &[(usize, usize)], ingredients: &[usize]) -> Option<u64> {
    let ranges = condense_ranges(ranges);
    let mut fresh_total = 0u64;

    'ingredient_loop: for &ingredient in ingredients {
        for (start, end) in &ranges {
            if ingredient >= *start && ingredient <= *end {
                fresh_total += 1;
                continue 'ingredient_loop;
            }
        }
    }

    Some(fresh_total)
}

/// Find total count of fresh ingredients in all ranges, just using simple math after condensing
/// the ranges to avoid double counting
fn find_total_fresh_count(ranges: &[(usize, usize)]) -> Option<u64> {
    let ranges = condense_ranges(ranges);
    let mut total_fresh = 0u64;

    for (start, end) in ranges {
        total_fresh += (end - start + 1) as u64;
    }

    Some(total_fresh)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = input.split("\n\n").collect_tuple().unwrap();
    let ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|range| range.split('-'))
        .map(|mut range| {
            (
                parse::<usize>(range.next().unwrap().trim().as_bytes()).unwrap(),
                parse::<usize>(range.next().unwrap().trim().as_bytes()).unwrap(),
            )
        })
        .collect();

    let ingredients: Vec<usize> = ingredients
        .lines()
        .map(|ingredient| parse(ingredient.as_bytes()).unwrap())
        .collect();

    find_fresh_totals(&ranges, &ingredients)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = input.split("\n\n").collect_tuple().unwrap();
    let ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|range| range.split('-'))
        .map(|mut range| {
            (
                parse::<usize>(range.next().unwrap().trim().as_bytes()).unwrap(),
                parse::<usize>(range.next().unwrap().trim().as_bytes()).unwrap(),
            )
        })
        .collect();

    find_total_fresh_count(&ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
