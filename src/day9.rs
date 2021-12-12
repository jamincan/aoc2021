use std::collections::HashSet;

const INPUT: &str = include_str!("./assets/day9.txt");

pub fn solve() -> String {
    let map = parse(INPUT);
    format!("{}, {}", part1(&map), part2(&map))
}

fn part1(map: &Map<u64>) -> u64 {
    let mut total = 0;
    for pt in map.points() {
        let adjacent = pt.adjacent();
        if adjacent.iter().all(|a| *pt < **a) {
            total += *pt + 1;
        }
    }
    total
}

fn part2(map: &Map<u64>) -> u64 {
    let mut basin_areas: Vec<_> = map
        .points()
        .filter(|pt| pt.adjacent().iter().all(|adj| **adj > **pt))
        .map(|pt| {
            let mut basin = HashSet::new();
            neighbours(&pt, &mut basin);
            basin.len() as u64
        })
        .collect();
    basin_areas.sort_unstable();
    basin_areas[basin_areas.len() - 3..]
        .iter()
        .cloned()
        .map(|a| a + 1)
        .reduce(|a, b| a * b)
        .unwrap()
}

fn neighbours(point: &Point<u64>, basin: &mut HashSet<(usize, usize)>) {
    let adjacent = point.adjacent();
    for adj in adjacent.iter().filter(|adj| ***adj > **point && ***adj < 9) {
        if !basin.contains(&adj.index) {
            basin.insert(adj.index);
            neighbours(adj, basin);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map<T> {
    points: Vec<Vec<T>>,
}

impl<T> Map<T> {
    fn rows(&self) -> usize {
        self.points.len()
    }
    fn columns(&self) -> usize {
        if let Some(row) = self.points.get(0) {
            row.len()
        } else {
            0
        }
    }
    fn point(&self, row: usize, col: usize) -> Point<T> {
        Point {
            index: (row, col),
            map: self,
        }
    }
    fn points(&self) -> impl Iterator<Item = Point<T>> {
        let row = 0..self.rows();
        let col = 0..self.columns();
        row.flat_map(move |row| {
            col.clone().map(move |col| Point {
                index: (row, col),
                map: self,
            })
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Point<'map, T> {
    index: (usize, usize),
    map: &'map Map<T>,
}

impl<'map, T> Point<'map, T> {
    fn adjacent(&self) -> Vec<Point<'map, T>> {
        let (row, col) = self.index;
        let mut adjacent = Vec::new();
        if row > 0 {
            adjacent.push(self.map.point(row - 1, col));
        }
        if col > 0 {
            adjacent.push(self.map.point(row, col - 1));
        }
        if row < self.map.rows() - 1 {
            adjacent.push(self.map.point(row + 1, col));
        }
        if col < self.map.columns() - 1 {
            adjacent.push(self.map.point(row, col + 1));
        }
        adjacent
    }
}

impl<'map, T> std::ops::Deref for Point<'map, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let (row, col) = self.index;
        &self.map.points[row][col]
    }
}

fn parse(input: &str) -> Map<u64> {
    let points = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch as u64 - b'0' as u64)
                .collect()
        })
        .collect();
    Map { points }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2199943210
                           3987894921
                           9856789892
                           8767896789
                           9899965678";

    #[test]
    fn test_parse() {
        let expected = Map {
            points: vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ],
        };
        assert_eq!(parse(EXAMPLE), expected);
    }

    #[test]
    fn test_deref() {
        let map = parse(EXAMPLE);
        let point = Point {
            index: (0, 0),
            map: &map,
        };
        assert_eq!(*point, 2);
    }

    #[test]
    fn test_adjacent() {
        let tests = [
            (0, 0, vec![1, 3]),
            (1, 1, vec![1, 3, 8, 8]),
            (4, 3, vec![7, 9, 9]),
        ];
        let map = parse(EXAMPLE);

        for (r, c, expected) in tests {
            let pt = Point {
                index: (r, c),
                map: &map,
            };
            let mut actual: Vec<_> = pt.adjacent().iter().map(|pt| **pt).collect();
            actual.sort_unstable();
            assert_eq!(&actual[..], &expected[..]);
        }
    }

    #[test]
    fn test1() {
        let map = parse(EXAMPLE);
        assert_eq!(part1(&map), 15);
    }

    #[test]
    fn test2() {
        let map = parse(EXAMPLE);
        assert_eq!(part2(&map), 1134);
    }
}
