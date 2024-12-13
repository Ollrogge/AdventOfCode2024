use regex::Regex;
use std::i64;

#[derive(Debug)]
struct Equation {
    da: (i64, i64),
    db: (i64, i64),
    goal: (i64, i64),
}

fn parse_input(input: &str) -> Vec<Equation> {
    let inp = input.split("\n\n");
    let re = Regex::new(r"\d+").unwrap();

    let mut equations = Vec::new();
    for eq in inp {
        let nums: Vec<i64> = re
            .find_iter(eq)
            .map(|v| v.as_str().parse::<i64>().unwrap())
            .collect();

        equations.push(Equation {
            da: (nums[0], nums[1]),
            db: (nums[2], nums[3]),
            goal: (nums[4], nums[5]),
        });
    }

    equations
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = extended_gcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

fn min_solution(eq: Equation) -> Option<u64> {
    let (ax, ay, cost_a) = (eq.da.0, eq.da.1, 3);
    let (bx, by, cost_b) = (eq.db.0, eq.db.1, 1);
    let (target_x, target_y) = (eq.goal.0, eq.goal.1);

    let denominator = ax * by - ay * bx;
    if denominator == 0 {
        return None; // Avoid division by zero
    }

    // Calculate ca and cb as i128 first
    let ca_numerator = target_x * by - target_y * bx;
    if ca_numerator % denominator != 0 {
        return None; // ca is not an integer
    }
    let ca = ca_numerator / denominator;

    let cb_numerator = target_x - ax * ca;
    if cb_numerator % bx != 0 {
        return None; // cb is not an integer
    }
    let cb = cb_numerator / bx;

    // Both ca and cb must be non-negative
    if ca >= 0 && cb >= 0 {
        Some((ca as u64) * cost_a + (cb as u64) * cost_b)
    } else {
        None
    }
}

fn part1(input: &str) {
    let equations = parse_input(input);

    let sum: u64 = equations
        .into_iter() // Parallel iteration over equations
        .filter_map(|e| {
            let res = min_solution(e);
            res
        })
        .sum(); // Sum up valid

    println!("{}", sum);
}

fn part2(input: &str) {
    let mut equations = parse_input(input);

    for eq in equations.iter_mut() {
        eq.goal.0 += 10000000000000;
        eq.goal.1 += 10000000000000;
    }

    let sum: u64 = equations
        .into_iter() // Parallel iteration over equations
        .filter_map(|e| {
            let res = min_solution(e);
            res
        })
        .sum(); // Sum up valid

    println!("{}", sum);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
