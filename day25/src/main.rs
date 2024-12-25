use std::collections::HashSet;
fn parse_input(input: &str) -> (u64, Vec<Vec<u64>>, Vec<Vec<u64>>) {
    let schematics = input.split("\n\n").collect::<Vec<&str>>();

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let mut col_height = 0x0;

    for s in schematics.into_iter() {
        let grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        if col_height == 0 {
            col_height = grid.len() as u64;
        }

        let is_lock = grid[0].iter().all(|&x| x == '#');

        let mut pin_heights = Vec::new();
        for col in 0..grid[0].len() {
            let mut pin_height = 0x0;
            for row in 0..grid.len() {
                if grid[row][col] == '#' {
                    pin_height += 1;
                }
            }

            pin_heights.push(pin_height);
        }

        if is_lock {
            locks.push(pin_heights);
        } else {
            keys.push(pin_heights);
        }
    }

    (col_height, keys, locks)
}

fn part1(input: &str) {
    let (col_height, keys, locks) = parse_input(input);
    let mut res = 0x0;
    for k in keys.iter() {
        for l in locks.iter() {
            if k.iter()
                .zip(l.iter())
                .all(|(&x1, &x2)| x1 + x2 <= col_height)
            {
                res += 1;
            }
        }
    }

    println!("{}", res);
}

fn part2(input: &str) {}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
