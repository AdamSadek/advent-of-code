use aoc_2025::utils;

struct Dial {
    value: i32,
}

impl Dial {
    fn new(value: i32) -> Self {
        Self { value: value }
    }

    fn increment(&mut self, num: i32) {
        // println!("before incr {}", self.value);
        self.value = (self.value + num).rem_euclid(100);
        // println!("after incr {}", self.value);
    }

    fn decrement(&mut self, num: i32) {
        // println!("before decr {}", self.value);
        self.value = (self.value - num).rem_euclid(100);
        // println!("after decr {}", self.value);
    }
}

fn part1(input: Vec<String>) -> i32 {
    // do smth
    dbg!("{}", &input);
    let mut dial: Dial = Dial::new(50);
    let mut count = 0;

    for mut instruction in input {
        let direction: char = instruction.remove(0);
        let num = instruction.parse::<i32>().expect("failed to prase str");

        println!("{direction}");
        match direction {
            'L' => dial.decrement(num),
            'R' => dial.increment(num),
            _ => print!("unknown direction"),
        }

        // println!("current val: {}", dial.value);

        if dial.value == 0 {
            count += 1;
        }
    }
    count
}
fn part2(input: Vec<String>) -> i32 {
    // do smth
    dbg!("{}", &input);
    let mut dial: Dial = Dial::new(50);
    let mut count = 0;

    for mut instruction in input {
        let direction: char = instruction.remove(0);
        let num = instruction.parse::<i32>().expect("failed to prase str");

        // println!("{direction}");
        match direction {
            'L' => {
                    for _ in 0..num {
                        dial.decrement(1);
                        if dial.value == 0 { count+=1; }
                    }   
                }
            'R' => {
                    for _ in 0..num {
                        dial.increment(1);
                        if dial.value == 0 { count+=1; }
                    }
                }
            _ => print!("unknown direction"),
        }
    }
    count
}

fn main() {
    let part_1 = part1(utils::parse_line("src/data/inputs/01.txt"));
    println!("{part_1}");
    let part_2 = part2(utils::parse_line("src/data/inputs/01.txt"));
    println!("{part_2}");
}
