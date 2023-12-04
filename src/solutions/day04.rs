use std::cmp;
use std::collections::{HashSet};

static EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

struct Card {
    winning_numbers: HashSet<u32>,
    chosen_numbers: HashSet<u32>,
}

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();
    let cards: Vec<Card> = data.lines().map(parse_card).collect();
    return if advanced {
        let solve_for_index = |i| solve_advanced_for_cards(&cards, i);
        let points: u32 = (0..cards.len()).map(solve_for_index).sum();
        format!("{}", points)
    } else {
        let points: u32 = cards.iter().map(solve_simple_for_card).sum();
        format!("{}", points)
    }
}

fn parse_card(card: &str) -> Card {
    let all_numbers = card.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>();
    let winning_numbers = parse_number_series(all_numbers[0]);
    let chosen_numbers: HashSet<u32> = parse_number_series(all_numbers[1]);
    Card {
        winning_numbers,
        chosen_numbers,
    }
}

fn solve_simple_for_card(card: &Card) -> u32 {
    let count = card.chosen_numbers.intersection(&card.winning_numbers).count() as u32;
    return if count > 0 {
        2u32.pow(count - 1)
    } else {
        0
    }
}

fn solve_advanced_for_cards(cards: &Vec<Card>, index: usize) -> u32 {
    // Somewhat inefficient algorithm, but who cares, this is AoC.
    let card = &cards[index];
    let matches = card.chosen_numbers.intersection(&card.winning_numbers).count();

    return if matches > 0 {
        let start_index = index + 1;
        let end_index = cmp::min(cards.len(), start_index + matches);
        let won_cards = start_index..end_index;
        1u32 + won_cards.map(|i| solve_advanced_for_cards(cards, i,)).sum::<u32>()
    } else {
        1
    }
}

fn parse_number_series(series: &str) -> HashSet<u32> {
    if series.len() > 0 {
        series.split(" ").filter(|n| !n.is_empty()).map(|n: &str| n.parse::<u32>().unwrap()).collect()
    } else {
        HashSet::new()
    }
}