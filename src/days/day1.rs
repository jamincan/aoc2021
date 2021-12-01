pub fn a(input: &str) -> usize {
    let numbers = parse(input);
    numbers.windows(2).filter(|w| w[1] > w[0]).count()
}

pub fn b(input: &str) -> usize {
    let numbers = parse(input);
    let windows: Vec<u64> = numbers.windows(3).map(|w| w.iter().sum()).collect();
    windows.windows(2).filter(|w| w[1] > w[0]).count()
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
    fn part_a() {
        assert_eq!(a(INPUT), 7);
    }

    #[test]
    fn part_b() {
        assert_eq!(b(INPUT), 5);
    }
}
