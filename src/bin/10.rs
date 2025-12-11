#![allow(dead_code, unused)]
use std::collections::VecDeque;

// Part 2 is adapted from michel-kramer's solution. I learned a lot from this one!
// https://raw.githubusercontent.com/michel-kraemer/adventofcode-rust/refs/heads/main/2025/day10/src/main.rs

use cached::proc_macro::cached;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(10);

// [indicators] (button1) (button2) (...) (buttonN) {joltage1,joltage2,...,joltageM}
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

struct Machine {
    indicator_lights: u16,
    buttons: Vec<u16>,
    joltage: Vec<usize>,
}

// receives [.#..#] and returns bitmask u16 representation
fn parse_indicators(s: &str) -> u16 {
    let s = s.as_bytes();
    let mut indicators: u16 = 0;

    s[1..s.len() - 1].iter().enumerate().for_each(|(i, &b)| {
        if b == b'#' {
            indicators |= 1 << i;
        }
    });

    indicators
}

fn parse_buttons(s: &str) -> Vec<u16> {
    s.split(' ')
        .map(|btn| {
            let btn = btn.trim_matches(&['(', ')'][..]);
            let mut bitmask: u16 = 0;
            btn.split(',')
                .map(|num_str| num_str.parse::<u8>().unwrap())
                .for_each(|num| {
                    bitmask |= 1 << num;
                });
            bitmask
        })
        .collect()
}

fn parse_joltage(s: &str) -> Vec<usize> {
    let s = s.trim_matches(&[' ', '{', '}'][..]);
    s.split(',')
        .map(|num_str| num_str.trim().parse::<usize>().unwrap())
        .collect()
}

impl Machine {
    fn new(line: &str) -> Self {
        let (indicators_str, rest) = line.split_once(' ').unwrap();
        let last_space_idx = rest.rfind(' ').unwrap();
        let (buttons_str, joltage_str) = rest.split_at(last_space_idx);

        let indicator_lights: u16 = parse_indicators(indicators_str);
        let buttons: Vec<u16> = parse_buttons(buttons_str);
        let joltage: Vec<usize> = parse_joltage(joltage_str);

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
            let mut state = 0u16;
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

    fn matching_buttons_iter(
        &self,
        mini: usize,
        available_buttons_mask: u32,
    ) -> impl Iterator<Item = (usize, &u16)> + '_ {
        self.buttons.iter().enumerate().filter(move |&(i, _)| {
            (available_buttons_mask & (1 << i)) > 0 && (self.buttons[i] & (1 << mini)) > 0
        })
    }

    fn count_matching_buttons(&self, mini: usize, available_buttons_mask: u32) -> usize {
        self.matching_buttons_iter(mini, available_buttons_mask)
            .count()
    }

    fn matching_buttons(&self, mini: usize, available_buttons_mask: u32) -> Vec<(usize, u16)> {
        self.matching_buttons_iter(mini, available_buttons_mask)
            .map(|(i, &mask)| (i, mask))
            .collect()
    }

    /// Optimized DFS for part 2 with strategic pruning
    fn dfs_part2(&self, joltage: &[usize], available_buttons_mask: u32) -> usize {
        if joltage.iter().all(|&j| j == 0) {
            return 0;
        }

        // Key optimization: Find joltage value with fewest affecting buttons
        // This allows maximum pruning of search space
        let (mini, &min) = joltage
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v > 0)
            .min_by_key(|&(i, &v)| {
                let button_count = self.count_matching_buttons(i, available_buttons_mask);
                (button_count, -(v as isize)) // Prefer fewer buttons, then higher values
            })
            .unwrap();

        // Get buttons that affect the selected joltage counter
        let matching_buttons = self.matching_buttons(mini, available_buttons_mask);

        let mut result = usize::MAX;

        if !matching_buttons.is_empty() {
            // Create new mask excluding buttons that affect this joltage counter
            let mut new_mask = available_buttons_mask;
            for (i, _) in &matching_buttons {
                new_mask &= !(1 << i);
            }

            // Try all combinations of button press counts that sum to min
            let mut new_joltage = joltage.to_vec();
            let mut counts = vec![0; matching_buttons.len()];
            counts[matching_buttons.len() - 1] = min;

            loop {
                // Apply button presses and check validity
                let mut good = true;
                new_joltage.copy_from_slice(joltage);

                'buttons: for (bi, &cnt) in counts.iter().enumerate() {
                    if cnt == 0 {
                        continue;
                    }

                    // Apply this button cnt times
                    let button_mask = matching_buttons[bi].1;
                    for (j, joltage) in new_joltage.iter_mut().enumerate() {
                        if (button_mask & (1 << j)) > 0 {
                            if *joltage >= cnt {
                                *joltage -= cnt;
                            } else {
                                good = false;
                                break 'buttons;
                            }
                        }
                    }
                }

                if good {
                    let r = self.dfs_part2(&new_joltage, new_mask);
                    if r != usize::MAX {
                        result = result.min(min + r);
                    }
                }

                // Generate next combination
                if !next_combination(&mut counts) {
                    break;
                }
            }
        }

        result
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
            .map(|machine| {
                let available_mask = (1 << machine.buttons.len()) - 1;
                machine.dfs_part2(&machine.joltage, available_mask) as u32
            })
            .sum(),
    )
}

/// Generate next combination of integers that sum to a fixed value
/// Returns false when no more combinations exist
fn next_combination(combinations: &mut [usize]) -> bool {
    let i = combinations.iter().rposition(|&v| v != 0).unwrap();
    if i == 0 {
        return false;
    }
    let v = combinations[i];
    combinations[i - 1] += 1;
    combinations[i] = 0;
    combinations[combinations.len() - 1] = v - 1;
    true
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
