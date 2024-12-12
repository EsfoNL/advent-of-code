use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufRead,
    ops::Index,
};

use ansi_term::Color;
use crossterm::{
    style::Stylize,
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
};

fn get_context<const SIZE: usize>(
    map: &Vec<Vec<char>>,
    offset_x: usize,
    offset_y: usize,
) -> [[Option<char>; SIZE]; SIZE] {
    let (offset_x_i, offset_y_i) = (offset_x as isize, offset_y as isize);
    assert!(SIZE & 1 == 1, "invalid context size");
    let radius = (SIZE - 1) / 2;
    let radius_i = radius as isize;
    let mut out = [[None; SIZE]; SIZE];
    for x in (-radius_i)..=(radius_i) {
        for y in (-radius_i)..=(radius_i) {
            if (offset_x_i + x) < 0
                || (offset_y_i + y) < 0
                || (offset_x_i + x) as usize >= map.len()
                || (offset_y_i + y) as usize >= map[0].len()
            {
                continue;
                // edge
            }
            let (x, y) = ((x + offset_x_i) as usize, (y + offset_y_i) as usize);
            // println!("offset_x: {offset_x}, offset_y: {offset_y}, x: {x}, y: {y}");
            out[x + radius - offset_x][y + radius - offset_y] = Some(map[x][y]);
        }
    }

    out
}

