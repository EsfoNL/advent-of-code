use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn run() {
    let (mut left_vec, mut right_vec): (Vec<_>, Vec<_>) =
        BufReader::new(File::open("inputs/day1").expect("input file not found"))
            .lines()
            .map_while(Result::ok)
            .filter(|e| !e.is_empty())
            .map(|e| {
                let mut split = e.split_whitespace();
                let left: u64 = split.next().unwrap().parse().unwrap();
                let right: u64 = split.next().unwrap().parse().unwrap();
                (left, right)
            })
            .unzip();

    left_vec.sort_unstable();
    right_vec.sort_unstable();

    println!(
        "{}",
        left_vec
            .iter()
            .zip(right_vec.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<u64>()
    );

    let mut score = 0;
    let mut right_index = 0;
    let mut left_index = 0;

    'lp: loop {
        let val = left_vec[left_index];
        // [l_0, l_1, ..., l_n, ..., l_e]
        // skip to r_n where r_n >= l_n
        while right_vec[right_index] < val {
            right_index += 1;
            if right_index >= right_vec.len() {
                break 'lp;
            }
        }
        let (mut l, mut r) = (0, 0);

        // count occurences of current val in left_vec
        while left_vec[left_index] == val {
            left_index += 1;
            l += 1;
            if left_index >= left_vec.len() {
                break 'lp;
            }
        }

        // count occurences of current val in right_vec
        while right_vec[right_index] == val {
            right_index += 1;
            r += 1;
            if right_index >= right_vec.len() {
                break 'lp;
            }
        }

        // ocurrances of val in left times right times the val
        score += l * r * val;

        // next val
    }

    println!("score: {score}");
}
