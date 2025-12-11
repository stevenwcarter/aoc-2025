#![allow(clippy::too_many_arguments)]

use hashbrown::HashMap;

advent_of_code::solution!(11);

const PATH_NODE_COUNT: usize = 605;

fn walk_paths(paths: &[&[u16]], current: u16, out_id: u16, visited: &mut [Option<u64>]) -> u64 {
    if let Some(result) = unsafe { *visited.get_unchecked(current as usize) } {
        return result;
    }
    if current == out_id {
        return 1;
    }

    let outs = unsafe { *paths.get_unchecked(current as usize) };
    let mut total = 0;
    for &next in outs {
        total += walk_paths(paths, next, out_id, visited);
    }

    visited[current as usize] = Some(total);

    total
}

fn walk_paths_part2(
    paths: &[&[u16]],
    current: u16,
    dac_seen: bool,
    fft_seen: bool,
    out_id: u16,
    dac_id: u16,
    fft_id: u16,
    visited: &mut [Option<u64>],
) -> u64 {
    let key = state_key(current, dac_seen, fft_seen);

    if let Some(result) = unsafe { *visited.get_unchecked(key) } {
        return result;
    }

    if current == out_id {
        let result = if dac_seen && fft_seen { 1 } else { 0 };
        visited[key] = Some(result);
        return result;
    }

    let curr_dac = dac_seen;
    let curr_fft = fft_seen;
    let outs = unsafe { *paths.get_unchecked(current as usize) };
    let mut total = 0;
    for &next in outs {
        let dac_seen_now = curr_dac || next == dac_id;
        let fft_seen_now = curr_fft || next == fft_id;
        total += walk_paths_part2(
            paths,
            next,
            dac_seen_now,
            fft_seen_now,
            out_id,
            dac_id,
            fft_id,
            visited,
        );
    }

    visited[key] = Some(total);

    total
}

fn parse_input(input: &str) -> (Vec<Vec<u16>>, HashMap<&str, u16>, u16) {
    // Preallocate based on known problem size
    let mut id_map: HashMap<&str, u16> = HashMap::with_capacity(PATH_NODE_COUNT);
    let mut next_id: u16 = 0;

    // Temporary adjacency list
    let mut temp_adj: Vec<(u16, Vec<u16>)> = Vec::with_capacity(PATH_NODE_COUNT);

    // ---- Parse input + assign IDs in one pass ----
    for line in input.lines() {
        let (key, rest) = line.split_once(':').unwrap();

        let key_id = *id_map.entry(key).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });

        // Preallocate outputs
        let mut outputs = Vec::with_capacity(21);

        for w in rest.split_whitespace() {
            let out_id = *id_map.entry(w).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
            outputs.push(out_id);
        }

        temp_adj.push((key_id, outputs));
    }

    // ---- Build final adjacency as Vec<&[u16]> ----
    let mut adj_storage: Vec<Vec<u16>> = vec![Vec::new(); next_id as usize];
    for (id, outs) in temp_adj {
        adj_storage[id as usize] = outs;
    }

    (adj_storage, id_map, next_id)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (adj_storage, id_map, next_id) = parse_input(input);

    // Convert into slices for cache locality
    let adj: Vec<&[u16]> = adj_storage.iter().map(|v| v.as_slice()).collect();

    let you_id = id_map["you"];
    let out_id = id_map["out"];

    let mut visited = vec![None; next_id as usize];

    Some(walk_paths(&adj, you_id, out_id, &mut visited))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (adj_storage, id_map, next_id) = parse_input(input);

    // Convert into slices for cache locality
    let adj: Vec<&[u16]> = adj_storage.iter().map(|v| v.as_slice()).collect();

    let svr_id = id_map["svr"];
    let out_id = id_map["out"];
    let dac_id = id_map["dac"];
    let fft_id = id_map["fft"];

    // Preallocate visited array
    let mut visited = vec![None; (next_id as usize) * 4];

    Some(walk_paths_part2(
        &adj,
        svr_id,
        false,
        false,
        out_id,
        dac_id,
        fft_id,
        &mut visited,
    ))
}

#[inline]
fn state_key(node: u16, dac_seen: bool, fft_seen: bool) -> usize {
    ((node as usize) << 2) | ((dac_seen as usize) << 1) | (fft_seen as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let example = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(example);
        assert_eq!(result, Some(2));
    }
}
