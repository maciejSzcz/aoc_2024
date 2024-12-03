use std::env;

use aoc::day1::day1;
use aoc::day2::day2;
use aoc::day3::day3;

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
        _ => Ok(()),
    }
    .unwrap();
}
