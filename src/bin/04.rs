use std::collections::VecDeque;

use advent_of_code::Coord;

advent_of_code::solution!(4);

const MOVEABLE_PAPER_LIMIT: usize = 4; // less than this amount

pub enum TileType {
    Paper,
    Empty,
}

pub struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<TileType>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let tiles: Vec<TileType> = input
            .lines()
            .flat_map(|line| {
                line.trim().as_bytes().iter().map(|&b| match b {
                    b'@' => TileType::Paper,
                    b'.' => TileType::Empty,
                    _ => panic!("Invalid character in input: {}", b as char),
                })
            })
            .collect();

        Grid {
            width,
            height,
            tiles,
        }
    }

    fn get_tile(&self, coord: &Coord) -> Option<&TileType> {
        if (coord.x() as usize) < self.width && (coord.y() as usize) < self.height {
            Some(&self.tiles[coord.y() as usize * self.width + coord.x() as usize])
        } else {
            None
        }
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        let x = (index % self.width) as i32;
        let y = (index / self.width) as i32;
        Coord::new(x, y)
    }

    fn coord_to_index(&self, coord: &Coord) -> usize {
        coord.y() as usize * self.width + coord.x() as usize
    }

    pub fn find_moveable_papers_part1(&self) -> Option<usize> {
        Some(
            self.tiles
                .iter()
                .enumerate()
                .filter(|(_, tile)| matches!(tile, TileType::Paper))
                .map(|(i, _)| self.index_to_coord(i))
                .map(|coord| {
                    let count = coord
                        .neighbors()
                        .iter()
                        .filter(|neighbor| matches!(self.get_tile(neighbor), Some(TileType::Paper)))
                        .count();
                    (coord, count)
                })
                .filter(|&(_, count)| count < MOVEABLE_PAPER_LIMIT)
                .map(|(coord, _)| coord)
                .count(),
        )
    }

    pub fn find_moveable_papers_part2(&mut self) -> Option<usize> {
        let mut q = VecDeque::new();
        let mut removed_count = 0;

        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| matches!(tile, TileType::Paper))
            .map(|(i, _)| self.index_to_coord(i))
            .for_each(|coord| {
                let count = coord
                    .neighbors()
                    .iter()
                    .filter(|neighbor| matches!(self.get_tile(neighbor), Some(TileType::Paper)))
                    .count();
                if count < MOVEABLE_PAPER_LIMIT {
                    q.push_back(coord);
                }
            });

        while let Some(coord) = q.pop_front() {
            let index = self.coord_to_index(&coord);

            if !matches!(self.tiles[index], TileType::Paper) {
                continue;
            }

            self.remove_paper(&coord);
            removed_count += 1;

            coord.neighbors().iter().for_each(|neighbor| {
                if !matches!(self.get_tile(neighbor), Some(TileType::Paper)) {
                    return;
                }
                let count = neighbor
                    .neighbors()
                    .iter()
                    .filter(|n| matches!(self.get_tile(n), Some(TileType::Paper)))
                    .count();
                if count < MOVEABLE_PAPER_LIMIT {
                    q.push_back(*neighbor);
                }
            });
        }
        Some(removed_count)
    }

    fn remove_paper(&mut self, coord: &Coord) {
        let index = self.coord_to_index(coord);
        self.tiles[index] = TileType::Empty;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Grid::new(input).find_moveable_papers_part1()
}

pub fn part_two(input: &str) -> Option<usize> {
    Grid::new(input).find_moveable_papers_part2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
