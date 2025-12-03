use aoc_2025::utils;
use num_bigint::BigUint;
use num_traits::Zero;

fn main() {
    let part_1 = part1(utils::read_file("src/data/examples/03.txt"));
    println!("{part_1}");
    let part_2 = part2(utils::read_file("src/data/inputs/03.txt"));
    println!("{part_2}");
}

fn part1(input: String) -> i32 {
    let batteries: Vec<String> = input.lines().map(String::from).collect();
    let mut total: i32 = 0;
    for battery in batteries {
        let chars_to_nums: Vec<i32> = battery
            .chars()
            .map(|num_char| {
                num_char
                    .to_string()
                    .parse::<i32>()
                    .expect("failed to parse str")
            })
            .collect();
        let mut max: i32 = 0;
        // dbg!("{}", &chars_to_nums);
        for i in 0..chars_to_nums.len() {
            let mut j = i + 1;
            while j < chars_to_nums.len() {
                let current_num = chars_to_nums[i] * 10 + chars_to_nums[j];
                if chars_to_nums[i] * 10 + chars_to_nums[j] > max {
                    max = current_num;
                }
                j += 1;
            }
        }
        total += max;
        // println!("max: {}", max);
    }

    // dbg!("{}", nums);
    total
}

fn part2(input: String) -> BigUint {
    let batteries: Vec<String> = input.lines().map(String::from).collect();
    let mut total: BigUint = BigUint::zero();
    for battery in batteries {
        let digits: Vec<u8> = battery
            .chars()
            .map(|num_char| {
                num_char
                    .to_string()
                    .parse::<u8>()
                    .expect("failed to parse str")
            })
            .collect();

        let to_remove = digits.len().saturating_sub(12);
        let mut stack: Vec<u8> = Vec::with_capacity(digits.len());
        let mut removed = 0;

        // remove smallest digits that are blocking larger ones from moving forward in the seq
        for digit in digits {
            while removed < to_remove && !stack.is_empty() && *stack.last().unwrap() < digit {
                stack.pop();
                removed += 1;
            }
            stack.push(digit);
        }

        while removed < to_remove {
            stack.pop();
            removed += 1;
        }

        let big_num = stack.iter().fold(BigUint::zero(), |acc, &digit| {
            acc * 10u8 + BigUint::from(digit)
        });

        total += big_num;
    }

    total
}
