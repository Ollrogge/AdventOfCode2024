use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_antennas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(i64, i64)>> {
    let mut antennas = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let c = grid[row][col];
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((row as i64, col as i64));
            }
        }
    }

    antennas
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn in_grid_bounds(g: &Vec<Vec<char>>, p: (i64, i64)) -> bool {
    p.0 >= 0 && p.0 < g.len() as i64 && p.1 >= 0 && p.1 < g[0].len() as i64
}

fn part1(input: &str) {
    let grid = parse_grid(input);
    let antennas = parse_antennas(&grid);

    let mut antinodes = HashSet::new();
    for antennas_x in antennas.values() {
        for (p1, p2) in antennas_x.iter().tuple_combinations() {
            // from p1 to p2
            let d1 = (p2.0 - p1.0, p2.1 - p1.1);
            // from p2 to p1
            let d2 = (p1.0 - p2.0, p1.1 - p2.1);

            let antinode1 = (p2.0 + d1.0, p2.1 + d1.1);
            let antinode2 = (p1.0 + d2.0, p1.1 + d2.1);

            if in_grid_bounds(&grid, antinode1) {
                antinodes.insert(antinode1);
            }
            if in_grid_bounds(&grid, antinode2) {
                antinodes.insert(antinode2);
            }
        }
    }

    println!("{}", antinodes.len());
}

fn part2(input: &str) {
    let grid = parse_grid(input);
    let antennas = parse_antennas(&grid);

    let mut antinodes = HashSet::new();
    for antennas_x in antennas.values() {
        for (p1, p2) in antennas_x.iter().tuple_combinations() {
            // from p1 to p2
            let d1 = (p2.0 - p1.0, p2.1 - p1.1);
            // from p2 to p1
            let d2 = (p1.0 - p2.0, p1.1 - p2.1);

            let mut cnt = 0x0;
            loop {
                let antinode1 = (p2.0 + d1.0 * cnt, p2.1 + d1.1 * cnt);
                let antinode2 = (p1.0 + d2.0 * cnt, p1.1 + d2.1 * cnt);

                cnt += 1;

                if !in_grid_bounds(&grid, antinode1) && !in_grid_bounds(&grid, antinode2) {
                    break;
                }
                if in_grid_bounds(&grid, antinode1) {
                    antinodes.insert(antinode1);
                }
                if in_grid_bounds(&grid, antinode2) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
