use atoi_simd::parse_pos;
use memchr::memchr;

advent_of_code::solution!(12, 1);

/// Parses the present fitting input and checks if all presents fit in the given area.
/// The input is exactly the same format, so we can use direct indexes and no searches here
/// Sample line for reference: `39x43: 23 41 27 30 29 31`
#[inline]
fn check_presents_fit(input: &[u8]) -> usize {
    let width: u32 = parse_pos(&input[0..2]).unwrap();
    let height: u32 = parse_pos(&input[3..5]).unwrap();

    let w = (width as f32 / 3.).ceil() as u32;
    let h = (height as f32 / 3.).ceil() as u32;

    let mut present_index = 7;
    let mut total_present_count = 0;
    while present_index < 24 {
        total_present_count += (input[present_index] - b'0') as u32 * 10;
        total_present_count += (input[present_index + 1] - b'0') as u32;
        present_index += 3;
    }
    if w * h >= total_present_count { 1 } else { 0 }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.as_bytes();

    // find first x, which is first in the dimensions for the blocks (don't need to parse
    // presents for this puzzle)
    let first_x = memchr(b'x', input).unwrap();

    // First number is 2 digits before the x
    let mut index = first_x - 2;
    let mut solution = 0;

    while index < input.len() {
        // each line is 24 bytes long + newline, which we don't need
        solution += check_presents_fit(&input[index..index + 24]);
        index += 25;
    }

    Some(solution)
}
