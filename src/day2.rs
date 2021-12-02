const INPUT: &str = include_str!("./assets/day2.txt");

pub fn solve() -> String {
    let data = parse(INPUT);
    format!("{}, {}", part1(&data), part2(&data))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

fn part1(input: &[Command]) -> i64 {
    let (pos, depth) = input.iter().fold((0, 0), |(pos, depth), cmd| match *cmd {
        Command::Forward(amount) => (pos + amount, depth),
        Command::Up(amount) => (pos, if depth > amount { depth - amount } else { 0 }),
        Command::Down(amount) => (pos, depth + amount),
    });
    pos * depth
}

fn part2(input: &[Command]) -> i64 {
    let (pos, _, depth) = input
        .iter()
        .fold((0, 0, 0), |(pos, aim, depth), cmd| match *cmd {
            Command::Forward(amount) => {
                let new_depth = depth + aim * amount;
                (pos + amount, aim, if new_depth > 0 { new_depth } else { 0 })
            }
            Command::Up(amount) => (pos, aim - amount, depth),
            Command::Down(amount) => (pos, aim + amount, depth),
        });
    pos * depth
}

fn parse(input: &str) -> Vec<Command> {
    use regex::Regex;

    let re = Regex::new(r#"(up|forward|down) (\d+)"#).unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let amount = cap[2].parse().expect("invalid number in input data");
            match &cap[1] {
                "forward" => Command::Forward(amount),
                "up" => Command::Up(amount),
                "down" => Command::Down(amount),
                _ => panic!("invalid command in input data"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
                        down 5
                        forward 8
                        up 3
                        down 8
                        forward 2";
    #[test]
    fn test_parse() {
        use Command::*;
        let expected = [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(&parse(INPUT), &expected);
    }
    #[test]
    fn test1() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 150);
    }
    #[test]
    fn test2() {
        let input = parse(INPUT);
        assert_eq!(part2(&input), 900);
    }
}
