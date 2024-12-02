use std::error::Error;
use std::str::FromStr;
use itertools::Itertools;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("inputs/day2.txt");
    let reports = input.lines().map(Report::from_str).collect::<Result<Vec<_>, _>>()?;

    let part1 = part1(&reports);

    println!("Part 1: {} out of {} reports are safe", part1, reports.len());

    Ok(())
}

fn part1(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.is_safe()).count()
}

struct Report {
    levels: Vec<u32>
}

impl Report {
    fn is_safe(&self) -> bool {
        if self.is_ascending() {
            return self.levels.iter().tuple_windows().map(|(a, b)| b - a).all(|x| x >= 1 && x <= 3)
        }
        else if self.is_descending() {
            return self.levels.iter().tuple_windows().map(|(a, b)| a - b).all(|x| x >= 1 && x <= 3)
        }

        false
    }

    fn is_ascending(&self) -> bool {
        let mut sorted = self.levels.clone();
        sorted.sort();
        sorted == self.levels
    }

    fn is_descending(&self) -> bool {
        let mut sorted = self.levels.clone();
        sorted.sort_by(|a, b| b.cmp(a));
        sorted == self.levels
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