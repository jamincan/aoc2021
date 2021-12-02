const INPUT: &str = include_str!("./assets/day1.txt");

pub fn solve() -> String {
    let data = parse(INPUT);
    format!("{}, {}", part1(&data), part2(&data))
}

fn part1(input: &[u64]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(input: &[u64]) -> usize {
    input.windows(4).filter(|w| w[3] > w[0]).count()
}

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199
                         200
                         208
                         210
                         200
                         207
                         240
                         269
                         260
                         263";
    #[test]
    fn test1() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test2() {
        let input = parse(INPUT);
        assert_eq!(part2(&input), 5);
    }
}
