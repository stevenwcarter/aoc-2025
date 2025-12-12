#![allow(dead_code)]
use std::str::FromStr;

use memchr::memchr;

advent_of_code::solution!(12, 1);

#[derive(Debug)]
struct Area {
    width: u8,
    height: u8,
    presents_needed: [u8; 6],
}

// 4x4: 0 0 0 0 2 0
// 12x5: 1 0 1 0 2 2
// 12x5: 1 0 1 0 3 2
impl FromStr for Area {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let colon_pos = memchr(b':', input.as_bytes()).unwrap();
        let (dims, present_counts) = input.split_at(colon_pos);

        let x_pos = memchr(b'x', dims.as_bytes()).unwrap();
        let width = dims[0..x_pos].trim().parse::<u8>().unwrap();
        let height = dims[x_pos + 1..].trim().parse::<u8>().unwrap();

        let mut presents_needed = [0u8; 6];
        for (i, count_str) in present_counts[1..].trim().split(' ').enumerate() {
            presents_needed[i] = count_str.parse::<u8>().unwrap();
        }

        Ok(Self {
            width,
            height,
            presents_needed,
        })
    }
}

#[derive(Debug)]
struct Grid {
    areas: Vec<Area>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // find first x, which is first in the dimensions for the blocks (don't need to parse
        // presents for this puzzle)
        let first_x = memchr(b'x', input.as_bytes()).unwrap();
        let start = input[..first_x].rfind('\n').unwrap() + 1;

        let (_, areas) = input.split_at(start);

        let areas: Vec<Area> = areas
            .lines()
            .map(|area| Area::from_str(area).unwrap())
            .collect();

        Ok(Self { areas })
    }
}

fn can_presents_fit_area(area: &Area) -> bool {
    let total_piece_count: u32 = area.presents_needed.iter().map(|&p| p as u32).sum();
    let w = (area.width as f32 / 3.).ceil() as u32;
    let h = (area.height as f32 / 3.).ceil() as u32;
    w * h >= total_piece_count
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid = Grid::from_str(input).unwrap();

    Some(grid.areas.into_iter().filter(can_presents_fit_area).count())
}
