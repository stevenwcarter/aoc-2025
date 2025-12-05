use atoi_simd::parse;

advent_of_code::solution!(1);

const DIAL_START: i64 = 50;
const SPOT_COUNT: i64 = 100;

pub struct Dial {
    dial: i64,
    part1_zeroes: u64,
    part2_zeroes: u64,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            dial: DIAL_START,
            part1_zeroes: 0,
            part2_zeroes: 0,
        }
    }
}

impl Dial {
    #[inline(always)]
    fn fast_mod_100(n: i64) -> i64 {
        let mut result = n % SPOT_COUNT;
        if result < 0 {
            result += SPOT_COUNT;
        }
        result
    }

    /// Receives the amount to shift, already adjusted for direction (negative for left, positive
    /// for right).
    pub fn handle_instruction_part1(&mut self, amount: i64) {
        self.dial = Self::fast_mod_100(self.dial + amount);

        // Branchless part 1 increment
        self.part1_zeroes += (self.dial == 0) as u64;
    }

    pub fn handle_instruction_part2(&mut self, amount: i64) {
        let steps = amount.abs();

        // branchless distance calculation
        let is_positive = (amount > 0) as i64;
        let mut dist_to_first =
            is_positive * (SPOT_COUNT - self.dial) + (1 - is_positive) * self.dial;
        dist_to_first = Self::fast_mod_100(dist_to_first);

        // If already on zero, have to go a full rotation to "touch" it again
        dist_to_first += SPOT_COUNT * (dist_to_first == 0) as i64;

        // Branchless part 2 calculation
        let crosses_zero = (steps >= dist_to_first) as u64;
        let additional_crosses = ((steps.saturating_sub(dist_to_first)) / SPOT_COUNT) as u64;
        self.part2_zeroes += crosses_zero * (1 + additional_crosses);

        // shift the actual dial by the amount
        self.dial = Self::fast_mod_100(self.dial + amount);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::default();
    input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(direction, amount)| (direction, parse::<i64>(amount.as_bytes()).unwrap()))
        .map(|(direction, amount)| if direction == "L" { -amount } else { amount })
        .for_each(|amount| dial.handle_instruction_part1(amount));

    Some(dial.part1_zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::default();
    input
        .lines()
        .map(|l| l.split_at(1)) // R 51 or L 21
        .map(|(direction, amount)| (direction, parse::<i64>(amount.as_bytes()).unwrap()))
        .map(|(direction, amount)| if direction == "L" { -amount } else { amount })
        .for_each(|l| dial.handle_instruction_part2(l));

    Some(dial.part2_zeroes)
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
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_part_two_2() {
        assert_eq!(part_two("R1000"), Some(10));
    }
    #[test]
    fn test_part_two_3() {
        assert_eq!(part_two("R1050"), Some(11));
    }
    #[test]
    fn test_edge_cases() {
        let input = "L51
R1
R100
L100
L200
R1000";
        assert_eq!(part_two(input), Some(16));
        let input = "L51
R101
";
        assert_eq!(part_two(input), Some(3));
    }
}
