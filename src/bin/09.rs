use std::{
    cmp::{max, min},
    collections::VecDeque,
};

use advent_of_code::{Coord, Rectangle};
use atoi_simd::parse_pos;
use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(9);

fn find_area(pair: &[&Coord]) -> usize {
    let min_x = pair.iter().map(|c| c.x()).min().unwrap();
    let max_x = pair.iter().map(|c| c.x()).max().unwrap();
    let min_y = pair.iter().map(|c| c.y()).min().unwrap();
    let max_y = pair.iter().map(|c| c.y()).max().unwrap();

    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize
}

pub fn part_one(input: &str) -> Option<usize> {
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            Coord::from((
                parse_pos::<usize>(a.trim().as_bytes()).unwrap() as i32,
                parse_pos::<usize>(b.trim().as_bytes()).unwrap() as i32,
            ))
        })
        .collect();

    coords
        .iter()
        .combinations(2)
        .map(|pair| find_area(&pair[..2]))
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut answer = 0;
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            Coord::from((
                parse_pos::<usize>(a.trim().as_bytes()).unwrap() as i32,
                parse_pos::<usize>(b.trim().as_bytes()).unwrap() as i32,
            ))
        })
        .collect();
    let edges = coords
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| Rectangle::new(*a, *b))
        .collect_vec();

    for (i, t1) in coords.iter().enumerate() {
        for t2 in coords[i + 1..].iter() {
            let area = ((t1.x() - t2.x()).abs() + 1) * ((t1.y() - t2.y()).abs() + 1);
            let rect = Rectangle::new(*t1, *t2);
            let inner_rect = Rectangle::new(
                Coord::from((rect.top_left.x() + 1, rect.top_left.y() + 1)),
                Coord::from((rect.bottom_right.x() - 1, rect.bottom_right.y() - 1)),
            );
            if edges.iter().all(|e| inner_rect.intersection(e).is_none()) {
                answer = answer.max(area);
            }
        }
    }

    Some(answer as usize)
}

fn display_grid(grid: &HashSet<(usize, usize)>) {
    let min_x = grid.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = grid.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = grid.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = grid.iter().map(|(_, y)| *y).max().unwrap();

    let mut s = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&(x, y)) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
