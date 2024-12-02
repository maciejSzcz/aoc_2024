use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

trait Sorted<Item> {
    fn into_sorted(self) -> Vec<Item>
    where
        Self: IntoIterator<Item = Item>,
        Item: Ord;
}

impl<T, I> Sorted<T> for I
where
    I: IntoIterator<Item = T>,
    T: Ord,
{
    fn into_sorted(self) -> Vec<T> {
        let mut vec: Vec<T> = self.into_iter().collect();
        vec.sort();
        vec
    }
}

pub fn day1() -> io::Result<()> {
    println!("Day 1:");
    let file = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let (col1, col2): (Vec<i64>, Vec<i64>) = reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|l| {
                let cols: Vec<&str> = l.split("   ").collect();

                Some((
                    cols[0].trim().parse::<i64>().ok()?,
                    cols[1].trim().parse::<i64>().ok()?,
                ))
            })
        })
        .unzip();
    let sorted1 = col1.into_iter().into_sorted();
    let sorted2 = col2.into_iter().into_sorted();

    let res = sorted1
        .iter()
        .zip(sorted2.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());
    println!("Result for part 1: {}", res);

    let vec2_count: HashMap<i64, i64> = sorted2.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let res2 = sorted1.iter().fold(0, |acc, x| {
        acc + (x * vec2_count.get(x).cloned().unwrap_or(0))
    });

    println!("Result for part 2: {}", res2);

    Ok(())
}
