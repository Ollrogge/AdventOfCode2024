use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn add(t1: (i32, i32), t2: (i32, i32)) -> (i32, i32) {
    return (t1.0 + t2.0, t1.1 + t2.1);
}

const UP: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);

fn turn_right(cur_dir: (i32, i32)) -> (i32, i32) {
    match cur_dir {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => panic!("Unknown dir"),
    }
}

fn find_start_and_dir(grid: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {
    let (start_row, start_col) = (0..grid.len())
        .flat_map(|row| (0..grid[0].len()).map(move |col| (row, col)))
        .find(|&(row, col)| vec!['>', '^', '>', 'v'].contains(&grid[row][col]))
        .unwrap();

    let cur_dir = match grid[start_row][start_col] {
        '>' => RIGHT,
        '^' => UP,
        '<' => LEFT,
        'v' => DOWN,
        _ => panic!("unexpected char"),
    };

    ((start_row as i32, start_col as i32), cur_dir)
}

fn part1(input: &str) {
    let mut grid = parse_input(input);

    let (mut cur_pos, mut cur_dir) = find_start_and_dir(&grid);

    grid[cur_pos.0 as usize][cur_pos.1 as usize] = '.';

    let mut visited = HashSet::new();
    loop {
        visited.insert(cur_pos);
        let new_pos = add(cur_pos, cur_dir);

        if new_pos.0 < 0
            || new_pos.0 >= grid[0].len() as i32
            || new_pos.1 < 0
            || new_pos.1 >= grid.len() as i32
        {
            break;
        }

        match grid[new_pos.0 as usize][new_pos.1 as usize] {
            '.' => {
                cur_pos = new_pos;
            }
            _ => cur_dir = turn_right(cur_dir),
        }
    }

    println!("{}", visited.len());
}

fn part2(input: &str) {
    let mut grid = parse_input(input);
    let (start_pos, start_dir) = find_start_and_dir(&grid);

    grid[start_pos.0 as usize][start_pos.1 as usize] = '.';

    let mut possible_loop_cnt = 0x0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut cur_pos = start_pos.clone();
            let mut cur_dir = start_dir.clone();
            if i == cur_pos.0 as usize && j == cur_pos.1 as usize {
                continue;
            }
            if grid[i][j] == '#' {
                continue;
            }

            grid[i][j] = '#';
            let mut steps = 0x0;
            // max steps in grid without loop must be < row_len * col_len
            // other way to detect loop: make hashset with (dir, pos) and check if already in set. If it is we are in a loop
            while steps < grid.len() * grid[0].len() {
                let new_pos = add(cur_pos, cur_dir);

                if new_pos.0 < 0
                    || new_pos.0 >= grid[0].len() as i32
                    || new_pos.1 < 0
                    || new_pos.1 >= grid.len() as i32
                {
                    break;
                }

                match grid[new_pos.0 as usize][new_pos.1 as usize] {
                    '.' => {
                        cur_pos = new_pos;
                    }
                    _ => cur_dir = turn_right(cur_dir),
                }

                steps += 1;
            }

            if steps == grid.len() * grid[0].len() {
                possible_loop_cnt += 1;
            }

            grid[i][j] = '.';
        }
    }
    println!("{}", possible_loop_cnt);
}

// If there is something directly in front of you, turn right 90 degrees.
// Otherwise, take a step forward.

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
