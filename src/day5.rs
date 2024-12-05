use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn day5() -> io::Result<()> {
    println!("Day 5:");
    let file = File::open("inputs/day5.txt")?;
    let reader = BufReader::new(file);
    let mut is_first_section = true;
    let mut order: HashSet<(i32, i32)> = HashSet::new();
    let mut res1 = 0;
    let mut res2 = 0;

    for line_result in reader.lines() {
        let line = line_result?;

        if line.trim().is_empty() {
            is_first_section = false;
            continue;
        }
        if is_first_section {
            let page_order = line.split_once('|').unwrap();
            order.insert((page_order.0.parse().unwrap(), page_order.1.parse().unwrap()));
        } else {
            let mut pages: Vec<i32> = line.split(',').map(|num| num.parse().unwrap()).collect();

            if (1..pages.len()).all(|i| order.contains(&(pages[i - 1], pages[i]))) {
                res1 += pages[pages.len() / 2]
            } else {
                pages.sort_by(|a, b| {
                    if order.contains(&(*a, *b)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                res2 += pages[pages.len() / 2]
            }
        }
    }
    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);
    Ok(())
}
