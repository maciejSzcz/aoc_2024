use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn get_stones_count_after_blinks(stones: &String, n: i32) -> i64 {
    let mut stone_count = 0;
    let mut prev_stones: HashMap<i64, i64> = stones
        .split_whitespace()
        .map(|stone| (stone.parse::<i64>().expect("should be number"), 1))
        .collect();
    let mut even_i_stones: Vec<i64> = prev_stones
        .clone()
        .into_iter()
        .filter_map(|(stone, _)| {
            if stone != 0 && stone.to_string().len() % 2 == 0 {
                Some(stone)
            } else {
                None
            }
        })
        .collect();
    let mut odd_i_stones: Vec<i64> = prev_stones
        .clone()
        .into_iter()
        .filter_map(|(stone, _)| {
            if stone != 0 && stone.to_string().len() % 2 == 1 {
                Some(stone)
            } else {
                None
            }
        })
        .collect();
    let mut key: i64;

    for _ in 0..n {
        let mut new_stones: HashMap<i64, i64> = HashMap::new();
        let mut new_even_i_stones: HashSet<i64> = HashSet::new();
        let mut new_odd_i_stones: HashSet<i64> = HashSet::new();

        if let Some(res) = prev_stones.get(&0) {
            new_stones.insert(1, *res);
        }
        if let Some(res) = prev_stones.get(&0) {
            if *res > 0 {
                new_odd_i_stones.insert(1);
            }
        }

        for j in 0..even_i_stones.len() {
            let evens = even_i_stones[j];
            let mid = evens.to_string().len() / 2;
            let first = evens.to_string()[..mid].parse::<i64>().unwrap();
            let second = evens.to_string()[mid..].parse::<i64>().unwrap();
            new_stones.insert(
                first,
                (new_stones.get(&first).unwrap_or(&0)) + prev_stones.get(&evens).unwrap_or(&0),
            );
            new_stones.insert(
                second,
                (new_stones.get(&second).unwrap_or(&0)) + prev_stones.get(&evens).unwrap_or(&0),
            );

            if first.to_string().len() % 2 == 0 {
                new_even_i_stones.insert(first);
            } else if first != 0 {
                new_odd_i_stones.insert(first);
            }
            if second.to_string().len() % 2 == 0 {
                new_even_i_stones.insert(second);
            } else if second != 0 {
                new_odd_i_stones.insert(second);
            }
        }
        for j in 0..odd_i_stones.len() {
            let current_odd_stone = odd_i_stones[j];
            key = current_odd_stone * 2024;
            new_stones.insert(
                key,
                new_stones.get(&key).unwrap_or(&0) + prev_stones.get(&current_odd_stone).unwrap(),
            );

            if key.to_string().len() % 2 == 0 {
                new_even_i_stones.insert(key);
            } else {
                new_odd_i_stones.insert(key);
            }
        }

        even_i_stones = new_even_i_stones.into_iter().collect();
        odd_i_stones = new_odd_i_stones.into_iter().collect();
        prev_stones = new_stones;
    }
    for (_, val) in prev_stones.clone() {
        stone_count += val;
    }
    stone_count
}

pub fn day11() -> io::Result<()> {
    println!("Day 11:");
    let file = File::open("inputs/day11.txt")?;
    let mut reader = BufReader::new(file);
    let mut stones = String::new();
    reader.read_line(&mut stones).unwrap();

    println!(
        "Result for part 1: {:?}",
        get_stones_count_after_blinks(&stones, 25)
    );
    println!(
        "Result for part 2: {:?}",
        get_stones_count_after_blinks(&stones, 75)
    );

    Ok(())
}
