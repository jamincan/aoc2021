pub fn get_input(day: usize) -> String {
    let filename = format!("./assets/day{}.txt", day);
    std::fs::read_to_string(filename).unwrap()
}
