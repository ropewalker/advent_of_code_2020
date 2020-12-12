use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::{AddAssign, Mul};

#[derive(Clone)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Vector2 {
    fn from((x, y): (i32, i32)) -> Self {
        Vector2 { x, y }
    }
}

type Position = Vector2;
type Direction = Vector2;

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Vector2 {
    fn rotate(&mut self, direction: &TurnDirection, degrees: i32) {
        use TurnDirection::*;

        let (cos, sin) = match degrees / 90 % 4 {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };

        let sign = match direction {
            Right => -1,
            Left => 1,
        };

        *self = Vector2 {
            x: self.x * cos - self.y * sin * sign,
            y: self.x * sin * sign + self.y * cos,
        }
    }
}

enum MoveDirection {
    North,
    South,
    East,
    West,
}

impl From<&MoveDirection> for Vector2 {
    fn from(move_direction: &MoveDirection) -> Self {
        use MoveDirection::*;

        match *move_direction {
            North => (0, 1).into(),
            South => (0, -1).into(),
            East => (1, 0).into(),
            West => (-1, 0).into(),
        }
    }
}

enum TurnDirection {
    Left,
    Right,
}

enum Action {
    MoveInDirection(MoveDirection),
    Turn(TurnDirection),
    MoveForward,
}

impl From<char> for Action {
    fn from(c: char) -> Self {
        use Action::*;
        use MoveDirection::*;
        use TurnDirection::*;

        match c {
            'N' => MoveInDirection(North),
            'S' => MoveInDirection(South),
            'E' => MoveInDirection(East),
            'W' => MoveInDirection(West),
            'L' => Turn(Left),
            'R' => Turn(Right),
            'F' => MoveForward,
            _ => unreachable!(),
        }
    }
}

struct Instruction {
    action: Action,
    value: i32,
}

#[derive(Clone)]
struct Coordinates {
    position: Position,
    waypoint: Direction,
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction {
            action: line.chars().next().unwrap().into(),
            value: (&line[1..]).parse::<i32>().unwrap(),
        })
        .collect()
}

fn navigate(
    starting_coordinates: &Coordinates,
    instructions: &[Instruction],
    use_waypoint: bool,
) -> Coordinates {
    use Action::*;

    let mut destination = starting_coordinates.clone();

    for instruction in instructions {
        match &instruction.action {
            MoveInDirection(direction) => {
                let mut direction: Vector2 = direction.into();
                direction = &direction * instruction.value;

                if use_waypoint {
                    destination.waypoint += direction;
                } else {
                    destination.position += direction;
                }
            }
            Turn(direction) => {
                destination.waypoint.rotate(direction, instruction.value);
            }
            Action::MoveForward => {
                destination.position += &destination.waypoint * instruction.value;
            }
        }
    }

    destination
}

#[aoc(day12, part1)]
fn part1(instructions: &[Instruction]) -> i32 {
    let destination = navigate(
        &Coordinates {
            position: Vector2 { x: 0, y: 0 },
            waypoint: Vector2 { x: 1, y: 0 },
        },
        instructions,
        false,
    );

    destination.position.x.abs() + destination.position.y.abs()
}

#[aoc(day12, part2)]
fn part2(instructions: &[Instruction]) -> i32 {
    let destination = navigate(
        &Coordinates {
            position: Vector2 { x: 0, y: 0 },
            waypoint: Vector2 { x: 10, y: 1 },
        },
        instructions,
        true,
    );

    destination.position.x.abs() + destination.position.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"F10
N3
F7
R90
F11";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT,)), 25);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT,)), 286);
    }
}
