use std::{error::Error, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("inputs/day3.txt");

    let part1 = part1(input)?;
    println!("Part 1: The sum is {}", part1);

    let part2 = part2(input)?;
    println!("Part 2: The sum is {}", part2);

    Ok(())
}

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
    static ref INSTRUCTIONS_REGEX: Regex =
        Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)|do\\(\\)|don't\\(\\)").unwrap();
}

fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    Ok(extract_mul_instructions(input)
        .into_iter()
        .map(evaluate_mul_instruction)
        .collect::<Result<Vec<u32>, _>>()?
        .into_iter()
        .sum())
}

fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    Ok(extract_enabled_muls(input)
        .into_iter()
        .map(evaluate_mul_instruction)
        .collect::<Result<Vec<u32>, _>>()?
        .into_iter()
        .sum())
}

fn extract_mul_instructions(input: &str) -> Vec<&str> {
    let mut start_pos = 0;
    let mut found = Vec::new();

    while let Some(m) = MUL_REGEX.find_at(input, start_pos) {
        start_pos = m.start() + 1;
        found.push(m.as_str())
    }

    found
}

fn extract_enabled_muls(input: &str) -> Vec<&str> {
    let mut start_pos = 0;
    let mut found = Vec::new();
    let mut enabled = true;

    while let Some(m) = INSTRUCTIONS_REGEX.find_at(input, start_pos) {
        start_pos = m.start() + 1;

        if enabled && m.as_str().starts_with("mul") {
            found.push(m.as_str())
        } else if enabled && m.as_str() == "don't()" {
            enabled = false;
        } else if !enabled && m.as_str() == "do()" {
            enabled = true;
        }
    }

    found
}

fn evaluate_mul_instruction(instruction: &str) -> Result<u32, Box<dyn Error>> {
    if let Some(c) = MUL_REGEX.captures(instruction) {
        let a = u32::from_str(&c[1])?;
        let b = u32::from_str(&c[2])?;
        Ok(a * b)
    } else {
        Err(format!(
            "MUL instruction {} didn't have any captures, are you sure it's a MUL instruction?",
            instruction
        )
        .into())
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
#[cfg(test)]
const TEST_INPUT_PART2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn test_part1_sample_extract() {
    assert_eq!(
        extract_mul_instructions(TEST_INPUT),
        vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]
    );
}

#[test]
fn test_evaluation() {
    assert_eq!(evaluate_mul_instruction("mul(1,2)").unwrap(), 2);
    assert_eq!(evaluate_mul_instruction("mul(100,2)").unwrap(), 200);
    assert_eq!(evaluate_mul_instruction("mul(333,200)").unwrap(), 66600);
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), 161);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT_PART2).unwrap(), 48);
}
