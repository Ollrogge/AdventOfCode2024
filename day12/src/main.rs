use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

const UP: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn in_grid_bounds(g: &Vec<Vec<char>>, p: (isize, isize)) -> bool {
    p.0 >= 0 && p.0 < g.len() as isize && p.1 >= 0 && p.1 < g[0].len() as isize
}

fn explore_region(
    grid: &Vec<Vec<char>>,
    typ: char,
    row: isize,
    col: isize,
    region: &mut HashSet<(isize, isize)>,
) {
    if !in_grid_bounds(grid, (row, col)) {
        return;
    }

    let c = grid[row as usize][col as usize];
    if c != typ {
        return;
    }

    if region.contains(&(row, col)) {
        return;
    }

    region.insert((row, col));

    vec![UP, RIGHT, DOWN, LEFT]
        .iter()
        .for_each(|dir| explore_region(grid, typ, row + dir.0, col + dir.1, region))
}

fn calculate_permiter(points: &Vec<(isize, isize)>) -> u64 {
    let mut perimeter = 0x0;

    for (row, col) in points.iter() {
        for (dx, dy) in vec![UP, RIGHT, LEFT, DOWN] {
            let neighbor = (row + dx, col + dy);
            if !points.contains(&neighbor) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn calculate_side_amt(points: &mut Vec<(isize, isize)>) -> u64 {
    // edge => same direction, adjacent
    let mut neighbors = HashMap::new();
    for p in points.iter() {
        for d in vec![UP, RIGHT, LEFT, DOWN] {
            let neighbor = (p.0 + d.0, p.1 + d.1);
            if !points.contains(&neighbor) {
                neighbors
                    .entry(d)
                    .or_insert(HashSet::new())
                    .insert(neighbor);
            }
        }
    }

    let mut sides = 0x0;
    for dir_neighbors in neighbors.values() {
        let mut seen_neighbors = HashSet::new();
        for n in dir_neighbors.iter() {
            if seen_neighbors.contains(n) {
                continue;
            }

            sides += 1;
            let mut work = vec![n.clone()];
            while let Some(p) = work.pop() {
                if !dir_neighbors.contains(&p) {
                    continue;
                }

                if seen_neighbors.contains(&p) {
                    continue;
                }

                seen_neighbors.insert(p);

                vec![UP, RIGHT, DOWN, LEFT]
                    .iter()
                    .for_each(|d| work.push((p.0 + d.0, p.1 + d.1)));
            }
        }
    }

    sides
}

fn get_regions(grid: Vec<Vec<char>>) -> Vec<Vec<(isize, isize)>> {
    let mut seen = HashSet::new();
    let mut regions = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !seen.contains(&(row as isize, col as isize)) {
                let mut region = HashSet::new();
                explore_region(
                    &grid,
                    grid[row][col],
                    row as isize,
                    col as isize,
                    &mut region,
                );

                for p in region.iter() {
                    seen.insert(p.clone());
                }

                regions.push(region.into_iter().collect::<Vec<(isize, isize)>>());
            }
        }
    }
    regions
}

fn part1(input: &str) {
    let grid = parse_input(input);
    let regions = get_regions(grid);

    let res: u64 = regions
        .iter()
        .map(|r| r.len() as u64 * calculate_permiter(r))
        .sum();

    println!("{}", res);
}

fn part2(input: &str) {
    let grid = parse_input(input);
    let mut regions = get_regions(grid);
    let res: u64 = regions
        .iter_mut()
        .map(|r| r.len() as u64 * calculate_side_amt(r))
        .sum();
    println!("{}", res);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
