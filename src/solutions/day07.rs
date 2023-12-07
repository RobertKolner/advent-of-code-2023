const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

type Hand = Vec<u8>;
type Bid = u64;

pub fn solve(input_data: Option<String>, advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).to_string();
    let mut hands: Vec<(Hand, Bid)> = data.lines().filter(|l| l.len() > 0).map(|l| parse_line(l, advanced)).collect();
    hands.sort_by(hands_cmp);

    let result = hands
        .iter()
        .map(|(_, bid)| bid)
        .enumerate()
        .fold(0, |acc, (index, bid)| acc + *bid * (index as u64 + 1));

    format!("{}", result)
}

fn parse_line(line: &str, advanced: bool) -> (Hand, Bid) {
    let (cards, bid) = line.split_once(" ").unwrap();
    let card_values = cards.chars().map(|c| match c {
        'T' => 10,
        'J' => if advanced {1} else {11},
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as u8
    });
    (card_values.collect(), bid.parse::<Bid>().unwrap())
}

fn hands_cmp(t1: &(Hand, Bid), t2: &(Hand, Bid)) -> std::cmp::Ordering {
    let h1 = t1.clone().0;
    let h2 = t2.clone().0;
    let cmp = hand_strength(&h1).cmp(&hand_strength(&h2));
    if cmp != std::cmp::Ordering::Equal {
        return cmp;
    }

    for (v1, v2) in h1.iter().zip(h2.iter()) {
        if v1 > v2 {
            return std::cmp::Ordering::Greater;
        }
        if v1 < v2 {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Equal;
}

fn hand_strength(hand: &Hand) -> u64 {
    let mut map = std::collections::HashMap::<u8, u64>::new();
    let mut joker_count = 0u64;
    for card in hand {
        if *card > 1 {
            *map.entry(*card).or_insert(0) += 1
        } else {
            joker_count += 1
        }
    }
    let mut groups: Vec<u64> = map.values().filter(|v| **v > 0).map(|v| *v).collect();
    groups.sort();
    groups.reverse();

    if groups.len() == 0 {
        // all jokers!
        groups = vec![5];
    } else {
        groups[0] += joker_count;
    }

    return match groups[..] {
        [5, ..] => 100,
        [4, ..] => 50,
        [3, 2, ..] => 25,
        [3, 1, ..] => 9,
        [2, 2, ..] => 6,
        [2, ..] => 4,
        _ => 0,
    };
}