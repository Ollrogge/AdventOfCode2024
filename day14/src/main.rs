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

fn part2(input: &str) {}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
