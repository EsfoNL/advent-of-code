#![feature(file_buffered)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

macro_rules! days {
    ($input:expr, $vis:expr, [$($i:ident),*]) => {
        {
        match $input {
            $(stringify!($i) => $i::run($vis),)*
            "all" => {
                $($i::run($vis);)*
            }
            _ => panic!("invalid day"),
        }
        }
    };
}

fn main() {
    if std::env::args().count() < 2 {
        panic!("specify day(s) or `all` to run");
    }
    for i in std::env::args().skip(1) {
        days!(
            i.as_str(),
            std::env::var("VISUALIZE").is_ok(),
            [day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12]
        );
    }
}
