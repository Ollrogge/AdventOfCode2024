fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

fn increasing_cnt(levels: &Vec<u64>) -> u64 {
    levels
        .windows(2)
        .filter(|w| {
            if w[0] < w[1] {
                return w[1] - w[0] <= 3;
            } else {
                false
            }
        })
        .count() as u64
}

fn increasing_cnt_ignore1(levels: &Vec<u64>) -> u64 {
    let mut v = levels.clone();
    if let Some(x) = levels.windows(2).position(|w| w[0] >= w[1]) {
        v.remove(x);
        increasing_cnt(&v)
    } else {
        increasing_cnt(levels)
    }
}

fn decreasing_cnt(levels: &Vec<u64>) -> u64 {
    levels
        .windows(2)
        .filter(|w| {
            if w[1] < w[0] {
                return w[0] - w[1] <= 3;
            } else {
                false
            }
        })
        .count() as u64
}

fn decreasing_cnt_ignore1(levels: &Vec<u64>) -> u64 {
    let mut v = levels.clone();
    if let Some(x) = levels.windows(2).position(|w| w[1] >= w[0]) {
        v.remove(x + 1);
        decreasing_cnt(&v)
    } else {
        decreasing_cnt(levels)
    }
}

fn part1(input: &str) {
    let parsed = parse_input(input);

    let cnt = parsed
        .iter()
        .filter(|x| {
            increasing_cnt(x) == (x.len() as u64 - 1) || decreasing_cnt(x) == (x.len() as u64 - 1)
        })
        .count();

    println!("{}", cnt)
}

fn part2(input: &str) {
    let parsed = parse_input(input);

    let cnt = parsed
        .iter()
        .filter(|x| {
            increasing_cnt_ignore1(x) >= (x.len() as u64 - 2)
                || decreasing_cnt_ignore1(x) >= (x.len() as u64 - 2)
        })
        .count();

    println!("{}", cnt)
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
