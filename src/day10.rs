const INPUT: &str = include_str!("./assets/day10.txt");

pub fn solve() -> String {
    let lines = parse(INPUT);
    format!("{}, {}", part1(&lines), part2(&lines))
}

fn part1(lines: &[&str]) -> u64 {
    lines
        .iter()
        .map(|line| {
            let mut stack = Vec::new();
            let mut score = 0;
            for ch in line.chars() {
                if "([{<".contains(ch) {
                    stack.push(ch);
                    continue;
                } else if let Some(last) = stack.last() {
                    match (last, ch) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                            stack.pop();
                            continue;
                        }
                        _ => (),
                    }
                }
                score = match ch {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unimplemented!(),
                };
                break;
            }
            score
        })
        .sum()
}

fn part2(lines: &[&str]) -> u64 {
    let incomplete = lines.iter().filter_map(|line| {
        let mut stack = Vec::new();
        let mut invalid = false;
        for ch in line.chars() {
            if "([{<".contains(ch) {
                stack.push(ch);
                continue;
            } else if let Some(last) = stack.last() {
                match (last, ch) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                        stack.pop();
                        continue;
                    }
                    _ => (),
                }
            }
            invalid = true;
            break;
        }
        if invalid {
            None
        } else {
            Some(stack)
        }
    });
    let mut scores: Vec<_> = incomplete
        .map(|stack| {
            let mut score = 0;
            for bracket in stack.into_iter().rev() {
                score = score * 5
                    + match bracket {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unimplemented!(),
                    }
            }
            score
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(|l| l.trim()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
                          [(()[<>])]({[<{<<[]>>(
                          {([(<{}[<>[]}>{[]{[(<()>
                          (((({<>}<{<{<>}{[]{[]{}
                          [[<[([]))<([[{}[[()]]]
                          [{[{({}]{}}([{[{{{}}([]
                          {<[[]]>}<{[{[{[]{()[[[]
                          [<(<(<(<{}))><([]([]()
                          <{([([[(<>()){}]>(<<{{
                          <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test1() {
        assert_eq!(part1(&parse(EXAMPLE)), 26397);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(EXAMPLE)), 288957);
    }
}
