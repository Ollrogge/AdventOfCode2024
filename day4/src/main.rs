use std::vec::Vec;
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

static search: &str = "XMAS";
static search2: &str = "SAMX";

fn check_horizontal(inp: &Vec<Vec<char>>, row: usize, col: usize) -> u64 {
    if col + 4 > inp[0].len() {
        return 0;
    }

    let check: String = (col..col + 4).map(|c| inp[row][c]).collect();

    (check == search || check == search2) as u64
}

fn check_vertical(inp: &Vec<Vec<char>>, row: usize, col: usize) -> u64 {
    if row + 4 > inp.len() {
        return 0;
    }

    let check: String = (row..row + 4).map(|r| inp[r][col]).collect();

    (check == search || check == search2) as u64
}

fn check_diagonal(inp: &Vec<Vec<char>>, row: usize, col: usize) -> u64 {
    let mut cnt = 0x0;

    // down right
    if row + 4 <= inp.len() && col + 4 <= inp[0].len() {
        let check: String = (0..4).map(|i| inp[row + i][col + i]).collect();
        if check == search || check == search2 {
            cnt += 1;
        }
    }

    // down left
    if row + 4 <= inp.len() && col >= 3 {
        let check: String = (0..4).map(|i| inp[row + i][col - i]).collect();
        if check == search || check == search2 {
            cnt += 1;
        }
    }

    // no need to check up left and up right since we traverse the 2D vector from (0, 0) to (last_idx, last_idx)

    cnt
}

fn part1(input: &str) {
    let parsed = parse_input(input);
    let mut cnt = 0x0;
    for row in 0..parsed[0].len() {
        for col in 0..parsed.len() {
            cnt += check_diagonal(&parsed, row, col);
            cnt += check_horizontal(&parsed, row, col);
            cnt += check_vertical(&parsed, row, col);
        }
    }

    println!("{}", cnt);
}

fn check_cross(inp: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let row_len = inp.len();
    let col_len = inp[0].len();

    // check that all directions work
    if row > 0 && col > 0 && row < row_len - 1 && col < col_len - 1 {
        let upper_left = inp[row - 1][col - 1];
        let upper_right = inp[row - 1][col + 1];
        let down_left = inp[row + 1][col - 1];
        let down_right = inp[row + 1][col + 1];

        let cross1 =
            (upper_left == 'M' && down_right == 'S') || (upper_left == 'S' && down_right == 'M');
        let cross2 =
            (upper_right == 'M' && down_left == 'S') || (upper_right == 'S' && down_left == 'M');

        return cross1 && cross2;
    } else {
        return false;
    }
}

fn part2(input: &str) {
    let parsed = parse_input(input);
    let mut cnt = 0x0;

    for row in 0..parsed[0].len() {
        for col in 0..parsed.len() {
            if parsed[row][col] == 'A' {
                if check_cross(&parsed, row, col) {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
