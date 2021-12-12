const INPUT: &str = include_str!("./assets/day11.txt");

pub fn solve() -> String {
    let mut grid: Grid = INPUT.parse().unwrap();
    format!(
        "{}, {}",
        part1(&mut grid.clone()),
        part2(&mut grid, u32::MAX)
    )
}

fn part1(grid: &mut Grid) -> u32 {
    let mut count = 0;
    for _ in 0..100 {
        count += grid.tick();
    }
    count
}

fn part2(grid: &mut Grid, max: u32) -> u32 {
    let step = (1..=max).find(|_| grid.tick() == 100).unwrap();
    step
}

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                f.write_fmt(format_args!("{}", self.cells[index]))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn get_coord(&self, index: usize) -> (u32, u32) {
        let row = (index / self.height as usize) as u32;
        let col = (index % self.height as usize) as u32;
        (row, col)
    }

    fn get_neighbours(&self, index: usize) -> Vec<usize> {
        let (row, col) = self.get_coord(index);
        let rows = row.saturating_sub(1)..=(row + 1).min(self.height - 1);
        let cols = col.saturating_sub(1)..=(col + 1).min(self.width - 1);
        rows.flat_map(|r| cols.clone().map(move |c| self.get_index(r, c)))
            .filter(|i| *i != index)
            .collect()
    }

    fn tick(&mut self) -> u32 {
        for octopus in self.cells.iter_mut() {
            *octopus += 1;
        }
        let mut tens: Vec<_> = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(index, energy)| match energy {
                10 => Some(index),
                _ => None,
            })
            .collect();

        while let Some(index) = tens.pop() {
            let neighbours = self.get_neighbours(index);
            self.cells[index] = 0;
            for neighbour in neighbours {
                let value = self.cells[neighbour];
                // Any value equal to zero flashed this round
                // Any value from 1 to 8 increases by 1
                // Any value equal to 9 increases by 1, which means it also flashes and is added to tens
                // Any value equal to 10 is already in the queue to flash
                match value {
                    1..=8 => self.cells[neighbour] += 1,
                    9 => {
                        self.cells[neighbour] += 1;
                        tens.push(neighbour);
                    }
                    _ => (),
                };
            }
        }

        self.cells.iter().filter(|v| **v == 0).count() as u32
    }
}

impl std::str::FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let cells: Vec<_> = input
            .lines()
            .flat_map(|line| {
                let chars = line.trim().chars();
                if width == None {
                    width = Some(chars.clone().count() as u32);
                }
                chars.map(|ch| match ch {
                    c @ '0'..='9' => {
                        let val = ((c as u32) - ('0' as u32)) as u8;
                        Ok(val)
                    }
                    _ => Err(()),
                })
            })
            .collect::<Result<_, _>>()?;
        if let Some(width) = width {
            let height = cells.len() as u32 / width;
            Ok(Grid {
                height,
                width,
                cells,
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "5483143223
                           2745854711
                           5264556173
                           6141336146
                           6357385478
                           4167524645
                           2176841721
                           6882881134
                           4846848554
                           5283751526";

    #[test]
    fn test_parse() {
        let string = "123
                           456
                           789";
        let cells: Vec<_> = (1..=9).collect();
        let expected = Grid {
            width: 3,
            height: 3,
            cells,
        };
        assert_eq!(string.parse(), Ok(expected));
    }

    #[test]
    fn test_get_neighbours() {
        let tests = [
            (0, vec![1, 10, 11]),
            (5, vec![4, 6, 14, 15, 16]),
            (9, vec![8, 18, 19]),
            (50, vec![40, 41, 51, 60, 61]),
            (55, vec![44, 45, 46, 54, 56, 64, 65, 66]),
            (59, vec![48, 49, 58, 68, 69]),
            (90, vec![80, 81, 91]),
            (95, vec![84, 85, 86, 94, 96]),
            (99, vec![88, 89, 98]),
        ];
        let grid: Grid = EXAMPLE.parse().unwrap();
        for (index, expected) in tests {
            let actual = grid.get_neighbours(index);
            assert_eq!(&actual, &expected);
        }
    }

    #[test]
    fn test_example() {
        let mut grid: Grid = EXAMPLE.parse().unwrap();
        let flashes = [0, 35, 45, 16, 8, 1, 7, 24, 39, 29];
        println!("{}", &grid);
        for count in flashes {
            let actual = grid.tick();
            println!("{}", &grid);
            assert_eq!(actual, count);
        }
    }

    #[test]
    fn test1() {
        let mut grid = EXAMPLE.parse().unwrap();
        assert_eq!(part1(&mut grid), 1656);
    }

    #[test]
    fn test2() {
        let mut grid = EXAMPLE.parse().unwrap();
        assert_eq!(part2(&mut grid, 195), 195);
    }
}
