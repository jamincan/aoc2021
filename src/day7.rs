use std::collections::HashMap;

use arrayvec::ArrayVec;
use itertools::Itertools;

const INPUT: &str = include_str!("./assets/day7.txt");

pub fn solve() -> String {
    let readings = parse(INPUT);
    format!("{}, {}", part1(&readings), part2(&readings))
}

type Reading<'r> = ([&'r str; 10], [&'r str; 4]);

fn part1(input: &[Reading]) -> u64 {
    input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|d| [2, 3, 4, 7].contains(&d.len()))
                .count() as u64
        })
        .sum()
}

fn part2(input: &[Reading]) -> u64 {
    input
        .iter()
        .map(|(examples, output)| {
            let mapping = build_map(examples);
            translate(output, &mapping)
        })
        .sum()
}

fn parse(input: &str) -> Vec<Reading> {
    input
        .lines()
        .map(|line| {
            let (examples, output) = line.trim().split_once('|').unwrap();
            let examples: ArrayVec<_, 10> = examples.split_ascii_whitespace().collect();
            let output: ArrayVec<_, 4> = output.split_ascii_whitespace().collect();
            let examples = examples.into_inner().unwrap();
            let output = output.into_inner().unwrap();
            (examples, output)
        })
        .collect()
}

// Takes the example digits and figures out what they represent
fn build_map(digits: &[&str; 10]) -> HashMap<String, u8> {
    let mut val_to_wires = vec![String::new(); 10];
    let mut len_six = ArrayVec::<_, 3>::new();
    let mut len_five = ArrayVec::<_, 3>::new();

    // Extract 0, 4, 7, and 8 and then separate out the ones with five segments from the ones with six.
    // Sort the wiring in the digits so they are alphabetical at the same time.
    for digit in digits.iter() {
        let digit: String = digit.chars().sorted_unstable().collect();
        match digit.len() {
            2 => val_to_wires[1] = digit,
            3 => val_to_wires[7] = digit,
            4 => val_to_wires[4] = digit,
            7 => val_to_wires[8] = digit,
            5 => len_five.push(digit),
            6 => len_six.push(digit),
            _ => unimplemented!(),
        };
    }

    // We can decipher which wires are c & f and which are e & g, which can be used to deduce the remaining numbers
    let bcdf: ArrayVec<_, 4> = val_to_wires[4].chars().collect();
    let acf: ArrayVec<_, 3> = val_to_wires[7].chars().collect();
    let cf: ArrayVec<_, 2> = val_to_wires[1].chars().collect();
    let eg: ArrayVec<_, 2> = val_to_wires[8]
        .chars()
        .filter(|ch| !bcdf.contains(ch) && !acf.contains(ch))
        .collect();

    for digit in len_five {
        let has_cf = cf.iter().all(|ch| digit.contains(*ch));
        let has_eg = eg.iter().all(|ch| digit.contains(*ch));
        if has_cf {
            val_to_wires[3] = digit;
        } else if has_eg {
            val_to_wires[2] = digit;
        } else {
            val_to_wires[5] = digit;
        }
    }

    for digit in len_six {
        let has_cf = cf.iter().all(|ch| digit.contains(*ch));
        let has_eg = eg.iter().all(|ch| digit.contains(*ch));
        if has_cf && has_eg {
            val_to_wires[0] = digit;
        } else if has_cf {
            val_to_wires[9] = digit;
        } else {
            val_to_wires[6] = digit;
        }
    }

    val_to_wires
        .into_iter()
        .enumerate()
        .map(|(val, wires)| (wires, val as u8))
        .collect()
}

fn translate(digits: &[&str], mapping: &HashMap<String, u8>) -> u64 {
    let mut result = 0;
    for (pos, digit) in digits.iter().rev().enumerate() {
        let digit: String = digit.chars().sorted_unstable().collect();
        let value = *mapping
            .get(&digit)
            .expect("output has digit that isn't in the examples") as u64;
        result += 10u64.pow(pos as u32) * value;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const EXAMPLE2: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_parse() {
        let expected = vec![(
            [
                "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb",
                "ab",
            ],
            ["cdfeb", "fcadb", "cdfeb", "cdbaf"],
        )];
        assert_eq!(&parse(EXAMPLE1), &expected);
    }

    #[test]
    fn test_build_map() {
        let input = parse(EXAMPLE1);
        let (examples, _) = input[0];
        let expected = HashMap::from([
            ("abcdefg".into(), 8),
            ("bcdef".into(), 5),
            ("acdfg".into(), 2),
            ("abcdf".into(), 3),
            ("abd".into(), 7),
            ("abcdef".into(), 9),
            ("bcdefg".into(), 6),
            ("abef".into(), 4),
            ("abcdeg".into(), 0),
            ("ab".into(), 1),
        ]);
        assert_eq!(build_map(&examples), expected);
    }

    #[test]
    fn test_translate() {
        let input = parse(EXAMPLE1);
        let (examples, output) = input[0];
        let mapping = build_map(&examples);
        assert_eq!(translate(&output, &mapping), 5353);
    }

    #[test]
    fn test1() {
        assert_eq!(part1(&parse(EXAMPLE2)), 26);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(EXAMPLE2)), 61229);
    }
}
