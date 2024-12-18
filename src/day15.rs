use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn solve(g: &mut Vec<Vec<char>>, insts: &str) -> usize {
    let (mut r, mut c) = (0..g.len() as i32)
        .cartesian_product(0..g[0].len() as i32)
        .find(|&(r, c)| g[r as usize][c as usize] == '@')
        .unwrap();
    'outer: for i in insts.bytes() {
        let (dr, dc) = match i {
            b'^' => (-1, 0),
            b'>' => (0, 1),
            b'v' => (1, 0),
            b'<' => (0, -1),
            _ => continue,
        };
        let mut q = VecDeque::from([(r as i32, c as i32)]);
        let mut seen = HashSet::new();
        while let Some((rr, cc)) = q.pop_front() {
            if !seen.insert((rr, cc)) {
                continue;
            }
            let (r2, c2) = (rr as i32 + dr, cc as i32 + dc);
            match g[r2 as usize][c2 as usize] {
                '#' => continue 'outer,
                'O' => q.push_back((r2, c2)),
                '[' => q.extend([(r2, c2), (r2, c2 + 1)]),
                ']' => q.extend([(r2, c2), (r2, c2 - 1)]),
                _ => continue,
            }
        }
        let boxes = seen
            .iter()
            .sorted_by_key(|&&(rr, cc)| (c.abs_diff(cc), r.abs_diff(rr)))
            .rev();
        for &(rr, cc) in boxes {
            let (r2, c2) = (rr + dr, cc + dc);
            g[r2 as usize][c2 as usize] = g[rr as usize][cc as usize];
            g[rr as usize][cc as usize] = '.';
        }
        (r, c) = (r + dr, c + dc);
    }
    (0..g.len())
        .cartesian_product(0..g[0].len())
        .filter(|&(r, c)| matches!(g[r][c], 'O' | '['))
        .map(|(r, c)| r * 100 + c)
        .sum()
}

pub fn day15() -> io::Result<()> {
    println!("Day 15:");
    let file = File::open("inputs/day15.txt")?;
    let reader = BufReader::new(file);
    let mut is_first_section = true;
    let mut grid = Vec::new();
    let mut extended_grid = Vec::new();
    let mut moves = "".to_string();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            is_first_section = false;
            continue;
        }

        if is_first_section {
            grid.push(line.chars().collect_vec());
            extended_grid.push(
                line.chars()
                    .flat_map(|b| match b {
                        '#' => "##".chars(),
                        'O' => "[]".chars(),
                        '.' => "..".chars(),
                        '@' => "@.".chars(),
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        } else {
            moves.push_str(line.as_str());
        }
    }

    let res1 = solve(&mut grid, &moves);
    let res2 = solve(&mut extended_grid, &moves);
    println!("Result for part 1: {}", res1);
    println!("Result for part 2: {}", res2);
    Ok(())
}
