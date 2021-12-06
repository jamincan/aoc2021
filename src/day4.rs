use itertools::Itertools;
use std::convert::TryInto;

const INPUT: &str = include_str!("./assets/day4.txt");

pub fn solve() -> String {
    let (random, boards) = parse(INPUT);
    format!(
        "{}, {}",
        part1(&random, &mut boards.clone()),
        part2(&random, &boards)
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Number {
    Marked(u64),
    Open(u64),
}

impl Number {
    fn mark(&mut self) {
        if let Number::Open(num) = self {
            *self = Number::Marked(*num);
        }
    }

    fn is_marked(&self) -> bool {
        match self {
            Number::Marked(_) => true,
            Number::Open(_) => false,
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Marked(num) => f.write_fmt(format_args!("[{:>3}]", num))?,
            Self::Open(num) => f.write_fmt(format_args!(" {:>3} ", num))?,
        };
        Ok(())
    }
}

impl PartialEq<u64> for Number {
    fn eq(&self, other: &u64) -> bool {
        match *self {
            Self::Marked(num) => num == *other,
            Self::Open(num) => num == *other,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BingoBoard {
    board: [[Number; 5]; 5],
}

impl BingoBoard {
    fn mark(&mut self, number: u64) {
        for row in self.board.iter_mut() {
            for num in row.iter_mut().filter(|num| **num == number) {
                num.mark()
            }
        }
    }

    fn check(&self) -> bool {
        let rows = self
            .board
            .iter()
            .any(|row| row.iter().all(Number::is_marked));
        let cols = (0..5).any(|col| self.board.iter().all(|row| row[col].is_marked()));
        rows || cols
    }

    fn score(&self) -> u64 {
        self.board
            .iter()
            .flat_map(|row| {
                row.iter().filter_map(|n| match *n {
                    Number::Marked(_) => None,
                    Number::Open(n) => Some(n),
                })
            })
            .sum()
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    // Collect the random numbers
    let random: Vec<_> = lines
        .next()
        .expect("Empty input")
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    // Collect the boards.
    let boards: Vec<BingoBoard> = lines
        .chunks(6)
        .into_iter()
        .filter_map(|chunk| {
            chunk
                .skip(1)
                .filter_map(|line| {
                    line.split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .map(Number::Open)
                        .collect::<Vec<_>>()
                        .try_into()
                        .ok()
                })
                .collect::<Vec<_>>()
                .try_into()
                .map(|board| BingoBoard { board })
                .ok()
        })
        .collect();
    (random, boards)
}

fn part1(random: &[u64], boards: &mut [BingoBoard]) -> u64 {
    for num in random {
        for board in boards.iter_mut() {
            board.mark(*num);
            if board.check() {
                return board.score() * num;
            }
        }
    }
    panic!("No solution found for D4P1.");
}

fn part2(random: &[u64], boards: &[BingoBoard]) -> u64 {
    let mut boards: Vec<_> = boards.iter().map(|b| (true, *b)).collect();
    for num in random {
        let mut current_index = None;
        for (index, (active, board)) in boards.iter_mut().enumerate().filter(|(_, (a, _))| *a) {
            board.mark(*num);
            if board.check() {
                *active = false;
                current_index = Some(index);
            }
        }
        if !boards.iter().any(|(a, _)| *a) {
            let (_, board) = boards[current_index.unwrap()];
            return num * board.score();
        }
    }
    panic!("No solution found for D4P2.");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn test_parse_random() {
        let expected = vec![
            7u64, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
            19, 3, 26, 1,
        ];
        let (actual, _) = parse(INPUT);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_parse_board() {
        use Number::*;
        let expected = BingoBoard {
            board: [
                [Open(22), Open(13), Open(17), Open(11), Open(0)],
                [Open(8), Open(2), Open(23), Open(4), Open(24)],
                [Open(21), Open(9), Open(14), Open(16), Open(7)],
                [Open(6), Open(10), Open(3), Open(18), Open(5)],
                [Open(1), Open(12), Open(20), Open(15), Open(19)],
            ],
        };
        let (_, actual) = parse(INPUT);
        assert_eq!(expected, actual[0]);
    }

    #[test]
    fn test1() {
        let (random, mut boards) = parse(INPUT);
        assert_eq!(part1(&random, &mut boards), 4512);
    }

    #[test]
    fn test2() {
        let (random, boards) = parse(INPUT);
        assert_eq!(part2(&random, &boards), 1924);
    }
}
