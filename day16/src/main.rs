use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::u64;

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

fn solve(input: &str) {
    let (start_pos, grid) = parse_input(input);

    let mut work = BinaryHeap::new();
    let mut lowest_score = u64::MAX;
    let mut seats = HashSet::new();

    // more optimal would be to remove the path vec and backtrack from finish
    // based on the lowest_costs hashmap. Would just need to track prev in there as well
    let mut lowest_costs = HashMap::new();

    lowest_costs.insert((start_pos, RIGHT), 0);

    // Binary min heap
    work.push(Reverse((0, start_pos, RIGHT, vec![start_pos])));

    while let Some(Reverse((score, cur_pos, dir, path))) = work.pop() {
        if score > lowest_score {
            continue;
        }

        if grid[cur_pos.0 as usize][cur_pos.1 as usize] == 'E' {
            if score < lowest_score {
                seats.clear();
                lowest_score = score;
            }

            if score <= lowest_score {
                path.into_iter().for_each(|p| {
                    let _ = seats.insert(p);
                });
            }
            continue;
        }

        match lowest_costs.entry((cur_pos, dir)) {
            Entry::Occupied(mut e) => {
                if score > *e.get() {
                    continue;
                }

                e.insert(score);
            }
            Entry::Vacant(e) => {
                e.insert(score);
            }
        }

        for &new_dir in &[UP, DOWN, LEFT, RIGHT] {
            // no 180 degree turns
            if new_dir == (dir.0 * -1, dir.1 * -1) {
                continue;
            }
            if new_dir == dir {
                // Move forward: add cost of 1
                let new_pos = (cur_pos.0 + dir.0, cur_pos.1 + dir.1);
                if grid[new_pos.0 as usize][new_pos.1 as usize] != '#' {
                    let mut new_path = path.clone();
                    new_path.push(new_pos);
                    work.push(Reverse((score + 1, new_pos, new_dir, new_path.clone())));
                }
            } else {
                // Rotate: add cost of 1000 (no position change)
                work.push(Reverse((score + 1000, cur_pos, new_dir, path.clone())));
            }
        }
    }

    println!("{}", lowest_score);
    println!("{}", seats.len());
}

fn main() {
    let input = include_str!("../input.txt");

    solve(input);
}
