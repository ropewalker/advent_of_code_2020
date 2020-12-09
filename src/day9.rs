use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

type Entry = i64;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Result<Vec<Entry>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn is_valid_entry(entry: &Entry, previous_entries: &[Entry]) -> bool {
    previous_entries
        .iter()
        .any(|&first| 2 * first != *entry && previous_entries.contains(&(entry - first)))
}

fn first_invalid_entry(entries: &[Entry], preamble_len: usize) -> Option<Entry> {
    for i in preamble_len..entries.len() {
        if !is_valid_entry(&entries[i], &entries[i - preamble_len..i]) {
            return Some(entries[i]);
        }
    }

    None
}

#[aoc(day9, part1)]
fn part1(entries: &[Entry]) -> Option<Entry> {
    first_invalid_entry(entries, 25)
}

fn encryption_weakness(entries: &[Entry], preamble_len: usize) -> i64 {
    let target = first_invalid_entry(entries, preamble_len).unwrap();

    for i in 0..entries.len() - 1 {
        let mut acc = entries[i];

        if acc > target {
            continue;
        }

        for j in i + 1..entries.len() {
            acc += entries[j];

            match acc {
                acc if acc == target => {
                    let range = &entries[i..j + 1];
                    return range.iter().min().unwrap() + range.iter().max().unwrap();
                }
                acc if acc > target => break,
                _ => (),
            };
        }
    }

    unreachable!();
}

#[aoc(day9, part2)]
fn part2(entries: &[Entry]) -> i64 {
    encryption_weakness(entries, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part1_example() {
        let first_25: Vec<_> = (1..=25).collect();

        assert!(is_valid_entry(&26, &first_25));
        assert!(is_valid_entry(&49, &first_25));
        assert!(!is_valid_entry(&100, &first_25));
        assert!(!is_valid_entry(&50, &first_25));

        let mut other_25: Vec<_> = (1..=19).collect();
        other_25.extend(21..=25);
        other_25.push(45);

        assert!(is_valid_entry(&26, &other_25));
        assert!(!is_valid_entry(&65, &other_25));
        assert!(is_valid_entry(&64, &other_25));
        assert!(is_valid_entry(&66, &other_25));

        assert_eq!(
            first_invalid_entry(&parse_input(TEST_INPUT).unwrap(), 5),
            Some(127)
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            encryption_weakness(&parse_input(TEST_INPUT).unwrap(), 5),
            62
        );
    }
}
