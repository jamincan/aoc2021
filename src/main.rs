use aoc2021::days::*;
use aoc2021::tools::get_input;

fn main() {
    let solutions = [(day1::a, day1::b)];

    for (day, (a, b)) in solutions.iter().enumerate() {
        let day = day + 1;
        let data = get_input(day);
        println!("Day {}: {}, {}", day, a(&data), b(&data));
    }
}
