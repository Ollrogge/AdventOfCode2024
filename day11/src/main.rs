use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn digits_cnt(n: u64) -> usize {
    n.to_string().len()
}

fn num_split(n: u64) -> (u64, u64) {
    let ns = n.to_string();
    let n1 = ns[..ns.len() / 2].parse::<u64>().unwrap();
    let n2 = ns[ns.len() / 2..].parse::<u64>().unwrap();

    (n1, n2)
}

fn blink(nums: &mut HashMap<u64, u64>) {
    let mut to_insert = Vec::new();
    for (k, v) in nums.iter_mut() {
        if *v == 0x0 {
            continue;
        }

        if *k == 0 {
            to_insert.push((1, *v));
        } else {
            let dcount = digits_cnt(*k);
            if dcount % 2 == 0 {
                let (a, b) = num_split(*k);
                to_insert.push((a, *v));
                to_insert.push((b, *v));
            } else {
                to_insert.push((k * 2024, *v));
            }
        }

        *v = 0x0;
    }

    for x in to_insert {
        *nums.entry(x.0).or_insert(0) += x.1;
    }
}

fn solve(input: &str) {
    let parsed = parse_input(input);
    let mut nums = HashMap::new();
    parsed.iter().for_each(|x| {
        nums.insert(*x, 1 as u64);
    });

    for i in 0..75 {
        if i == 25 {
            let sum: u64 = nums.values().sum();
            println!("{}", sum);
        }
        blink(&mut nums);
    }

    let sum: u64 = nums.values().sum();
    println!("{}", sum);
}

fn main() {
    let input = include_str!("../input.txt");

    solve(&input);
}
