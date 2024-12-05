use std::{error::Error, fmt::Display};

use grid::{grid, Grid};

use crate::grid_extensions::GridExtensions;

pub fn run() -> Result<(), Box<dyn Error>> {
    let grid = load_grid(include_str!("inputs/day4.txt"));

    let part1 = count_xmas(&grid);
    println!("Part 1: XMAS appears {} times", part1);

    Ok(())
}

fn load_grid(input: &str) -> Grid<Letter> {
    let mut letters = Vec::new();
    let mut line_width = 0;

    for line in input.lines().map(|l| l.trim()) {
        line_width = line.len();
        for c in line.chars() {
            letters.push(Letter::from_char(c));
        }
    }

    Grid::from_vec(letters, line_width)
}

fn all_xs(grid: &Grid<Letter>) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    grid.indexed_iter()
        .filter_map(|(p, letter)| if letter == &Letter::X { Some(p) } else { None })
}

fn count_xmas(grid: &Grid<Letter>) -> usize {
    let target = vec![Letter::X, Letter::M, Letter::A, Letter::S];
    all_xs(grid)
        .flat_map(|(row, col)| grid.lines_from(row, col, 4))
        .map(|line| {
            line.into_iter()
                .map(|(_, l)| l)
                .cloned()
                .collect::<Vec<Letter>>()
        })
        .filter(|line| *line == target)
        .count()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
    Boring,
}

impl Letter {
    fn from_char(c: char) -> Letter {
        match c {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => Self::Boring,
        }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Letter::X => write!(f, "X"),
            Letter::M => write!(f, "M"),
            Letter::A => write!(f, "A"),
            Letter::S => write!(f, "S"),
            Letter::Boring => write!(f, "."),
        }
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn test_load_grid_easy() {
    let input = "XMA
S%!";
    let grid = load_grid(input);
    assert_eq!(
        grid,
        grid![[Letter::X, Letter::M, Letter::A][Letter::S, Letter::Boring,Letter::Boring]]
    );
}

#[test]
fn test_find_xs() {
    let input = "XMA
SXX";
    let grid = load_grid(input);
    let xs = all_xs(&grid).collect::<Vec<_>>();
    assert_eq!(xs, vec![(0, 0), (1, 1), (1, 2)]);
}

#[test]
fn test_part1() {
    let grid = load_grid(TEST_INPUT);
    let all_xmas = count_xmas(&grid);
    assert_eq!(all_xmas, 18);
}

#[test]
fn test_all_xmas_simple() {
    let grid = load_grid(
        "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
    );
    let all_xmas = count_xmas(&grid);
    assert_eq!(all_xmas, 4);
}
