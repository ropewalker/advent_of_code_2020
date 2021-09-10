use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Not;

#[derive(Eq, PartialEq, Copy, Clone)]
enum CubeState {
    Active,
    Inactive,
}

impl Not for CubeState {
    type Output = Self;

    fn not(self) -> Self::Output {
        use CubeState::*;

        match self {
            Active => Inactive,
            Inactive => Active,
        }
    }
}

impl From<char> for CubeState {
    fn from(c: char) -> Self {
        use CubeState::*;

        match c {
            '.' => Inactive,
            '#' => Active,
            _ => unreachable!(),
        }
    }
}

trait Position: Sized + Eq + PartialEq + Hash + Clone {
    type Direction;

    fn adjacent_position(&self, direction: &Self::Direction) -> Self;
    fn adjacent_positions(&self) -> Vec<Self>;
}

trait Grid {
    type CubePosition: Position;

    fn check_cube_state(&self, position: &Self::CubePosition) -> CubeState;

    fn count_adjacent_active(&self, position: &Self::CubePosition) -> usize {
        position
            .adjacent_positions()
            .iter()
            .filter(|adjacent_position| {
                self.check_cube_state(adjacent_position) == CubeState::Active
            })
            .count()
    }
}

type HashMapGrid<T> = HashMap<T, CubeState>;

impl<T> Grid for HashMapGrid<T>
where
    T: Position,
{
    type CubePosition = T;

    fn check_cube_state(&self, position: &Self::CubePosition) -> CubeState {
        if let Some(&state) = (*self).get(position) {
            state
        } else {
            CubeState::Inactive
        }
    }
}

trait Infinite {
    fn expand(&mut self);
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&Vector3> for (i32, i32, i32) {
    fn from(vector3: &Vector3) -> Self {
        (vector3.x, vector3.y, vector3.z)
    }
}

impl From<(i32, i32, i32)> for Vector3 {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Vector3 { x, y, z }
    }
}

type Position3 = Vector3;

impl Position for Position3 {
    type Direction = Vector3;

    fn adjacent_position(&self, direction: &Self::Direction) -> Self {
        let (x_delta, y_delta, z_delta) = direction.into();

        (self.x + x_delta, self.y + y_delta, self.z + z_delta).into()
    }

    fn adjacent_positions(&self) -> Vec<Position3> {
        let mut result = Vec::with_capacity(26);

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if (x, y, z) != (0, 0, 0) {
                        result.push(self.adjacent_position(&(x, y, z).into()));
                    }
                }
            }
        }

        result
    }
}

type Grid3 = HashMapGrid<Position3>;

impl Infinite for Grid3 {
    fn expand(&mut self) {
        let min_x = self.keys().map(|position| position.x).min().unwrap() - 1;
        let max_x = self.keys().map(|position| position.x).max().unwrap() + 1;
        let min_y = self.keys().map(|position| position.y).min().unwrap() - 1;
        let max_y = self.keys().map(|position| position.y).max().unwrap() + 1;
        let min_z = self.keys().map(|position| position.z).min().unwrap() - 1;
        let max_z = self.keys().map(|position| position.z).max().unwrap() + 1;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if x == min_x
                        || x == max_x
                        || y == min_y
                        || y == max_y
                        || z == min_z
                        || z == max_z
                    {
                        self.entry((x, y, z).into()).or_insert(CubeState::Inactive);
                    }
                }
            }
        }
    }
}

#[aoc_generator(day17, part1)]
fn parse_input1(input: &str) -> Grid3 {
    let mut grid = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            grid.insert(
                Position3 {
                    x: column as i32,
                    y: row as i32,
                    z: 0,
                },
                c.into(),
            );
        }
    }

    grid
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Vector4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl From<&Vector4> for (i32, i32, i32, i32) {
    fn from(vector4: &Vector4) -> Self {
        (vector4.x, vector4.y, vector4.z, vector4.w)
    }
}

impl From<(i32, i32, i32, i32)> for Vector4 {
    fn from((x, y, z, w): (i32, i32, i32, i32)) -> Self {
        Vector4 { x, y, z, w }
    }
}

