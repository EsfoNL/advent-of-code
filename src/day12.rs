use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufRead,
    ops::Index,
};

pub fn run() {
    let map: Vec<Vec<_>> = File::open_buffered("inputs/day12")
        .unwrap()
        .lines()
        .map(|e| e.unwrap().chars().collect())
        .collect();

    let mut claim_map = vec![vec![0; map[0].len()]; map.len()];
    let mut claims = VecLinkedList::new();
    // let mut unclaimed: HashMap<(usize, usize), char> = HashMap::new();
    // for x in 0..map.len() {
    //     for y in 0..map[0].len() {
    //         unclaimed.insert((x, y), map[x][y]);
    //     }
    // }
    let mut sum = 0;
    for i in 
}
