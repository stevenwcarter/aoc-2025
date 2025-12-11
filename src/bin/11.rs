use std::sync::OnceLock;

use cached::proc_macro::cached;
use hashbrown::HashMap;

advent_of_code::solution!(11);

static PATHS: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();

#[cached]
fn walk_paths(current: String) -> u64 {
    let paths = PATHS.get().unwrap();
    if current == "out" {
        return 1;
    }

    let mut total = 0;
    for next in paths.get(&current).unwrap_or(&vec![]) {
        total += walk_paths(next.to_string());
    }
    total
}

#[cached]
fn walk_paths_part2(current: String, dac_seen: bool, fft_seen: bool) -> u64 {
    let paths = PATHS.get().unwrap();
    if current == "out" {
        if dac_seen && fft_seen {
            return 1;
        } else {
            return 0;
        }
    }

    let mut total = 0;
    for next in paths.get(current.as_str()).unwrap_or(&vec![]) {
        if !dac_seen && next == "dac" {
            total += walk_paths_part2(next.to_string(), true, fft_seen);
            continue;
        }
        if !fft_seen && next == "fft" {
            total += walk_paths_part2(next.to_string(), dac_seen, true);
            continue;
        }
        total += walk_paths_part2(next.to_string(), dac_seen, fft_seen);
    }

    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let (input, outputs) = line.split_once(":").expect("Invalid input");
        let outputs: Vec<String> = outputs.split_whitespace().map(|a| a.to_owned()).collect();

        paths.insert(input.to_owned(), outputs);
    });
    PATHS.get_or_init(|| paths.clone());

    Some(walk_paths("you".to_string()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let (input, outputs) = line.split_once(":").expect("Invalid input");
        let outputs: Vec<String> = outputs.split_whitespace().map(|a| a.to_owned()).collect();

        paths.insert(input.to_owned(), outputs);
    });
    PATHS.get_or_init(|| paths.clone());

    Some(walk_paths_part2("svr".to_string(), false, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let example = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(example);
        assert_eq!(result, Some(2));
    }
}
