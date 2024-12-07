use std::{
    fs::File,
    io::{BufReader, Read},
};

use regex::Regex;

pub fn run() {
    let chars = std::fs::read_to_string("inputs/day3").unwrap();
    let mut enabled = true;

    let sum = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)")
        .unwrap()
        .captures_iter(&chars)
        .map(|caps| {
            let mut val = 0;
            match caps.get(0).unwrap().as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        val = caps.get(1).unwrap().as_str().parse::<i64>().unwrap()
                            * caps.get(2).unwrap().as_str().parse::<i64>().unwrap()
                    }
                }
            }
            val
        })
        .sum::<i64>();

    println!("sum: {sum}")
}
