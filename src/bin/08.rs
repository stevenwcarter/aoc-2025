use std::collections::BTreeMap;

use atoi_simd::parse;
use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord3(usize, usize, usize);

impl Coord3 {
    /// Create a new 3D coordinate given x, y, and z values
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Coord3(x, y, z)
    }

    /// Returns x portion of coordinate
    #[inline]
    pub fn x(&self) -> usize {
        self.0
    }

    /// Returns y portion of coordinate
    #[inline]
    pub fn y(&self) -> usize {
        self.1
    }

    /// Returns z portion of coordinate
    #[inline]
    pub fn z(&self) -> usize {
        self.2
    }

    pub fn distance(&self, other: &Coord3) -> usize {
        ((self.x().abs_diff(other.x()).pow(2)
            + self.y().abs_diff(other.y()).pow(2)
            + self.z().abs_diff(other.z()).pow(2)) as f32)
            .sqrt() as usize
    }
}

impl From<(usize, usize, usize)> for Coord3 {
    fn from(value: (usize, usize, usize)) -> Self {
        Coord3(value.0, value.1, value.2)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut circuit_id = 0;
    let mut circuits: HashMap<Coord3, usize> = HashMap::new();
    let coords = input
        .lines()
        .map(|l| {
            Coord3::from(
                l.split(',')
                    .map(|c| parse::<usize>(c.trim().as_bytes()).unwrap())
                    .collect_tuple::<(usize, usize, usize)>()
                    .unwrap(),
            )
        })
        .collect::<Vec<Coord3>>();

    let iters = if coords.len() < 100 { 10 } else { 1000 };

    let mut combinations: BTreeMap<usize, Vec<(Coord3, Coord3)>> = BTreeMap::new();
    coords.iter().combinations(2).for_each(|v| {
        let c1 = *v[0];
        let c2 = *v[1];
        let dist = c1.distance(&c2);
        combinations.entry(dist).or_default().push((c1, c2));
    });

    let mut max_count = iters;
    for distance in combinations.keys() {
        if max_count <= 0 {
            break;
        }
        let combinations = combinations.get(distance).unwrap();
        dbg!(combinations.len());
        for (a, b) in combinations {
            let id1 = *circuits.get(a).unwrap_or(&0);
            let id2 = *circuits.get(b).unwrap_or(&0);
            match (id1, id2) {
                (0, 0) => {
                    circuit_id += 1;
                    circuits.insert(*a, circuit_id);
                    circuits.insert(*b, circuit_id);
                }
                (0, _) => {
                    circuits.insert(*a, id2);
                }
                (_, 0) => {
                    circuits.insert(*b, id1);
                }
                _ => {
                    circuits
                        .iter_mut()
                        .filter(|(_coord, cid)| **cid == id2)
                        .for_each(|(_coord, cid)| {
                            *cid = id1;
                        });
                }
            }
            max_count -= 1;
        }
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for cid in circuits.values() {
        *counts.entry(*cid).or_default() += 1;
    }

    let mut values: Vec<usize> = counts.values().copied().collect();

    values.sort();
    values.reverse();

    Some(values[0..3].iter().product())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut circuit_id = 0;
    let mut circuits: HashMap<Coord3, usize> = HashMap::new();
    let coords = input
        .lines()
        .map(|l| {
            Coord3::from(
                l.split(',')
                    .map(|c| parse::<usize>(c.trim().as_bytes()).unwrap())
                    .collect_tuple::<(usize, usize, usize)>()
                    .unwrap(),
            )
        })
        .inspect(|v| {
            circuit_id += 1;
            circuits.insert(*v, circuit_id);
        })
        .collect::<Vec<Coord3>>();

    let mut combinations: BTreeMap<usize, Vec<(Coord3, Coord3)>> = BTreeMap::new();
    coords.iter().combinations(2).for_each(|v| {
        let c1 = *v[0];
        let c2 = *v[1];
        let dist = c1.distance(&c2);
        combinations.entry(dist).or_default().push((c1, c2));
    });

    let mut result_found = false;
    let mut xs: Option<(usize, usize)> = None;
    for distance in combinations.keys() {
        if result_found {
            break;
        }
        let combinations = combinations.get(distance).unwrap();
        dbg!(combinations.len());
        for (a, b) in combinations {
            let id1 = *circuits.get(a).unwrap_or(&0);
            let id2 = *circuits.get(b).unwrap_or(&0);
            match (id1, id2) {
                (0, 0) => {
                    circuit_id += 1;
                    circuits.insert(*a, circuit_id);
                    circuits.insert(*b, circuit_id);
                }
                (0, _) => {
                    circuits.insert(*a, id2);
                }
                (_, 0) => {
                    circuits.insert(*b, id1);
                }
                _ => {
                    circuits
                        .iter_mut()
                        .filter(|(_coord, cid)| **cid == id2)
                        .for_each(|(_coord, cid)| {
                            *cid = id1;
                        });
                    if circuits.values().all(|&cid| cid == id1) {
                        xs = Some((a.x(), b.x()));
                        result_found = true;
                        break;
                    }
                }
            }
        }
    }

    Some(xs.unwrap().0 * xs.unwrap().1)
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
