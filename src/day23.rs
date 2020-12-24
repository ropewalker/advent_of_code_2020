use aoc_runner_derive::{aoc, aoc_generator};

type Cup = u32;

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<Cup> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn r#move(cups: &mut [Cup]) {
    let max_cup = cups.len() - 1;
    let current_cup = cups[0];

    let wrap = |cup: u32| -> u32 { ((cup as usize + (max_cup - 1)) % max_cup + 1) as u32 };

    let next = |cup: u32| -> u32 {
        if cups[cup as usize] != 0 {
            cups[cup as usize]
        } else {
            wrap(cup + 1)
        }
    };

    let mut four_cups = [0; 4];
    let mut start = current_cup;

    for cup in &mut four_cups {
        start = next(start);
        *cup = start;
    }

    let mut destination_cup = wrap(current_cup - 1);

    while four_cups[0..3].contains(&destination_cup) {
        destination_cup = wrap(destination_cup - 1);
    }

    let next_after_destination = next(destination_cup);
    let [first, _, third, fourth] = four_cups;

    cups[0] = fourth;
    cups[current_cup as usize] = fourth;
    cups[destination_cup as usize] = first;
    cups[third as usize] = next_after_destination;
}

fn move_few_cups(cups: &[Cup], moves: usize) -> [u32; 10] {
    let mut next_cups = [0; 10];

    next_cups[0] = cups[0];

    for i in 0..cups.len() - 1 {
        next_cups[cups[i] as usize] = cups[i + 1];
    }

    next_cups[cups[cups.len() - 1] as usize] = cups[0];

    for _ in 0..moves {
        r#move(&mut next_cups);
    }

    next_cups
}

fn labels_after_first(next_cups: &[u32; 10]) -> String {
    let mut next_cup = next_cups[1];
    let mut result = String::new();

    while next_cup != 1 {
        result.push_str(&next_cup.to_string());
        next_cup = next_cups[next_cup as usize];
    }

    result
}

#[aoc(day23, part1)]
fn part1(cups: &[Cup]) -> String {
    labels_after_first(&move_few_cups(cups, 100))
}

#[aoc(day23, part2)]
fn part2(cups: &[Cup]) -> u64 {
    let mut first_next_cups = [0; 10];

    first_next_cups[0] = cups[0];

    for i in 0..cups.len() - 1 {
        first_next_cups[cups[i] as usize] = cups[i + 1];
    }

    first_next_cups[cups[cups.len() - 1] as usize] = (cups.len() + 1) as u32;

    let mut all_next_cups: Vec<Cup> = Vec::with_capacity(1_000_000 + 1);
    all_next_cups.extend(&first_next_cups);

    for i in 10..1_000_000 {
        all_next_cups.push(i + 1);
    }

    all_next_cups.push(cups[0]);

    for _ in 0..10_000_000 {
        r#move(&mut all_next_cups);
    }

    let first = all_next_cups[1];
    let second = all_next_cups[first as usize];

    first as u64 * second as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"389125467";

    #[test]
    fn part1_example() {
        assert_eq!(
            labels_after_first(&move_few_cups(&parse_input(TEST_INPUT), 10)),
            "92658374"
        );
        assert_eq!(part1(&parse_input(TEST_INPUT)), "67384529");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 149_245_887_792);
    }
}
