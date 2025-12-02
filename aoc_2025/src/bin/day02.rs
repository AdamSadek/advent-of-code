use aoc_2025::utils;

fn main() {
    let part_1 = part1(utils::split_hyphen(utils::split_commas("src/data/examples/02.txt")));
    println!("{part_1}");
    let part_2 = part2(utils::split_hyphen(utils::split_commas("src/data/inputs/02.txt")));
    println!("{part_2}");
}

fn part1(input: Vec<(String, String)>) -> usize {
    // dbg!("{}", &input);
    let mut invalid_ids: usize = 0;

    for (start_str, end_str) in input {
        println!("range: {}-{}", &start_str, &end_str);
        let range_start = start_str.parse::<usize>().expect("failed to parse string");
        let range_end = end_str.parse::<usize>().expect("failed to parse string");
        for current_number in range_start..=range_end {
            let number_str = current_number.to_string();
            if number_str.len() % 2 == 0 {
                let mid = number_str.len() / 2;
                let left_half = &number_str[..mid];
                let right_half = &number_str[mid..];
                if left_half == right_half {
                    invalid_ids += current_number;
                }
            }
        }
    }

    invalid_ids
}

fn part2(input: Vec<(String, String)>) -> usize {
    // dbg!("{}", &input);
    let mut invalid_ids: usize = 0;

    for (start_str, end_str) in input {
        println!("range: {}-{}", &start_str, &end_str);
        let range_start = start_str.parse::<usize>().expect("failed to parse string");
        let range_end = end_str.parse::<usize>().expect("failed to parse string");
        for current_number in range_start..=range_end {
            let number_str = current_number.to_string();
            let mut pattern_length = 1;
            while pattern_length < number_str.len() {
                let pattern = &number_str[..pattern_length];
                let repetitions = number_str.len() / pattern_length;

                let repeated_pattern = pattern.repeat(repetitions);

                if repeated_pattern == number_str {
                    invalid_ids += current_number;
                    break;
                }
                pattern_length += 1;

            }
        }
    }

    invalid_ids
}

