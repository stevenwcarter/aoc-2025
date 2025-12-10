advent_of_code::solution!(1);

const DIAL_START: i16 = 50;
const SPOT_COUNT: i16 = 100;

pub struct Dial {
    dial: i16,
    part1_zeroes: u16,
    part2_zeroes: u16,
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

#[inline(always)]
fn fast_mod_100(n: i16) -> i16 {
    // I started small with i16, converted to i32 and added 40000 which is beyond what i16 can hold.
    // This guarantees a positive result for the modulo operation, which is further hinted to the
    // compiler by converting to u32 before applying the modulo.
    ((n as i32 + 40000) as u32 % 100) as i16
}
impl Dial {
    /// Receives the amount to shift, already adjusted for direction (negative for left, positive
    /// for right).
    #[inline]
    pub fn handle_instruction_part1(&mut self, amount: i16) {
        self.dial = fast_mod_100(self.dial + amount);

        // Branchless part 1 increment
        self.part1_zeroes += (self.dial == 0) as u16;
    }

    pub fn handle_instruction_part2(&mut self, amount: i16) {
        let steps = amount.abs();

        // branchless distance calculation
        let is_positive = (amount > 0) as i16;
        let mut dist_to_first =
            is_positive * (SPOT_COUNT - self.dial) + (1 - is_positive) * self.dial;
        dist_to_first = fast_mod_100(dist_to_first);

        // If already on zero, have to go a full rotation to "touch" it again
        dist_to_first += SPOT_COUNT * (dist_to_first == 0) as i16;

        // Branchless part 2 calculation
        let crosses_zero = (steps >= dist_to_first) as u16;
        let additional_crosses = ((steps - dist_to_first) / SPOT_COUNT) as u16;
        self.part2_zeroes += crosses_zero * (1 + additional_crosses);

        // shift the actual dial by the amount
        self.dial = fast_mod_100(self.dial + amount);
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut dial = Dial::default();

    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut amount: i16 = 0;
        for &b in &bytes[1..] {
            amount = amount * 10 + (b - b'0') as i16;
        }

        if bytes[0] == b'L' {
            amount = -amount;
        }

        dial.handle_instruction_part1(amount);
    }

    Some(dial.part1_zeroes)
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut dial = Dial::default();

    for line in input.lines() {
        let bytes = line.as_bytes();

        let mut amount: i16 = 0;
        for &b in &bytes[1..] {
            amount = amount * 10 + (b - b'0') as i16;
        }

        if bytes[0] == b'L' {
            amount = -amount;
        }

        dial.handle_instruction_part2(amount);
    }

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
