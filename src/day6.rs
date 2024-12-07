use grid::Grid;
use std::error::Error;

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

fn patrol(map: &mut Map, mut pos: (usize, usize)) {
    let mut facing = Facing::North;

    loop {
        if let Some(r) = map.0.get_mut(pos.0, pos.1) {
            *r = MapSquare::Visited;
        }

        match step(map, pos, facing) {
            Instruction::Turn => facing = facing.turn_right(),
            Instruction::MoveTo { row, col } => pos = (row, col),
            Instruction::LeaveArea => break,
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
