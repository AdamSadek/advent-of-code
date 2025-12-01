pub fn parse_line(file: &str) -> Vec<String> {
    let input = std::fs::read_to_string(file).expect("failed to read file");
    input.split_whitespace().map(String::from).collect()
}
