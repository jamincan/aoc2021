use std::collections::HashMap;

const INPUT: &str = include_str!("./assets/day14.txt");
type Pair = (char, char);

pub fn solve() -> String {
    let (template, mapping) = parse(INPUT);
    format!(
        "{}, {}",
        freq(template, &mapping, 10),
        freq(template, &mapping, 40)
    )
}

fn freq(template: &str, mapping: &HashMap<Pair, char>, depth: usize) -> usize {
    let chars: Vec<_> = template.chars().collect();

    // Determine the initial count of each pair
    let mut pair_counts: HashMap<Pair, usize> = HashMap::new();
    for pair in chars.windows(2) {
        let pair = (pair[0], pair[1]);
        *pair_counts.entry(pair).or_default() += 1;
    }

    // Each time we do an insertion of X into AB, AX and XB have the same count as AB
    // therefore we can repeatedly recalculate the pair counts from the previous
    // to determine final counts
    for _ in 0..depth {
        let mut new_counts = HashMap::new();
        for (pair, count) in pair_counts {
            match mapping.get(&pair) {
                Some(insertion) => {
                    *new_counts.entry((pair.0, *insertion)).or_default() += count;
                    *new_counts.entry((*insertion, pair.1)).or_default() += count;
                }
                None => *new_counts.entry(pair).or_default() += count,
            }
        }
        pair_counts = new_counts;
    }

    // Using the pairs, determine the count of the individual chars
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for ((first, _), count) in pair_counts {
        *char_counts.entry(first).or_default() += count;
    }
    // We have only counted the first character of each pair, so we also need to count the last char in the template
    let last = template.chars().last().unwrap();
    *char_counts.entry(last).or_default() += 1;

    // Calculate min and max
    let max = char_counts
        .iter()
        .reduce(|a, b| if b.1 > a.1 { b } else { a })
        .unwrap();
    let min = char_counts
        .iter()
        .reduce(|a, b| if b.1 < a.1 { b } else { a })
        .unwrap();
    max.1 - min.1
}

fn parse(input: &str) -> (&str, HashMap<(char, char), char>) {
    use regex::Regex;
    let re = Regex::new(r"(.)(.) -> (.)$").unwrap();

    let (template, pair_insertions) = match input.split_once("\r\n\r\n") {
        Some(data) => data,
        None => input.split_once("\n\n").unwrap(),
    };
    let pair_insertions: HashMap<_, _> = pair_insertions
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            let first = cap[1].chars().next().unwrap();
            let second = cap[2].chars().next().unwrap();
            let insertion = cap[3].chars().next().unwrap();
            ((first, second), insertion)
        })
        .collect();
    (template.trim(), pair_insertions)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EX: &str = "NNCB

                      CH -> B
                      HH -> N
                      CB -> H
                      NH -> C
                      HB -> C
                      HC -> B
                      HN -> C
                      NN -> C
                      BH -> H
                      NC -> B
                      NB -> B
                      BN -> B
                      BB -> N
                      BC -> B
                      CC -> N
                      CN -> C";

    #[test]
    fn tests() {
        let (template, mapping) = parse(EX);
        assert_eq!(freq(template, &mapping, 10), 1588);
        assert_eq!(freq(template, &mapping, 40), 2188189693529);
    }
}
