#![feature(linked_list_cursors)]
use std::collections::LinkedList;

#[derive(Clone, Debug)]
struct FileInfo {
    id: u64,
    sz: u64,
    free: u64,
}

fn parse_input(input: &str) -> LinkedList<FileInfo> {
    let inp: Vec<u64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut chunks: LinkedList<FileInfo> = inp
        .chunks_exact(2)
        .enumerate()
        .map(|(idx, chunk)| FileInfo {
            id: idx as u64,
            sz: chunk[0],
            free: chunk[1],
        })
        .collect();

    if inp.chunks_exact(2).remainder().len() > 0 {
        chunks.push_back(FileInfo {
            id: chunks.len() as u64,
            sz: inp.chunks_exact(2).remainder()[0],
            free: 0,
        })
    }

    chunks
}

fn part1(input: &str) {
    let mut parsed = parse_input(input);

    'outer: loop {
        let mut to_insert = parsed.back().unwrap().clone();
        let mut cursor = parsed.cursor_front_mut();
        while let Some(_) = cursor.current() {
            if {
                let current = cursor.current().unwrap();
                current.free > 0 && current.id != to_insert.id
            } {
                let (to_allocate, free) = {
                    let current = cursor.current().unwrap();
                    let to_allocate = u64::min(to_insert.sz, current.free);
                    let free = current.free;
                    current.free = 0x0;
                    (to_allocate, free)
                };

                cursor.insert_after(FileInfo {
                    id: to_insert.id,
                    sz: to_allocate,
                    free: free - to_allocate,
                });

                to_insert.sz -= to_allocate;
            }

            if to_insert.sz == 0 {
                break;
            }

            cursor.move_next();
        }

        // fully inserted the item into the free space
        if to_insert.sz == 0x0 {
            parsed.pop_back();
        } else {
            // we haven't found any spot to put all of to_insert into => we filled all of the free space and are done
            parsed.back_mut().unwrap().sz = to_insert.sz;
            break 'outer;
        }
    }

    let mut checksum: u64 = 0x0;
    let mut idx: u64 = 0x0;
    for block in parsed {
        for _ in 0..block.sz {
            checksum += idx * block.id;
            idx += 1;
        }
    }

    println!("{}", checksum);
}

fn part2(input: &str) {
    let mut parsed = parse_input(input);

    let mut work_queue: Vec<u64> = parsed.iter().map(|x| x.id).collect::<Vec<u64>>();
    work_queue.sort();

    while work_queue.len() > 0 {
        let next_id = work_queue.pop().unwrap();
        let to_insert = parsed.iter_mut().find(|x| x.id == next_id).unwrap().clone();

        let mut cursor = parsed.cursor_front_mut();
        while let Some(_) = cursor.current() {
            if {
                let current = cursor.current().unwrap();
                current.free >= to_insert.sz && current.id != to_insert.id
            } {
                let (to_allocate, free) = {
                    let current = cursor.current().unwrap();
                    let to_allocate = to_insert.sz;
                    let free = current.free;
                    current.free = 0x0;
                    (to_allocate, free)
                };

                cursor.insert_after(FileInfo {
                    id: to_insert.id,
                    sz: to_allocate,
                    free: free - to_allocate,
                });

                let mut delete_cursor = parsed.cursor_back_mut();
                // delete the file we just moved from its initial position
                while let Some(current) = delete_cursor.current() {
                    if current.id == to_insert.id {
                        current.free += current.sz;
                        current.sz = 0x0;
                        break;
                    }
                    delete_cursor.move_prev();
                }
                break;
            }

            cursor.move_next();
        }
    }

    let mut checksum: u64 = 0x0;
    let mut idx: u64 = 0x0;
    for block in parsed {
        for _ in 0..block.sz {
            checksum += idx * block.id;
            idx += 1;
        }
        for _ in 0..block.free {
            idx += 1;
        }
    }

    println!("{}", checksum);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
