use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

fn parse_group(group: &str) -> Vec<HashSet<char>> {
    group.lines().map(|line| line.chars().collect()).collect()
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|group| parse_group(group))
        .collect()
}

fn count_any_yes(group: &[HashSet<char>]) -> usize {
    group
        .iter()
        .fold(group[0].clone(), |union, individual| {
            union.union(individual).cloned().collect()
        })
        .len()
}

#[aoc(day6, part1)]
fn part1(groups: &[Vec<HashSet<char>>]) -> usize {
    groups.iter().map(|group| count_any_yes(group)).sum()
}

fn count_all_yes(group: &[HashSet<char>]) -> usize {
    group
        .iter()
        .fold(group[0].clone(), |union, individual| {
            union.intersection(individual).cloned().collect()
        })
        .len()
}

#[aoc(day6, part2)]
fn part2(groups: &[Vec<HashSet<char>>]) -> usize {
    groups.iter().map(|group| count_all_yes(group)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_GROUP_1: &str = "abc";
    static TEST_GROUP_2: &str = r"a
b
c";
    static TEST_GROUP_3: &str = r"ab
ac";
    static TEST_GROUP_4: &str = r"a
a
a
a";
    static TEST_GROUP_5: &str = "b";
    static TEST_INPUT: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1_example() {
        assert_eq!(
            count_any_yes(&parse_group(
                r"abcx
abcy
abcz"
            )),
            6
        );

        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_1)), 3);
        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_2)), 3);
        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_3)), 3);
        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_4)), 1);
        assert_eq!(count_any_yes(&parse_group(TEST_GROUP_5)), 1);

        assert_eq!(part1(&parse_input(TEST_INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_1)), 3);
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_2)), 0);
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_3)), 1);
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_4)), 1);
        assert_eq!(count_all_yes(&parse_group(TEST_GROUP_5)), 1);

        assert_eq!(part2(&parse_input(TEST_INPUT)), 6);
    }
}
