use aoc_runner_derive::{aoc, aoc_generator};

type Point = (usize, usize);
type Slope = Point;

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
                trees.push((right, down));
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
        .filter(|(right, down)| {
            down * slope.0 % (map.columns * slope.1) == right * slope.1 % (map.columns * slope.1)
        })
        .count()
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
    count_trees(map, &(3, 1))
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, slope| acc * count_trees(&map, slope))
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

        assert_eq!(count_trees(&map, &(1, 1)), 2);
        assert_eq!(count_trees(&map, &(3, 1)), 7);
        assert_eq!(count_trees(&map, &(5, 1)), 3);
        assert_eq!(count_trees(&map, &(7, 1)), 4);
        assert_eq!(count_trees(&map, &(1, 2)), 2);
        assert_eq!(part2(&map), 336);
    }
}
