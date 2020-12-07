use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash)]
struct Color {
    primary: String,
    shade: String,
}

type Rule = (Color, HashMap<Color, usize>);
type Rules = HashMap<Color, HashMap<Color, usize>>;

fn parse_rule(rule_str: &str) -> Rule {
    let mut rule_split = rule_str.split("contain ");

    let mut outer_bag_split = rule_split.next().unwrap().split_whitespace();
    let (shade, primary) = (
        outer_bag_split.next().unwrap().to_string(),
        outer_bag_split.next().unwrap().to_string(),
    );

    let outer_color = Color { primary, shade };

    let inner_bags_str = rule_split.next().unwrap();

    if inner_bags_str == "no other bags." {
        return (outer_color, HashMap::with_capacity(0));
    }

    let inner_bags_split = inner_bags_str.split(", ");
    let mut inner_colors_with_qty = HashMap::new();

    for inner_bag_str in inner_bags_split {
        let mut inner_bag_split = inner_bag_str.split_whitespace();
        let (qty, shade, primary) = (
            inner_bag_split.next().unwrap().parse::<usize>().unwrap(),
            inner_bag_split.next().unwrap().to_string(),
            inner_bag_split.next().unwrap().to_string(),
        );

        inner_colors_with_qty.insert(Color { primary, shade }, qty);
    }

    (outer_color, inner_colors_with_qty)
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Rules {
    input.lines().map(|rule_str| parse_rule(rule_str)).collect()
}

struct Node<'a> {
    color: &'a Color,
    previous_colors: Vec<&'a Color>,
}

#[aoc(day7, part1)]
fn part1(rules: &Rules) -> usize {
    let target_color = Color {
        primary: "gold".to_string(),
        shade: "shiny".to_string(),
    };

    let mut visited: HashSet<&Color> = HashSet::new();
    let mut stack: Vec<Node> = rules
        .keys()
        .map(|color| Node {
            color,
            previous_colors: Vec::new(),
        })
        .collect();
    let mut can_contain_target = HashSet::new();

    while !stack.is_empty() {
        let node = stack.pop().unwrap();

        if *node.color == target_color {
            can_contain_target.extend(node.previous_colors);
            continue;
        }

        if !visited.contains(node.color) {
            visited.insert(node.color);

            for color in rules.get(node.color).unwrap().keys() {
                let mut previous_colors = node.previous_colors.clone();
                previous_colors.push(node.color);

                stack.push(Node {
                    color,
                    previous_colors,
                })
            }
        } else if can_contain_target.contains(node.color) {
            can_contain_target.extend(node.previous_colors);
        }
    }

    can_contain_target.len()
}

fn count_inner_bags(target_color: &Color, rules: &HashMap<Color, HashMap<Color, usize>>) -> usize {
    rules
        .get(target_color)
        .unwrap()
        .iter()
        .map(|(color, &qty)| count_inner_bags(color, &rules) * qty + qty)
        .sum::<usize>()
}

#[aoc(day7, part2)]
fn part2(rules: &Rules) -> usize {
    let target_color = Color {
        primary: "gold".to_string(),
        shade: "shiny".to_string(),
    };

    count_inner_bags(&target_color, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    static TEST_INPUT_2: &str = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 32);
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 126);
    }
}
