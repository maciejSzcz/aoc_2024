use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn solve(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;
    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return 0;
    }
    a * 3 + b
}

pub fn day13() -> io::Result<()> {
    println!("Day 13:");
    let file = File::open("inputs/day13.txt")?;
    let reader = BufReader::new(file);
    let input = reader.lines().filter_map(Result::ok).join("");
    let correction_amount: i64 = 10000000000000;
    let xs = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .map(|w| w.parse().unwrap())
        .tuples();

    let mut res1 = 0;
    let mut res2 = 0;
    for (x1, x2, y1, y2, z1, z2) in xs {
        res1 += solve(x1, x2, y1, y2, z1, z2);
        res2 += solve(
            x1,
            x2,
            y1,
            y2,
            z1 + correction_amount,
            z2 + correction_amount,
        );
    }

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);

    Ok(())
}