type Position4 = Vector4;

impl Position for Position4 {
    type Direction = Vector4;

    fn adjacent_position(&self, direction: &Self::Direction) -> Self {
        let (x_delta, y_delta, z_delta, w_delta) = direction.into();

        (
            self.x + x_delta,
            self.y + y_delta,
            self.z + z_delta,
            self.w + w_delta,
        )
            .into()
    }

    fn adjacent_positions(&self) -> Vec<Position4> {
        let mut result = Vec::with_capacity(80);

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if (x, y, z, w) != (0, 0, 0, 0) {
                            result.push(self.adjacent_position(&(x, y, z, w).into()));
                        }
                    }
                }
            }
        }

        result
    }
}

type Grid4 = HashMapGrid<Position4>;

impl Infinite for Grid4 {
    fn expand(&mut self) {
        let min_x = self.keys().map(|position| position.x).min().unwrap() - 1;
        let max_x = self.keys().map(|position| position.x).max().unwrap() + 1;
        let min_y = self.keys().map(|position| position.y).min().unwrap() - 1;
        let max_y = self.keys().map(|position| position.y).max().unwrap() + 1;
        let min_z = self.keys().map(|position| position.z).min().unwrap() - 1;
        let max_z = self.keys().map(|position| position.z).max().unwrap() + 1;
        let min_w = self.keys().map(|position| position.w).min().unwrap() - 1;
        let max_w = self.keys().map(|position| position.w).max().unwrap() + 1;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    for w in min_w..=max_w {
                        if x == min_x
                            || x == max_x
                            || y == min_y
                            || y == max_y
                            || z == min_z
                            || z == max_z
                            || w == min_w
                            || w == max_w
                        {
                            self.entry((x, y, z, w).into())
                                .or_insert(CubeState::Inactive);
                        }
                    }
                }
            }
        }
    }
}

#[aoc_generator(day17, part2)]
fn parse_input2(input: &str) -> Grid4 {
    let mut grid = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            grid.insert(
                Position4 {
                    x: column as i32,
                    y: row as i32,
                    z: 0,
                    w: 0,
                },
                c.into(),
            );
        }
    }

    grid
}

fn active_flip_rule<G: Grid>(grid: &G, position: &G::CubePosition) -> bool {
    let adjacent_active = grid.count_adjacent_active(position);
    adjacent_active != 2 && adjacent_active != 3
}

fn inactive_flip_rule<G: Grid>(grid: &G, position: &G::CubePosition) -> bool {
    grid.count_adjacent_active(position) == 3
}

fn cycle<P>(grid: &mut HashMapGrid<P>)
where
    HashMapGrid<P>: Infinite,
    P: Position,
{
    let mut to_flip = Vec::new();

    grid.expand();

    for (position, cube_state) in grid.iter() {
        if (*cube_state == CubeState::Inactive && inactive_flip_rule(&(*grid), position))
            || (*cube_state == CubeState::Active && active_flip_rule(&(*grid), position))
        {
            to_flip.push(position.clone())
        }
    }

    for position in to_flip.iter() {
        if let Some(cube_state) = grid.get_mut(position) {
            *cube_state = !(*cube_state);
        }
    }
}

fn count_active_after<P>(grid: &mut HashMapGrid<P>, cycles: usize) -> usize
where
    HashMapGrid<P>: Infinite,
    P: Position,
{
    for _ in 1..=cycles {
        cycle(grid);
    }

    grid.values()
        .filter(|&state| *state == CubeState::Active)
        .count()
}

#[aoc(day17, part1)]
fn part1(grid: &Grid3) -> usize {
    let mut grid = grid.clone();
    count_active_after(&mut grid, 6)
}

#[aoc(day17, part2)]
fn part2(grid: &Grid4) -> usize {
    let mut grid = grid.clone();
    count_active_after(&mut grid, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STARTING_GRID: &str = r".#.
..#
###";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input1(TEST_STARTING_GRID,)), 112);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input2(TEST_STARTING_GRID,)), 848);
    }
}
