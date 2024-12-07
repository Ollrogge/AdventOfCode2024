use core::num;
use regex::Regex;
use std::vec::Vec;

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .filter_map(|l| {
            let numbers: Vec<u64> = re
                .find_iter(l)
                .map(|l| l.as_str().parse::<u64>().unwrap())
                .collect();

            Some((numbers[0], numbers[1..].to_vec()))
        })
        .collect()
}

fn try_calc(goal: u64, acc: u64, vals: &[u64], part2: bool) -> bool {
    if vals.len() == 0 {
        return acc == goal;
    } else {
        return try_calc(goal, acc + vals[0], &vals[1..], part2)
            || try_calc(goal, acc * vals[0], &vals[1..], part2)
            || (part2
                && try_calc(
                    goal,
                    format!("{}{}", acc, vals[0]).parse::<u64>().unwrap(),
                    &vals[1..],
                    part2,
                ));
    }
}

fn part1(input: &str) {
    let parsed = parse_input(input);

    let mut total = 0;
    for (goal, nums) in parsed {
        if try_calc(goal, 0, &nums, false) {
            total += goal;
        }
    }

    println!("{}", total);
}

fn part2(input: &str) {
    let parsed = parse_input(input);

    let mut total = 0;
    for (goal, nums) in parsed {
        if try_calc(goal, 0, &nums, true) {
            total += goal;
        }
    }

    println!("{}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
