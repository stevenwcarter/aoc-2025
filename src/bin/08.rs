use std::collections::BTreeMap;

use advent_of_code::Coord3;
use hashbrown::HashMap;
use itertools::Itertools;
use nohash::BuildNoHashHasher;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

advent_of_code::solution!(8);

#[inline(always)]
fn parse_usize(input: &str) -> usize {
    let mut result = 0;
    for &b in input.as_bytes() {
        result = result * 10 + (b - b'0') as usize;
    }
    result
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

fn component_sizes(uf: &mut QuickUnionUf<UnionBySize>) -> HashMap<usize, usize> {
    let mut sizes = HashMap::new();

    for i in 0..uf.size() {
        let root = uf.find(i); // also does path compression
        *sizes.entry(root).or_insert(0) += 1;
    }

    sizes
}

// Connect the first thousand circuits, then return the product of the sizes of the three largest
// connected graphs
pub fn part_one(input: &str) -> Option<usize> {
    let circuits = parse_circuits_indexed(input);

    let combinations = compute_combinations_indexed(&circuits);

    // let mut uf = UnionFind::new(circuits.len());
    // let mut uf = UnionFind::new(circuits.len());
    let mut uf = QuickUnionUf::<UnionBySize>::new(circuits.len());

    #[allow(unused)]
    let mut connections = 1000;
    #[cfg(test)]
    let mut connections = 10;

    for (_, pairs) in combinations {
        if connections <= 0 {
            break;
        }
        // let pairs = combinations.get(distance).unwrap();
        for &(i, j) in pairs.iter() {
            if uf.union(i, j) && connections == 0 {
                break;
            }
            connections -= 1;
        }
    }

    let mut sizes = component_sizes(&mut uf)
        .values()
        .cloned()
        .collect::<Vec<usize>>();
    sizes.sort_by(|a, b| b.cmp(a));
    Some(sizes[0..3].iter().product())
}

/// Connect all circuits, then return the product of the x-coordinates of the two last connected
/// (which caused them all to be connected)
pub fn part_two(input: &str) -> Option<usize> {
    let circuits = parse_circuits_indexed(input);
    let combinations = compute_combinations_indexed(&circuits);

    let circuits_len = circuits.len();

    let mut uf = QuickUnionUf::<UnionBySize>::new(circuits_len);

    for distance in combinations.keys() {
        let pairs = combinations.get(distance).unwrap();
        for &(i, j) in pairs {
            if uf.union(i, j) && uf.get(0).size() == circuits_len {
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
