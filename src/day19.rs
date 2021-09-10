use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

enum Rule {
    Character(char),
    Product(Vec<usize>),
    Sum(Vec<Vec<usize>>),
}

fn validate<'a>(message: &'a str, rules: &Rules, rule: &Rule) -> Result<&'a str, ()> {
    use Rule::*;

    match rule {
        Character(c) => {
            if message.starts_with(*c) {
                Ok(&message[1..])
            } else {
                Err(())
            }
        }
        Product(rules_numbers) => {
            let mut remainder = message;

            for number in rules_numbers {
                remainder = validate(remainder, rules, &rules[number])?;
            }

            Ok(remainder)
        }
        Sum(products) => {
            for product in products {
                let result = validate(message, rules, &Product(product.clone()));

                if result.is_ok() {
                    return result;
                }
            }

            Err(())
        }
    }
}

type Rules = HashMap<usize, Rule>;

fn parse_rule(rule_str: &str) -> Rule {
    use Rule::*;

    if rule_str.starts_with('"') {
        Character(rule_str.chars().nth(1).unwrap())
    } else if rule_str.contains('|') {
        Sum(rule_str
            .split('|')
            .map(|product_str| {
                product_str
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            })
            .collect())
    } else {
        Product(
            rule_str
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect(),
        )
    }
}

fn parse_rules(rules_str: &str) -> Rules {
    rules_str
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                parse_rule(split.next().unwrap()),
            )
        })
        .collect()
}

type Message = String;
type Messages = Vec<Message>;

fn parse_messages(messages_str: &str) -> Messages {
    messages_str.lines().map(|line| line.to_string()).collect()
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> (Rules, Messages) {
    let mut split = input.split("\n\n");

    (
        parse_rules(split.next().unwrap()),
        parse_messages(split.next().unwrap()),
    )
}

#[aoc(day19, part1)]
fn part1((rules, messages): &(Rules, Messages)) -> usize {
    messages
        .iter()
        .filter(|&message| {
            if let Ok(remainder) = validate(message, rules, &rules[&0]) {
                remainder.is_empty()
            } else {
                false
            }
        })
        .count()
}

#[aoc(day19, part2)]
fn part2((rules, messages): &(Rules, Messages)) -> usize {
    messages
        .iter()
        .filter(|&message| {
            let mut count_42 = 0;
            let mut remainder = message.as_str();
            let mut result = validate(remainder, rules, &rules[&42]);

            while let Ok(new_remainder) = result {
                count_42 += 1;
                remainder = new_remainder;
                result = validate(remainder, rules, &rules[&42]);
            }

            if count_42 < 2 {
                return false;
            }

            let mut count_31 = 0;
            result = validate(remainder, rules, &rules[&31]);

            while let Ok(new_remainder) = result {
                count_31 += 1;
                remainder = new_remainder;
                result = validate(remainder, rules, &rules[&31]);
            }

            remainder.is_empty() && count_31 > 0 && count_42 > count_31
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse_input(
                r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

aab
aba"#
            )),
            2
        );
        assert_eq!(
            part1(&parse_input(
                r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        static TEST_INPUT: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

        assert_eq!(part1(&parse_input(TEST_INPUT)), 3);
        assert_eq!(part2(&parse_input(TEST_INPUT)), 12);
    }
}
