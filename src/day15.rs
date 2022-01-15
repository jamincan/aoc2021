use array2d::Array2D;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("./assets/day15.txt");

pub fn solve() -> String {
    let grid = parse(INPUT);
    let width = grid.num_columns();
    let height = grid.num_rows();
    let part1 = shortest_path(&grid, (0, 0), (height - 1, width - 1)).unwrap();
    let grid = repeat_grid(&grid, 5, 5);
    let width = grid.num_columns();
    let height = grid.num_rows();
    let part2 = shortest_path(&grid, (0, 0), (height - 1, width - 1)).unwrap();
    format!("{part1}, {part2}")
}

type Coordinate = (usize, usize);

#[derive(Clone, Copy, Debug, Eq)]
struct Node {
    position: (usize, usize),
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn parse(input: &str) -> Array2D<u32> {
    let mut lines = input.lines();
    let num_columns = lines.next().unwrap().trim().len();
    let num_rows = lines.count() + 1;
    let digits = input.chars().filter_map(|ch| ch.to_digit(10));
    Array2D::from_iter_row_major(digits, num_rows, num_columns)
}

fn repeat_grid(grid: &Array2D<u32>, repeat_v: u32, repeat_h: u32) -> Array2D<u32> {
    let grid = grid.as_rows();
    let digits = (0..repeat_v).flat_map(|row_offset| {
        grid.iter().flat_map(move |row| {
            (0..repeat_h).flat_map(move |col_offset| {
                row.iter()
                    .map(move |d| transform(*d, col_offset + row_offset))
            })
        })
    });
    Array2D::from_iter_row_major(
        digits,
        grid.len() * repeat_v as usize,
        grid[0].len() * repeat_h as usize,
    )
}

fn transform(digit: u32, offset: u32) -> u32 {
    debug_assert!((1..=9).contains(&digit));
    ((digit + offset - 1) % 9) + 1
}

fn shortest_path(grid: &Array2D<u32>, start: Coordinate, goal: Coordinate) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    let mut risk = Array2D::filled_with(u32::MAX, grid.num_rows(), grid.num_columns());
    risk[start] = 0;
    queue.push(Node {
        position: start,
        cost: 0,
    });

    while let Some(Node { position, cost }) = queue.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > risk[position] {
            continue;
        }
        for adjacent in adjacent(position, grid.num_rows(), grid.num_columns()) {
            let next = Node {
                cost: cost + grid[adjacent],
                position: adjacent,
            };
            if next.cost < risk[adjacent] {
                queue.push(next);
                risk[adjacent] = next.cost;
            }
        }
    }

    None
}

fn print_array(array: &Array2D<u32>, col_width: usize) {
    for row in array.rows_iter() {
        for digit in row {
            print!("{digit:>col_width$}");
        }
        println!();
    }
}

fn adjacent(position: Coordinate, width: usize, height: usize) -> impl Iterator<Item = Coordinate> {
    let (row, col) = position;
    let mut adjacent = Vec::new();
    if row > 0 {
        adjacent.push((row - 1, col));
    }
    if row < height - 1 {
        adjacent.push((row + 1, col));
    }
    if col > 0 {
        adjacent.push((row, col - 1));
    }
    if col < width - 1 {
        adjacent.push((row, col + 1));
    }
    adjacent.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "1163751742
                      1381373672
                      2136511328
                      3694931569
                      7463417111
                      1319128137
                      1359912421
                      3125421639
                      1293138521
                      2311944581";

    #[test]
    fn test1() {
        let grid = parse(EX);
        let goal = (grid.num_rows() - 1, grid.num_columns() - 1);
        //let goal = (2, 2);
        let actual = shortest_path(&grid, (0, 0), goal).unwrap();
        assert_eq!(actual, 40);
    }

    #[test]
    fn test_transform() {
        let offset_one: Vec<_> = (1..=9).map(|d| transform(d, 1)).collect();
        let expect_one = vec![2, 3, 4, 5, 6, 7, 8, 9, 1];
        assert_eq!(offset_one, expect_one);

        let offset_two: Vec<_> = (1..=9).map(|d| transform(d, 2)).collect();
        let expect_two = vec![3, 4, 5, 6, 7, 8, 9, 1, 2];
        assert_eq!(offset_two, expect_two);

        let offset_three: Vec<_> = (1..=9).map(|d| transform(d, 3)).collect();
        let expect_three = vec![4, 5, 6, 7, 8, 9, 1, 2, 3];
        assert_eq!(offset_three, expect_three);

        let offset_four: Vec<_> = (1..=9).map(|d| transform(d, 4)).collect();
        let expect_four = vec![5, 6, 7, 8, 9, 1, 2, 3, 4];
        assert_eq!(offset_four, expect_four);

        let offset_five: Vec<_> = (1..=9).map(|d| transform(d, 5)).collect();
        let expect_five = vec![6, 7, 8, 9, 1, 2, 3, 4, 5];
        assert_eq!(offset_five, expect_five);
    }

    #[test]
    fn test_repeat_grid() {
        let single_str = "1234
                               2345
                               3456
                               4567";
        let single = parse(single_str);

        let two_column_str = "12342345
                                   23453456
                                   34564567
                                   45675678";
        let two_column = parse(two_column_str);
        assert_eq!(two_column, repeat_grid(&single, 1, 2));

        let two_row_str = "1234
                                2345
                                3456
                                4567
                                2345
                                3456
                                4567
                                5678";
        let two_row = parse(two_row_str);
        assert_eq!(two_row, repeat_grid(&single, 2, 1));

        let two_row_and_column_str = "12342345
                                           23453456
                                           34564567
                                           45675678
                                           23453456
                                           34564567
                                           45675678
                                           56786789";
        let two_row_and_column = parse(two_row_and_column_str);
        assert_eq!(two_row_and_column, repeat_grid(&single, 2, 2));

        let five_by_one_str = "12342345345645675678
                                    23453456456756786789
                                    34564567567867897891
                                    45675678678978918912";
        let five_by_one = parse(five_by_one_str);
        assert_eq!(five_by_one, repeat_grid(&single, 1, 5));
    }

    #[test]
    fn test2() {
        let grid = parse(EX);
        let grid = repeat_grid(&grid, 5, 5);
        let goal = (grid.num_rows() - 1, grid.num_columns() - 1);
        let actual = shortest_path(&grid, (0, 0), goal).unwrap();
        assert_eq!(actual, 315);
    }
}
