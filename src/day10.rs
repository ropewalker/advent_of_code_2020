use aoc_runner_derive::{aoc, aoc_generator};

type Joltage = u64;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Joltage> {
    let mut entries: Vec<Joltage> = input.lines().map(|l| l.parse().unwrap()).collect();
    entries.push(0);
    entries.sort_unstable();
    entries.push(entries.last().unwrap() + 3);
    entries
}

fn count_ones_and_threes(entries: &[Joltage]) -> (usize, usize) {
    entries
        .iter()
        .enumerate()
        .skip(1)
        .fold((0, 0), |(ones, threes), (i, entry)| {
            match entry - entries[i - 1] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            }
        })
}

#[aoc(day10, part1)]
fn part1(entries: &[Joltage]) -> usize {
    let (ones, threes) = count_ones_and_threes(entries);
    ones * threes
}

fn count_arrangements(cluster: &[u64]) -> usize {
    if cluster.len() <= 1 {
        return 1;
    }

    let start = cluster[0];
    let target = cluster[cluster.len() - 1];
    let mut count = 0;

    for i in 1usize..=3 {
        let next = cluster[i];

        if next - start > 3 {
            break;
        } else if next == target {
            count += 1;
            break;
        } else {
            count += count_arrangements(&cluster[i..]);
        }
    }

    count
}

#[aoc(day10, part2)]
fn part2(entries: &[Joltage]) -> usize {
    let mut start = 0;
    let mut end = 1;
    let mut count = 1;

    loop {
        while entries[end] - entries[end - 1] < 3 && end + 1 < entries.len() {
            end += 1;
        }

        count *= count_arrangements(&entries[start..end]);
        start = end;

        while entries[start] - entries[start - 1] >= 3 {
            start += 1;

            if start >= entries.len() {
                return count;
            }
        }

        end = start;
        start -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SMALL_TEST_INPUT: &str = r"16
10
15
5
1
11
7
19
6
12
4";
    static LARGER_TEST_INPUT: &str = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn part1_example() {
        assert_eq!(
            count_ones_and_threes(&parse_input(SMALL_TEST_INPUT)),
            (7, 5)
        );
        assert_eq!(
            count_ones_and_threes(&parse_input(LARGER_TEST_INPUT)),
            (22, 10)
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(SMALL_TEST_INPUT)), 8);
        assert_eq!(part2(&parse_input(LARGER_TEST_INPUT)), 19_208);
    }
}
