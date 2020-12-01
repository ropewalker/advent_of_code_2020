use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(entries: &[i32]) -> Option<i32> {
    let dictionary: HashSet<_> = entries.iter().collect();

    for first in entries {
        if let Some(&second) = dictionary.get(&(2020 - first)) {
            return Some(first * second);
        }
    }

    None
}

#[aoc(day1, part2)]
fn part2(entries: &[i32]) -> Option<i32> {
    let dictionary: HashSet<_> = entries.iter().collect();

    for i in 0..entries.len() {
        for j in i..entries.len() {
            let first = entries[i];
            let second = entries[j];

            if let Some(&third) = dictionary.get(&(2020 - (first + second))) {
                return Some(first * second * third);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&[1_721, 979, 366, 299, 675, 1_456]), Some(514_579));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            part2(&[1_721, 979, 366, 299, 675, 1_456]),
            Some(241_861_950)
        );
    }
}
