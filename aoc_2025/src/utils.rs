pub fn read_file(file: &str) -> String {
    std::fs::read_to_string(file).expect("failed to read file")
}

pub fn parse_line(file: &str) -> Vec<String> {
    let input = read_file(file);
    input.split_whitespace().map(String::from).collect()
}

pub fn split_commas(file: &str) -> Vec<String> {
    let input = read_file(file);
    input.split(',').map(String::from).collect()
}

pub fn split_hyphen(input: Vec<String>) -> Vec<(String, String)> {
    input
        .into_iter()
        .map(|line| {
            let mut id_split: Vec<String> = line.split('-').map(String::from).collect();
            (id_split.remove(0), id_split.remove(0))
            // removes first elem; [1,2] -> [1] and then we call remove again on the first idx since it shifted
        })
        .collect()
}
