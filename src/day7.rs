use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter::Peekable,
    str::Chars,
};

fn get_valid_rows(rows: &Vec<String>) -> Result<isize, ()> {
    let valid_rows: Vec<isize> = rows
        .iter()
        .filter_map(|row| {
            let (total_string, calibrations_string) = row.split_once(":").unwrap();
            let total = total_string
                .parse::<isize>()
                .expect("Failed toparse total string");
            let calibrations: Vec<isize> = calibrations_string
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse calibration strings"))
                .collect();
            let total_combinations = 2_usize.pow((calibrations.len() - 1) as u32);

            for i in 0..total_combinations {
                let mut expr = String::new();
                expr.push_str(&calibrations[0].to_string());

                for j in 1..calibrations.len() {
                    let op = if (i & (1 << (j - 1))) != 0 { '*' } else { '+' };

                    expr.push(op);
                    expr.push_str(&calibrations[j].to_string());
                }
                if evaluate_expression(&expr) == total {
                    return Some(total);
                }
            }
            return None;
        })
        .collect();
    Ok(valid_rows.iter().sum())
}

fn evaluate_expression(expr: &str) -> isize {
    let mut tokens = expr.chars().peekable();

    let mut result = parse_number(&mut tokens);

    while tokens.peek().is_some() {
        let op = tokens.next().unwrap();

        let next_num = parse_number(&mut tokens);

        result = match op {
            '+' => result + next_num,
            '*' => result * next_num,
            '|' => {
                let result_str = result.to_string();
                let next_str = next_num.to_string();
                format!("{}{}", result_str, next_str)
                    .parse::<isize>()
                    .unwrap()
            }
            _ => panic!("Invalid operation: {}", op),
        };
    }

    result
}

fn parse_number(tokens: &mut Peekable<Chars>) -> isize {
    let mut num_str = String::new();

    while tokens.peek().map_or(false, |&c| c.is_digit(10)) {
        num_str.push(tokens.next().unwrap());
    }

    num_str.parse().expect("Failed to parse number")
}

fn get_valid_rows_with_pipe(rows: &Vec<String>) -> Result<isize, ()> {
    let valid_rows: Vec<isize> = rows
        .iter()
        .filter_map(|row| {
            let (total_string, calibrations_string) = row.split_once(":").unwrap();
            let total = total_string
                .parse::<isize>()
                .expect("Failed toparse total string");
            let calibrations: Vec<isize> = calibrations_string
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse calibration strings"))
                .collect();
            let total_combinations = 3_usize.pow((calibrations.len() - 1) as u32);

            for i in 0..total_combinations {
                let mut expr = String::new();
                expr.push_str(&calibrations[0].to_string());
                let mut temp_i = i;

                for j in 1..calibrations.len() {
                    let op = match temp_i % 3 {
                        0 => '+',
                        1 => '*',
                        2 => '|',
                        _ => unreachable!(),
                    };

                    expr.push(op);
                    expr.push_str(&calibrations[j].to_string());

                    temp_i /= 3;
                }
                if evaluate_expression(&expr) == total {
                    return Some(total);
                }
            }
            return None;
        })
        .collect();
    Ok(valid_rows.iter().sum())
}

pub fn day7() -> io::Result<()> {
    println!("Day 7:");
    let file = File::open("inputs/day7.txt")?;
    let reader = BufReader::new(file);

    let rows: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let res1 = get_valid_rows(&rows);
    let res2 = get_valid_rows_with_pipe(&rows);

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);
    Ok(())
}
