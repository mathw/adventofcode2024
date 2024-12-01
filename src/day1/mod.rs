use std::{error::Error, str::FromStr};

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../inputs/day1.txt");

    let part1 = part1(input)?;

    println!("Part 1: Combined distance {}", part1);
    Ok(())
}

fn parse_pairs(input: &str) -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    let pairs = input
        .lines()
        .map(|l| -> Result<(&str, &str), String> {
            let mut parts = l.split_whitespace();
            if let Some(first) = parts.next() {
                if let Some(second) = parts.next() {
                    Ok((first, second))
                } else {
                    Err(format!("Line {} didn't have a second part", l))
                }
            } else {
                Err(format!("Line {} didn't have a first part", l))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut first = pairs
        .iter()
        .map(|p| u32::from_str(p.0))
        .collect::<Result<Vec<u32>, _>>()?;

    first.sort();

    let mut second = pairs
        .iter()
        .map(|p| u32::from_str(p.1))
        .collect::<Result<Vec<u32>, _>>()?;

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

fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let pairs = parse_pairs(input)?;
    Ok(pairs.iter().map(pair_distance).sum())
}

#[test]
fn test_parse_pairs() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let pairs = parse_pairs(input).expect("This should parse");
    assert_eq!(pairs, vec![(1, 3), (2, 3), (3, 3), (3, 4), (3, 5), (4, 9)]);
}

#[test]
fn test_part1() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let answer = part1(input).expect("No errors here please");
    assert_eq!(answer, 11);
}
