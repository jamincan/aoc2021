const INPUT: &str = include_str!("./assets/day6.txt");

pub fn solve() -> String {
    let fish = parse(INPUT);
    format!("{}, {}", part1(&fish, 80), part1(&fish, 256))
}

fn part1(input: &[u8], final_day: usize) -> usize {
    // Model the school of fish as an array with the index
    // representing the number of days to reproducing, and the
    // value the number of fish on that day.
    let mut fish_school = [0; 9];
    for number in input {
        fish_school[*number as usize] += 1;
    }

    for day in 0..final_day {
        let count = fish_school[day % 9];
        fish_school[(day + 7) % 9] += count;
    }

    fish_school.iter().sum()
}

fn parse(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_parse() {
        assert_eq!(&parse(INPUT), &[3u8, 4, 3, 1, 2]);
    }

    #[test]
    fn test1() {
        let data = parse(INPUT);
        assert_eq!(part1(&data, 80), 5934);
        assert_eq!(part1(&data, 256), 26984457539);
    }
}
