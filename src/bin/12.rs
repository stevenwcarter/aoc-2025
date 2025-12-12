use memchr::memchr;

advent_of_code::solution!(12, 1);

#[inline(always)]
fn parse_2d(input: &[u8], i: usize) -> usize {
    ((input[i] - b'0') as usize) * 10 + ((input[i + 1] - b'0') as usize)
}

/// Parses the present fitting input and checks if all presents fit in the given area.
/// The input is exactly the same format, so we can use direct indexes and no searches here
/// Sample line for reference: `39x43: 23 41 27 30 29 31`
#[inline(always)]
fn check_presents_fit(input: &[u8]) -> usize {
    let width: usize = (input[0] - b'0') as usize * 10 + (input[1] - b'0') as usize;
    let height: usize = (input[3] - b'0') as usize * 10 + (input[4] - b'0') as usize;

    let w = width.div_ceil(3);
    let h = height.div_ceil(3);

    let total_present_count = parse_2d(input, 7)
        + parse_2d(input, 10)
        + parse_2d(input, 13)
        + parse_2d(input, 16)
        + parse_2d(input, 19)
        + parse_2d(input, 22);

    (w * h >= total_present_count) as usize
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
