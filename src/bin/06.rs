use std::ops::Range;

advent_of_code::solution!(6);

fn mult_reducer(acc: usize, num: usize) -> usize {
    acc * num
}

fn add_reducer(acc: usize, num: usize) -> usize {
    acc + num
}

fn reducer(operand: u8) -> (usize, impl Fn(usize, usize) -> usize) {
    match operand {
        // accumulator needs to start as 1 for multiplication case
        b'*' => (1, mult_reducer as fn(usize, usize) -> usize),
        b'+' => (0, add_reducer as fn(usize, usize) -> usize),
        _ => unreachable!("Unknown symbol {}", operand),
    }
}

fn calculate_part1(chars: &[&[u8]], range: Range<usize>) -> usize {
    let operand = chars.last().unwrap()[range.start];

    let (acc, reducer_fn) = reducer(operand);

    // for part 1, sum numbers in each row
    chars[0..chars.len() - 1]
        .iter()
        .map(|c| {
            c[range.clone()]
                .iter()
                .filter_map(|&c| match c {
                    b'0'..=b'9' => Some(c - b'0'),
                    _ => None,
                })
                .fold(0usize, |acc, digit| acc * 10 + digit as usize)
        })
        .fold(acc, reducer_fn)
}

fn calculate_part2(chars: &[&[u8]], range: Range<usize>) -> usize {
    let operand = chars.last().unwrap()[range.start];

    let (acc, reducer_fn) = reducer(operand);

    // for part 2, sum numbers in each column
    range
        .rev()
        .map(|i| {
            chars[0..chars.len() - 1]
                .iter()
                .filter_map(|line| match line[i] {
                    b'0'..=b'9' => Some(line[i] - b'0'),
                    _ => None,
                })
                .fold(0usize, |acc, d| acc * 10 + d as usize)
        })
        .fold(acc, reducer_fn)
}

// lots of allocation, but it runs fast and is easy to reason about
fn parse_operand_ranges(operands: &[u8]) -> Vec<Range<usize>> {
    let mut operand_ranges: Vec<Range<usize>> = Vec::new();

    let mut start = 0;
    for (i, c) in operands[1..].iter().enumerate() {
        match c {
            b'+' | b'*' => {
                operand_ranges.push(start..i);
                start = i + 1;
            }
            _ => {}
        }
    }
    // push the last range
    operand_ranges.push(start..operands.len());

    operand_ranges
}

pub fn part_one(input: &str) -> Option<usize> {
    let chars: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let operand_ranges = parse_operand_ranges(chars.last().unwrap());

    Some(
        operand_ranges
            .iter()
            .map(|r| calculate_part1(&chars, r.clone()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let chars: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let operand_ranges = parse_operand_ranges(chars.last().unwrap());

    Some(
        operand_ranges
            .iter()
            .map(|r| calculate_part2(&chars, r.clone()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
