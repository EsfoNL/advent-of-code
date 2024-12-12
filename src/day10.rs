use std::{collections::HashSet, fs::File, io::BufRead};

use crate::day6::Coord;

pub fn run(_: bool) {
    let map = File::open_buffered("inputs/day10")
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .map(|e| e.bytes().map(|e| e - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let offsets: &[Coord<isize>] = &[
        // top middle
        Coord(0, -1),
        // middle left
        Coord(-1, 0),
        // middle right
        Coord(1, 0),
        // bottom middle
        Coord(0, 1),
    ];
    let mut connections = vec![vec![HashSet::new(); map[0].len()]; map.len()];
    for coord in (0..map.len()).flat_map(|x| (0..map[0].len()).map(move |y| Coord(x, y))) {
        if map[&coord] == 9 {
            connections[&coord].insert(coord);
        }
    }

    for i in (0..9).rev() {
        for coord in (0..map.len()).flat_map(|x| (0..map[0].len()).map(move |y| Coord(x, y))) {
            if map[&coord] == i {
                let mut output = HashSet::new();
                for offset in offsets {
                    let computed_coord: Coord<isize> = offset + &Coord::<isize>::from(coord);
                    let (Some(l), Some(val)) = (
                        computed_coord.get_from(&map),
                        (computed_coord).get_from(&connections),
                    ) else {
                        continue;
                    };
                    if *l != (i + 1) {
                        continue;
                    }
                    for i in val {
                        output.insert(*i);
                    }
                }
                connections[&coord] = output;
            }
        }
    }

    let mut sum = 0;
    for coord in (0..map.len()).flat_map(|x| (0..map[0].len()).map(move |y| Coord(x, y))) {
        if map[&coord] == 0 {
            sum += connections[&coord].len();
        }
    }
    println!("sum: {sum}");

    let mut num_cons = vec![vec![0; map[0].len()]; map.len()];

    for coord in (0..map.len()).flat_map(|x| (0..map[0].len()).map(move |y| Coord(x, y))) {
        if map[&coord] == 9 {
            num_cons[&coord] = 1;
        }
    }

    for i in (0..9).rev() {
        for coord in (0..map.len()).flat_map(|x| (0..map[0].len()).map(move |y| Coord(x, y))) {
            if map[&coord] == i {
                for offset in offsets {
                    let computed_coord: Coord<isize> = offset + &Coord::<isize>::from(coord);
                    let (Some(l), Some(val)) = (
                        computed_coord.get_from(&map),
                        (computed_coord).get_from(&num_cons).copied(),
                    ) else {
                        continue;
                    };
                    if *l != (i + 1) {
                        continue;
                    }
                    num_cons[&coord] += val;
                }
            }
        }
    }
    let mut sum2 = 0usize;
    for coord in (0..map.len()).flat_map(|x| (0..map[0].len()).map(move |y| Coord(x, y))) {
        if map[&coord] == 0 {
            sum2 += num_cons[&coord];
        }
    }
    println!("sum: {sum2}");
}
