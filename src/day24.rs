use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Not};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl From<(i32, i32, i32)> for Vector3 {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Vector3 { x, y, z }
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y, self.z + rhs.z).into()
    }
}

impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

type CubeCoordinates = Vector3;

impl CubeCoordinates {
    fn adjacent_coordinates(&self) -> Vec<CubeCoordinates> {
        use Direction::*;

        let mut result = Vec::with_capacity(6);

        for direction in &[East, Southeast, Southwest, West, Northwest, Northeast] {
            result.push(*self + &direction.into())
        }

        result
    }
}

enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TileColor {
    White,
    Black,
}

impl Not for TileColor {
    type Output = TileColor;

    fn not(self) -> Self::Output {
        use TileColor::*;

        match self {
            White => Black,
            Black => White,
        }
    }
}

struct TileFloor(HashMap<CubeCoordinates, TileColor>);

impl TileFloor {
    fn count_adjacent_black(&self, coordinates: &CubeCoordinates) -> usize {
        coordinates
            .adjacent_coordinates()
            .iter()
            .filter(|&adjacent_position| self.0.get(adjacent_position) == Some(&TileColor::Black))
            .count()
    }

    fn expand(&mut self) {
        use TileColor::*;

        let adjacent = self
            .0
            .iter()
            .filter(|(_, color)| **color == Black)
            .map(|(coordinates, _)| coordinates.adjacent_coordinates())
            .flatten()
            .collect::<HashSet<_>>();

        for adjacent_coordinates in adjacent {
            self.0
                .entry(adjacent_coordinates)
                .or_insert(TileColor::White);
        }
    }
}

impl From<&[Vec<Direction>]> for TileFloor {
    fn from(tiles_directions: &[Vec<Direction>]) -> Self {
        use TileColor::*;

        let mut floor: TileFloor = TileFloor(HashMap::new());
        let start = (0, 0, 0).into();

        for tile_directions in tiles_directions {
            let coordinates = follow_directions(&start, &tile_directions);

            let color: TileColor = if let Some(&current_color) = floor.0.get(&coordinates) {
                !current_color
            } else {
                Black
            };

            floor.0.insert(coordinates, color);
        }

        floor
    }
}

impl From<&Direction> for Vector3 {
    fn from(direction: &Direction) -> Self {
        use Direction::*;

        match direction {
            East => (1, -1, 0).into(),
            Southeast => (0, -1, 1).into(),
            Southwest => (-1, 0, 1).into(),
            West => (-1, 1, 0).into(),
            Northwest => (0, 1, -1).into(),
            Northeast => (1, 0, -1).into(),
        }
    }
}

fn parse_tile_directions(tile_directions_str: &str) -> Vec<Direction> {
    use Direction::*;

    let mut i = 0;
    let mut directions = Vec::new();

    while i < tile_directions_str.len() {
        match &tile_directions_str[i..=i] {
            "e" => {
                directions.push(East);
                i += 1;
            }
            "w" => {
                directions.push(West);
                i += 1;
            }
            _ => match &tile_directions_str[i..=i + 1] {
                "se" => {
                    directions.push(Southeast);
                    i += 2;
                }
                "sw" => {
                    directions.push(Southwest);
                    i += 2;
                }
                "nw" => {
                    directions.push(Northwest);
                    i += 2;
                }
                "ne" => {
                    directions.push(Northeast);
                    i += 2;
                }
                _ => unreachable!(),
            },
        }
    }

    directions
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| parse_tile_directions(line))
        .collect()
}

fn follow_directions(start: &CubeCoordinates, directions: &[Direction]) -> CubeCoordinates {
    directions
        .iter()
        .fold(*start, |acc, direction| acc + &direction.into())
}

#[aoc(day24, part1)]
fn part1(tiles_directions: &[Vec<Direction>]) -> usize {
    let floor: TileFloor = tiles_directions.into();

    floor
        .0
        .values()
        .filter(|color| **color == TileColor::Black)
        .count()
}

fn black_flip_rule(floor: &TileFloor, coordinates: &CubeCoordinates) -> bool {
    let adjacent_black = floor.count_adjacent_black(coordinates);
    adjacent_black == 0 || adjacent_black > 2
}

fn white_flip_rule(floor: &TileFloor, coordinates: &CubeCoordinates) -> bool {
    floor.count_adjacent_black(coordinates) == 2
}

fn update_floor(floor: &mut TileFloor) {
    use TileColor::*;

    floor.expand();

    let mut to_flip = Vec::new();

    for (coordinates, color) in floor.0.iter() {
        if (*color == Black && black_flip_rule(&(*floor), &coordinates))
            || (*color == White && white_flip_rule(&(*floor), &coordinates))
        {
            to_flip.push(*coordinates)
        }
    }

    for coordinates in to_flip.iter() {
        if let Some(color) = floor.0.get_mut(&coordinates) {
            *color = !(*color);
        } else {
            floor.0.insert(*coordinates, Black);
        }
    }
}

fn count_black_after(floor: &mut TileFloor, days: usize) -> usize {
    for _ in 1..=days {
        update_floor(floor);
    }

    floor
        .0
        .values()
        .filter(|&color| *color == TileColor::Black)
        .count()
}

#[aoc(day24, part2)]
fn part2(tiles_directions: &[Vec<Direction>]) -> usize {
    let mut floor: TileFloor = tiles_directions.into();
    count_black_after(&mut floor, 100)
}
