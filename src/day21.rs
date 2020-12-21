use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Ingredient = String;
type Allergen = String;

struct Food {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

fn parse_food(food_str: &str) -> Food {
    let mut split = food_str.split(" (contains ");

    let ingredients = split
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|ingredient| ingredient.to_string())
        .collect();

    let allergens = split
        .next()
        .unwrap()
        .trim_end_matches(')')
        .split(", ")
        .map(|allergen| allergen.to_string())
        .collect();

    Food {
        ingredients,
        allergens,
    }
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Vec<Food> {
    input.lines().map(|line| parse_food(line)).collect()
}

fn compile_dictionary_draft(shopping_list: &[Food]) -> HashMap<&str, HashSet<&str>> {
    let mut allergens_to_possible_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();

    let all_allergens: HashSet<_> = shopping_list
        .iter()
        .map(|food| &food.allergens)
        .flatten()
        .collect();

    for allergen in all_allergens {
        for food in shopping_list {
            if food.allergens.contains(allergen) {
                if let Some(translations) =
                    allergens_to_possible_ingredients.get_mut(allergen.as_str())
                {
                    *translations = (*translations)
                        .intersection(
                            &food
                                .ingredients
                                .iter()
                                .map(|ingredient| ingredient.as_str())
                                .collect(),
                        )
                        .cloned()
                        .collect();
                } else {
                    allergens_to_possible_ingredients.insert(
                        allergen.as_str(),
                        food.ingredients
                            .iter()
                            .map(|ingredient| ingredient.as_str())
                            .collect(),
                    );
                }
            }
        }
    }

    allergens_to_possible_ingredients
}

#[aoc(day21, part1)]
fn part1(shopping_list: &[Food]) -> usize {
    let allergens_to_possible_ingredients = compile_dictionary_draft(shopping_list);

    let ingredients_with_allergens: HashSet<_> = allergens_to_possible_ingredients
        .values()
        .flatten()
        .cloned()
        .collect();

    let all_ingredients: HashSet<_> = shopping_list
        .iter()
        .map(|food| &food.ingredients)
        .flatten()
        .map(|ingredient| ingredient.as_str())
        .collect();

    let non_allergenic: HashSet<_> = all_ingredients
        .difference(&ingredients_with_allergens)
        .cloned()
        .collect();

    let mut count = 0;

    for ingredient in non_allergenic.iter() {
        count += shopping_list
            .iter()
            .filter(|food| food.ingredients.contains(&ingredient.to_string()))
            .count();
    }

    count
}

#[aoc(day21, part2)]
fn part2(shopping_list: &[Food]) -> String {
    let mut allergens_to_possible_ingredients = compile_dictionary_draft(shopping_list);

    let mut dictionary: Vec<(&str, &str)> = Vec::new();

    while !allergens_to_possible_ingredients.is_empty() {
        let mut ingredient = "";

        for (&allergen, possible_ingredients) in allergens_to_possible_ingredients.iter_mut() {
            if possible_ingredients.len() == 1 {
                ingredient = possible_ingredients.drain().next().unwrap();
                dictionary.push((&allergen, &ingredient));

                allergens_to_possible_ingredients.remove(&allergen);
                break;
            }
        }

        for (_, possible_ingredients) in allergens_to_possible_ingredients.iter_mut() {
            possible_ingredients.remove(&ingredient);
        }
    }

    dictionary.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut canonical_dangerous_ingredient_list =
        dictionary
            .iter()
            .fold(String::new(), |mut acc, (_, ingredient)| {
                acc.push_str(ingredient);
                acc.push(',');
                acc
            });

    canonical_dangerous_ingredient_list = canonical_dangerous_ingredient_list
        .trim_end_matches(',')
        .to_string();

    canonical_dangerous_ingredient_list
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse_input(TEST_INPUT)),
            "mxmxvkd,sqjhc,fvjkl".to_string()
        );
    }
}
