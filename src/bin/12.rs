use atoi_simd::parse_pos;
use memchr::memchr;

advent_of_code::solution!(12, 1);

fn check_presents_fit(input: &str) -> bool {
    let colon_pos = memchr(b':', input.as_bytes()).unwrap();
    let (dims, present_counts) = input.split_at(colon_pos);

    let x_pos = memchr(b'x', dims.as_bytes()).unwrap();
    let width: u32 = parse_pos(&dims.as_bytes()[0..x_pos]).unwrap();
    let height: u32 = parse_pos(&dims.as_bytes()[x_pos + 1..]).unwrap();

    let w = (width as f32 / 3.).ceil() as u32;
    let h = (height as f32 / 3.).ceil() as u32;
    let total_present_count = present_counts[1..]
        .trim()
        .split(' ')
        .map(|count_str| parse_pos::<u32>(count_str.as_bytes()).unwrap())
        .sum::<u32>();
    w * h >= total_present_count
}

pub fn part_one(input: &str) -> Option<usize> {
    // find first x, which is first in the dimensions for the blocks (don't need to parse
    // presents for this puzzle)
    let first_x = memchr(b'x', input.as_bytes()).unwrap();
    let start = input[..first_x].rfind('\n').unwrap() + 1;

    let (_, areas) = input.split_at(start);

    let solution: usize = areas
        .lines()
        .filter(|area| check_presents_fit(area))
        .count();

    Some(solution)
}
