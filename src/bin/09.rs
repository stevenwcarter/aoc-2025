use advent_of_code::{Coord, Maxer, Rectangle};
use atoi_simd::parse_pos;
use hashbrown::HashSet;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(9);

fn find_area(pair: &[&Coord]) -> usize {
    let min_x = pair.iter().map(|c| c.x()).min().unwrap();
    let max_x = pair.iter().map(|c| c.x()).max().unwrap();
    let min_y = pair.iter().map(|c| c.y()).min().unwrap();
    let max_y = pair.iter().map(|c| c.y()).max().unwrap();

    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize
}

fn parse_coords(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Coord::from((
                parse_pos::<usize>(x.trim().as_bytes()).unwrap() as i32,
                parse_pos::<usize>(y.trim().as_bytes()).unwrap() as i32,
            ))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let coords = parse_coords(input);

    coords
        .iter()
        .combinations(2)
        .map(|pair| Rectangle::new(*pair[0], *pair[1]).area_inclusive())
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let maxer = Maxer::default();
    let coords = parse_coords(input);
    let edges = coords
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| Rectangle::new(*a, *b))
        .collect_vec();

    coords.par_iter().enumerate().for_each(|(i, t1)| {
        let mut max = maxer.clone();
        coords[i + 1..].iter().for_each(|t2| {
            let rect = Rectangle::new(*t1, *t2);
            let inner_rect = rect.inset(1);

            if edges.iter().all(|e| inner_rect.intersection(e).is_none()) {
                max.max(rect.area_inclusive());
            }
        });
    });

    Some(maxer.get())
}

#[allow(unused)]
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
    #[test]
    fn test_find_area() {
        let coords = [Coord::from((1, 1)), Coord::from((4, 5))];
        let area = find_area(&coords.iter().collect::<Vec<&Coord>>());
        assert_eq!(area, 20);
        let rect = Rectangle::new(coords[0], coords[1]);
        assert_eq!(rect.area_inclusive(), 20);
    }
}
