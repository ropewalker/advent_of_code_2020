use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

fn van_ecks_nth(n: usize, starting_numbers: &[usize]) -> usize {
    let mut last_spoken_turns = vec![0; n];

    starting_numbers
        .iter()
        .enumerate()
        .for_each(|(index, number)| last_spoken_turns[*number] = index + 1);

    let mut last_number_spoken = starting_numbers[starting_numbers.len() - 1];

    for turn in starting_numbers.len()..n {
        let last_spoken_on = last_spoken_turns[last_number_spoken];
        last_spoken_turns[last_number_spoken] = turn;

        last_number_spoken = if last_spoken_on != 0 {
            turn - last_spoken_on
        } else {
            0
        }
    }

    last_number_spoken
}

#[aoc(day15, part1)]
fn part1(input: &[usize]) -> usize {
    van_ecks_nth(2_020, input)
}

#[aoc(day15, part2)]
fn part2(input: &[usize]) -> usize {
    van_ecks_nth(30_000_000, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&(parse_input("0,3,6")).unwrap()), 436);
        assert_eq!(part1(&(parse_input("1,3,2")).unwrap()), 1);
        assert_eq!(part1(&(parse_input("2,1,3")).unwrap()), 10);
        assert_eq!(part1(&(parse_input("1,2,3")).unwrap()), 27);
        assert_eq!(part1(&(parse_input("2,3,1")).unwrap()), 78);
        assert_eq!(part1(&(parse_input("3,2,1")).unwrap()), 438);
        assert_eq!(part1(&(parse_input("3,1,2")).unwrap()), 1_836);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&(parse_input("0,3,6")).unwrap()), 175_594);
        assert_eq!(part2(&(parse_input("1,3,2")).unwrap()), 2_578);
        assert_eq!(part2(&(parse_input("2,1,3")).unwrap()), 3_544_142);
        assert_eq!(part2(&(parse_input("1,2,3")).unwrap()), 261_214);
        assert_eq!(part2(&(parse_input("2,3,1")).unwrap()), 6_895_259);
        assert_eq!(part2(&(parse_input("3,2,1")).unwrap()), 18);
        assert_eq!(part2(&(parse_input("3,1,2")).unwrap()), 362);
    }
}
