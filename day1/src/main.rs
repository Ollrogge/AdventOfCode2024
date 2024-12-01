use std::cmp;
use std::vec::Vec;

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let nums: Vec<u64> = l
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .fold((Vec::new(), Vec::new()), |(mut vec1, mut vec2), (a, b)| {
            vec1.push(a);
            vec2.push(b);
            (vec1, vec2)
        })
}

fn part1(input: &str) {
    let (mut a, mut b) = parse_input(input);
    a.sort();
    b.sort();

    let res: u64 = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| cmp::max(a, b) - cmp::min(a, b))
        .sum();

    println!("Part1: {}", res);
}

fn part2(input: &str) {
    let (a, b) = parse_input(input);

    let res: u64 = a
        .iter()
        .map(|n| n * b.iter().filter(|&&x| x == *n).count() as u64)
        .sum();

    println!("Part2: {}", res);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
