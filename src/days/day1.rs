use crate::tools::get_input;

pub fn a() -> usize {
    let numbers: Vec<u64> = get_input(1)
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    numbers.windows(2).filter(|w| w[1] > w[0]).count()
}

pub fn b() -> usize {
    let numbers: Vec<u64> = get_input(1)
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    let windows: Vec<u64> = numbers.windows(3).map(|w| w.iter().sum()).collect();
    windows.windows(2).filter(|w| w[1] > w[0]).count()
}
