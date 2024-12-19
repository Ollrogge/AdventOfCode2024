use std::vec::Vec;

fn parse_input(input: &str) -> (Vec<&str>, Vec<Vec<&str>>) {
    let s = input.split("\n\n").collect::<Vec<&str>>();
    let patterns: Vec<&str> = s[0].split(",").map(|p| p.trim().to_string()).collect();

    let flags = s[1].lines().collect();

    (patterns, flags)
}

fn part1(input: &str) {}

fn part2(input: &str) {}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
