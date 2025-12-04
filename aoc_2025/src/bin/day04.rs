use aoc_2025::utils;

fn main() {
    let part_1 = part1(utils::parse_line("src/data/inputs/04.txt"));
    println!("{part_1}");
    let part_2 = part2(utils::parse_line("src/data/inputs/04.txt"));
    println!("{part_2}");
}

fn part1(input: Vec<String>) -> i32 {
    let mut grid = make_str_to_char(input);
    if grid.is_empty() {
        return 0;
    }

    let original = grid.clone();
    let height = original.len();
    let width = original[0].len();
    let mut ans = 0;

    for i in 0..height {
        for j in 0..width {
            let count = count_neighbors(&original, i, j, '@');
            if count < 4 && original[i][j] == '@' {
                ans += 1;
                grid[i][j] = '.';
            }
        }
    }
    ans
}

fn part2(input: Vec<String>) -> i32 {
    let mut grid = make_str_to_char(input);
    if grid.is_empty() {
        return 0;
    }

    let mut total = 0;
    loop {
        let original = grid.clone();
        let height = original.len();
        let width = original[0].len();
        let mut removed = 0;

        for i in 0..height {
            for j in 0..width {
                let count = count_neighbors(&original, i, j, '@');
                if count < 4 && original[i][j] == '@' {
                    grid[i][j] = '.';
                    removed += 1;
                }
            }
        }

        if removed == 0 {
            break;
        }
        total += removed;
    }

    total
}

fn make_str_to_char(input: Vec<String>) -> Vec<Vec<char>> {
    input.into_iter().map(|row| row.chars().collect()).collect()
}

fn is_valid_pos(i: isize, j: isize, height: usize, width: usize) -> bool {
    i >= 0 && (i as usize) < height && j >= 0 && (j as usize) < width
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_neighbors(grid: &[Vec<char>], i: usize, j: usize, target: char) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    DIRECTIONS
        .iter()
        .filter(|&&(di, dj)| {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            is_valid_pos(ni, nj, height, width) && grid[ni as usize][nj as usize] == target
        })
        .count()
}
