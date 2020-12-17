use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone)]
enum TileKind {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl TileKind {
    fn flip(&mut self) {
        *self = match self {
            TileKind::Floor => TileKind::Floor,
            TileKind::EmptySeat => TileKind::OccupiedSeat,
            TileKind::OccupiedSeat => TileKind::EmptySeat,
        }
    }
}

impl From<char> for TileKind {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::EmptySeat,
            '#' => Self::OccupiedSeat,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position {
    column: i32,
    row: i32,
}

impl Position {
    fn adjacent_position(&self, direction: &Direction) -> Self {
        let (column_delta, row_delta) = direction.into();

        Position {
            column: self.column + column_delta,
            row: self.row + row_delta,
        }
    }

    fn adjacent_positions(&self) -> Vec<Position> {
        use Direction::*;

        let mut result = Vec::with_capacity(8);

        for direction in &[
            Northwest, North, Northeast, West, East, Southwest, South, Southeast,
        ] {
            result.push(self.adjacent_position(direction));
        }

        result
    }
}

#[derive(Clone)]
struct Layout(HashMap<Position, TileKind>);

#[derive(Debug)]
enum Direction {
    Northwest,
    North,
    Northeast,
    West,
    East,
    Southwest,
    South,
    Southeast,
}

impl From<&Direction> for (i32, i32) {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Northwest => (-1, -1),
            Direction::North => (0, -1),
            Direction::Northeast => (1, -1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
            Direction::Southwest => (-1, 1),
            Direction::South => (0, 1),
            Direction::Southeast => (1, 1),
        }
    }
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Layout {
    let mut tiles = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            tiles.insert(
                Position {
                    column: column as i32,
                    row: row as i32,
                },
                c.into(),
            );
        }
    }

    Layout(tiles)
}

#[derive(Eq, PartialEq)]
enum RoundResult {
    Changed,
    Stabilized,
}

fn count_adjacent_occupied(layout: &Layout, position: &Position) -> usize {
    position
        .adjacent_positions()
        .iter()
        .filter_map(|adjacent_position| layout.0.get(&adjacent_position))
        .filter(|&adjacent_tile_kind| *adjacent_tile_kind == TileKind::OccupiedSeat)
        .count()
}

fn count_visible_occupied(layout: &Layout, position: &Position) -> usize {
    use Direction::*;

    let mut count = 0;

    for direction in &[
        Northwest, North, Northeast, West, East, Southwest, South, Southeast,
    ] {
        let mut next_position = position.adjacent_position(&direction);

        while let Some(tile_kind) = layout.0.get(&next_position) {
            match tile_kind {
                TileKind::OccupiedSeat => {
                    count += 1;
                    break;
                }
                TileKind::EmptySeat => break,
                TileKind::Floor => next_position = next_position.adjacent_position(&direction),
            }
        }
    }

    count
}

fn empty_seat_flip_rule(
    layout: &Layout,
    position: &Position,
    count_function: fn(&Layout, &Position) -> usize,
) -> bool {
    count_function(layout, position) == 0
}

fn occupied_seat_flip_rule(
    layout: &Layout,
    position: &Position,
    count_function: fn(&Layout, &Position) -> usize,
    threshold: usize,
) -> bool {
    count_function(layout, position) >= threshold
}

fn round(
    layout: &mut Layout,
    count_function: fn(&Layout, &Position) -> usize,
    occupied_seats_threshold: usize,
) -> RoundResult {
    let mut to_flip = Vec::new();

    for (position, tile_kind) in layout.0.iter() {
        if (*tile_kind == TileKind::EmptySeat
            && empty_seat_flip_rule(&layout, &position, count_function))
            || (*tile_kind == TileKind::OccupiedSeat
                && occupied_seat_flip_rule(
                    &layout,
                    &position,
                    count_function,
                    occupied_seats_threshold,
                ))
        {
            to_flip.push(position.clone())
        }
    }

    for position in to_flip.iter() {
        if let Some(tile_kind) = layout.0.get_mut(position) {
            (*tile_kind).flip()
        }
    }

    use RoundResult::*;

    if !to_flip.is_empty() {
        Changed
    } else {
        Stabilized
    }
}

fn stabilize(
    layout: &mut Layout,
    count_function: fn(&Layout, &Position) -> usize,
    occupied_seats_threshold: usize,
) -> usize {
    use RoundResult::*;
    let mut round_result = Changed;

    while round_result == Changed {
        round_result = round(layout, count_function, occupied_seats_threshold);
    }

    layout
        .0
        .values()
        .filter(|&tile_kind| *tile_kind == TileKind::OccupiedSeat)
        .count()
}

#[aoc(day11, part1)]
fn part1(layout: &Layout) -> usize {
    let mut layout = layout.clone();
    stabilize(&mut layout, count_adjacent_occupied, 4)
}

#[aoc(day11, part2)]
fn part2(layout: &Layout) -> usize {
    let mut layout = layout.clone();
    stabilize(&mut layout, count_visible_occupied, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STARTING_LAYOUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_STARTING_LAYOUT,)), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_STARTING_LAYOUT,)), 26);
    }
}
