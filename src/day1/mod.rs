use std::{collections::HashMap, error::Error, str::FromStr};

// https://adventofcode.com/2024/day/1

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inputs/day1.txt");

    let (first, second) = parse_lists(input)?;

    let part2 = part2(&first, &second)?;
    let part1 = part1(first, second)?;

    println!("Part 1: Combined distance {}", part1);
    println!("Part 2: Similarity score {}", part2);

    Ok(())
}

fn parse_lists(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let pairs = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            if let Some(first) = parts.next() {
                if let Some(second) = parts.next() {
                    Ok((u32::from_str(first)?, u32::from_str(second)?))
                } else {
                    Err(format!("Line {} didn't have a second part", l).into())
                }
            } else {
                Err(format!("Line {} didn't have a first part", l).into())
            }
        })
        .collect::<Result<Vec<(u32, u32)>, Box<dyn Error>>>()?;

    Ok(pairs.into_iter().unzip())
}

fn make_sorted_pairs(
    mut first: Vec<u32>,
    mut second: Vec<u32>,
) -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    first.sort();
    second.sort();

    Ok(first.into_iter().zip(second).collect())
}

fn pair_distance(pair: &(u32, u32)) -> u32 {
    if pair.0 < pair.1 {
        pair.1 - pair.0
    } else {
        pair.0 - pair.1
    }
}

fn part1(first: Vec<u32>, second: Vec<u32>) -> Result<u32, Box<dyn Error>> {
    let pairs = make_sorted_pairs(first, second)?;
    Ok(pairs.iter().map(pair_distance).sum())
}

fn occurrance_map(i: &[u32]) -> HashMap<u32, u32> {
    let mut map = HashMap::new();

    for n in i.iter() {
        let times_in_list = i.iter().filter(|x| *x == n).count() as u32;
        map.insert(*n, times_in_list);
    }

    map
}

fn part2(left: &[u32], right: &[u32]) -> Result<u32, Box<dyn Error>> {
    let occ_map = occurrance_map(right);

    let similarity_score = left.iter().map(|l| l * occ_map.get(l).unwrap_or(&0)).sum();
    Ok(similarity_score)
}

#[test]
fn test_part1() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let (first, second) = parse_lists(input).expect("No errors here please");
    let answer = part1(first, second).expect("No errors here please");
    assert_eq!(answer, 11);
}

#[test]
fn test_occurrance_map() {
    let input = vec![1, 2, 2, 3, 3, 3];
    let map = occurrance_map(&input);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&1), Some(&1));
    assert_eq!(map.get(&2), Some(&2));
    assert_eq!(map.get(&3), Some(&3));
}

#[test]
fn test_part2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let (first, second) = parse_lists(input).expect("No errors here please");
    assert_eq!(part2(&first, &second).expect("No errors wanted"), 31);
}
