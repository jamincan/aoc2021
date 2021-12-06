use std::collections::HashMap;

const INPUT: &str = include_str!("./assets/day5.txt");

pub fn solve() -> String {
    let lines = parse(INPUT);
    format!("{}, {}", part1(&lines), part2(&lines))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Line {
    cur: Option<(u64, u64)>,
    end: (u64, u64),
}

impl Line {
    fn new(start: (u64, u64), end: (u64, u64)) -> Line {
        Line {
            cur: Some(start),
            end,
        }
    }
}

impl Iterator for Line {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.cur {
            use std::cmp::Ordering;

            if self.end == cur {
                self.cur = None;
                return Some(cur);
            }
            let new_x = match cur.0.cmp(&self.end.0) {
                Ordering::Less => cur.0 + 1,
                Ordering::Equal => cur.0,
                Ordering::Greater => cur.0 - 1,
            };
            let new_y = match cur.1.cmp(&self.end.1) {
                Ordering::Less => cur.1 + 1,
                Ordering::Equal => cur.1,
                Ordering::Greater => cur.1 - 1,
            };
            self.cur = Some((new_x, new_y));
            Some(cur)
        } else {
            None
        }
    }
}

fn part1(lines: &[Line]) -> usize {
    // Only look at horizontal or vertical lines
    let lines = lines.iter().filter(|line| {
        let (ax, ay) = line.cur.unwrap();
        let (bx, by) = line.end;
        ax == bx || ay == by
    });
    let points: HashMap<_, u64> =
        lines
            .flat_map(|line| *line)
            .fold(HashMap::new(), |mut pts, pt| {
                *pts.entry(pt).or_default() += 1;
                pts
            });
    let danger = points.iter().filter(|(_, freq)| **freq >= 2);
    danger.count()
}

fn part2(lines: &[Line]) -> usize {
    let points: HashMap<_, u64> =
        lines
            .iter()
            .flat_map(|line| *line)
            .fold(HashMap::new(), |mut pts, pt| {
                *pts.entry(pt).or_default() += 1;
                pts
            });
    let danger = points.iter().filter(|(_, freq)| **freq >= 2);
    danger.count()
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter_map(|l| {
            let (a, b) = l.trim().split_once(" -> ")?;
            let (a_1, a_2) = a.split_once(',')?;
            let (b_1, b_2) = b.split_once(',')?;
            let start = (a_1.parse().ok()?, a_2.parse().ok()?);
            let end = (b_1.parse().ok()?, b_2.parse().ok()?);
            Some(Line::new(start, end))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

    #[test]
    fn test_parse() {
        let expected = vec![
            Line::new((0, 9), (5, 9)),
            Line::new((8, 0), (0, 8)),
            Line::new((9, 4), (3, 4)),
            Line::new((2, 2), (2, 1)),
            Line::new((7, 0), (7, 4)),
            Line::new((6, 4), (2, 0)),
            Line::new((0, 9), (2, 9)),
            Line::new((3, 4), (1, 4)),
            Line::new((0, 0), (8, 8)),
            Line::new((5, 5), (8, 2)),
        ];
        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn test1() {
        let lines = parse(INPUT);
        assert_eq!(part1(&lines), 5);
    }

    #[test]
    fn test2() {
        let lines = parse(INPUT);
        assert_eq!(part2(&lines), 12);
    }
}
