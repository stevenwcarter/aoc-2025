use advent_of_code::Coord;
use hashbrown::HashSet;

advent_of_code::solution!(4);

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
        assert_eq!(tiles.len(), width * height);
        Grid {
            width,
            height,
            tiles,
        }
    }

    pub fn get(&self, coord: &Coord) -> Option<&TileType> {
        if (coord.x() as usize) < self.width && (coord.y() as usize) < self.height {
            Some(&self.tiles[coord.y() as usize * self.width + coord.x() as usize])
        } else {
            None
        }
    }

    pub fn find_moveable_papers(&self) -> Vec<Coord> {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| match tile {
                TileType::Paper => {
                    let x = (i % self.width) as i32;
                    let y = (i / self.width) as i32;
                    let coord = Coord::new(x, y);
                    let count = coord
                        .neighbors()
                        .iter()
                        .filter(|neighbor| matches!(self.get(neighbor), Some(TileType::Paper)))
                        .count();
                    if count < 4 { Some(coord) } else { None }
                }
                TileType::Empty => None,
            })
            .collect()
    }
    pub fn find_moveable_papers_count(&self) -> usize {
        self.find_moveable_papers().len()
    }
    pub fn remove_paper(&mut self, coord: &Coord) {
        self.tiles[coord.y() as usize * self.width + coord.x() as usize] = TileType::Empty;
    }
    pub fn find_moveable_papers_count_part2(&mut self) -> usize {
        let mut loop_count = 0;
        let mut total = 0;
        loop {
            let moveable_papers = self.find_moveable_papers();
            total += moveable_papers.len();
            if moveable_papers.is_empty() {
                break;
            }

            if loop_count > 5000 {
                break;
            }
            loop_count += 1;

            moveable_papers.iter().for_each(|c| self.remove_paper(c));
        }
        total
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);

    Some(grid.find_moveable_papers_count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::new(input);

    Some(grid.find_moveable_papers_count_part2() as u64)
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
