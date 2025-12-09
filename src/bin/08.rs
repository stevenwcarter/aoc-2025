use std::collections::BTreeMap;

use advent_of_code::Coord3;
use atoi_simd::parse;
use hashbrown::HashMap;
use itertools::Itertools;
use nohash::BuildNoHashHasher;

advent_of_code::solution!(8);

fn parse_circuits(input: &str) -> HashMap<Coord3, usize> {
    let mut circuit_id = 0;
    let mut circuits: HashMap<Coord3, usize> = HashMap::new();
    input
        .lines()
        .map(|l| {
            Coord3::from(
                l.split(',')
                    .map(|c| parse::<usize>(c.trim().as_bytes()).unwrap())
                    .collect_tuple::<(usize, usize, usize)>()
                    .unwrap(),
            )
        })
        .for_each(|v| {
            circuit_id += 1;
            circuits.insert(v, circuit_id);
        });

    circuits
}

fn compute_combinations(
    circuits: &HashMap<Coord3, usize>,
) -> BTreeMap<usize, Vec<(Coord3, Coord3)>> {
    // no hashing to speed up initial insertion
    let mut combinations: HashMap<usize, Vec<(Coord3, Coord3)>, BuildNoHashHasher<usize>> =
        HashMap::with_hasher(BuildNoHashHasher::default());
    circuits.keys().combinations(2).for_each(|v| {
        let c1 = *v[0];
        let c2 = *v[1];
        let dist = c1.distance(&c2);
        if dist < 20_000 {
            // simple distance cutoff to reduce number of combinations
            combinations.entry(dist).or_default().push((c1, c2));
        }
    });
    // convert back to BTreeMap for ordered keys once at the end, more performant
    combinations.into_iter().collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut circuits = parse_circuits(input);

    let combinations = compute_combinations(&circuits);

    let mut iters = if circuits.len() < 100 { 10 } else { 1000 };
    for distance in combinations.keys() {
        if iters <= 0 {
            break;
        }
        let combinations = combinations.get(distance).unwrap();
        for (a, b) in combinations {
            let id1 = *circuits.get(a).unwrap_or(&0);
            let id2 = *circuits.get(b).unwrap_or(&0);
            if id1 == id2 {
                iters -= 1;
                continue;
            }
            // combine circuits
            circuits
                .iter_mut()
                .filter(|(_coord, cid)| **cid == id2)
                .for_each(|(_coord, cid)| {
                    *cid = id1;
                });
            iters -= 1;
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
    let mut circuits = parse_circuits(input);

    let combinations = compute_combinations(&circuits);

    let mut xs: Option<(usize, usize)> = None;
    combinations.keys().any(|distance| {
        let combinations = combinations.get(distance).unwrap();
        for (a, b) in combinations {
            let id1 = *circuits.get(a).unwrap();
            let id2 = *circuits.get(b).unwrap();
            if id1 == id2 {
                // already connected
                continue;
            }
            circuits
                .iter_mut()
                .filter(|(_coord, cid)| **cid == id2)
                .for_each(|(_coord, cid)| {
                    *cid = id1;
                });
            if circuits.values().all(|&cid| cid == id1) {
                xs = Some((a.x(), b.x()));
                return true; // short-circuit outer loop (why I used any)
            }
        }
        false
    });

    let xs = xs.unwrap();
    Some(xs.0 * xs.1)
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
