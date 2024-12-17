use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn day13() -> io::Result<()> {
    println!("Day 13:");
    let file = File::open("inputs/day13.txt")?;
    let reader = BufReader::new(file);
    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    println!("Result for part 1: {:?}", 0);
    println!("Result for part 2: {:?}", 0);

    Ok(())
}
