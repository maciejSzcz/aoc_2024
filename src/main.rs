use std::env;

use aoc::day1::day1;
use aoc::day10::day10;
use aoc::day11::day11;
use aoc::day12::day12;
use aoc::day13::day13;
use aoc::day14::day14;
use aoc::day2::day2;
use aoc::day3::day3;
use aoc::day4::day4;
use aoc::day5::day5;
use aoc::day6::day6;
use aoc::day7::day7;
use aoc::day8::day8;
use aoc::day9::day9;

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
        "day7" => day7(),
        "day8" => day8(),
        "day9" => day9(),
        "day10" => day10(),
        "day11" => day11(),
        "day12" => day12(),
        "day13" => day13(),
        "day14" => day14(),
        _ => Ok(()),
    }
    .unwrap();
}
