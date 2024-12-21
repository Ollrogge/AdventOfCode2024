use itertools::Itertools;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::u64;
use std::vec::Vec;

const UP: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);

const DIRS: [(isize, isize); 4] = [UP, RIGHT, DOWN, LEFT];

const NUMERIC_PAD: [[char; 3]; 4] = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], [
    ' ', '0', 'A',
]];

const DIR_PAD: [[char; 3]; 2] = [
    [' ', '^', 'A'], // Space used as a placeholder in the third column
    ['<', 'v', '>'],
];

fn in_grid_bounds<const ROWS: usize, const COLS: usize>(
    g: &[[char; COLS]; ROWS],
    p: (isize, isize),
) -> bool {
    p.0 >= 0
        && p.0 < g.len() as isize
        && p.1 >= 0
        && p.1 < g[0].len() as isize
        && g[p.0 as usize][p.1 as usize] != ' '
}

fn dir_to_char(dir: (isize, isize)) -> char {
    match dir {
        UP => '^',
        RIGHT => '>',
        DOWN => 'v',
        LEFT => '<',
        _ => panic!("Unknown dir"),
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<u64>) {
    let parsed = input.lines().map(|l| l.chars().collect()).collect();

    let re = Regex::new(r"\d+").unwrap();
    let nums = input
        .lines()
        .map(|l| {
            re.find(l)
                .map(|m| m.as_str().parse::<u64>().unwrap())
                .unwrap()
        })
        .collect();

    (parsed, nums)
}

fn solve_pad<const ROWS: usize, const COLS: usize>(
    start_pos: char,
    end_pos: char,
    pad: &[[char; COLS]; ROWS],
    c_to_pos: &HashMap<char, (isize, isize)>,
) -> Vec<String> {
    let mut work = BinaryHeap::new();

    let start_pos = *c_to_pos.get(&start_pos).unwrap();
    let end_pos = *c_to_pos.get(&end_pos).unwrap();
    let mut sols = Vec::new();

    let mut min_steps = u64::MAX;

    if start_pos == end_pos {
        return vec![String::from("A")];
    }

    work.push(Reverse((0, start_pos, Vec::new())));

    while let Some(Reverse((steps, pos, mut path))) = work.pop() {
        if !in_grid_bounds(pad, pos) {
            continue;
        }

        if steps > min_steps {
            continue;
        }

        if pos == end_pos && steps <= min_steps {
            min_steps = steps;
            path.push('A');
            sols.push(path);
            continue;
        }

        DIRS.into_iter().for_each(|d| {
            let new_pos = (pos.0 + d.0, pos.1 + d.1);
            let mut _path = path.clone();
            _path.push(dir_to_char(d));
            work.push(Reverse((steps + 1, new_pos, _path)));
        });
    }

    if sols.len() == 0 {
        panic!("Unable to find a path ?:");
    } else {
        sols.into_iter()
            .map(|x| x.into_iter().collect::<String>())
            .collect()
    }
}

fn get_seqs<const ROWS: usize, const COLS: usize>(
    pad: &[[char; COLS]; ROWS],
) -> HashMap<(char, char), Vec<String>> {
    let mut seqs = HashMap::new();
    let mut c_to_pos = HashMap::new();
    for row in 0..pad.len() {
        for col in 0..pad[0].len() {
            if pad[row][col] == ' ' {
                continue;
            }
            c_to_pos.insert(pad[row][col], (row as isize, col as isize));
        }
    }

    for &c1 in c_to_pos.keys() {
        for &c2 in c_to_pos.keys() {
            seqs.insert((c1, c2), solve_pad(c1, c2, pad, &c_to_pos));
        }
    }

    seqs
}

fn compute_length(
    seq: &String,
    depth: u64,
    seqs_dir: &HashMap<(char, char), Vec<String>>,
    cache: &mut HashMap<(String, u64), u64>,
) -> u64 {
    if let Some(&val) = cache.get(&(seq.clone(), depth)) {
        return val;
    }

    let seq = format!("A{}", seq);
    if depth == 1 {
        // last layer / lowest depth in the tree so simply return the sum of moves required
        let length = seq
            .chars()
            .tuple_windows()
            .map(|(c1, c2)| seqs_dir.get(&(c1, c2)).unwrap()[0].len() as u64)
            .sum();

        cache.insert((seq.clone(), depth), length);

        return length;
    }

    let mut length = 0x0;
    for (c1, c2) in seq.chars().tuple_windows() {
        // recursively calculate the length of the sequences created by a move at `depth`
        length += seqs_dir
            .get(&(c1, c2))
            .unwrap()
            .iter()
            .map(|seq| compute_length(seq, depth - 1, seqs_dir, cache))
            .min()
            .unwrap();
    }

    cache.insert((seq[1..].to_string(), depth), length);

    length
}

fn solve(input: &str, depth: u64) {
    let seqs_num = get_seqs(&NUMERIC_PAD);
    let seqs_dir = get_seqs(&DIR_PAD);

    let (codes, nums) = parse_input(input);

    let mut res = 0x0;

    let mut cache = HashMap::new();

    for (mut code, num) in codes.into_iter().zip(nums.into_iter()) {
        // get all possible combinations for layer 1
        // add "A" because this is where we start
        code.insert(0, 'A');
        let options: Vec<Vec<String>> = code
            .iter()
            .tuple_windows()
            .map(|(&c1, &c2)| seqs_num.get(&(c1, c2)).unwrap().clone())
            .collect();

        // now we have found all possibilities for the first layer of how to insert the code. Now we just need to find the cost
        // of the possibilities after going through the layers
        let possibilities: Vec<String> = options
            .into_iter()
            .map(|v| v.into_iter())
            .multi_cartesian_product()
            .map(|comb| comb.into_iter().collect())
            .collect();

        let best = possibilities
            .into_iter()
            .map(|p| compute_length(&p, depth, &seqs_dir, &mut cache))
            .min()
            .unwrap();

        res += num * best;
    }

    println!("Res: {}", res);
}

fn main() {
    let input = include_str!("../input.txt");

    solve(input, 2);
    solve(input, 25);
}
