use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"-?\d+").unwrap();

    let mut robots = Vec::new();
    for li in input.lines() {
        let nums: Vec<i64> = re
            .find_iter(li)
            .map(|v| v.as_str().parse::<i64>().unwrap())
            .collect();

        robots.push(Robot {
            pos: (nums[0], nums[1]),
            vel: (nums[2], nums[3]),
        });
    }

    robots
}

fn get_quadrant(pos: (i64, i64)) -> Option<u64> {
    if pos.0 > WIDTH / 2 && pos.1 < HEIGHT / 2 {
        return Some(1);
    }

    if pos.0 < WIDTH / 2 && pos.1 < HEIGHT / 2 {
        return Some(2);
    }

    if pos.0 < WIDTH / 2 && pos.1 > HEIGHT / 2 {
        return Some(3);
    }

    if pos.0 > WIDTH / 2 && pos.1 > HEIGHT / 2 {
        return Some(4);
    }

    None
}

fn part1(input: &str) {
    let mut robots = parse_input(input);

    robots.iter_mut().for_each(|r| {
        r.pos.0 = (r.pos.0 + r.vel.0 * 100).rem_euclid(WIDTH);
        r.pos.1 = (r.pos.1 + r.vel.1 * 100).rem_euclid(HEIGHT);
    });

    let mut cnts = HashMap::new();
    robots
        .iter()
        .filter_map(|x| get_quadrant(x.pos))
        .for_each(|x| *cnts.entry(x).or_insert(0) += 1);

    let res: u64 = cnts.values().product();

    println!("{}", res);
}

fn get_entropy(grid: &Vec<Vec<char>>, pos: (i64, i64)) -> u64 {
    let mut cnt = 0x0;
    for i in -1..=1 {
        for j in -1..=1 {
            if grid[(pos.1 + i).rem_euclid(HEIGHT) as usize][(pos.0 + j).rem_euclid(WIDTH) as usize]
                == '#'
            {
                cnt += 1;
            }
        }
    }
    cnt
}

fn print_tree(input: &str, steps: i64) {
    let mut robots = parse_input(input);
    robots.iter_mut().for_each(|r| {
        r.pos.0 = (r.pos.0 + r.vel.0 * steps).rem_euclid(WIDTH);
        r.pos.1 = (r.pos.1 + r.vel.1 * steps).rem_euclid(HEIGHT);
    });

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];

    robots
        .iter()
        .for_each(|r| grid[r.pos.1 as usize][r.pos.0 as usize] = '#');

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            print!("{}", grid[row as usize][col as usize]);
        }
        println!("");
    }
}

fn part2(input: &str) {
    let mut robots = parse_input(input);
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];

    robots
        .iter()
        .for_each(|r| grid[r.pos.1 as usize][r.pos.0 as usize] = '#');

    let mut max_entropy = 0x0;
    let mut steps = 0x0;
    // assumption: christmas tree has a lot of robots close to each other, so the entropy is very high of the picture
    for i in 0..WIDTH * HEIGHT {
        for r in robots.iter_mut() {
            grid[r.pos.1 as usize][r.pos.0 as usize] = '.';
            r.pos.0 = (r.pos.0 + r.vel.0).rem_euclid(WIDTH);
            r.pos.1 = (r.pos.1 + r.vel.1).rem_euclid(HEIGHT);
            grid[r.pos.1 as usize][r.pos.0 as usize] = '#';
        }

        let sum: u64 = robots.iter().map(|r| get_entropy(&grid, r.pos)).sum();

        if sum > max_entropy {
            max_entropy = sum;
            steps = i;
        }
    }

    print_tree(input, steps + 1);

    println!("Steps: {}", steps + 1);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
