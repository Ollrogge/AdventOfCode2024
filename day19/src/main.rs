use std::vec::Vec;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let s = input.split("\n\n").collect::<Vec<&str>>();
    let patterns: Vec<&str> = s[0].split(",").map(|p| p.trim()).collect();

    let flags = s[1].lines().collect();

    (flags, patterns)
}

fn solve(input: &str) {
    let (flags, patterns) = parse_input(input);

    let mut possibilities: u64 = 0;
    let mut possible: u64 = 0x0;

    for flag in flags {
        let n = flag.len();
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;

        for i in 1..(n + 1) {
            for pattern in patterns.iter() {
                let l = pattern.len();
                if i >= l && &flag[i - l..i] == *pattern {
                    dp[i] += dp[i - l];
                }
            }
        }

        possibilities += dp[n];
        if dp[n] > 0 {
            possible += 1;
        }
    }

    println!("{}", possible);
    println!("{}", possibilities);
}

fn main() {
    let input = include_str!("../input.txt");

    solve(input);
}
