use std::env;

use aoc::day1::day1;
use aoc::day2::day2;
use aoc::day3::day3;
use aoc::day4::day4;
use aoc::day5::day5;
use aoc::day6::day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide which days task should be run (e.g. day1");
        return;
    }
    let day_arg = &args[1];
    match day_arg.as_str() {
        "day1" => day1(),
        "day2" => day2(),
        "day3" => day3(),
        "day4" => day4(),
        "day5" => day5(),
        "day6" => day6(),
        _ => Ok(()),
    }
    .unwrap();
}
