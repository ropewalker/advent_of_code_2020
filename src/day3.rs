use aoc_runner_derive::{aoc, aoc_generator};

struct Vec2 {
    right: usize,
    down: usize,
}

impl From<(usize, usize)> for Vec2 {
    fn from((right, down): (usize, usize)) -> Self {
        Vec2 { right, down }
    }
}

type Point = Vec2;
type Slope = Vec2;

struct Map {
    trees: Vec<Point>,
    columns: usize,
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Map {
    let columns = input.lines().next().unwrap().len();
    let mut trees = Vec::new();

    for (down, line) in input.lines().enumerate() {
        for (right, c) in line.chars().enumerate() {
            if c == '#' {
                trees.push((right, down).into());
            } else {
                assert_eq!(c, '.');
            }
        }
    }

    Map { trees, columns }
}

fn count_trees(map: &Map, slope: &Slope) -> usize {
    map.trees
        .iter()
        .filter(|&point| {
            point.down * slope.right % (map.columns * slope.down)
                == point.right * slope.down % (map.columns * slope.down)
        })
        .count()
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
    count_trees(map, &(3, 1).into())
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
    [
        (1, 1).into(),
        (3, 1).into(),
        (5, 1).into(),
        (7, 1).into(),
        (1, 2).into(),
    ]
    .iter()
    .fold(1, |acc, slope| acc * count_trees(map, slope))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 7);
    }

    #[test]
    fn part2_example() {
        let map = parse_input(TEST_INPUT);

        assert_eq!(count_trees(&map, &(1, 1).into()), 2);
        assert_eq!(count_trees(&map, &(3, 1).into()), 7);
        assert_eq!(count_trees(&map, &(5, 1).into()), 3);
        assert_eq!(count_trees(&map, &(7, 1).into()), 4);
        assert_eq!(count_trees(&map, &(1, 2).into()), 2);
        assert_eq!(part2(&map), 336);
    }
}
