use grid::Grid;
use std::{collections::HashSet, error::Error};

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = include_str!("inputs/day6.txt");
    let map = Map::try_from(input)?;
    let start_pos =
        find_start_pos(input).ok_or_else(|| "Unable to find start position".to_owned())?;

    let part1 = part1(map.clone(), start_pos);

    println!("Part 1: {part1} spaces visited");

    Ok(())
}

fn part1(mut map: Map, pos: (usize, usize)) -> usize {
    patrol(&mut map, pos);
    map.0.iter().filter(|s| s == &&MapSquare::Visited).count()
}

/// Runs patrols by inserting a random new obstruction into the map and seeing if it makes a loop
/// Returns the count of how many different obstructions could be added which cause a loop
/// This is brute-force and may prove far too slow to actually run in reality
/// Initial testing shows it's surprisingly fast, but the loop detection appears to detect too many loops
fn part2(map: Map, pos: (usize, usize)) -> usize {
    let possible_obstruction_positions = map
        .0
        .indexed_iter()
        .filter(|x| x.1 == &MapSquare::Empty)
        .map(|x| x.0);

    let mut loop_causing_positions = 0;

    for obstruction_pos in possible_obstruction_positions {
        let mut obstructed_map = map.clone();
        *(obstructed_map
            .0
            .get_mut(obstruction_pos.0, obstruction_pos.1)
            .unwrap()) = MapSquare::Obstacle;
        if patrol(&mut obstructed_map, pos) {
            loop_causing_positions += 1;
        }
    }

    loop_causing_positions
}

fn find_start_pos(input: &str) -> Option<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(
                move |(col, c)| {
                    if c == '^' {
                        Some((row, col))
                    } else {
                        None
                    }
                },
            )
        })
        .next()
}

/// Runs a guard patrol on the given map with the given starting position.
/// Always starts facing north.
/// Returns true if the patrol ends with a loop
/// Returns false if the patrol ends by leaving the map
fn patrol(map: &mut Map, mut pos: (usize, usize)) -> bool {
    let mut facing = Facing::North;
    let mut step_cache = HashSet::new();

    loop {
        if let Some(r) = map.0.get_mut(pos.0, pos.1) {
            *r = MapSquare::Visited;
        }

        let cache_pos = pos;
        let instruction = step(map, pos, facing);
        let cache_entry = (cache_pos.0, cache_pos.1, instruction.clone());
        if step_cache.contains(&cache_entry) {
            // looks like we're looping, because we've been in this position with this instruction before
            return true;
        }

        step_cache.insert(cache_entry);

        match step(map, pos, facing) {
            Instruction::Turn => facing = facing.turn_right(),
            Instruction::MoveTo { row, col } => pos = (row, col),
            Instruction::LeaveArea => return false,
        }
    }
}

fn step(map: &Map, guard_pos: (usize, usize), guard_facing: Facing) -> Instruction {
    match map.coords_ahead(guard_pos, guard_facing) {
        Some(ahead_pos) => match map.0.get(ahead_pos.0, ahead_pos.1) {
            Some(ms) => match ms {
                MapSquare::Obstacle => Instruction::Turn,
                _ => Instruction::MoveTo { row: ahead_pos.0, col: ahead_pos.1 }
            }
            None => panic!(
                "{:?} wasn't a valid position in the map, which should be utterly impossible, there's a bug in Map::coords_ahead",
                ahead_pos
            ),
        },
        None => Instruction::LeaveArea,
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Instruction {
    Turn,
    MoveTo { row: usize, col: usize },
    LeaveArea,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn turn_right(&self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::South => Facing::West,
            Facing::East => Facing::South,
            Facing::West => Facing::North,
        }
    }
}

#[derive(Clone)]
struct Map(Grid<MapSquare>);

impl Map {
    /// What is ahead of the position if facing this way? Some coords, or None = leave the area
    fn coords_ahead(&self, pos: (usize, usize), facing: Facing) -> Option<(usize, usize)> {
        match facing {
            Facing::North => {
                if pos.0 == 0 {
                    // edge of area
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            Facing::South => {
                if pos.0 + 1 == self.0.size().0 {
                    None
                } else {
                    Some((pos.0 + 1, pos.1))
                }
            }
            Facing::East => {
                if pos.1 + 1 == self.0.size().1 {
                    None
                } else {
                    Some((pos.0, pos.1 + 1))
                }
            }
            Facing::West => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
        }
    }
}

impl TryFrom<&str> for Map {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cols = 0;
        let squares = value
            .lines()
            .flat_map(|l| {
                cols = l.len();
                l.chars().map(MapSquare::try_from)
            })
            .collect::<Result<Vec<MapSquare>, Self::Error>>()?;
        Ok(Map(Grid::from_vec(squares, cols)))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MapSquare {
    Empty,
    Obstacle,
    Visited,
}

impl TryFrom<char> for MapSquare {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(MapSquare::Empty),
            '^' => Ok(MapSquare::Empty),
            '#' => Ok(MapSquare::Obstacle),
            _ => Err(format!("Map square character '{}' is not understood", value).into()),
        }
    }
}

#[cfg(test)]
static TEST_MAP: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn test_part1() {
    let map = Map::try_from(TEST_MAP).unwrap();
    let start_pos = find_start_pos(TEST_MAP).unwrap();

    assert_eq!(start_pos, (6, 4));

    let visited_count = part1(map, start_pos);
    assert_eq!(visited_count, 41);
}

#[test]
fn test_part2() {
    let map = Map::try_from(TEST_MAP).unwrap();
    let start_pos = find_start_pos(TEST_MAP).unwrap();

    assert_eq!(start_pos, (6, 4));

    let obstruction_positions = part2(map, start_pos);
    assert_eq!(obstruction_positions, 6);
}
