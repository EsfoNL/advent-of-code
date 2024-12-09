use std::{collections::HashMap, fs::File, io::BufRead};

use crate::day6::Coord;

fn inside(c: &Coord<isize>, (x, y): (usize, usize)) -> bool {
    !(c.0 < 0 || c.1 < 0 || c.0 > (x as isize) || c.1 > (y as isize))
}

pub fn run() {
    let mut mapmap = HashMap::new();

    let map = File::open_buffered("inputs/day8")
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .map(|e| e.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (x, v) in map.iter().enumerate() {
        for (y, v) in v.iter().enumerate() {
            if v != &b'.' {
                mapmap
                    .entry(v)
                    .or_insert_with(Vec::new)
                    .push(Coord::<isize>(x as isize, y as isize));
            }
        }
    }
    {
        let mut nodes = vec![vec![false; map[0].len()]; map.len()];
        for node in mapmap
            .iter()
            .flat_map(|(_, c)| {
                c.iter()
                    .flat_map(|e| c.iter().map(|v| (e.clone(), v.clone())))
            })
            .filter_map(|e| {
                if e.0 == e.1 {
                    return None;
                }
                let diff = &e.0 - &e.1;

                Some((&e.1 - &diff, &e.0 + &diff))
            })
        {
            if let Ok(v) = Coord::<usize>::try_from(node.0) {
                if v.0 < nodes.len() && v.1 < nodes[0].len() {
                    nodes[&v] = true;
                }
            }
            if let Ok(v) = Coord::<usize>::try_from(node.1) {
                if v.0 < nodes.len() && v.1 < nodes[0].len() {
                    nodes[&v] = true;
                }
            }
        }

        for i in nodes.iter() {
            for v in i {
                print!("{}", if *v { '#' } else { '.' })
            }
            println!()
        }

        println!();

        println!(
            "{}",
            nodes
                .iter()
                .map(|e| e.iter().map(|e| *e as usize).sum::<usize>())
                .sum::<usize>()
        );
    }
    {
        let mut nodes = vec![vec![false; map[0].len()]; map.len()];
        let size = (nodes.len(), nodes[0].len());
        for node in mapmap
            .iter()
            .flat_map(|(_, c)| {
                c.iter()
                    .flat_map(|e| c.iter().map(|v| (e.clone(), v.clone())))
            })
            .filter(|e| e.0 != e.1)
            .flat_map(|e| {
                let diff = &e.0 - &e.1;
                use gcd::Gcd;
                let mut g = diff.0.unsigned_abs().gcd(diff.1.unsigned_abs()) as isize;
                if g == 0 {
                    g = 1
                }
                let diff = Coord(diff.0 / g, diff.1 / g);
                let e_0 = e.1.clone();
                let difff = diff.clone();
                (0..)
                    .map_while(move |i| {
                        let v = &e.0 + &(&diff * i);
                        inside(&v, size).then_some(v)
                    })
                    .chain((0..).map_while(move |i| {
                        let v = &e_0 + &(&difff * (-i));
                        inside(&v, size).then_some(v)
                    }))
            })
        {
            if let Ok(v) = Coord::<usize>::try_from(node) {
                if v.0 < nodes.len() && v.1 < nodes[0].len() {
                    nodes[&v] = true;
                }
            }

            for i in nodes.iter() {
                for v in i {
                    print!("{}", if *v { '#' } else { '.' })
                }
                println!()
            }
            println!("=================>")
        }

        println!(
            "{}",
            nodes
                .iter()
                .map(|e| e.iter().map(|e| *e as usize).sum::<usize>())
                .sum::<usize>()
        );
    }
}
