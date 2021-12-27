use std::{collections::HashSet, fmt::Display};

const INPUT: &str = include_str!("./assets/day13.txt");

pub fn solve() -> String {
    let (paper, instructions) = parse(INPUT);
    format!(
        "{},\n{}",
        part1(paper.clone(), &instructions),
        part2(paper, &instructions)
    )
}

fn part1(mut paper: Paper, instructions: &[Fold]) -> usize {
    paper.fold(instructions[0]);
    paper.coords.len()
}

fn part2(mut paper: Paper, instructions: &[Fold]) -> String {
    for instruction in instructions {
        paper.fold(*instruction);
    }
    //dbg!(paper.coords);
    format!("{}", paper)
    //"TODO!".to_string()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fold {
    X(u64),
    Y(u64),
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Paper {
    width: u64,
    height: u64,
    coords: HashSet<(u64, u64)>,
}

impl Paper {
    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::X(pos) => {
                let left_width = pos;
                let right_width = self.width - pos - 1;
                self.width = std::cmp::max(left_width, right_width);
                let left_offset = self.width - left_width;
                self.coords = self
                    .coords
                    .iter()
                    .filter_map(|(x, y)| {
                        use std::cmp::Ordering::*;
                        match x.cmp(&pos) {
                            Less => Some((x + left_offset, *y)),
                            Greater => Some((2 * pos - x, *y)),
                            Equal => None,
                        }
                    })
                    .collect()
            }
            Fold::Y(pos) => {
                let top_height = pos;
                let bottom_height = self.height - pos - 1;
                self.height = std::cmp::max(top_height, bottom_height);
                let top_offset = self.height - top_height;
                self.coords = self
                    .coords
                    .iter()
                    .filter_map(|(x, y)| {
                        use std::cmp::Ordering::*;
                        match y.cmp(&pos) {
                            Less => Some((*x, y + top_offset)),
                            Greater => Some((*x, 2 * pos - y)),
                            Equal => None,
                        }
                    })
                    .collect()
            }
        }
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.coords.contains(&(x, y)) {
                    f.write_str("▉")?;
                } else {
                    f.write_str(" ")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> (Paper, Vec<Fold>) {
    let (paper, instructions) = match input.split_once("\r\n\r\n") {
        Some(data) => data,
        None => input.split_once("\n\n").unwrap(),
    };
    let coords: HashSet<_> = paper
        .lines()
        .map(|line| {
            let coord = line.trim();
            let (x, y) = coord.split_once(',').unwrap();
            let x: u64 = x.parse().unwrap();
            let y: u64 = y.parse().unwrap();
            (x, y)
        })
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in coords.iter() {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    let paper = Paper {
        width: max_x + 1,
        height: max_y + 1,
        coords,
    };

    let instructions: Vec<_> = instructions
        .lines()
        .map(|line| {
            let (axis, pos) = line.trim().split_once('=').unwrap();
            let pos = pos.parse().unwrap();
            match axis {
                "fold along x" => Fold::X(pos),
                "fold along y" => Fold::Y(pos),
                _ => unimplemented!(),
            }
        })
        .collect();

    (paper, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "6,10
                      0,14
                      9,10
                      0,3
                      10,4
                      4,11
                      6,0
                      6,12
                      4,1
                      0,13
                      10,12
                      3,4
                      3,0
                      8,4
                      1,10
                      2,14
                      8,10
                      9,0

                      fold along y=7
                      fold along x=5";

    #[test]
    fn test_parse() {
        let exp_paper = Paper {
            width: 11,
            height: 15,
            coords: HashSet::from([
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ]),
        };
        let exp_instructions = vec![Fold::Y(7), Fold::X(5)];
        let (act_paper, act_instructions) = parse(EX);
        assert_eq!(&act_paper, &exp_paper);
        assert_eq!(&act_instructions, &exp_instructions);
    }

    #[test]
    fn test1() {
        let (paper, instructions) = parse(EX);
        assert_eq!(part1(paper, &instructions), 17);
    }
}
