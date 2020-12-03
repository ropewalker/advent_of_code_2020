use aoc_runner_derive::{aoc, aoc_generator};

fn parse_line(line: &str) -> PasswordEntry {
    let mut tokens = line.split_ascii_whitespace();
    let mut range_split = tokens.next().unwrap().split('-');
    let min = range_split.next().unwrap().parse::<usize>().unwrap();
    let max = range_split.next().unwrap().parse::<usize>().unwrap();
    let character = tokens.next().unwrap().chars().next().unwrap();
    let password = tokens.next().unwrap().to_string();

    PasswordEntry {
        min,
        max,
        character,
        password,
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<PasswordEntry> {
    input
        .lines()
        .map(|line| -> _ { parse_line(line) })
        .collect()
}

fn validate_password_part1(entry: &PasswordEntry) -> bool {
    let count = entry
        .password
        .chars()
        .filter(|c| *c == entry.character)
        .count();
    count >= entry.min && count <= entry.max
}

#[aoc(day2, part1)]
fn part1(passwords: &[PasswordEntry]) -> usize {
    passwords
        .iter()
        .filter(|&entry| validate_password_part1(entry))
        .count()
}

fn contains_nth(string: &str, character: &char, index: &usize) -> bool {
    if let Some(c) = string.chars().nth(index - 1) {
        c == *character
    } else {
        false
    }
}

fn validate_password_part2(entry: &PasswordEntry) -> bool {
    let contains_min = contains_nth(&entry.password, &entry.character, &entry.max);
    let contains_max = contains_nth(&entry.password, &entry.character, &entry.min);

    (contains_min || contains_max) && !(contains_min && contains_max)
}

#[aoc(day2, part2)]
fn part2(passwords: &[PasswordEntry]) -> usize {
    passwords
        .iter()
        .filter(|&entry| validate_password_part2(entry))
        .count()
}

struct PasswordEntry {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert!(validate_password_part1(&parse_line("1-3 a: abcde")));
    }

    #[test]
    fn part1_example2() {
        assert!(!validate_password_part1(&parse_line("1-3 b: cdefg")));
    }

    #[test]
    fn part1_example3() {
        assert!(validate_password_part1(&parse_line("2-9 c: ccccccccc")));
    }

    #[test]
    fn part2_example1() {
        assert!(validate_password_part2(&parse_line("1-3 a: abcde")));
    }

    #[test]
    fn part2_example2() {
        assert!(!validate_password_part2(&parse_line("1-3 b: cdefg")));
    }

    #[test]
    fn part2_example3() {
        assert!(!validate_password_part2(&parse_line("2-9 c: ccccccccc")));
    }
}
