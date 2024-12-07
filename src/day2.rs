use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    println!(
        "{}",
        BufReader::new(File::open("inputs/day2").unwrap())
            .lines()
            .map(Result::unwrap)
            .map(|e| {
                let mut delta_iter = e
                    .split_whitespace()
                    .zip(e.split_whitespace().skip(1))
                    .map(|(l, r)| l.parse::<i32>().unwrap() - r.parse::<i32>().unwrap());
                let first_delta = delta_iter.next().unwrap();
                let sign = first_delta.signum();
                if !(1..=3).contains(&i32::abs(first_delta)) {
                    return false;
                }

                delta_iter
                    .map(|e| (e, i32::abs(e), e.signum()))
                    .all(|(_, abs, sgn)| (1..=3).contains(&abs) && sgn == sign)
            })
            .map(i32::from)
            .sum::<i32>()
    );

    println!(
        "{}",
        BufReader::new(File::open("inputs/day2").unwrap())
            .lines()
            .map(Result::unwrap)
            .map(|e| {
                let input: Vec<i32> = e
                    .split_whitespace()
                    .map(|e| e.parse().unwrap())
                    .collect::<Vec<_>>();
                let delta_iter = input
                    .iter()
                    .zip(input.iter().skip(1))
                    .map(|(l, r)| l - r)
                    .collect::<Vec<_>>();
                let most_common_sign = delta_iter.iter().map(|e| e.signum()).sum::<i32>().signum();
                let wrong_signs = delta_iter.iter().any(|e| (e.signum() != most_common_sign));
                let wrong_diff = delta_iter.iter().any(|e| !(0..=3).contains(&e.abs()));

                if !(wrong_diff || wrong_signs) {
                    return true;
                }

                // attempt to remove the fault
                for i in 0..input.len() {
                    let mut input = input.clone();
                    input.remove(i);
                    let delta_iter = input
                        .iter()
                        .zip(input.iter().skip(1))
                        .map(|(l, r)| l - r)
                        .collect::<Vec<_>>();
                    let most_common_sign =
                        delta_iter.iter().map(|e| e.signum()).sum::<i32>().signum();
                    let wrong_signs = delta_iter.iter().any(|e| (e.signum() != most_common_sign));
                    let wrong_diff = delta_iter.iter().any(|e| !(0..=3).contains(&e.abs()));

                    if !(wrong_diff || wrong_signs) {
                        return true;
                    }
                }

                false
            })
            .map(i32::from)
            .sum::<i32>()
    );
}
