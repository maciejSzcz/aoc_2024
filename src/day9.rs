use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter,
    ops::Range,
};

fn create_filesystem(row: &String) -> Vec<Option<u64>> {
    let mut res: Vec<Option<u64>> = Vec::new();
    for (index, char) in row.chars().enumerate() {
        let index = index as u64;

        let space_size = char.to_digit(10).unwrap_or(0) as usize;
        res.extend(
            iter::repeat(if index % 2 == 0 {
                Some(index / 2)
            } else {
                None
            })
            .take(space_size),
        )
    }
    res
}

fn create_non_fragmented_fs(row: &String) -> (Vec<(Range<usize>, u64)>, Vec<Range<usize>>) {
    let mut file_list: Vec<(Range<usize>, u64)> = Vec::new();
    let mut free_list: Vec<Range<usize>> = Vec::new();
    let mut current_index = 0;

    for (i, space_count) in row.chars().enumerate() {
        let space_count = space_count.to_digit(10).unwrap_or(0) as usize;
        let space_range = current_index..current_index + space_count;
        current_index = space_range.end;
        if i % 2 == 0 {
            file_list.push((space_range, i as u64 / 2));
        } else {
            free_list.push(space_range);
        }
    }

    (file_list, free_list)
}

fn reduce_filesystem(filesystem: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut res = filesystem.clone();
    let mut head = 0usize;
    let mut tail = res.len() - 1;

    loop {
        while res[head].is_some() {
            head += 1;
        }
        while res[tail].is_none() {
            tail -= 1;
        }
        if head >= tail {
            break;
        }
        res.swap(head, tail);
    }

    res
}

fn reduce_non_fragmented_fs(
    (mut file_list, mut free_list): (Vec<(Range<usize>, u64)>, Vec<Range<usize>>),
) -> Vec<(Range<usize>, u64)> {
    for file in file_list.iter_mut().rev() {
        if let Some((i, free_slot)) = free_list
            .iter_mut()
            .enumerate()
            .find(|(_, slot)| slot.end <= file.0.start && slot.len() >= file.0.len())
        {
            let slot_start = free_slot.start;
            *free_slot = slot_start + file.0.len()..free_slot.end;
            *file = (slot_start..slot_start + file.0.len(), file.1);
            if free_slot.len() == 0 {
                free_list.remove(i);
            }
        }
    }

    file_list
}

fn calculate_checksum(res: Vec<Option<u64>>) -> u64 {
    res.iter()
        .enumerate()
        .map(|(i, item)| (i as u64) * item.unwrap_or(0) as u64)
        .sum()
}

fn calculate_non_fragmented_checksum(res: Vec<(Range<usize>, u64)>) -> u64 {
    res.into_iter()
        .map(|f| f.0.sum::<usize>() as u64 * f.1)
        .sum()
}

pub fn day9() -> io::Result<()> {
    println!("Day 9:");
    let file = File::open("inputs/day9.txt")?;
    let reader = BufReader::new(file);

    let row: String = reader.lines().filter_map(Result::ok).collect();
    let filesystem = create_filesystem(&row);
    let reduced_system = reduce_filesystem(filesystem);
    let res1 = calculate_checksum(reduced_system);

    let non_fragmented_fs = create_non_fragmented_fs(&row);
    let reduced_non_fragmented_fs = reduce_non_fragmented_fs(non_fragmented_fs);
    let res2 = calculate_non_fragmented_checksum(reduced_non_fragmented_fs);

    println!("Result for part 1: {:?}", res1);
    println!("Result for part 2: {:?}", res2);

    Ok(())
}
