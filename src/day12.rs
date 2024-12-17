use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

type Point = (i32, i32);

#[derive(Debug)]
struct Region {
    plant_type: char,
    points: HashSet<Point>,
}

fn part1(mut grid: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let width = grid[0].len();

    for i in 0..width * grid.len() {
        let x = i % width;
        let y = i / width;

        let cell = grid[y][x];

        if cell == '#' {
            continue;
        }

        let mut cells = Vec::new();
        let perimeter = search_matrix(x, y, cell, &mut grid, &mut cells);

        sum += cells.len() as i32 * perimeter;
    }
    sum
}

fn search_matrix(
    x: usize,
    y: usize,
    cell: char,
    grid: &mut Vec<Vec<char>>,
    cells: &mut Vec<(usize, usize)>,
) -> i32 {
    if cells.iter().any(|&(cy, cx)| cy == y && cx == x) {
        return 0;
    }

    if let Some(other) = grid.get(y).and_then(|res| res.get(x as usize)) {
        if *other == cell {
            cells.push((y, x));
            grid[y][x] = '#';

            let val = search_matrix(x, (y as isize - 1) as usize, cell, grid, cells)
                + search_matrix(x + 1, y, cell, grid, cells)
                + search_matrix(x, y + 1, cell, grid, cells)
                + search_matrix((x as isize - 1) as usize, y, cell, grid, cells);

            return val;
        } else {
            return 1;
        }
    }

    1
}

fn find_regions(grid: &Vec<Vec<char>>) -> Vec<Region> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let plant_type = grid[r][c];
                let mut region_points = HashSet::new();

                let mut queue = vec![(r, c)];
                visited[r][c] = true;

                while let Some((current_r, current_c)) = queue.pop() {
                    region_points.insert((current_r as i32, current_c as i32));

                    let neighbors = [
                        (current_r.wrapping_sub(1), current_c),
                        (current_r + 1, current_c),
                        (current_r, current_c.wrapping_sub(1)),
                        (current_r, current_c + 1),
                    ];

                    for (nr, nc) in neighbors {
                        if nr < rows && nc < cols {
                            if grid[nr][nc] == plant_type && !visited[nr][nc] {
                                queue.push((nr, nc));
                                visited[nr][nc] = true;
                            }
                        }
                    }
                }

                regions.push(Region {
                    plant_type,
                    points: region_points,
                });
            }
        }
    }

    regions
}

fn count_sides(region: &Region) -> usize {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut side_count = 0;

    let points_set = &region.points;

    for &(dr, dc) in &directions {
        let mut sides = HashSet::new();
        let mut remove: HashSet<(i32, i32)> = HashSet::default();

        for &point in points_set {
            let neighbor = (point.0 + dr, point.1 + dc);
            if !points_set.contains(&neighbor) {
                sides.insert(neighbor);
            }
        }

        for side in &sides {
            let mut tmp = (side.0 + dc, side.1 + dr);
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (tmp.0 + dc, tmp.1 + dr);
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count
}

fn part_2(grid: &Vec<Vec<char>>) -> usize {
    let regions = find_regions(&grid);

    regions
        .iter()
        .map(|region| {
            let area = region.points.len();
            let sides = count_sides(region);
            let price = area * sides;
            price
        })
        .sum()
}

pub fn day12() -> io::Result<()> {
    println!("Day 12:");
    let file = File::open("inputs/day12.txt")?;
    let reader = BufReader::new(file);
    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    println!("Result for part 1: {:?}", part1(rows.clone()));
    println!("Result for part 2: {:?}", part_2(&rows));

    Ok(())
}
