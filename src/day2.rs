use itertools::Itertools;
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("inputs/day2.txt");
    let reports = input
        .lines()
        .map(Report::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let part1 = part1(&reports);

    println!(
        "Part 1: {} out of {} reports are safe",
        part1,
        reports.len()
    );

    let part2 = part2(&reports);

    println!(
        "Part 2: {} out of {} reports are safe",
        part2,
        reports.len()
    );
    Ok(())
}

fn part1(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.is_safe()).count()
}

fn part2(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.is_safe_when_dampened()).count()
}

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        Self::is_safe_impl(&self.levels)
    }

    fn is_safe_impl(levels: &[u32]) -> bool {
        let ascending = Self::is_ascending(levels);
        if !ascending {
            let descending = Self::is_descending(levels);
            if !descending {
                // neither ascending nor descending; not safe
                return false;
            }
        }

        levels
            .iter()
            .tuple_windows()
            .map(|(a, b)| if ascending { b - a } else { a - b })
            .all(|x| (1..=3).contains(&x))
    }

    fn is_ascending(levels: &[u32]) -> bool {
        let mut sorted = levels.to_vec();
        sorted.sort();
        sorted == levels
    }

    fn is_descending(levels: &[u32]) -> bool {
        let mut sorted = levels.to_vec();
        sorted.sort_by(|a, b| b.cmp(a));
        sorted == levels
    }

    fn is_safe_when_dampened(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        // okay so it's unsafe
        // but we're allowed to declare it safe by removing one single level, if the remaining levels evaluate to safe
        // so we're going to try all of the possibilities until we either get a safe one or we run out of them
        for index_to_remove in 0..self.levels.len() {
            let mut new_levels = self.levels.clone();
            new_levels.remove(index_to_remove);
            if Self::is_safe_impl(&new_levels) {
                return true;
            }
        }
        false
    }
}

impl FromStr for Report {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut levels = Vec::new();
        for i in s.split_whitespace() {
            levels.push(i.parse()?);
        }
        Ok(Report { levels })
    }
}

#[test]
fn test_sample_reports_safety() {
    fn do_test(input: &str, should_be_safe: bool) {
        let report = Report::from_str(input).expect("Should parse");
        assert_eq!(report.is_safe(), should_be_safe);
    }
    do_test("7 6 4 2 1", true);
    do_test("1 2 7 8 9", false);
    do_test("9 7 6 2 1", false);
    do_test("1 3 2 4 5", false);
    do_test("8 6 4 4 1", false);
    do_test("1 3 6 7 9", true);
}

#[test]
fn test_sample_reports_dampened_safety() {
    fn do_test(input: &str, should_be_safe: bool) {
        let report = Report::from_str(input).expect("Should parse");
        assert_eq!(report.is_safe_when_dampened(), should_be_safe);
    }
    do_test("7 6 4 2 1", true);
    do_test("1 2 7 8 9", false);
    do_test("9 7 6 2 1", false);
    do_test("1 3 2 4 5", true);
    do_test("8 6 4 4 1", true);
    do_test("1 3 6 7 9", true);
}
