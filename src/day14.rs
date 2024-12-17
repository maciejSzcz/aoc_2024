use itertools::Itertools;
use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CyclicPoint {
    x: i32,
    y: i32,
    x_max: i32,
    y_max: i32,
}

impl CyclicPoint {
    pub fn new(start_x: i32, start_y: i32, max_x: i32, max_y: i32) -> Self {
        CyclicPoint {
            x: start_x,
            y: start_y,
            x_max: max_x,
            y_max: max_y,
        }
    }

    pub fn get_current(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn move_point(&mut self, dx: i32, dy: i32) -> (i32, i32) {
        self.x = self.cyclic_move(self.x, dx, self.x_max);
        self.y = self.cyclic_move(self.y, dy, self.y_max);

        (self.x, self.y)
    }

    fn cyclic_move(&self, current: i32, delta: i32, max: i32) -> i32 {
        let new_pos = current + delta;

        if new_pos < 0 {
            max - ((-new_pos - 1) % (max + 1))
        } else if new_pos > max {
            new_pos % (max + 1)
        } else {
            new_pos
        }
    }
}

fn get_position_after_n_iterations(
    initial_position: (i32, i32),
    direction: (i32, i32),
    max_x: i32,
    max_y: i32,
    iterations: i32,
) -> (i32, i32) {
    let mut point = CyclicPoint::new(initial_position.1, initial_position.0, max_x, max_y);
    for _ in 0..iterations {
        point.move_point(direction.1, direction.0);
    }
    point.get_current()
}

fn parse_coordinate(part: &str) -> (i32, i32) {
    let coords = part.split('=').nth(1).expect("Invalid coordinate format");

    let coord_parts: Vec<&str> = coords.split(',').collect();

    let x = coord_parts[0]
        .parse::<i32>()
        .expect("Failed to parse x coordinate");
    let y = coord_parts[1]
        .parse::<i32>()
        .expect("Failed to parse y coordinate");

    (x, y)
}

fn get_score(points: Vec<(i32, i32)>, max_x: i32, max_y: i32) -> i32 {
    let mut quadrant_1 = 0;
    let mut quadrant_2 = 0;
    let mut quadrant_3 = 0;
    let mut quadrant_4 = 0;
    let mid_x = max_x / 2;
    let mid_y = max_y / 2;
    for point in points {
        match (point.0.cmp(&mid_x), point.1.cmp(&mid_y)) {
            (Ordering::Greater, Ordering::Greater) => quadrant_1 += 1,
            (Ordering::Less, Ordering::Greater) => quadrant_2 += 1,
            (Ordering::Less, Ordering::Less) => quadrant_3 += 1,
            (Ordering::Greater, Ordering::Less) => quadrant_4 += 1,
            _ => (),
        };
    }
    quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4
}

fn get_iteration_for_christmas_tree(
    rows: &Vec<((i32, i32), (i32, i32))>,
    max_x: i32,
    max_y: i32,
) -> i32 {
    let mut res = 0;
    let mut points: Vec<(CyclicPoint, (i32, i32))> = rows
        .iter()
        .map(|row| (CyclicPoint::new(row.0 .1, row.0 .0, max_x, max_y), row.1))
        .collect();
    for i in 1.. {
        points = points
            .into_iter()
            .map(|mut point| {
                point.0.move_point(point.1 .1, point.1 .0);

                point
            })
            .collect();
        if points
            .iter()
            .map(|&(r, _)| (r.get_current().0, r.get_current().1))
            .all_unique()
        {
            res = i;
            break;
        }
    }
    res
}

pub fn day14() -> io::Result<()> {
    println!("Day 14:");
    let file = File::open("inputs/day14.txt")?;
    let reader = BufReader::new(file);
    let rows: Vec<((i32, i32), (i32, i32))> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();

            (parse_coordinate(parts[0]), parse_coordinate(parts[1]))
        })
        .collect();
    let max_y = 100;
    let max_x = 102;

    let points_after_100_seconds: Vec<(i32, i32)> = rows
        .iter()
        .map(|row| get_position_after_n_iterations(row.0, row.1, max_x, max_y, 100))
        .collect();

    let res1 = get_score(points_after_100_seconds, max_x, max_y);
    let res2 = get_iteration_for_christmas_tree(&rows, max_x, max_y);

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);

    Ok(())
}
