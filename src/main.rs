use aoc2021::*;

fn main() {
    let solutions = [day1::solve, day2::solve];

    for (day, solve) in solutions.iter().enumerate() {
        let day = day + 1;
        println!("Day {}: {}", day, solve());
    }
}
