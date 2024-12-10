use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
struct Antenna {
    x: isize,
    y: isize,
}

impl Antenna {
    fn get_antinode(&self, other: &Antenna) -> Antenna {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        Antenna {
            x: self.x + dx * 2,
            y: self.y + dy * 2,
        }
    }

    fn get_all_antinodes_in_grid(
        &self,
        other: &Antenna,
        max_x: isize,
        max_y: isize,
    ) -> Vec<Antenna> {
        let mut res: Vec<Antenna> = Vec::new();
        let mut point = self.to_owned();
        let mut antinode = other.to_owned();
        res.push(antinode);
        loop {
            let prev = antinode;
            antinode = point.get_antinode(&antinode);
            point = prev;

            if antinode.x >= max_x || antinode.x < 0 || antinode.y >= max_y || antinode.y < 0 {
                break;
            }
            res.push(antinode)
        }
        res
    }
}

pub fn get_all_antinodes_for_antenna(rows: &Vec<Vec<char>>) -> usize {
    let mut antinodes_map: HashSet<(isize, isize)> = HashSet::new();
    let max_x = rows.len() as isize;
    let max_y = rows[0].len() as isize;
    println!("{} {}", max_x, max_y);
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            let current_char = rows[i][j];
            if current_char == '.' {
                continue;
            }

            for i2 in i..rows.len() {
                for j2 in (if i2 == i { j + 1 } else { 0 })..rows[i2].len() {
                    if rows[i2][j2] == current_char {
                        let antenna_a = Antenna {
                            x: i as isize,
                            y: j as isize,
                        };
                        let antenna_b = Antenna {
                            x: i2 as isize,
                            y: j2 as isize,
                        };
                        antenna_a
                            .get_all_antinodes_in_grid(&antenna_b, max_x, max_y)
                            .iter()
                            .for_each(|antinode| {
                                antinodes_map.insert((antinode.x, antinode.y));
                            });
                        antenna_b
                            .get_all_antinodes_in_grid(&antenna_a, max_x, max_y)
                            .iter()
                            .for_each(|antinode| {
                                antinodes_map.insert((antinode.x, antinode.y));
                            });
                    }
                }
            }
        }
    }

    antinodes_map.len()
}

pub fn get_antinodes_for_antenna(rows: &Vec<Vec<char>>) -> usize {
    let mut antinodes_map: HashSet<(isize, isize)> = HashSet::new();
    let max_x = rows.len() as isize;
    let max_y = rows[0].len() as isize;

    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            let current_char = rows[i][j];
            if current_char == '.' {
                continue;
            }

            for i2 in i..rows.len() {
                for j2 in (if i2 == i { j + 1 } else { 0 })..rows[i2].len() {
                    if rows[i2][j2] == current_char {
                        let antenna_a = Antenna {
                            x: i as isize,
                            y: j as isize,
                        };
                        let antenna_b = Antenna {
                            x: i2 as isize,
                            y: j2 as isize,
                        };
                        let res1 = antenna_a.get_antinode(&antenna_b);
                        let res2 = antenna_b.get_antinode(&antenna_a);
                        if res1.x >= 0 && res1.x < max_x && res1.y >= 0 && res1.y < max_y {
                            antinodes_map.insert((res1.x, res1.y));
                        }
                        if res2.x >= 0 && res2.x < max_x && res2.y >= 0 && res2.y < max_y {
                            antinodes_map.insert((res2.x, res2.y));
                        }
                    }
                }
            }
        }
    }

    antinodes_map.len()
}

pub fn day8() -> io::Result<()> {
    println!("Day 8:");
    let file = File::open("inputs/day8.txt")?;
    let reader = BufReader::new(file);

    let rows: Vec<Vec<char>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    let res1 = get_antinodes_for_antenna(&rows);

    let res2 = get_all_antinodes_for_antenna(&rows);

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);
    Ok(())
}
