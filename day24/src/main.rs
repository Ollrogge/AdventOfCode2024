use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    XOR,
    AND,
    OR,
}

impl From<&str> for Operation {
    fn from(val: &str) -> Self {
        match val {
            "XOR" => Self::XOR,
            "AND" => Self::AND,
            "OR" => Self::OR,
            _ => panic!("Unknown operation: {}", val),
        }
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    r1: &'a str,
    r2: &'a str,
    op: Operation,
    r_res: &'a str,
}

fn parse_input<'a>(input: &'a str) -> (HashMap<&'a str, u64>, Vec<Instruction<'a>>) {
    let split = input.split("\n\n").collect::<Vec<&str>>();

    let mut vals = HashMap::new();
    let re1 = Regex::new(r"(\w+):\s(\d)").unwrap();

    for l in split[0].lines() {
        let caps = re1.captures(l).unwrap();
        vals.insert(
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str().parse::<u64>().unwrap(),
        );
    }

    let mut insts = Vec::new();
    let re2 = Regex::new(r"(\w+)\s(\w+)\s(\w+)\s->\s(\w+)").unwrap();

    for l in split[1].lines() {
        let caps = re2.captures(l).unwrap();
        insts.push(Instruction {
            r1: caps.get(1).unwrap().as_str(),
            r2: caps.get(3).unwrap().as_str(),
            op: Operation::from(caps.get(2).unwrap().as_str()),
            r_res: caps.get(4).unwrap().as_str(),
        })
    }

    (vals, insts)
}

fn do_op(r1: u64, r2: u64, op: Operation) -> u64 {
    match op {
        Operation::XOR => r1 ^ r2,
        Operation::AND => r1 & r2,
        Operation::OR => r1 | r2,
    }
}

fn part1(input: &str) {
    let (mut vals, insts) = parse_input(input);

    let mut work = VecDeque::from(insts);

    while let Some(next) = work.pop_front() {
        if !vals.contains_key(next.r1) || !vals.contains_key(next.r2) {
            work.push_back(next);
            continue;
        }

        let val1 = vals.get(next.r1).unwrap();
        let val2 = vals.get(next.r2).unwrap();

        vals.insert(next.r_res, do_op(*val1, *val2, next.op));
    }

    let mut res = 0x0;
    for i in 0..0x40 {
        res |= vals.get(&format!("z{:02}", i).as_str()).unwrap_or(&0) << i;
    }

    println!("Total amount of gates: {}", vals.len());
    println!("{}", res);
}

fn make_wire(c: &str, num: u64) -> String {
    format!("{}{:02}", c, num)
}

fn verify_z(wire: &str, bit_num: u64, formulas: &HashMap<&str, (&str, &str, Operation)>) -> bool {
    if !formulas.contains_key(wire) {
        return false;
    }

    let (r1, r2, op) = formulas.get(wire).unwrap();

    if *op != Operation::XOR {
        return false;
    }

    if bit_num == 0x0 {
        let mut sorted = [*r1, *r2];
        sorted.sort();

        return sorted == ["x00", "y00"];
    }

    (verify_intermediate_xor(r1, bit_num, formulas) && verify_carry_bit(r2, bit_num, formulas))
        || (verify_intermediate_xor(r2, bit_num, formulas)
            && verify_carry_bit(r1, bit_num, formulas))
}

fn verify_intermediate_xor(
    wire: &str,
    bit_num: u64,
    formulas: &HashMap<&str, (&str, &str, Operation)>,
) -> bool {
    if !formulas.contains_key(wire) {
        return false;
    }

    let (r1, r2, op) = formulas.get(wire).unwrap();

    if *op != Operation::XOR {
        return false;
    }

    let mut sorted = [*r1, *r2];
    sorted.sort();

    if bit_num == 0x0 {
        sorted == ["x00", "y00"]
    } else {
        sorted
            == [
                make_wire("x", bit_num).as_str(),
                make_wire("y", bit_num).as_str(),
            ]
    }
}

fn verify_carry_bit(
    wire: &str,
    bit_num: u64,
    formulas: &HashMap<&str, (&str, &str, Operation)>,
) -> bool {
    if !formulas.contains_key(wire) {
        return false;
    }

    let (r1, r2, op) = formulas.get(wire).unwrap();
    let mut sorted = [*r1, *r2];
    sorted.sort();

    if bit_num == 1 {
        if *op != Operation::AND {
            return false;
        }

        return sorted == ["x00", "y00"];
    }

    if *op != Operation::OR {
        return false;
    }

    (verify_direct_carry(r1, bit_num - 1, formulas) && verify_recarry(r2, bit_num - 1, formulas))
        || (verify_direct_carry(r2, bit_num - 1, formulas)
            && verify_recarry(r1, bit_num - 1, formulas))
}

fn verify_direct_carry(
    wire: &str,
    bit_num: u64,
    formulas: &HashMap<&str, (&str, &str, Operation)>,
) -> bool {
    if !formulas.contains_key(wire) {
        return false;
    }

    let (r1, r2, op) = formulas.get(wire).unwrap();
    let mut sorted = [*r1, *r2];
    sorted.sort();

    if *op != Operation::AND {
        return false;
    }

    sorted
        == [
            make_wire("x", bit_num).as_str(),
            make_wire("y", bit_num).as_str(),
        ]
}

fn verify_recarry(
    wire: &str,
    bit_num: u64,
    formulas: &HashMap<&str, (&str, &str, Operation)>,
) -> bool {
    if !formulas.contains_key(wire) {
        return false;
    }

    let (r1, r2, op) = formulas.get(wire).unwrap();
    let mut sorted = [*r1, *r2];
    sorted.sort();

    if *op != Operation::AND {
        return false;
    }

    (verify_intermediate_xor(r1, bit_num, formulas) && verify_carry_bit(r2, bit_num, formulas))
        || verify_intermediate_xor(r2, bit_num, formulas) && verify_carry_bit(r1, bit_num, formulas)
}

fn verify(bit_num: u64, formulas: &HashMap<&str, (&str, &str, Operation)>) -> bool {
    verify_z(&make_wire("z", bit_num), bit_num, formulas)
}

fn progress(formulas: &HashMap<&str, (&str, &str, Operation)>) -> u64 {
    let mut i = 0x0;
    loop {
        if !verify(i, formulas) {
            break;
        }

        i += 1;
    }

    i
}

fn part2(input: &str) {
    let (_, insts) = parse_input(input);
    let mut formulas: HashMap<&str, (&str, &str, Operation)> = insts
        .iter()
        .map(|i| (i.r_res, (i.r1, i.r2, i.op)))
        .collect();

    let mut swaps = Vec::new();
    for _ in 0..4 {
        let baseline = progress(&formulas);

        'outer: for r1 in formulas.keys().cloned().collect::<Vec<_>>() {
            for r2 in formulas.keys().cloned().collect::<Vec<_>>() {
                if r1 == r2 {
                    continue;
                }

                let (v1, v2) = (
                    formulas.get(&r1).unwrap().clone(),
                    formulas.get(&r2).unwrap().clone(),
                );

                formulas.insert(r1, v2);
                formulas.insert(r2, v1);
                if progress(&formulas) > baseline {
                    swaps.push(r1);
                    swaps.push(r2);
                    break 'outer;
                }
                formulas.insert(r1, v1);
                formulas.insert(r2, v2);
            }
        }
    }

    swaps.sort();
    println!("{}", swaps.join(","));
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
