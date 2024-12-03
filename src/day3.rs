use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

fn get_valid_muls(lines: &Vec<Result<String, Error>>) -> i32 {
    let capture_mul_numbers_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    lines
        .iter()
        .filter_map(|line| {
            if let Ok(l) = line {
                Some(
                    capture_mul_numbers_pattern
                        .captures_iter(&l)
                        .map(|caps| {
                            let num1: i32 = caps[1].parse().unwrap();
                            let num2: i32 = caps[2].parse().unwrap();
                            num1 * num2
                        })
                        .sum::<i32>(),
                )
            } else {
                None
            }
        })
        .sum::<i32>()
}

fn get_valid_enabled_muls(lines: &Vec<Result<String, Error>>) -> i32 {
    let capture_mul_numbers_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let remove_disabled_regex = Regex::new(r"don't\(\)[\s\S]*?(do\(\)|$)").unwrap();
    let res = lines.iter().fold("".to_string(), |acc, cur| {
        if let Ok(cur) = cur {
            return acc + cur;
        }
        return acc;
    });
    let whole_file = remove_disabled_regex.replace_all(&res, "");

    capture_mul_numbers_pattern
        .captures_iter(&whole_file)
        .map(|caps| {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            num1 * num2
        })
        .sum::<i32>()
}

pub fn day3() -> io::Result<()> {
    println!("Day 3:");
    let file = File::open("inputs/day3.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<Result<String, Error>> = reader.lines().collect::<Vec<Result<String, Error>>>();

    let valid_mul = get_valid_muls(&lines);
    let valid_enabled_mul = get_valid_enabled_muls(&lines);
    println!("Result for part 1: {:?}", valid_mul);
    println!("Result for part 2: {:?}", valid_enabled_mul);
    Ok(())
}