pub fn run(vis: bool) {
    let map: Vec<Vec<_>> = File::open_buffered("inputs/day12")
        .unwrap()
        .lines()
        .map(|e| e.unwrap().chars().collect())
        .collect();
    {
        let mut unclaimed: HashMap<(usize, usize), char> = HashMap::new();
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                unclaimed.insert((x, y), map[x][y]);
            }
        }
        let mut sum = 0;

        while !unclaimed.is_empty() {
            let (claimed, _) = unclaimed.iter().next().unwrap();
            let mut island_area = 0;
            let mut island_circumference = 0;
            let (claimed, cur_char) = unclaimed.remove_entry(&claimed.clone()).unwrap();

            // unexplored positions
            let mut unexplored = vec![claimed];
            // explored positions part of island
            let mut explored = HashSet::new();
            while let Some(exploring) = unexplored.pop() {
                if explored.contains(&exploring) {
                    continue;
                }
                let mut neighbours = 0;
                explored.insert(exploring);
                let (x, y) = (exploring.0 as isize, exploring.1 as isize);
                for (x, y) in [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)] {
                    if x < 0 || y < 0 || x as usize >= map.len() || y as usize >= map[0].len() {
                        continue;
                        // edge
                    }
                    let (x, y) = (x as usize, y as usize);
                    if explored.contains(&(x, y)) {
                        neighbours += 1;
                        continue;
                        // already part of island
                    }

                    if map[x][y] == cur_char {
                        neighbours += 1;
                        unexplored.push((x, y));
                        unclaimed.remove(&(x, y));
                    }
                }

                island_area += 1;
                island_circumference += 4 - neighbours;
                if vis {
                    let _ = crossterm::execute!(std::io::stdout(), BeginSynchronizedUpdate);

                    for i in map.len()..(crossterm::terminal::window_size().unwrap().rows as usize)
                    {
                        println!()
                    }

                    for x in 0..map.len() {
                        for y in 0..map[0].len() {
                            if explored.contains(&(x, y)) {
                                print!("{}", Color::Red.paint(String::from(map[x][y])))
                            } else {
                                print!("{}", map[x][y])
                            }
                        }
                        println!()
                    }
                    let _ = crossterm::execute!(std::io::stdout(), EndSynchronizedUpdate);
                }
                // println!(
                //     "{island_area} * {island_circumference} = {}",
                //     island_area * island_circumference
                // );
            }
            // println!("{cur_char}: {island_area} * {island_circumference} {explored:?}");
            sum += island_area * island_circumference;
        }
        println!("sum: {sum}");
    }
    {
        let mut unclaimed: HashMap<(usize, usize), char> = HashMap::new();
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                unclaimed.insert((x, y), map[x][y]);
            }
        }
        let mut sum = 0;

        while !unclaimed.is_empty() {
            let (claimed, _) = unclaimed.iter().next().unwrap();
            let mut island_area = 0;
            let mut island_edges = 0;
            let (claimed, cur_char) = unclaimed.remove_entry(&claimed.clone()).unwrap();

            // unexplored positions
            let mut unexplored = vec![claimed];
            // explored positions part of island
            let mut explored = HashSet::new();
            while let Some(exploring) = unexplored.pop() {
                if explored.contains(&exploring) {
                    continue;
                }
                explored.insert(exploring);
                let (x, y) = (exploring.0 as isize, exploring.1 as isize);
                let (x, y) = (x as usize, y as usize);
                let ctx1 = get_context(&map, x, y);
                let ctx = ctx1.map(|e| e.map(|e| e.is_some_and(|e| e == cur_char)));

                let old_edges = island_edges;
                // for x in ctx.iter() {
                //     for y in x.iter() {
                //         print!("{}", *y as usize);
                //     }
                //     println!()
                // }
                // for x in ctx1.iter() {
                //     for y in x.iter() {
                //         if let Some(c) = y {
                //             print!("{}", c)
                //         } else {
                //             print!(".")
                //         };
                //     }
                //     println!()
                // }
                println!("cur_char: {cur_char}");

                if let [[_, false, _], [false, _self, _], [_, _, _]] = ctx {
                    island_edges += 1
                }
                if let [[_, false, _], [_, _self, false], [_, _, _]] = ctx {
                    island_edges += 1
                }
                if let [[_, _, _], [false, _self, _], [_, false, _]] = ctx {
                    island_edges += 1
                }
                if let [[_, _, _], [_, _self, false], [_, false, _]] = ctx {
                    island_edges += 1
                }

                if let [[false, true, _], [true, _self, _], [_, _, _]] = ctx {
                    island_edges += 1
                }
                if let [[_, true, false], [_, _self, true], [_, _, _]] = ctx {
                    island_edges += 1
                }
                if let [[_, _, _], [true, _self, _], [false, true, _]] = ctx {
                    island_edges += 1
                }
                if let [[_, _, _], [_, _self, true], [_, true, false]] = ctx {
                    island_edges += 1
                }

                println!("diff: {}", island_edges - old_edges);

                if ctx[0][1] {
                    unexplored.push((x - 1, y));
                    unclaimed.remove(&(x - 1, y));
                }
                if ctx[1][0] {
                    unexplored.push((x, y - 1));
                    unclaimed.remove(&(x, y - 1));
                }
                if ctx[2][1] {
                    // 000
                    // 00x-1
                    // 000
                    //   |
                    //   2
                    unexplored.push((x + 1, y));
                    unclaimed.remove(&(x + 1, y));
                }
                if ctx[1][2] {
                    // 000
                    // 000
                    // 0x0-2
                    //  |
                    //  1
                    unexplored.push((x, y + 1));
                    unclaimed.remove(&(x, y + 1));
                }
                println!();

                island_area += 1;
                // island_edges += 4 - neighbours;
                if vis {
                    let _ = crossterm::execute!(std::io::stdout(), BeginSynchronizedUpdate);

                    for i in map.len()..(crossterm::terminal::window_size().unwrap().rows as usize)
                    {
                        println!()
                    }

                    for x in 0..map.len() {
                        for y in 0..map[0].len() {
                            if explored.contains(&(x, y)) {
                                print!("{}", Color::Red.paint(String::from(map[x][y])))
                            } else {
                                print!("{}", map[x][y])
                            }
                        }
                        println!()
                    }
                    let _ = crossterm::execute!(std::io::stdout(), EndSynchronizedUpdate);
                }
                // println!(
                //     "{island_area} * {island_circumference} = {}",
                //     island_area * island_circumference
                // );
            }
            // println!("{cur_char}: {island_area} * {island_circumference} {explored:?}");
            sum += island_area * island_edges;
        }
        println!("sum: {sum}");
    }
}
