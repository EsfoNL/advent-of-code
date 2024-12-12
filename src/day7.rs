use core::panic;
use std::{fs::File, io::BufRead};

pub fn run(_: bool) {
    let res = File::open_buffered("inputs/day7")
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .filter_map(|e| {
            let (test_res_str, rest) = e.split_once(": ").unwrap();
            let test_res: u64 = test_res_str.parse().unwrap();
            let rest_vec = rest
                .split(' ')
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let rest_vec_len = rest_vec.len();
            // println!("test_res_str: {test_res_str}, ops: {}", rest_vec_len - 1);
            (0..)
                // generate all permutations of [(+ | *); res_vec_len - 1]
                .map_while(|e: usize| {
                    // one boolean value per fence between the values
                    // v_n (* | +) v_2 ... v_n-2 (* | +) v_n-1
                    let v = 1 << (rest_vec_len - 1);
                    // println!("v:  {v:b}, e: {e:b}, e < v: {}", e < v);
                    if e < v {
                        Some(e)
                    } else {
                        None
                    }
                })
                // check if any permutation matches the test value
                .any(|mut e| {
                    // println!("vec: {e:00$b}", rest_vec_len - 1);
                    if rest_vec[1..].iter().fold(rest_vec[0], |acc, val| {
                        let res = if (e & 1) > 0 { acc * val } else { acc + val };
                        // println!(
                        //     "{acc} {} {val} = {res}",
                        //     if (e & 1) > 0 { '*' } else { '+' }
                        // );
                        e >>= 1;
                        res
                    }) == test_res
                    {
                        // println!("match! {test_res}");
                        true
                    } else {
                        false
                    }
                })
                .then_some(test_res)
        })
        .sum::<u64>();
    println!("{}", res);

    let res = File::open_buffered("inputs/day7")
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .filter_map(|e| {
            let (test_res_str, rest) = e.split_once(": ").unwrap();
            let test_res: u64 = test_res_str.parse().unwrap();
            let rest_vec = rest
                .split(' ')
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let rest_vec_len = rest_vec.len();
            // println!("test_res_str: {test_res_str}, ops: {}", rest_vec_len - 1);
            (0..)
                // generate all permutations of [(+ | *); res_vec_len - 1]
                .map_while(|e: usize| {
                    // one boolean value per fence between the values
                    // v_n (* | +) v_2 ... v_n-2 (* | +) v_n-1
                    let v = 3usize.pow((rest_vec_len - 1) as u32);
                    // println!("v:  {v:b}, e: {e:b}, e < v: {}", e < v);
                    if e < v {
                        Some(e)
                    } else {
                        None
                    }
                })
                // check if any permutation matches the test value
                .any(|mut e| {
                    // println!("vec: {e:00$b}", rest_vec_len - 1);
                    if rest_vec[1..].iter().fold(rest_vec[0], |acc, val| {
                        let res = match e % 3 {
                            0 => acc + val,
                            1 => acc * val,
                            2 => acc * 10u64.pow(val.ilog10() + 1) + val,
                            _ => panic!(),
                        };
                        // println!(
                        //     "{acc} {} {val} = {res}",
                        //     match e % 3 {
                        //         0 => '+',
                        //         1 => '*',
                        //         2 => '|',
                        //         _ => panic!(),
                        //     }
                        // );
                        e /= 3;
                        res
                    }) == test_res
                    {
                        // println!("match! {test_res}");
                        true
                    } else {
                        false
                    }
                })
                .then_some(test_res)
        })
        .sum::<u64>();
    println!("{}", res);
}
