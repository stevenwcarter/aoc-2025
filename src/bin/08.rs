use std::collections::BTreeMap;

use advent_of_code::Coord3;
use atoi_simd::parse;
use hashbrown::HashMap;
use itertools::Itertools;
use nohash::BuildNoHashHasher;

advent_of_code::solution!(8);

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // already connected
        }

        // union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.rank[root_x] += 1;
        }
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();
        for i in 0..self.parent.len() {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }

    fn all_connected(&mut self) -> bool {
        let root = self.find(0);
        (1..self.parent.len()).all(|i| self.find(i) == root)
    }
}

#[inline(always)]
fn parse_usize(input: &str) -> usize {
    parse::<usize>(input.as_bytes()).unwrap()
}

fn parse_circuits_indexed(input: &str) -> Vec<Coord3> {
    input
        .lines()
        .map(|l| {
            Coord3::from(
                l.split(',')
                    .map(parse_usize)
                    .collect_tuple::<(usize, usize, usize)>()
                    .unwrap(),
            )
        })
        .collect()
}

fn compute_combinations_indexed(circuits: &[Coord3]) -> BTreeMap<u32, Vec<(usize, usize)>> {
    // no hashing to speed up initial insertion
    let mut combinations: HashMap<u32, Vec<(usize, usize)>, BuildNoHashHasher<u32>> =
        HashMap::with_hasher(BuildNoHashHasher::default());

    (0..circuits.len()).combinations(2).for_each(|pair| {
        let i = pair[0];
        let j = pair[1];
        let dist = circuits[i].distance(&circuits[j]);
        if dist < 15_000 {
            // simple distance cutoff to reduce number of combinations
            combinations.entry(dist).or_default().push((i, j));
        }
    });
    // convert back to BTreeMap for ordered keys once at the end, more performant
    combinations.into_iter().collect()
}

// Connect the first thousand circuits, then return the product of the sizes of the three largest
// connected graphs
pub fn part_one(input: &str) -> Option<usize> {
    let circuits = parse_circuits_indexed(input);

    let combinations = compute_combinations_indexed(&circuits);

    let mut uf = UnionFind::new(circuits.len());

    let mut connections = if input.lines().collect_vec().len() < 100 {
        10
    } else {
        1000
    };

    for distance in combinations.keys() {
        if connections <= 0 {
            break;
        }
        let pairs = combinations.get(distance).unwrap();
        for &(i, j) in pairs {
            if uf.union(i, j) && connections <= 0 {
                break;
            }
            connections -= 1;
        }
    }

    let mut sizes = uf.component_sizes();
    sizes.sort_by(|a, b| b.cmp(a));
    Some(sizes[0..3].iter().product())
}

/// Connect all circuits, then return the product of the x-coordinates of the two last connected
/// (which caused them all to be connected)
pub fn part_two(input: &str) -> Option<usize> {
    let circuits = parse_circuits_indexed(input);
    let combinations = compute_combinations_indexed(&circuits);
    let mut uf = UnionFind::new(circuits.len());

    for distance in combinations.keys() {
        let pairs = combinations.get(distance).unwrap();
        for &(i, j) in pairs {
            if uf.union(i, j) && uf.all_connected() {
                return Some(circuits[i].x() * circuits[j].x());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
