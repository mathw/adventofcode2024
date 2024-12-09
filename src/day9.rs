use std::error::Error;

use itertools::Itertools;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("inputs/day9.txt");

    println!("The checksum is {}", part1(input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    let mut disk = expand_input(input);
    compact(&mut disk);
    checksum(&disk)
}

fn expand_input(input: &str) -> Vec<Option<usize>> {
    input
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .flat_map(|(file_id, mut c)| {
            let file_len =
                c.next()
                    .and_then(|ch| ch.to_digit(10))
                    .expect("Input should be all digits (file len)") as usize;
            let space_len = c.next().and_then(|ch| ch.to_digit(10)).unwrap_or(0) as usize;
            itertools::repeat_n(Some(file_id), file_len).chain(itertools::repeat_n(None, space_len))
        })
        .collect()
}

fn compact(disk: &mut Vec<Option<usize>>) {
    let mut first_free_space_index = disk.iter().position(|i| i.is_none()).unwrap_or(disk.len());
    if first_free_space_index == disk.len() {
        // there's no free space, so we don't have to compact! Woo.
        return;
    }

    let last_block_index = disk.iter().rposition(|i| i.is_some());
    if last_block_index.is_none() {
        // there are no blocks!
        return;
    }

    let mut last_block_index = last_block_index.unwrap();

    let mut num_swaps = 0;
    #[cfg(test)]
    println!("{}", render_disk(disk));
    // begin!
    loop {
        disk.swap(first_free_space_index, last_block_index);
        #[cfg(test)]
        println!("{}", render_disk(disk));
        num_swaps += 1;
        while first_free_space_index < disk.len() && disk[first_free_space_index].is_some() {
            first_free_space_index += 1;
        }
        last_block_index -= 1;
        if first_free_space_index >= last_block_index {
            println!("Indexes have crossed - must be time to stop after {num_swaps} swaps");
            return;
        }
        if first_free_space_index == disk.len() {
            println!("No free space (this shouldn't happen if we've been swapping) after {num_swaps} swaps");
            return;
        }
        if last_block_index == 0 {
            println!("Last block is at 0 (this shouldn't happen with any realistic puzzle input) after {num_swaps} swaps");
            return;
        }
    }
}

fn checksum(disk: &[Option<usize>]) -> u64 {
    disk.iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|file_id| (i * file_id) as u64))
        .sum()
}

#[cfg(test)]
fn render_disk(disk: &[Option<usize>]) -> String {
    disk.iter()
        .map(|b| match b {
            Some(i) => i.to_string(),
            None => ".".to_owned(),
        })
        .collect()
}

#[test]
fn test_expand_input() {
    assert_eq!(render_disk(&expand_input("2011")), "001.");
    assert_eq!(render_disk(&expand_input("12345")), "0..111....22222");
    assert_eq!(
        render_disk(&expand_input("2333133121414131402")),
        "00...111...2...333.44.5555.6666.777.888899"
    );
}

#[test]
fn test_swaps_1() {
    let mut disk = expand_input("12345");
    compact(&mut disk);
    assert_eq!(render_disk(&disk), "022111222......");
}

#[test]
fn test_swaps_2() {
    let input = "2333133121414131402";
    let mut disk = expand_input(input);
    compact(&mut disk);
    assert_eq!(
        render_disk(&disk),
        "0099811188827773336446555566.............."
    )
}

#[test]
fn test_checksum() {
    let input = "2333133121414131402";
    let mut disk = expand_input(input);
    compact(&mut disk);
    assert_eq!(
        render_disk(&disk),
        "0099811188827773336446555566.............."
    );
    assert_eq!(checksum(&disk), 1928);
}
