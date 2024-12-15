use std::vec::Vec;

type Step = (isize, isize);

const UP: Step = (-1, 0);
const RIGHT: Step = (0, 1);
const DOWN: Step = (1, 0);
const LEFT: Step = (0, -1);

fn parse_input(input: &str, part2: bool) -> ((isize, isize), Vec<Vec<char>>, Vec<Step>) {
    let split = input.split("\n\n").collect::<Vec<&str>>();

    let mut grid: Vec<Vec<char>> = split[0].lines().map(|l| l.chars().collect()).collect();
    let dirs = split[1]
        .chars()
        .filter(|x| vec!['^', '>', 'v', '<'].contains(x))
        .map(|x| match x {
            '^' => UP,
            '>' => RIGHT,
            'v' => DOWN,
            '<' => LEFT,
            _ => panic!("Unexpected char: {}", x as usize),
        })
        .collect();

    if part2 {
        let mut new_grid = vec![vec![]; grid.len()];
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                match grid[row][col] {
                    '#' => new_grid[row].extend(vec!['#', '#']),
                    'O' => new_grid[row].extend(vec!['[', ']']),
                    '.' => new_grid[row].extend(vec!['.', '.']),
                    '@' => new_grid[row].extend(vec!['@', '.']),
                    _ => panic!("Unexpected char"),
                }
            }
        }

        grid = new_grid;
    }

    let mut robot = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '@' {
                robot = (row as isize, col as isize);
            }
        }
    }

    (robot, grid, dirs)
}

fn part1(input: &str) {
    let (mut cur_pos, mut grid, steps) = parse_input(input, false);
    for s in steps {
        let new_pos = (cur_pos.0 + s.0, cur_pos.1 + s.1);
        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            continue;
        }

        if grid[new_pos.0 as usize][new_pos.1 as usize] == 'O' {
            let mut tmp_pos = new_pos;
            while grid[tmp_pos.0 as usize][tmp_pos.1 as usize] == 'O' {
                tmp_pos = (tmp_pos.0 + s.0, tmp_pos.1 + s.1);
            }

            if grid[tmp_pos.0 as usize][tmp_pos.1 as usize] == '#' {
                continue;
            }

            while tmp_pos != new_pos {
                grid[tmp_pos.0 as usize][tmp_pos.1 as usize] = 'O';
                tmp_pos = (tmp_pos.0 - s.0, tmp_pos.1 - s.1);
            }
        }
        grid[new_pos.0 as usize][new_pos.1 as usize] = '@';
        grid[cur_pos.0 as usize][cur_pos.1 as usize] = '.';
        cur_pos = new_pos;
    }

    let mut res = 0x0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'O' {
                res += row * 100 + col;
            }
        }
    }

    println!("Res: {}", res);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            print!("{}", grid[row][col]);
        }

        println!("");
    }
}

fn part2(input: &str) {
    let (mut cur_pos, mut grid, steps) = parse_input(input, true);

    'outer: for s in steps {
        let new_pos = (cur_pos.0 + s.0, cur_pos.1 + s.1);
        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            continue;
        }

        if vec!['[', ']'].contains(&grid[new_pos.0 as usize][new_pos.1 as usize]) {
            let mut i = 0x0;
            let mut to_move = vec![cur_pos];

            while i < to_move.len() {
                let new_pos = (to_move[i].0 + s.0, to_move[i].1 + s.1);
                let c = grid[new_pos.0 as usize][new_pos.1 as usize];
                // already seen entries can happen when horizontal move
                if to_move.contains(&new_pos) {
                    i += 1;
                    continue;
                }
                match c {
                    '[' => {
                        to_move.push((new_pos.0, new_pos.1));
                        to_move.push((new_pos.0 + RIGHT.0, new_pos.1 + RIGHT.1))
                    }
                    ']' => {
                        to_move.push((new_pos.0, new_pos.1));
                        to_move.push((new_pos.0 + LEFT.0, new_pos.1 + LEFT.1))
                    }
                    '#' => continue 'outer,
                    _ => (),
                }

                i += 1;
            }

            // reverse the vec to not corrupt the previous move
            to_move.reverse();
            for p in to_move {
                let c = grid[p.0 as usize][p.1 as usize];
                grid[(p.0 + s.0) as usize][(p.1 + s.1) as usize] = c;
                grid[p.0 as usize][p.1 as usize] = '.';
            }
        }
        grid[new_pos.0 as usize][new_pos.1 as usize] = '@';
        grid[cur_pos.0 as usize][cur_pos.1 as usize] = '.';
        cur_pos = new_pos;
    }

    let mut res = 0x0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '[' {
                res += row * 100 + col;
            }
        }
    }

    println!("Res: {}", res);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
