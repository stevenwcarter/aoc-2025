advent_of_code::solution!(3);

fn maximize_battery_from_line(line: &[u8], length: usize) -> usize {
    let mut index_tracker = 0;
    (0..length)
        .map(|i| {
            // only consider up to len - length + i + 1 to ensure enough digits remain
            let best_index = (index_tracker..line.len() - length + i + 1)
                .rev()
                // find the index of the max digit, preferring earlier indices on ties (hence the
                // rev)
                .max_by_key(|&index| line[index])
                .expect("not enough digits remaining");
            index_tracker = best_index + 1;
            line[best_index]
        })
        // convert digits to a single number
        .fold(0usize, |acc, d| acc * 10 + d as usize)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
            .map(|line| maximize_battery_from_line(&line, 2))
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
            .map(|line| maximize_battery_from_line(&line, 12))
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
