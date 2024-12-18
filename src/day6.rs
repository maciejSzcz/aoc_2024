use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn get_cursor_position(board: &Vec<Vec<char>>) -> Result<(usize, usize), ()> {
    let cursor = Regex::new(r"(\^|>|<|v)").unwrap();
    board
        .iter()
        .enumerate()
        .find_map(|(row_id, row)| {
            row.iter()
                .position(|&item| cursor.is_match(&item.to_string()))
                .map(|col_id| (row_id, col_id))
        })
        .ok_or(())
}

fn get_visited_indexes_until_obstacle(
    board: &Vec<Vec<char>>,
    current_location: (usize, usize),
    direction: &Direction,
) -> (Vec<(usize, usize, Direction)>, (i32, i32)) {
    let mut elements = Vec::new();
    let (mut current_row, mut current_col) = (current_location.0, current_location.1);
    let mut last_location: (i32, i32) = (
        current_row.try_into().expect("parsing current row"),
        current_col.try_into().expect("parsing current col"),
    );

    loop {
        match direction {
            Direction::Up => {
                if current_row == 0 {
                    last_location = (-1, current_col.try_into().unwrap());
                    break;
                }
                current_row -= 1;
            }
            Direction::Down => {
                if current_row >= board.len() - 1 {
                    break;
                }
                current_row += 1;
            }
            Direction::Left => {
                if current_col == 0 {
                    last_location = (current_row.try_into().unwrap(), -1);
                    break;
                }
                current_col -= 1;
            }
            Direction::Right => {
                if current_col >= board[current_row].len() - 1 {
                    break;
                }
                current_col += 1;
            }
        }

        let current_element = board[current_row][current_col];
        if current_element.to_string() == "#" {
            break;
        }

        elements.push((current_row, current_col, direction.clone()));
        last_location = (
            current_row
                .try_into()
                .expect("parsing current row into last location"),
            current_col
                .try_into()
                .expect("parsing current col into last location"),
        );
    }
    (elements, last_location)
}

fn get_visited(board: &Vec<Vec<char>>) -> Result<HashSet<(usize, usize)>, ()> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let max_x: i32 = (board.len() - 1).try_into().unwrap();
    let max_y: i32 = (board[0].len() - 1).try_into().unwrap();

    let mut location = get_cursor_position(board)?;
    visited.insert(location);
    let mut direction = Direction::from_str(board[location.0][location.1].to_string().as_str())?;

    loop {
        let (visited_indexes, last_location) =
            get_visited_indexes_until_obstacle(board, location, &direction);

        visited_indexes.into_iter().for_each(|item| {
            visited.insert((item.0, item.1));
        });
        if last_location.0 >= max_x
            || last_location.0 < 0
            || last_location.1 >= max_y
            || last_location.1 < 0
        {
            break;
        }
        location = (
            last_location.0.try_into().unwrap(),
            last_location.1.try_into().unwrap(),
        );
        direction = direction.turn();
    }

    Ok(visited)
}

fn get_possible_loops(board: &Vec<Vec<char>>) -> Result<usize, ()> {
    let possible_obstacles = get_visited(board)?;
    // let possible_obstacles: Vec<(usize, usize)> = (0..board.len())
    //     .flat_map(|row| (0..board[0].len()).map(move |col| (row, col)))
    //     .collect();
    let mut count = 0;
    for (x, y) in possible_obstacles {
        let mut new_board = board.clone();
        if (x, y) == get_cursor_position(board)? {
            continue;
        }
        new_board[x][y] = '#';

        let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
        let max_x: i32 = (new_board.len() - 1).try_into().unwrap();
        let max_y: i32 = (new_board[0].len() - 1).try_into().unwrap();
        let mut location = get_cursor_position(&new_board).expect("failed to get position");
        let mut direction =
            Direction::from_str(new_board[location.0][location.1].to_string().as_str())?;
        visited.insert((location.0, location.1, direction));

        loop {
            let mut new = true;
            let (visited_indexes, last_location) =
                get_visited_indexes_until_obstacle(&new_board, location, &direction);

            for item in visited_indexes.into_iter() {
                new = visited.insert(item);
                if !new {
                    println!("repeating {}, {:?}, {}, x{},y {}", !new, item, count, x, y);
                    count += 1;
                    break;
                }
            }
            if !new
                || last_location.0 >= max_x
                || last_location.0 < 0
                || last_location.1 >= max_y
                || last_location.1 < 0
            {
                break;
            }
            location = (
                last_location.0.try_into().unwrap(),
                last_location.1.try_into().unwrap(),
            );
            direction = direction.turn();
        }
    }
    Ok(count)
}

fn get_visited_count(board: &Vec<Vec<char>>) -> Result<usize, ()> {
    if let Ok(visited) = get_visited(board) {
        return Ok(visited.len());
    }
    Err(())
}

pub fn day6() -> io::Result<()> {
    println!("Day 6:");
    let file = File::open("inputs/day6.txt")?;
    let reader = BufReader::new(file);

    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let res1 = get_visited_count(&rows).unwrap();
    let res2 = get_possible_loops(&rows).unwrap();

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);
    Ok(())
}
