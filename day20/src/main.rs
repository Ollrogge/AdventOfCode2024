use std::collections::HashSet;
use std::vec::Vec;

const UP: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);

fn in_grid_bounds(g: &Vec<Vec<char>>, p: (isize, isize)) -> bool {
    p.0 >= 0 && p.0 < g.len() as isize && p.1 >= 0 && p.1 < g[0].len() as isize
}

fn parse_input(input: &str) -> ((isize, isize), Vec<Vec<char>>) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut start_pos = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                start_pos = (row as isize, col as isize);
                break;
            }
        }
    }

    (start_pos, grid)
}

fn manhattan_distance(pos1: &(isize, isize), pos2: &(isize, isize)) -> isize {
    (pos2.0 - pos1.0).abs() + (pos2.1 - pos1.1).abs()
}

fn solve(input: &str) {
    let (start_pos, grid) = parse_input(input);
    let mut seen = HashSet::new();
    let mut work = Vec::new();
    let dirs = vec![UP, RIGHT, DOWN, LEFT];

    work.push((start_pos, Vec::new()));
    let mut full_path = None;

    while let Some((pos, mut path)) = work.pop() {
        path.push(pos);
        if grid[pos.0 as usize][pos.1 as usize] == 'E' {
            full_path = Some(path);
            break;
        }

        if grid[pos.0 as usize][pos.1 as usize] == '#' {
            continue;
        }

        if !seen.insert(pos) {
            continue;
        }

        dirs.iter()
            .for_each(|d| work.push(((pos.0 + d.0, pos.1 + d.1), path.clone())));
    }

    // map to coordinates
    let full_path: Vec<(isize, isize)> = full_path
        .unwrap()
        .iter()
        .map(|pos| (pos.1, pos.0))
        .collect();

    let mut res1 = 0x0;
    let mut res2 = 0x0;

    for (idx1, pos1) in full_path.iter().enumerate() {
        // definitely cant save 100 steps if pos2 isn't at least 100 steps away
        for (idx2, pos2) in full_path.iter().enumerate().skip(100) {
            let d = manhattan_distance(pos1, pos2);
            // steps saves = original path - the shorter path
            let steps_saved = idx2 as isize - idx1 as isize - d;
            if d <= 2 && steps_saved >= 100 {
                res1 += 1;
            }

            if d <= 20 && steps_saved >= 100 {
                res2 += 1;
            }
        }
    }

    println!("{}", res1);
    println!("{}", res2);
}

fn main() {
    let input = include_str!("../input.txt");

    solve(input);
}
