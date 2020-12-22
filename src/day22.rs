use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

type Card = u8;
type Deck = VecDeque<Card>;
type Score = usize;

#[derive(Eq, PartialEq)]
enum Player {
    Player1,
    Player2,
}

enum RoundResult {
    Playing,
    Victory(Player),
}

type GameStateHash = u64;

type GameStates = HashSet<GameStateHash>;

fn parse_deck(deck: &str) -> Deck {
    deck.lines()
        .skip(1)
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> (Deck, Deck) {
    let mut spilt = input.split("\n\n");

    (
        parse_deck(spilt.next().unwrap()),
        parse_deck(spilt.next().unwrap()),
    )
}

fn round(player1_deck: &mut Deck, player2_deck: &mut Deck) -> RoundResult {
    use Player::*;
    use RoundResult::*;

    if !player1_deck.is_empty() && !player2_deck.is_empty() {
        let player1_card = player1_deck.pop_front().unwrap();
        let player2_card = player2_deck.pop_front().unwrap();

        if player1_card > player2_card {
            player1_deck.push_back(player1_card);
            player1_deck.push_back(player2_card);
        } else {
            player2_deck.push_back(player2_card);
            player2_deck.push_back(player1_card);
        }

        Playing
    } else if player1_deck.is_empty() {
        Victory(Player2)
    } else if player2_deck.is_empty() {
        Victory(Player1)
    } else {
        unreachable!()
    }
}

fn count_score(winning_deck: &Deck) -> Score {
    winning_deck
        .iter()
        .enumerate()
        .map(|(index, card)| *card as usize * (winning_deck.len() - index))
        .sum()
}

fn play_combat(player1_deck: &mut Deck, player2_deck: &mut Deck) -> Player {
    use RoundResult::*;

    loop {
        if let Victory(winner) = round(player1_deck, player2_deck) {
            return winner;
        }
    }
}

#[aoc(day22, part1)]
fn part1((player1_deck, player2_deck): &(Deck, Deck)) -> Score {
    use Player::*;

    let mut player1_deck = player1_deck.clone();
    let mut player2_deck = player2_deck.clone();

    let winner = play_combat(&mut player1_deck, &mut player2_deck);

    match winner {
        Player1 => count_score(&player1_deck),
        Player2 => count_score(&player2_deck),
    }
}

fn recursive_round(
    player1_deck: &mut Deck,
    player2_deck: &mut Deck,
    game_states: &mut GameStates,
    quick_mode: bool,
) -> RoundResult {
    use Player::*;
    use RoundResult::*;

    if !player1_deck.is_empty() && !player2_deck.is_empty() {
        if quick_mode && player1_deck.iter().max().unwrap() > player2_deck.iter().max().unwrap() {
            return Victory(Player1);
        }

        let mut hasher = DefaultHasher::new();

        (&player1_deck, &player2_deck).hash(&mut hasher);
        let game_state = hasher.finish();

        if game_states.contains(&game_state) {
            return Victory(Player1);
        }

        game_states.insert(game_state);

        let player1_card = player1_deck.pop_front().unwrap();
        let player2_card = player2_deck.pop_front().unwrap();

        let round_winner = if player1_card as usize <= player1_deck.len()
            && player2_card as usize <= player2_deck.len()
        {
            play_recursive_combat(
                &mut player1_deck
                    .iter()
                    .take(player1_card as usize)
                    .cloned()
                    .collect(),
                &mut player2_deck
                    .iter()
                    .take(player2_card as usize)
                    .cloned()
                    .collect(),
                true,
            )
        } else if player1_card > player2_card {
            Player1
        } else {
            Player2
        };

        if round_winner == Player1 {
            player1_deck.push_back(player1_card);
            player1_deck.push_back(player2_card);
        } else {
            player2_deck.push_back(player2_card);
            player2_deck.push_back(player1_card);
        }

        Playing
    } else if player1_deck.is_empty() {
        Victory(Player2)
    } else if player2_deck.is_empty() {
        Victory(Player1)
    } else {
        unreachable!()
    }
}

fn play_recursive_combat(
    player1_deck: &mut Deck,
    player2_deck: &mut Deck,
    quick_mode: bool,
) -> Player {
    use RoundResult::*;

    let mut game_states = HashSet::new();

    loop {
        if let Victory(winner) =
            recursive_round(player1_deck, player2_deck, &mut game_states, quick_mode)
        {
            return winner;
        }
    }
}

#[aoc(day22, part2)]
fn part2((player1_deck, player2_deck): &(Deck, Deck)) -> Score {
    use Player::*;

    let mut player1_deck = player1_deck.clone();
    let mut player2_deck = player2_deck.clone();

    let winner = play_recursive_combat(&mut player1_deck, &mut player2_deck, false);

    match winner {
        Player1 => count_score(&player1_deck),
        Player2 => count_score(&player2_deck),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 306);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 291)
    }
}
