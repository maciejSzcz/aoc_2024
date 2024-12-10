use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn bfs_unique(matrix: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut reachable_9s = Vec::new();

    queue.push_back((start_row, start_col));

    while let Some((cur_row, cur_col)) = queue.pop_front() {
        if matrix[cur_row][cur_col] == '9' {
            reachable_9s.push((cur_row, cur_col));
        }

        if cur_row > 0
            && matrix[cur_row - 1][cur_col].to_digit(10).unwrap_or(0)
                == matrix[cur_row][cur_col].to_digit(10).unwrap_or(0) + 1
        {
            queue.push_back((cur_row - 1, cur_col))
        }
        if cur_row < (matrix.len() - 1)
            && matrix[cur_row + 1][cur_col].to_digit(10).unwrap_or(0)
                == matrix[cur_row][cur_col].to_digit(10).unwrap_or(0) + 1
        {
            queue.push_back((cur_row + 1, cur_col))
        }
        if cur_col > 0
            && matrix[cur_row][cur_col - 1].to_digit(10).unwrap_or(0)
                == matrix[cur_row][cur_col].to_digit(10).unwrap_or(0) + 1
        {
            queue.push_back((cur_row, cur_col - 1))
        }
        if cur_col < (matrix[0].len() - 1)
            && matrix[cur_row][cur_col + 1].to_digit(10).unwrap_or(0)
                == matrix[cur_row][cur_col].to_digit(10).unwrap_or(0) + 1
        {
            queue.push_back((cur_row, cur_col + 1))
        }
    }

    reachable_9s
}

fn bfs(matrix: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut reachable_9s = HashSet::new();

    queue.push_back((start_row, start_col));

    while let Some((cur_row, cur_col)) = queue.pop_front() {
        if matrix[cur_row][cur_col] == '9' {
            reachable_9s.insert((cur_row, cur_col));
        }

        if cur_row > 0
            && matrix[cur_row - 1][cur_col].to_digit(10).unwrap_or(0)
                == matrix[cur_row][cur_col].to_digit(10).unwrap_or(0) + 1
        {
            queue.push_back((cur_row - 1, cur_col))
        }
        if cur_row < (matrix.len() - 1)
            && matrix[cur_row + 1][cur_col].to_digit(10).unwrap_or(0)
                == matrix[cur_row][cur_col].to_digit(10).unwrap_or(0) + 1
        {
            queue.push_back((cur_row + 1, cur_col))
        }
        if cur_col > 0
            && matrix[cur_row][cur_col - 1].to_digit(10).unwrap_or(0)
                == matrix[cur_row][cur_col].to_digit(10).unwrap_or(0) + 1
        {
            queue.push_back((cur_row, cur_col - 1))
        }
        if cur_col < (matrix[0].len() - 1)
            && matrix[cur_row][cur_col + 1].to_digit(10).unwrap_or(0)
                == matrix[cur_row][cur_col].to_digit(10).unwrap_or(0) + 1
        {
            queue.push_back((cur_row, cur_col + 1))
        }
    }

    reachable_9s
}

fn get_trailheads_scores_sum(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    let mut total_score_1 = 0;
    let mut total_score_2 = 0;

    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &item) in row.iter().enumerate() {
            if item == '0' {
                total_score_1 += bfs(matrix, row_idx, col_idx).len();
                total_score_2 += bfs_unique(matrix, row_idx, col_idx).len();
            }
        }
    }

    (total_score_1, total_score_2)
}

pub fn day10() -> io::Result<()> {
    println!("Day 10:");
    let file = File::open("inputs/day10.txt")?;
    let reader = BufReader::new(file);
    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let (res1, res2) = get_trailheads_scores_sum(&rows);

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);

    Ok(())
}
