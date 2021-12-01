pub fn get_input(day: u8) -> String {
    let filename = format!("./assets/day{}.txt", day);
    std::fs::read_to_string(filename).unwrap()
}