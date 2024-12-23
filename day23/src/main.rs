use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut res = HashMap::new();
    for l in input.lines() {
        let s = l.split("-").collect::<Vec<&str>>();

        res.entry(s[0]).or_insert_with(HashSet::new).insert(s[1]);
        res.entry(s[1]).or_insert_with(HashSet::new).insert(s[0]);
    }

    res
}

fn part1(input: &str) {
    let network = parse_input(input);

    let mut sets = HashSet::new();

    for x in network.keys() {
        for y in network.get(x).unwrap() {
            for z in network.get(y).unwrap() {
                if x != z && network.get(z).unwrap().contains(x) {
                    let mut arr = [x, y, z];
                    arr.sort();

                    sets.insert(arr);
                }
            }
        }
    }

    let res = sets
        .iter()
        .filter(|n| n.iter().any(|x| x.starts_with('t')))
        .count();

    println!("{}", res);
}

fn search<'a>(
    mut set: Vec<&'a str>,
    sets: &mut HashSet<Vec<&'a str>>,
    network: &'a HashMap<&str, HashSet<&str>>,
) {
    set.sort();

    if !sets.insert(set.clone()) {
        return;
    }

    let node = *set.last().unwrap();

    for n in network.get(node).unwrap() {
        if set.contains(n) {
            continue;
        }

        // check that new node in set is connected to all other nodes
        if !set.iter().all(|n2| network.get(n2).unwrap().contains(n)) {
            continue;
        }

        let mut new_set = set.clone();
        new_set.push(*n);

        search(new_set, sets, network);
    }
}

fn part2(input: &str) {
    let network = parse_input(input);

    let mut sets = HashSet::new();

    for &n in network.keys() {
        search(vec![n], &mut sets, &network);
    }

    let res = sets.iter().max_by_key(|set| set.len()).unwrap();

    let res = res.iter().map(|x| x.to_string()).join(",");

    println!("{}", res);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
