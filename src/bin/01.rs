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
    /// Receives the amount to shift, already adjusted for direction (negative for left, positive
    /// for right). Populates the zero counters for part 1 and part 2.
    pub fn handle_instruction(&mut self, amount: i64) {
        let steps = amount.abs();

        // Find how far before zero is "touched" the first time
        let mut dist_to_first = if amount > 0 {
            (SPOT_COUNT - self.dial).rem_euclid(SPOT_COUNT)
        } else {
            self.dial.rem_euclid(SPOT_COUNT)
        };

        // If already on zero, have to go a full rotation to "touch" it again
        if dist_to_first == 0 {
            dist_to_first = SPOT_COUNT;
        }

        self.part2_zeroes += if steps < dist_to_first {
            0 // didn't move enough to touch zero
        } else {
            // crossed zero, plus to count how many more times that happened
            1 + ((steps - dist_to_first) / SPOT_COUNT) as u64
        };

        // shift the actual dial by the amount
        self.dial = (self.dial + amount).rem_euclid(SPOT_COUNT);

        if self.dial == 0 {
            self.part1_zeroes += 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::default();
    input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(direction, amount)| (direction, parse::<i64>(amount.as_bytes()).unwrap()))
        .map(|(direction, amount)| if direction == "L" { -amount } else { amount })
        .for_each(|amount| dial.handle_instruction(amount));

    Some(dial.part1_zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::default();
    input
        .lines()
        .map(|l| l.split_at(1)) // R 51 or L 21
        .map(|(direction, amount)| (direction, parse::<i64>(amount.as_bytes()).unwrap()))
        .map(|(direction, amount)| if direction == "L" { -amount } else { amount })
        .for_each(|l| dial.handle_instruction(l));

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
