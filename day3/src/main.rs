use regex::Regex;
use std::vec::Vec;

enum Token {
    Do,
    Mul(u64, u64),
    Dont,
}

fn parse_input(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    for captures in re.captures_iter(input) {
        if captures.get(0).unwrap().as_str().starts_with("mul") {
            let num1: u64 = captures[1].parse().unwrap();
            let num2: u64 = captures[2].parse().unwrap();
            tokens.push(Token::Mul(num1, num2));
        } else if captures.get(0).unwrap().as_str() == "do()" {
            tokens.push(Token::Do);
        } else if captures.get(0).unwrap().as_str() == "don't()" {
            tokens.push(Token::Dont);
        }
    }

    tokens
}

fn part1(input: &str) {
    let tokens = parse_input(input);
    let mut res = 0x0;
    for token in tokens.iter() {
        match token {
            Token::Mul(n1, n2) => res += n1 * n2,
            _ => (),
        }
    }

    println!("Part1: {}", res);
}

fn part2(input: &str) {
    let tokens = parse_input(input);
    let mut res = 0x0;
    let mut is_enabled = true;
    for token in tokens.iter() {
        match token {
            Token::Do => is_enabled = true,
            Token::Mul(n1, n2) => {
                if is_enabled {
                    res += n1 * n2
                }
            }
            Token::Dont => is_enabled = false,
        }
    }

    println!("Part2: {}", res);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
