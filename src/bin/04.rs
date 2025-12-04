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

    fn index_to_coord(&self, index: usize) -> Coord {
        let x = (index % self.width) as i32;
        let y = (index / self.width) as i32;
        Coord::new(x, y)
    }
    fn coord_to_index(&self, coord: &Coord) -> usize {
        coord.y() as usize * self.width + coord.x() as usize
    }

    pub fn find_moveable_papers(&self) -> Option<Vec<Coord>> {
        let tiles: Vec<Coord> = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| matches!(tile, TileType::Paper))
            .map(|(i, _)| self.index_to_coord(i))
            .map(|coord| {
                let count = coord
                    .neighbors()
                    .iter()
                    .filter(|neighbor| matches!(self.get(neighbor), Some(TileType::Paper)))
                    .count();
                (coord, count)
            })
            .filter(|&(_, count)| count < MOVEABLE_PAPER_LIMIT)
            .map(|(coord, _)| coord)
            .collect();

        if tiles.is_empty() { None } else { Some(tiles) }
    }
    pub fn find_moveable_papers_count(&self) -> Option<usize> {
        Some(self.find_moveable_papers().unwrap().len())
    }
    pub fn remove_paper(&mut self, coord: &Coord) {
        let index = self.coord_to_index(coord);
        self.tiles[index] = TileType::Empty;
    }
    pub fn find_moveable_papers_count_part2(&mut self) -> Option<usize> {
        let mut total = 0;
        while let Some(moveable_papers) = self.find_moveable_papers() {
            total += moveable_papers.len();

            moveable_papers.iter().for_each(|c| self.remove_paper(c));
        }
        Some(total)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Grid::new(input).find_moveable_papers_count()
}

pub fn part_two(input: &str) -> Option<usize> {
    Grid::new(input).find_moveable_papers_count_part2()
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
