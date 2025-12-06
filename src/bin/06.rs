use std::ops::Range;

advent_of_code::solution!(6);

fn mult_reducer(acc: usize, num: usize) -> usize {
    if acc == 0 { num } else { acc * num }
}

fn add_reducer(acc: usize, num: usize) -> usize {
    acc + num
}

fn reducer(operand: char) -> impl Fn(usize, usize) -> usize {
    match operand {
        '*' => mult_reducer,
        '+' => add_reducer,
        _ => unreachable!("Unknown symbol {}", operand),
    }
}

fn calculate_part1(chars: &[Vec<char>], range: Range<usize>) -> usize {
    let operand = chars.last().unwrap()[range.start];

    let reducer_fn = reducer(operand);

    // for part 1, sum numbers in each row
    chars[0..chars.len() - 1]
        .iter()
        .map(|c| {
            c[range.clone()]
                .iter()
                .filter_map(|&c| match c {
                    '0'..='9' => Some((c as u8) - b'0'),
                    _ => None,
                })
                .fold(0usize, |acc, digit| acc * 10 + digit as usize)
        })
        .fold(0usize, reducer_fn)
}

fn calculate_part2(chars: &[Vec<char>], range: Range<usize>) -> usize {
    let operand = chars.last().unwrap()[range.start];

    let reducer_fn = reducer(operand);

    // for part 2, sum numbers in each column
    range
        .rev()
        .map(|i| {
            chars[0..chars.len() - 1]
                .iter()
                .filter_map(|line| match line[i] {
                    '0'..='9' => Some(line[i] as u8 - b'0'),
                    _ => None,
                })
                .fold(0usize, |acc, d| acc * 10 + d as usize)
        })
        .fold(0usize, reducer_fn)
}

// lots of allocation, but it runs fast and is easy to reason about
fn parse_chars_and_operands(input: &str) -> (Vec<Vec<char>>, Vec<Range<usize>>) {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let operands = chars.last().unwrap();

    let mut operand_ranges: Vec<Range<usize>> = Vec::new();

    let mut start = 0;
    for (i, c) in operands[1..].iter().enumerate() {
        match c {
            '+' | '*' => {
                operand_ranges.push(start..i);
                start = i + 1;
            }
            _ => {}
        }
    }
    operand_ranges.push(start..operands.len());

    (chars, operand_ranges)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (chars, operand_ranges) = parse_chars_and_operands(input);

    Some(
        operand_ranges
            .iter()
            .map(|r| calculate_part1(&chars, r.clone()))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (chars, operand_ranges) = parse_chars_and_operands(input);

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
