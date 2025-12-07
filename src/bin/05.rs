use advent_of_code::condense_ranges;
use atoi_simd::parse;

advent_of_code::solution!(5);

fn find_fresh_totals(
    ranges: &[(usize, usize)],
    ingredients_iter: impl Iterator<Item = usize>,
) -> Option<u64> {
    // Ensure these are sorted by start position and disjoint (non-overlapping)
    let ranges = condense_ranges(ranges);

    let fresh_total = ingredients_iter
        .filter(|&ingredient| {
            let idx = ranges.partition_point(|&(start, _)| start <= ingredient);

            // If idx is 0, the ingredient is smaller than the first range's start.
            if idx == 0 {
                return false;
            }

            // Otherwise, check the range immediately before the partition point.
            let (_range_start, range_end) = ranges[idx - 1];
            range_end >= ingredient
        })
        .count();

    Some(fresh_total as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let split_index = memchr::memmem::find(input.as_bytes(), b"\n\n")?;
    let (ranges, ingredients) = input.split_at(split_index);
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

    let ingredients_iter = ingredients[2..] // Skip the leading double newline
        .lines()
        .map(|ingredient| parse(ingredient.as_bytes()).unwrap());

    find_fresh_totals(&ranges, ingredients_iter)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Find the split index with simd for most efficient parsing
    let split_index = memchr::memmem::find(input.as_bytes(), b"\n\n")?;
    let ranges: Vec<(usize, usize)> = input[..split_index]
        .lines()
        .map(|range| range.split('-'))
        .map(|mut range| {
            (
                parse::<usize>(range.next().unwrap().trim().as_bytes()).unwrap(),
                parse::<usize>(range.next().unwrap().trim().as_bytes()).unwrap(),
            )
        })
        .collect();

    // Find total count of fresh ingredients in all ranges, just using simple math after condensing
    // the ranges to avoid double counting
    Some(
        condense_ranges(&ranges)
            .iter()
            .fold(0u64, |acc, &(start, end)| acc + (end - start + 1) as u64),
    )
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
