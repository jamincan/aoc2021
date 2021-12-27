use aoc2021::*;

fn main() {
    let solutions = [
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
        day8::solve,
        day9::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
    ];

    for (day, solve) in solutions.iter().enumerate() {
        let day = day + 1;
        println!("Day {}: {}", day, solve());
    }
}
