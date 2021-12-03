const INPUT: &str = include_str!("./assets/day3.txt");

pub fn solve() -> String {
    let data = get_numbers(INPUT);
    format!("{}, {}", part1(&data, 12), part2(&data, 12))
}

fn get_numbers(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter_map(|line| u64::from_str_radix(line.trim(), 2).ok())
        .collect()
}

/// Returns the number with the most common bit at each bit of the number.
fn most_common(data: &[u64], bits: u32) -> u64 {
    let mut most_common = 0;
    let half = (data.len() as u64 / 2) + (data.len() as u64 % 2); // Ceiling div
                                                                  // Calculate each bit separately
    for bit in (0..bits).rev() {
        most_common <<= 1;
        let one_count: u64 = data.iter().map(|num| (num >> bit) & 1).sum();
        if one_count >= half {
            most_common += 1
        }
    }
    most_common
}

fn least_common(most_common: u64, bits: u32) -> u64 {
    2u64.pow(bits) - most_common - 1
}

fn part1(data: &[u64], bits: u32) -> u64 {
    let gamma = most_common(data, bits);
    let epsilon = least_common(gamma, bits);
    gamma * epsilon
}

fn part2(data: &[u64], bits: u32) -> u64 {
    let mut o2_nums = data.to_vec();
    let mut co2_nums = data.to_vec();
    let mut o2_done = false;
    let mut co2_done = false;

    for bit in (0..bits).rev() {
        if !o2_done {
            let most_common = (most_common(&o2_nums, bits) >> bit) & 1;
            o2_nums = o2_nums
                .into_iter()
                .filter(|num| (*num >> bit) & 1 == most_common)
                .collect();
            if o2_nums.len() <= 1 {
                o2_done = true;
            }
        }
        if !co2_done {
            let least_common = (least_common(most_common(&co2_nums, bits), bits) >> bit) & 1;
            co2_nums = co2_nums
                .into_iter()
                .filter(|num| (*num >> bit) & 1 == least_common)
                .collect();
            if co2_nums.len() <= 1 {
                co2_done = true;
            }
        }
        if co2_done && o2_done {
            break;
        }
    }
    if o2_nums.len() != 1 {
        panic!("o2_nums not reduced to 1")
    }
    if co2_nums.len() != 1 {
        panic!("co2_nums not reduced to 1")
    }

    o2_nums[0] * co2_nums[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
                        11110
                        10110
                        10111
                        10101
                        01111
                        00111
                        11100
                        10000
                        11001
                        00010
                        01010";

    #[test]
    fn test_get_numbers() {
        let expected = [4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        assert_eq!(&get_numbers(INPUT), &expected);
    }

    #[test]
    fn test1() {
        let data = get_numbers(INPUT);
        assert_eq!(part1(&data, 5), 198);
    }

    #[test]
    fn test2() {
        let data = get_numbers(INPUT);
        assert_eq!(part2(&data, 5), 230);
    }
}
