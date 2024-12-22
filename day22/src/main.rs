use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec::Vec;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

fn calc_round(mut num: u64) -> u64 {
    num = (num ^ (num * 0x40)) % 16777216;
    num = (num ^ (num / 0x20)) % 16777216;
    num = (num ^ (num * 2048)) % 16777216;

    num
}

fn part1(input: &str) {
    let nums = parse_input(input);

    let mut sum = 0x0;
    for mut num in nums.into_iter() {
        for _ in 0..2000 {
            num = calc_round(num);
        }

        sum += num;
    }

    println!("{}", sum);
}

fn part2(input: &str) {
    let nums = parse_input(input);
    let mut seqs = HashMap::new();

    for start_num in nums.into_iter() {
        let mut num = start_num;
        let mut buyer = vec![(num % 10) as i64];
        for _ in 0..2000 {
            num = calc_round(num);
            buyer.push((num % 10) as i64);
        }

        let mut seen = HashSet::new();
        for window in buyer.windows(5) {
            if let [a, b, c, d, e] = window {
                let seq = (b - a, c - b, d - c, e - d);
                // monkey sells the first time it sees a sequence and then not again
                // for this buyer
                if seen.insert(seq) {
                    *seqs.entry(seq).or_insert(0) += e;
                }
            }
        }
    }

    let max = seqs.values().max().unwrap();

    println!("{}", max);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
