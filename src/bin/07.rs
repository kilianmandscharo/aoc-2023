use std::{cmp::Ordering, cmp::PartialEq, collections::HashMap, iter::zip};

use aoc::readlines;

fn main() {
    a();
    b();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn get_hand_type_from_card_counts(counts: &mut [i32]) -> HandType {
    counts.sort_by(|a, b| a.cmp(b));
    match counts {
        [5] => return HandType::FiveKind,
        [1, 4] => return HandType::FourKind,
        [2, 3] => return HandType::FullHouse,
        [1, 1, 3] => return HandType::ThreeKind,
        [1, 2, 2] => return HandType::TwoPair,
        [1, 1, 1, 2] => return HandType::OnePair,
        [1, 1, 1, 1, 1] => return HandType::HighCard,
        _ => panic!(),
    }
}

fn get_hand_type(s: &str) -> HandType {
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    let mut counts: Vec<_> = map.values().cloned().collect();
    get_hand_type_from_card_counts(&mut counts)
}

fn get_hand_type_with_joker(s: &str) -> HandType {
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    let jokers = if map.contains_key(&'J') { map[&'J'] } else { 0 };

    if jokers != 0 {
        map.remove(&'J');
        if let Some(max_entry) = map
            .iter()
            .max_by_key(|&(_, value)| value)
            .map(|(key, _)| key)
        {
            let new_count = map[max_entry] + jokers;
            map.entry(*max_entry).and_modify(|count| *count = new_count);
        } else {
            // Five Jokers
            return HandType::FiveKind;
        }
    }

    let mut counts: Vec<_> = map.values().cloned().collect();
    get_hand_type_from_card_counts(&mut counts)
}

fn compare_hands_by_type(
    first: &str,
    second: &str,
    first_type: HandType,
    second_type: HandType,
    cards: &[char],
) -> Ordering {
    if first_type > second_type {
        Ordering::Greater
    } else if second_type > first_type {
        Ordering::Less
    } else {
        for (c_first, c_second) in zip(first.chars(), second.chars()) {
            let c_first_val = cards.iter().position(|c| *c == c_first).unwrap();
            let c_second_val = cards.iter().position(|c| *c == c_second).unwrap();
            if c_first_val > c_second_val {
                return Ordering::Greater;
            } else if c_second_val > c_first_val {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

fn compare_hands(first: &str, second: &str) -> Ordering {
    let cards = vec![
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    let first_type = get_hand_type(first);
    let second_type = get_hand_type(second);

    return compare_hands_by_type(first, second, first_type, second_type, &cards);
}

fn compare_hands_with_joker(first: &str, second: &str) -> Ordering {
    let cards = vec![
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    let first_type = get_hand_type_with_joker(first);
    let second_type = get_hand_type_with_joker(second);

    return compare_hands_by_type(first, second, first_type, second_type, &cards);
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bet: i32,
    with_jokers: bool,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.with_jokers {
            true => return compare_hands_with_joker(&self.hand, &other.hand),
            false => return compare_hands(&self.hand, &other.hand),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.with_jokers {
            true => return Some(compare_hands_with_joker(&self.hand, &other.hand)),
            false => return Some(compare_hands(&self.hand, &other.hand)),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.with_jokers {
            true => return compare_hands_with_joker(&self.hand, &other.hand) == Ordering::Equal,
            false => return compare_hands(&self.hand, &other.hand) == Ordering::Equal,
        }
    }
}

impl Eq for Hand {}

fn get_winnings(with_jokers: bool) -> i32 {
    let mut hands: Vec<Hand> = Vec::new();
    for line in readlines("07") {
        let (hand, bet) = line.split_once(" ").unwrap();
        hands.push(Hand {
            hand: hand.to_owned(),
            bet: bet.parse::<i32>().unwrap(),
            with_jokers,
        })
    }
    hands.sort();
    return hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as i32 + 1) * h.bet)
        .fold(0, |acc, val| acc + val);
}

fn a() {
    println!("{}", get_winnings(false));
}

fn b() {
    println!("{}", get_winnings(true));
}
