#![allow(dead_code, unused)]

use minilp::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(10);

// [indicators] (button1) (button2) (...) (buttonN) {joltage1,joltage2,...,joltageM}
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

struct Machine {
    indicator_lights: u32,
    buttons: Vec<u32>,
    joltage: Vec<i64>,
}

// receives [.#..#] and returns bitmask u32 representation
fn parse_indicators(s: &str) -> u32 {
    let s = s.as_bytes();
    let mut indicators: u32 = 0;

    s[1..s.len() - 1].iter().enumerate().for_each(|(i, &b)| {
        if b == b'#' {
            indicators |= 1 << i;
        }
    });

    indicators
}

fn parse_buttons(s: &str) -> Vec<u32> {
    s.split(' ')
        .map(|btn| {
            let btn = btn.trim_matches(&['(', ')'][..]);
            let mut bitmask: u32 = 0;
            btn.split(',')
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .for_each(|num| {
                    bitmask |= 1 << num;
                });
            bitmask
        })
        .collect()
}

fn parse_joltage(s: &str) -> Vec<i64> {
    let s = s.trim_matches(&[' ', '{', '}'][..]);
    s.split(',')
        .map(|num_str| num_str.trim().parse::<i64>().unwrap())
        .collect()
}

impl Machine {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let goal_str = &parts[0][1..parts[0].len() - 1];
        let mut indicator_lights = 0;
        for (i, c) in goal_str.chars().enumerate() {
            if c == '#' {
                indicator_lights |= 1 << i;
            }
        }

        let last_part = parts.last().unwrap();
        let counter_str = &last_part[1..last_part.len() - 1];
        let joltage = counter_str
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut buttons = Vec::new();
        for part in &parts[1..parts.len() - 1] {
            let mut mask = 0;
            let inner = if part.starts_with('(') || part.starts_with('{') {
                &part[1..part.len() - 1]
            } else {
                part
            };

            for num_str in inner.split(',') {
                if let Ok(bit) = num_str.parse::<u32>() {
                    mask |= 1 << bit;
                }
            }
            buttons.push(mask);
        }

        Machine {
            indicator_lights,
            buttons,
            joltage,
        }
    }

    fn find_required_presses_part1(&self) -> Option<usize> {
        let n = self.buttons.len();
        let mut best: Option<usize> = None;

        for mask in 0u32..(1u32 << n) {
            let mut state = 0u32;
            let mut presses = 0;

            for i in 0..n {
                if (mask & (1 << i)) != 0 {
                    state ^= self.buttons[i]; // XOR toggles bits
                    presses += 1;
                }
            }

            if state == self.indicator_lights {
                best = match best {
                    Some(b) => Some(b.min(presses)),
                    None => Some(presses),
                };
            }
        }

        best
    }

    fn solve_lp(&self) -> i32 {
        let num_buttons = self.buttons.len();
        let num_goals = self.joltage.len();

        let mut problem = Problem::new(OptimizationDirection::Minimize);

        let button_vars: Vec<Variable> = (0..num_buttons)
            .map(|_| problem.add_var(1.0, (0.0, f64::INFINITY)))
            .collect();

        // constraint: each joltage counter must equal its target
        for (goal_idx, &target) in self.joltage.iter().enumerate() {
            let mut constraint = LinearExpr::empty();
            for (button_idx, &button_mask) in self.buttons.iter().enumerate() {
                if (button_mask >> goal_idx) & 1 == 1 {
                    constraint.add(button_vars[button_idx], 1.0);
                }
            }
            problem.add_constraint(constraint, ComparisonOp::Eq, target as f64);
        }

        let solution = problem.solve().unwrap();

        button_vars
            .iter()
            .map(|&var| solution[var].round() as i32)
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines: Vec<Machine> = input.lines().map(Machine::new).collect();

    Some(
        machines
            .par_iter()
            .map(|machine| {
                // For each machine, determine the minimum number of button presses required to match the indicator lights
                machine
                    .find_required_presses_part1()
                    .expect("could not find solution for this machine")
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let machines: Vec<Machine> = input.lines().map(Machine::new).collect();

    Some(
        machines
            .par_iter()
            .map(|machine| machine.solve_lp() as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_parse_indicator1() {
        assert_eq!(parse_indicators("[.##.]"), 0b0110);
    }
    #[test]
    fn test_parse_indicator2() {
        assert_eq!(parse_indicators("[...#.]"), 0b01000);
    }
    #[test]
    fn test_parse_indicator3() {
        assert_eq!(parse_indicators("[.###.#]"), 0b101110);
    }
}
