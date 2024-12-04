use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn find_all_overlapping<'a>(haystack: &'a str, pattern: &Regex) -> i32 {
    let mut res = 0;
    let mut start = 0;

    while let Some(idx) = pattern.find(&haystack[start..]) {
        let actual_idx = start + idx.start();
        start = actual_idx + 1;
        res += 1;
    }

    res
}

fn match_horizontal(rows: &Vec<Vec<char>>, pattern: &Regex) -> i32 {
    rows.iter().fold(0, |acc, row| {
        let row_str: String = row.iter().collect();
        acc + find_all_overlapping(&row_str, pattern)
    })
}

fn match_vertical(rows: &Vec<Vec<char>>, pattern: &Regex) -> i32 {
    let cols_count = rows[0].len();
    let mut res = 0;
    for current_col in 0..cols_count {
        let column: String = rows.iter().map(|row| row[current_col]).collect();
        res += find_all_overlapping(&column, pattern);
    }

    res
}

fn match_diagonal(rows: &Vec<Vec<char>>, pattern: &Regex) -> i32 {
    let mut res = 0;
    let rows_count = rows.len();
    let cols_count = rows[0].len();

    for start_col in 0..cols_count {
        let mut diagonal: String = String::new();
        let mut current_row = 0;
        let mut current_col = start_col;
        while current_row < rows_count && current_col < cols_count {
            diagonal.push(rows[current_row][current_col]);
            current_row += 1;
            current_col += 1;
        }

        res += find_all_overlapping(&diagonal, pattern)
    }

    for start_row in 1..rows_count {
        let mut diagonal: String = String::new();
        let mut current_row = start_row;
        let mut current_col = 0;

        while current_row < rows_count && current_col < cols_count {
            diagonal.push(rows[current_row][current_col]);
            current_row += 1;
            current_col += 1;
        }

        res += find_all_overlapping(&diagonal, pattern)
    }

    for start_col_3 in (0..cols_count).rev() {
        let mut diagonal: String = String::new();
        let mut current_row = 0;
        let mut current_col: i32 = start_col_3.try_into().unwrap();

        while current_row < rows_count
            && current_col < cols_count.try_into().unwrap()
            && current_col >= 0
        {
            let current_col_index: usize = current_col.try_into().unwrap();
            diagonal.push(rows[current_row][current_col_index]);
            current_row += 1;
            if current_col >= 0 {
                current_col -= 1;
            }
        }

        res += find_all_overlapping(&diagonal, pattern)
    }

    for start_row in 1..rows_count {
        let mut diagonal: String = String::new();
        let mut current_row = start_row;
        let mut current_col = cols_count - 1;

        while current_row < rows_count && current_col < cols_count && current_col >= 0 {
            diagonal.push(rows[current_row][current_col]);
            current_row += 1;
            if current_col > 0 {
                current_col -= 1;
            }
        }

        res += find_all_overlapping(&diagonal, pattern)
    }

    res
}

fn match_all(rows: &Vec<Vec<char>>, pattern: &Regex) -> i32 {
    match_horizontal(rows, pattern) + match_vertical(rows, pattern) + match_diagonal(rows, pattern)
}

fn match_x_mas(rows: &Vec<Vec<char>>, pattern: &Regex) -> i32 {
    let mut res = 0;
    for row in 1..(rows.len() - 1) {
        for col in 1..(rows[0].len() - 1) {
            let mut diag_a: String = "".to_string();
            diag_a.push(rows[row - 1][col - 1]);
            diag_a.push(rows[row][col]);
            diag_a.push(rows[row + 1][col + 1]);
            let mut diag_b: String = "".to_string();
            diag_b.push(rows[row - 1][col + 1]);
            diag_b.push(rows[row][col]);
            diag_b.push(rows[row + 1][col - 1]);
            if pattern.find(&diag_a).is_some() && pattern.find(&diag_b).is_some() {
                res += 1;
            }
            println!("{}, {}", diag_a, diag_b);
        }
    }
    res
}

pub fn day4() -> io::Result<()> {
    println!("Day 3:");
    let file = File::open("inputs/day4.txt")?;
    let reader = BufReader::new(file);
    let pattern = Regex::new(r"(XMAS|SAMX)").unwrap();
    let pattern_x_mas = Regex::new(r"(MAS|SAM)").unwrap();
    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let res1 = match_all(&rows, &pattern);

    let res2 = match_x_mas(&rows, &pattern_x_mas);

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);
    Ok(())
}
