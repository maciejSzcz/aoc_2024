use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

fn validate_row(acc: i64, line: Vec<i64>) -> i64 {
    let invalid_diff = line.windows(2).any(|window| {
        let [a, b] = window else { unreachable!() };
        let diff = (a - b).abs();
        diff == 0 || diff > 3
    });
    if !invalid_diff {
        if line.windows(2).all(|w| w[0] > w[1]) || line.windows(2).all(|w| w[0] < w[1]) {
            acc + 1
        } else {
            acc
        }
    } else {
        acc
    }
}

fn count_valid_in_row(lines: &Vec<Result<String, Error>>) -> i64 {
    lines
        .iter()
        .filter_map(|line| {
            if let Ok(line) = line {
                let reports: Vec<i64> = line
                    .split(" ")
                    .map(|value| value.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                Some(reports)
            } else {
                None
            }
        })
        .fold(0, validate_row)
}

fn count_valid_in_row_with_dampener(lines: &Vec<Result<String, Error>>) -> i64 {
    lines.iter().fold(0, |acc, line| {
        if let Ok(line) = line {
            let reports: Vec<i64> = line
                .split(" ")
                .map(|value| value.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            if validate_row(0, reports.clone()) > 0 {
                return acc + 1;
            }

            let len = reports.len();

            let mutations = (0..len).map(move |skip_index| {
                reports
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != skip_index)
                    .map(|(_, val)| *val)
                    .collect::<Vec<i64>>()
            });

            let valid = mutations.fold(0, validate_row) > 0;
            if valid {
                acc + 1
            } else {
                acc
            }
        } else {
            acc
        }
    })
}

pub fn day2() -> io::Result<()> {
    println!("Day 2:");
    let file = File::open("inputs/day2.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<Result<String, Error>> = reader.lines().collect::<Vec<Result<String, Error>>>();
    let res = count_valid_in_row(&lines);
    let res2 = count_valid_in_row_with_dampener(&lines);

    println!("Result for part 1: {}", res);
    println!("Result for part 2: {}", res2);
    Ok(())
}
