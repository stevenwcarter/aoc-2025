use advent_of_code::Coord3;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

advent_of_code::solution!(8);

#[derive(Copy, Clone)]
struct Edge {
    d: u32,
    i: usize,
    j: usize,
}

#[inline(always)]
fn parse_usize(input: &str) -> usize {
    let mut result = 0;
    for &b in input.as_bytes() {
        result = result * 10 + (b - b'0') as usize;
    }
    result
}

fn parse_circuits(input: &str) -> Vec<Coord3> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(',');
            let x = parse_usize(it.next().unwrap());
            let y = parse_usize(it.next().unwrap());
            let z = parse_usize(it.next().unwrap());
            Coord3::from((x, y, z))
        })
        .collect()
}

#[inline(always)]
fn edge_cmp(a: &Edge, b: &Edge) -> core::cmp::Ordering {
    a.d.cmp(&b.d)
        .then_with(|| a.i.cmp(&b.i))
        .then_with(|| a.j.cmp(&b.j))
}

fn compute_edges(circuits: &[Coord3]) -> Vec<Edge> {
    let n = circuits.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = circuits[i].distance(&circuits[j]); // IMPORTANT: use your existing metric
            edges.push(Edge { d, i, j });
        }
    }

    edges
}

fn radix_sort_edges(edges: &mut [Edge]) {
    const RADIX: usize = 256;
    let mut tmp = vec![edges[0]; edges.len()];

    for shift in (0..32).step_by(8) {
        let mut count = [0usize; RADIX];

        for e in edges.iter() {
            count[((e.d >> shift) & 0xFF) as usize] += 1;
        }

        let mut sum = 0;
        for c in count.iter_mut() {
            let t = *c;
            *c = sum;
            sum += t;
        }

        for &e in edges.iter() {
            let idx = ((e.d >> shift) & 0xFF) as usize;
            tmp[count[idx]] = e;
            count[idx] += 1;
        }

        edges.copy_from_slice(&tmp);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let circuits = parse_circuits(input);
    let mut edges = compute_edges(&circuits);

    let k = if cfg!(test) { 10 } else { 1000 };

    // only sort the first edges that are needed
    edges.select_nth_unstable_by(k, edge_cmp);
    edges[..k].sort_unstable_by(edge_cmp);

    let mut uf = QuickUnionUf::<UnionBySize>::new(circuits.len());

    for e in &edges[..k] {
        uf.union(e.i, e.j);
    }

    let mut sizes = Vec::new();
    for i in 0..uf.size() {
        if uf.find(i) == i {
            sizes.push(uf.get(i).size());
        }
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    Some(sizes[0] * sizes[1] * sizes[2])
}

pub fn part_two(input: &str) -> Option<usize> {
    let circuits = parse_circuits(input);
    let mut edges = compute_edges(&circuits);

    radix_sort_edges(&mut edges);

    let n = circuits.len();
    let mut uf = QuickUnionUf::<UnionBySize>::new(n);
    let mut components = n;

    for e in edges {
        if uf.union(e.i, e.j) {
            components -= 1;
            if components == 1 {
                return Some(circuits[e.i].x() * circuits[e.j].x());
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
