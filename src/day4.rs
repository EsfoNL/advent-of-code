use std::{fs::File, io::BufRead};

pub fn run() {
    let input = File::open_buffered("inputs/day4")
        .unwrap()
        .lines()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut x_count = 0;
    for x in 0..(input.len() as isize) {
        for y in 0..(input[0].len() as isize) {
            for pattern in [
                [(0, 1), (0, 2), (0, 3)],
                [(1, 1), (2, 2), (3, 3)],
                [(1, -1), (2, -2), (3, -3)],
                [(1, 0), (2, 0), (3, 0)],
            ] {
                if pattern.iter().any(|e| {
                    x + e.0 < 0
                        || y + e.1 < 0
                        || x + e.0 >= input.len() as isize
                        || y + e.1 >= input[0].len() as isize
                }) {
                    continue; // pattern not valid, outside bounds
                }
                match (
                    input[x as usize].chars().nth(y as usize),
                    input[(x + pattern[0].0) as usize]
                        .chars()
                        .nth((y + pattern[0].1) as usize),
                    input
                        .get((x + pattern[1].0) as usize)
                        .and_then(|e| e.chars().nth((y + pattern[1].1) as usize)),
                    input
                        .get((x + pattern[2].0) as usize)
                        .and_then(|e| e.chars().nth((y + pattern[2].1) as usize)),
                ) {
                    (Some('X'), Some('M'), Some('A'), Some('S'))
                    | (Some('S'), Some('A'), Some('M'), Some('X')) => count += 1,
                    _ => {}
                }
            }
            if x == 0 || y == 0 || x + 1 >= input.len() as isize || y + 1 >= input[0].len() as isize
            {
                continue;
            }
            if let ('A', ('M', 'S') | ('S', 'M'), ('M', 'S') | ('S', 'M')) = (
                input[x as usize].chars().nth(y as usize).unwrap(),
                (
                    input[(x - 1) as usize]
                        .chars()
                        .nth((y - 1) as usize)
                        .unwrap(),
                    input[(x + 1) as usize]
                        .chars()
                        .nth((y + 1) as usize)
                        .unwrap(),
                ),
                (
                    input[(x + 1) as usize]
                        .chars()
                        .nth((y - 1) as usize)
                        .unwrap(),
                    input[(x - 1) as usize]
                        .chars()
                        .nth((y + 1) as usize)
                        .unwrap(),
                ),
            ) {
                x_count += 1
            }
        }
    }
    println!("{:?}", count);
    println!("{:?}", x_count);
}
