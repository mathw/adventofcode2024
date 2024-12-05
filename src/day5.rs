use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, str::FromStr};

pub fn run() -> Result<(), Box<dyn Error>> {
    let rules = parse_rules(include_str!("inputs/day5/rules.txt"))?;
    let updates = parse_updates(include_str!("inputs/day5/updates.txt"))?;

    let part1 = part1(&rules, &updates);
    println!("The result for part 1 is {part1}");

    Ok(())
}

fn part1(rules: &[Rule], updates: &[Vec<u16>]) -> u32 {
    updates
        .iter()
        .filter(|p| rules.are_satisfied_by(p))
        .filter_map(|p| middle_of(p))
        .map(|p| *p as u32)
        .sum()
}

fn middle_of<T>(pages: &[T]) -> Option<&T> {
    if pages.is_empty() || pages.len() % 2 == 0 {
        // doesn't have a middle
        None
    } else {
        Some(&pages[pages.len() / 2])
    }
}

fn parse_rules(input: &str) -> Result<Vec<Rule>, Box<dyn Error>> {
    input
        .lines()
        .map(Rule::from_str)
        .collect::<Result<Vec<Rule>, _>>()
}

fn parse_updates(input: &str) -> Result<Vec<Vec<u16>>, Box<dyn Error>> {
    Ok(input
        .lines()
        .map(|l| {
            l.split(',')
                .map(u16::from_str)
                .collect::<Result<Vec<u16>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?)
}

lazy_static! {
    static ref RULE_REGEX: Regex =
        Regex::new(r"(\d+)\|(\d+)").expect("You should learn to write regex properly");
}
struct Rule {
    page: u16,
    must_be_before: u16,
}

impl Rule {
    fn is_satisfied_by(&self, pages: &[u16]) -> bool {
        if let Some((pi, _)) = pages.iter().enumerate().find(|p| p.1 == &self.page) {
            let prequel = &pages[0..pi];
            !prequel.iter().any(|p| p == &self.must_be_before)
        } else {
            true
        }
    }
}

trait Rules {
    fn are_satisfied_by(&self, pages: &[u16]) -> bool;
}

impl Rules for &[Rule] {
    fn are_satisfied_by(&self, pages: &[u16]) -> bool {
        self.iter().all(|r| r.is_satisfied_by(pages))
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = RULE_REGEX.captures(s) {
            Ok(Rule {
                page: u16::from_str(&c[1])?,
                must_be_before: u16::from_str(&c[2])?,
            })
        } else {
            Err(format!("'{s}' didn't parse as a rule").into())
        }
    }
}

#[cfg(test)]
const TEST_RULES: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
#[cfg(test)]
const TEST_PAGES: &str = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn test_are_satisfied_by() {
    let rules = parse_rules(TEST_RULES).expect("These should parse");
    let pages = parse_updates(TEST_PAGES).expect("These should parse");

    assert!(rules.as_slice().are_satisfied_by(&pages[0]));
    assert!(rules.as_slice().are_satisfied_by(&pages[1]));
    assert!(rules.as_slice().are_satisfied_by(&pages[2]));
    assert!(!rules.as_slice().are_satisfied_by(&pages[3]));
    assert!(!rules.as_slice().are_satisfied_by(&pages[4]));
    assert!(!rules.as_slice().are_satisfied_by(&pages[5]));
}

#[test]
fn test_part1() {
    let rules = parse_rules(TEST_RULES).expect("These should parse");
    let pages = parse_updates(TEST_PAGES).expect("These should parse");

    assert_eq!(part1(&rules, &pages), 143);
}

#[test]
fn test_is_satisfied_by() {
    let rule = Rule {
        page: 7,
        must_be_before: 4,
    };
    let pages = &[44, 2, 38, 0, 2, 7, 4];
    assert!(rule.is_satisfied_by(pages));
}

#[test]
fn test_parse_rules() {
    let rules = parse_rules(TEST_RULES).expect("These should parse");
    assert_eq!(rules.len(), 21);
    assert_eq!(rules[20].page, 53);
    assert_eq!(rules[20].must_be_before, 13);
}

#[test]
fn test_parse_pages() {
    let pages = parse_updates(TEST_PAGES).expect("These should parse");
    assert_eq!(pages.len(), 6);
    assert_eq!(pages[1], vec![97, 61, 53, 29, 13]);
}
