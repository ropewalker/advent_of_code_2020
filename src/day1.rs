use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::num::ParseIntError;

type Entry = i32;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<Entry>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(entries: &[Entry]) -> Option<Entry> {
    let dictionary: HashSet<_> = entries.iter().collect();

    for first in entries {
        if let Some(&second) = dictionary.get(&(2020 - first)) {
            return Some(first * second);
        }
    }

    None
}

#[aoc(day1, part1, BoolArray)]
fn part1_bool_array(entries: &[Entry]) -> Option<Entry> {
    let dictionary: [bool; 2021] = entries.iter().fold([false; 2021], |mut acc, entry| {
        acc[*entry as usize] = true;
        acc
    });

    for first in entries {
        let second = 2020 - first;

        if second >= 0 && dictionary[second as usize] {
            return Some(first * (2020 - first));
        }
    }

    None
}

#[aoc(day1, part2)]
fn part2(entries: &[Entry]) -> Option<Entry> {
    let dictionary: HashSet<_> = entries.iter().collect();

    for i in 0..entries.len() - 1 {
        for j in i + 1..entries.len() {
            let first = entries[i];
            let second = entries[j];

            if let Some(&third) = dictionary.get(&(2020 - (first + second))) {
                return Some(first * second * third);
            }
        }
    }

    None
}

#[aoc(day1, part2, BoolArray)]
fn part2_bool_array(entries: &[Entry]) -> Option<Entry> {
    let dictionary: [bool; 2021] = entries.iter().fold([false; 2021], |mut acc, entry| {
        acc[*entry as usize] = true;
        acc
    });

    for i in 0..entries.len() - 1 {
        for j in i + 1..entries.len() {
            let first = entries[i];
            let second = entries[j];
            let third = 2020 - (first + second);

            if third >= 0 && dictionary[third as usize] {
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
    fn part1_example() {
        assert_eq!(part1(&[1_721, 979, 366, 299, 675, 1_456]), Some(514_579));
        assert_eq!(
            part1_bool_array(&[1_721, 979, 366, 299, 675, 1_456]),
            Some(514_579)
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&[1_721, 979, 366, 299, 675, 1_456]),
            Some(241_861_950)
        );
        assert_eq!(
            part2_bool_array(&[1_721, 979, 366, 299, 675, 1_456]),
            Some(241_861_950)
        );
    }
}
