use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
pub fn day8() -> io::Result<()> {
    println!("Day 7:");
    let file = File::open("inputs/day7.txt")?;
    let reader = BufReader::new(file);

    let rows: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let res1 = 0;
    let res2 = 0;

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);
    Ok(())
}
