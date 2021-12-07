const INPUT: &str = include_str!("./assets/day7.txt");

pub fn solve() -> String {
    let mut positions = parse(INPUT);
    format!("{}, {}", part1(&mut positions), part2(&positions))
}

fn fuel_expenditure_linear(sub_positions: &[i64], final_position: i64) -> i64 {
    sub_positions
        .iter()
        .map(|pos| (pos - final_position).abs())
        .sum()
}

fn fuel_expenditure_geometric(sub_positions: &[i64], final_position: i64) -> i64 {
    sub_positions
        .iter()
        .map(|pos| {
            let n = (pos - final_position).abs();
            n * (n + 1) / 2
        })
        .sum()
}

fn part1(positions: &mut [i64]) -> i64 {
    // Median positions always has minimum distance to all points
    positions.sort_unstable();
    let median = positions[positions.len() / 2];
    fuel_expenditure_linear(positions, median)
}

fn part2(positions: &[i64]) -> i64 {
    // For the geometric case, the mean is the minimum distance
    let mean = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
    let pos_low = mean.floor() as i64;
    let pos_hi = mean.ceil() as i64;
    let fuel_low = fuel_expenditure_geometric(positions, pos_low);
    let fuel_hi = fuel_expenditure_geometric(positions, pos_hi);
    fuel_low.min(fuel_hi)
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parse() {
        let expected = [16i64, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(&parse(INPUT), &expected);
    }

    #[test]
    fn test_fuel_expenditure_linear() {
        let positions = parse(INPUT);
        assert_eq!(fuel_expenditure_linear(&positions, 2), 37);
        assert_eq!(fuel_expenditure_linear(&positions, 1), 41);
        assert_eq!(fuel_expenditure_linear(&positions, 3), 39);
        assert_eq!(fuel_expenditure_linear(&positions, 10), 71);
    }

    #[test]
    fn test_fuel_expenditure_geometric() {
        let positions = parse(INPUT);
        assert_eq!(fuel_expenditure_geometric(&positions, 5), 168);
        assert_eq!(fuel_expenditure_geometric(&positions, 2), 206)
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&mut parse(INPUT)), 37);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(INPUT)), 168);
    }
}
