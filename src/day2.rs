use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn day2() -> io::Result<()> {
    println!("Day 2:");
    let file = File::open("inputs/day2.txt")?;
    let reader = BufReader::new(file);

    let res = reader.lines().fold(0, |acc, line| match line {
        Ok(l) => {
            let res: Vec<&str> = l.split(" ").collect();

            let invalid_diff = res.windows(2).any(|window| {
                let [a, b] = window else { unreachable!() };
                let a_parsed = a.parse::<i64>().ok().unwrap();
                let b_parsed = b.parse::<i64>().ok().unwrap();
                let diff = (a_parsed - b_parsed).abs();
                diff == 0 || diff > 3
            });
            if !invalid_diff {
                let numbers: Vec<i64> = res.iter().map(|s| s.parse::<i64>().unwrap()).collect();

                if numbers.windows(2).all(|w| w[0] > w[1])
                    || numbers.windows(2).all(|w| w[0] < w[1])
                {
                    acc + 1
                } else {
                    acc
                }
            } else {
                acc
            }
        }
        Err(_) => acc,
    });
    println!("Result for part 1: {}", res);
    Ok(())
}
