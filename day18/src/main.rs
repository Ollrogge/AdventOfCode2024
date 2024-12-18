use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::u64;
use std::vec::Vec;

const UP: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    let re = Regex::new(r"\d+").unwrap();
    let mut parsed = Vec::new();
    for l in input.lines() {
        let caps = re.find_iter(l).map(|m| m.as_str()).collect::<Vec<&str>>();
        parsed.push((
            // y = row
            caps[1].parse::<isize>().unwrap(),
            // x = col
            caps[0].parse::<isize>().unwrap(),
        ));
    }

    parsed
}

fn in_grid_bounds(g: &Vec<Vec<char>>, p: (isize, isize)) -> bool {
    p.0 >= 0 && p.0 < g.len() as isize && p.1 >= 0 && p.1 < g[0].len() as isize
}

fn find_min_steps(grid: &Vec<Vec<char>>) -> u64 {
    let mut work = BinaryHeap::new();
    let mut min_steps = u64::MAX;

    let mut seen = HashSet::new();
    let dirs = vec![UP, RIGHT, DOWN, LEFT];

    work.push(Reverse((0 as u64, (0 as isize, 0 as isize), RIGHT)));

    while let Some(Reverse((steps, pos, dir))) = work.pop() {
        if !in_grid_bounds(&grid, pos) {
            continue;
        }

        if grid[pos.0 as usize][pos.1 as usize] == '#' {
            continue;
        }

        if seen.contains(&(dir, pos)) {
            continue;
        }

        seen.insert((dir, pos));

        if pos == (70, 70) && steps < min_steps {
            min_steps = steps;
        }

        dirs.iter().for_each(|d| {
            let new_pos = (pos.0 + d.0, pos.1 + d.1);
            work.push(Reverse((steps + 1, new_pos, *d)));
        });
    }

    min_steps
}

// (0,0) - (70,70)
fn part1(input: &str) {
    let corrupted = parse_input(input);
    let mut grid = vec![vec!['.'; 71]; 71];

    for p in corrupted.iter().take(1024) {
        grid[p.0 as usize][p.1 as usize] = '#';
    }

    let min_steps = find_min_steps(&grid);

    println!("{}", min_steps);
}

fn part2(input: &str) {
    let corrupted = parse_input(input);
    let base_grid = vec![vec!['.'; 71]; 71];

    // do binary search to find the pos which closes the path
    let mut range = (0, corrupted.len());
    loop {
        if range.0 == range.1 {
            println!(
                "{},{}",
                corrupted[range.0 as usize].1, corrupted[range.0 as usize].0
            );
            break;
        }

        let mut grid = base_grid.clone();
        let range_mid: usize = range.0 + (range.1 - range.0) / 2;
        for (i, p) in corrupted.iter().enumerate() {
            grid[p.0 as usize][p.1 as usize] = '#';
            if i == range_mid {
                break;
            }
        }
        let min_steps = find_min_steps(&grid);
        if min_steps == u64::MAX {
            range = (range.0, range_mid);
        } else {
            range = (range_mid + 1, range.1);
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
