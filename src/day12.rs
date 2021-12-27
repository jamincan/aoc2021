use std::collections::HashMap;

type CaveMap<'map> = HashMap<&'map str, Vec<&'map str>>;

const INPUT: &str = include_str!("./assets/day12.txt");

pub fn solve() -> String {
    let map = build_map(INPUT);
    format!("{}, {}", part1(&map), part2(&map))
}

fn build_map(input: &str) -> CaveMap {
    let mut map = HashMap::new();
    for line in input.lines().map(|l| l.trim()) {
        let (a, b) = line.split_once('-').unwrap();
        let a_leaves = map.entry(a).or_insert_with(Vec::new);
        a_leaves.push(b);
        let b_leaves = map.entry(b).or_insert_with(Vec::new);
        b_leaves.push(a);
    }
    map
}

fn part1(map: &CaveMap) -> usize {
    let mut partial_paths = vec![vec!["start"]];
    let mut complete_paths = Vec::new();
    while let Some(path) = partial_paths.pop() {
        // Find all nodes adjacent to the last node in the path
        let last_node = path.last().unwrap();
        let adjoining = map.get(last_node).unwrap();
        for next_node in adjoining {
            // If the node is "end" it marks the end of a path and we push it to complete
            if *next_node == "end" {
                let mut complete = path.clone();
                complete.push(next_node);
                complete_paths.push(complete);
            }
            // If the node is uppercase, we can visit it under any circumstance
            // or, if it's not already in the list, we can visit it
            else if &next_node.to_ascii_uppercase() == next_node || !path.contains(next_node) {
                let mut partial = path.clone();
                partial.push(next_node);
                partial_paths.push(partial);
            }
        }
    }
    complete_paths.len()
}

fn part2(map: &CaveMap) -> usize {
    let mut partial_paths = vec![(vec!["start"], false)];
    let mut complete_paths = Vec::new();
    while let Some((path, small_visited)) = partial_paths.pop() {
        // Find all nodes adjacent to the last node in the path
        let last_node = path.last().unwrap();
        let adjoining = map.get(last_node).unwrap();
        for next_node in adjoining {
            // If the node is "end" it marks the end of a path and we push it to complete
            if *next_node == "end" {
                let mut complete = path.clone();
                complete.push(next_node);
                complete_paths.push(complete);
            }
            // Drop the path trying to return to start, as that isn't allowed
            else if *next_node == "start" {
                continue;
            }
            // If the node is uppercase, we can visit it under any circumstance
            // if it's a small cave, we can only visit it once
            else if &next_node.to_ascii_uppercase() == next_node || !path.contains(next_node) {
                let mut partial = path.clone();
                partial.push(next_node);
                partial_paths.push((partial, small_visited));
            }
            // We can visit one small cave twice
            else if !small_visited && path.contains(next_node) {
                let mut partial = path.clone();
                partial.push(next_node);
                partial_paths.push((partial, true));
            }
        }
    }
    complete_paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "start-A
                       start-b
                       A-c
                       A-b
                       b-d
                       A-end
                       b-end";

    const EX2: &str = "dc-end
                       HN-start
                       start-kj
                       dc-start
                       dc-HN
                       LN-dc
                       HN-end
                       kj-sa
                       kj-HN
                       kj-dc";

    const EX3: &str = "fs-end
                       he-DX
                       fs-he
                       start-DX
                       pj-DX
                       end-zg
                       zg-sl
                       zg-pj
                       pj-he
                       RW-he
                       fs-DX
                       pj-RW
                       zg-RW
                       start-pj
                       he-WI
                       zg-he
                       pj-fs
                       start-RW";

    #[test]
    fn test1() {
        let tests = [
            (build_map(EX1), 10),
            (build_map(EX2), 19),
            (build_map(EX3), 226),
        ];

        for (map, expected) in tests {
            assert_eq!(part1(&map), expected);
        }
    }

    #[test]
    fn test2() {
        let tests = [
            (build_map(EX1), 36),
            (build_map(EX2), 103),
            (build_map(EX3), 3509),
        ];

        for (map, expected) in tests {
            assert_eq!(part2(&map), expected);
        }
    }
}
