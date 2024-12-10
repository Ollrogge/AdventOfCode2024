use std::collections::{HashMap, HashSet};

const UP: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<u64>>) {
    let grid: Vec<Vec<u64>> = input
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(10).unwrap() as u64).collect())
        .collect();

    let mut start_points = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 0 {
                start_points.push((row, col));
            }
        }
    }

    (start_points, grid)
}

fn get_trail_coordinates(
    row: isize,
    col: isize,
    prev: u64,
    grid: &Vec<Vec<u64>>,
    seen: &mut HashSet<(isize, isize)>,
) -> u64 {
    if row < 0 || row >= grid.len() as isize || col < 0 || col >= grid[0].len() as isize {
        return 0;
    }

    let cur = grid[row as usize][col as usize];

    if cur != prev + 1 {
        return 0;
    }

    if cur == 9 {
        seen.insert((row, col));
        return 1;
    }

    let mut sum = 0x0;
    for dir in &[UP, RIGHT, DOWN, LEFT] {
        let new_row = row + dir.0;
        let new_col = col + dir.1;

        sum += get_trail_coordinates(new_row, new_col, cur, grid, seen);
    }

    sum
}

fn solve(input: &str) {
    let (start_points, grid) = parse_input(input);

    let mut score_sum_1 = 0x0;
    let mut score_sum_2 = 0x0;
    for start in start_points {
        let mut seen = HashSet::new();
        vec![UP, RIGHT, DOWN, LEFT].iter().for_each(|dir| {
            score_sum_2 += get_trail_coordinates(
                dir.0 + start.0 as isize,
                dir.1 + start.1 as isize,
                0,
                &grid,
                &mut seen,
            );
        });

        score_sum_1 += seen.len();
    }

    println!("{}", score_sum_1);
    println!("{}", score_sum_2);
}

fn main() {
    let input = include_str!("../input.txt");

    solve(input);
}
