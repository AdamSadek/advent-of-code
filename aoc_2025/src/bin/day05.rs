use std::{collections::HashSet, error::Error, fs, process::id};

fn main() {
    // let part_1 = part1("src/data/inputs/05.txt");
    // println!("{part_1}");
    let part_2 = part2("src/data/inputs/05.txt");
    println!("{part_2}");
}

#[derive(Debug)]
struct Ingredients {
    ids: Vec<(u128, u128)>,
    current_ingr: Vec<u128>,
}

impl Ingredients {
    fn new() -> Self {
        Self {
            ids: Vec::new(),
            current_ingr: Vec::new(),
        }
    }
}

fn part1(input: &str) -> u128 {
    let ingr: Result<Ingredients, std::io::Error> = parse_input(input);

    dbg!("{}", &ingr);
    let mut seen: HashSet<u128> = HashSet::new();

    for (low, high) in &ingr.as_ref().unwrap().ids {
        for num in &ingr.as_ref().unwrap().current_ingr {
            if num >= low && num <= high {
                seen.insert(*num);
            }
        }
    }
    seen.len() as u128
}

fn part2(input: &str) -> u128 {
    let ingr: Result<Ingredients, std::io::Error> = parse_input(input);

    dbg!("{}", &ingr);
    let mut ranges = ingr.unwrap().ids;
    ranges.sort_by_key(|(low, _)| *low);

    let (mut low, mut high) = ranges[0];
    let mut total = 0;

    for (start, end) in ranges.into_iter().skip(1) {
        if start > high {
            total += high - low + 1;
            low = start;
            high = end;
        } else {
            if end > high {
                high = end;
            }
        }
    }
    total += high - low + 1;

    total
}

fn parse_input(file: &str) -> Result<Ingredients, std::io::Error> {
    let mut ingr = Ingredients::new();
    let input = fs::read_to_string(file).expect("failed to read file");

    let mut second_set = false;
    for line in input.lines() {
        if line.is_empty() {
            second_set = true;
            continue;
        }

        if second_set {
            ingr.current_ingr
                .push(line.parse::<u128>().expect("failed to parse input"));
        } else {
            let mut id_range_it = line.split('-');

            let lower_id = id_range_it
                .next()
                .unwrap()
                .parse::<u128>()
                .expect("failed to parse");
            let higher_id = id_range_it
                .next()
                .unwrap()
                .parse::<u128>()
                .expect("failed to parse");

            let ids: (u128, u128) = (lower_id, higher_id);

            ingr.ids.push(ids);
        }
    }
    Ok(ingr)
}
