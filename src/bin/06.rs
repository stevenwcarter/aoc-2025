use atoi_simd::parse;
use hashbrown::HashMap;

advent_of_code::solution!(6);

fn solve_problem(problem: &[&str]) -> usize {
    let symbol_str = problem.last().unwrap();
    let math_fn = match *symbol_str {
        "*" => |mut acc, n| {
            acc = if acc == 0 { n } else { acc * n };
            acc
        },
        "+" => |acc, n| acc + n,
        _ => unreachable!("Unknown symbol {}", symbol_str),
    };

    problem[0..problem.len() - 1]
        .iter()
        .fold(0usize, |acc, problem| {
            math_fn(acc, parse(problem.as_bytes()).unwrap())
        })
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut problems: HashMap<usize, Vec<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        l.split_whitespace().enumerate().for_each(|(i, col)| {
            problems.entry(i).or_default().push(col);
        });
    });

    Some(problems.values().map(|p| solve_problem(p)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let operands = chars.last().unwrap();
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
