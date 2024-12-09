use std::collections::HashMap;
use std::vec::Vec;

fn parse_input(input: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let split = input.split("\n\n").collect::<Vec<&str>>();

    let mut ordering_rules = HashMap::new();

    for l in split[0].lines() {
        let nums: Vec<u64> = l.split("|").map(|n| n.parse::<u64>().unwrap()).collect();
        // keep a list of which pages must come before num[1]
        ordering_rules
            .entry(nums[1])
            .or_insert_with(Vec::new)
            .push(nums[0]);

        ordering_rules.entry(nums[0]).or_insert_with(Vec::new);
    }

    let mut updates: Vec<Vec<u64>> = Vec::new();
    for l in split[1].lines() {
        updates.push(l.split(",").map(|x| x.parse::<u64>().unwrap()).collect());
    }

    (ordering_rules, updates)
}

fn part1(input: &str) {
    let (rules, updates) = parse_input(input);
    let mut valid_nums = 0x0;

    for update in updates {
        let mut _rules = rules.clone();

        let mut is_valid = true;
        for page in update.iter() {
            // no number of the update should be left in the rulset of the page
            if _rules
                .get(&page)
                .unwrap()
                .iter()
                .any(|num| num != page && update.contains(num))
            {
                is_valid = false;
                break;
            }

            _rules.values_mut().for_each(|x| x.retain(|e| e != page));
        }

        if is_valid {
            valid_nums += update.get(update.len() / 2).unwrap();
        }
    }

    println!("{}", valid_nums);
}

fn part2(input: &str) {
    let (rules, updates) = parse_input(input);
    let mut incorrect_updates = Vec::new();
    for update in updates {
        let mut _rules = rules.clone();

        for page in update.iter() {
            // no number of the update should be left in the rulset of the page
            if _rules
                .get(&page)
                .unwrap()
                .iter()
                .any(|num| num != page && update.contains(num))
            {
                incorrect_updates.push(update);
                break;
            }

            _rules.values_mut().for_each(|x| x.retain(|e| e != page));
        }
    }

    let mut corrected_rules = Vec::new();
    for mut update in incorrect_updates {
        let mut corrected_rule = Vec::new();
        while !update.is_empty() {
            // do a topological sort kind of approach
            // => always search for the next page that has no dependencies
            if let Some(&val) = update.iter().find(|page| {
                // if page has no dependency with the numbers left in the update, then pick that page next
                !rules
                    .get(page)
                    .unwrap()
                    .iter()
                    .any(|num| update.contains(num))
            }) {
                corrected_rule.push(val);
                update.retain(|&x| x != val);
            } else {
                panic!("Cant find next correct val");
            }
        }

        corrected_rules.push(corrected_rule);
    }

    let res: u64 = corrected_rules
        .iter()
        .map(|r| r.get(r.len() / 2).unwrap())
        .sum();

    println!("{}", res);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
