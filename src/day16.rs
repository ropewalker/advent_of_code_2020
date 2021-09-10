use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

type Rule = (RangeInclusive<usize>, RangeInclusive<usize>);
type FieldName = String;
type RuleEntry = (FieldName, Rule);
type Rules = HashMap<FieldName, Rule>;
type RawTicket = Vec<usize>;

struct Notes {
    rules: Rules,
    tickets: Vec<RawTicket>,
}

fn parse_rule(rule_str: &str) -> RuleEntry {
    let mut key_value_split = rule_str.split(": ");
    let field_name = key_value_split.next().unwrap().to_string();
    let mut range_split = key_value_split.next().unwrap().split(" or ");

    let range_str = range_split.next().unwrap();
    let mut boundaries_split = range_str.split('-');
    let lower_bounds = RangeInclusive::new(
        boundaries_split.next().unwrap().parse::<usize>().unwrap(),
        boundaries_split.next().unwrap().parse::<usize>().unwrap(),
    );

    let range_str = range_split.next().unwrap();
    let mut boundaries_split = range_str.split('-');
    let upper_bounds = RangeInclusive::new(
        boundaries_split.next().unwrap().parse::<usize>().unwrap(),
        boundaries_split.next().unwrap().parse::<usize>().unwrap(),
    );

    (field_name, (lower_bounds, upper_bounds))
}

fn parse_rules(rules_str: &str) -> Rules {
    rules_str
        .lines()
        .map(|rule_str| parse_rule(rule_str))
        .collect()
}

fn parse_raw_ticket(raw_ticket_str: &str) -> RawTicket {
    raw_ticket_str
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Notes {
    let mut notes_split = input.split("\n\n");

    let rules = parse_rules(notes_split.next().unwrap());

    let mut tickets = vec![parse_raw_ticket(
        notes_split.next().unwrap().lines().nth(1).unwrap(),
    )];

    tickets.extend(
        notes_split
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|raw_ticket_str| parse_raw_ticket(raw_ticket_str)),
    );

    Notes { rules, tickets }
}

#[aoc(day16, part1)]
fn part1(notes: &Notes) -> usize {
    let mut united_rule = vec![false; 1000];

    for (i, is_valid) in united_rule.iter_mut().enumerate() {
        for rule in notes.rules.values() {
            if apply_rule(rule, &i) {
                *is_valid = true;
                break;
            }
        }
    }

    notes
        .tickets
        .iter()
        .skip(1)
        .map(|raw_ticket| {
            raw_ticket
                .iter()
                .filter(|&value| !united_rule[*value])
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn apply_rule(rule: &Rule, value: &usize) -> bool {
    rule.0.contains(value) || rule.1.contains(value)
}

fn get_correct_tickets(notes: &Notes) -> Vec<RawTicket> {
    let mut united_rule = vec![false; 1000];

    for (i, is_valid) in united_rule.iter_mut().enumerate() {
        for rule in notes.rules.values() {
            if apply_rule(rule, &i) {
                *is_valid = true;
                break;
            }
        }
    }

    notes
        .tickets
        .iter()
        .filter(|&raw_ticket| raw_ticket.iter().all(|value| united_rule[*value]))
        .cloned()
        .collect()
}

fn get_possible_positions_by_field(notes: &Notes) -> HashMap<&FieldName, HashSet<usize>> {
    let correct_tickets = get_correct_tickets(notes);

    notes
        .rules
        .iter()
        .map(|(field_name, rule)| {
            (
                field_name,
                (0..notes.rules.len())
                    .filter(|i| {
                        correct_tickets
                            .iter()
                            .all(|ticket| apply_rule(rule, &ticket[*i]))
                    })
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn get_actual_positions_by_field(notes: &Notes) -> HashMap<&FieldName, usize> {
    let mut all_possible_positions = get_possible_positions_by_field(notes);
    let mut actual_positions = HashMap::with_capacity(notes.rules.len());

    while !all_possible_positions.is_empty() {
        let mut position = 0;

        for (&field_name, possible_field_positions) in all_possible_positions.iter_mut() {
            if possible_field_positions.len() == 1 {
                position = possible_field_positions.drain().next().unwrap();
                actual_positions.insert(field_name, position);
                all_possible_positions.remove(field_name);
                break;
            }
        }

        for (_, possible_field_positions) in all_possible_positions.iter_mut() {
            possible_field_positions.remove(&position);
        }
    }

    actual_positions
}

#[aoc(day16, part2)]
fn part2(notes: &Notes) -> usize {
    get_actual_positions_by_field(notes)
        .into_iter()
        .filter(|(field_name, _)| field_name.starts_with("departure"))
        .map(|(_, position)| notes.tickets[0][position])
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                &(parse_input(
                    r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
                ))
            ),
            71
        );
    }

    #[test]
    fn part2_example() {
        let notes = parse_input(
            r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9",
        );

        let positions_by_field = get_actual_positions_by_field(&(notes));

        assert_eq!(
            notes.tickets[0][*positions_by_field.get(&"class".to_string()).unwrap()],
            12
        );
        assert_eq!(
            notes.tickets[0][*positions_by_field.get(&"row".to_string()).unwrap()],
            11
        );
        assert_eq!(
            notes.tickets[0][*positions_by_field.get(&"seat".to_string()).unwrap()],
            13
        );
    }
}
