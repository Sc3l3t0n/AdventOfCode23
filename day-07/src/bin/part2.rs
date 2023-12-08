use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    println!("Part 2");
    let input = include_str!("./input2.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        use HandType::*;

        let mut card_counts: HashMap<u8, u8> = HashMap::new();
        for char in value.chars() {
            let card_value = Hand::parse_card(&char);
            *card_counts.entry(card_value).or_insert(0) += 1;
        }

        let joker_count = *card_counts.iter().find(|&(&k, _)| k == 1).unwrap_or((&1, &0)).1;

        let hand_type = match card_counts.values().max() {
            Some(&5) => FiveOfAKind,
            Some(&4) => {

                    FourOfAKind
            }
            Some(&3) => {
                if let Some(_) = card_counts.iter().find(|&(_, &v)| v == 2) {
                    FullHouse
                } else {
                    ThreeOfAKind
                }
            }
            Some(&2) => {
                let pairs: Vec<(&u8, &u8)> =
                    card_counts.iter().filter(|&(_, &v)| v == 2).collect();
                if pairs.len() == 2 {
                    TwoPair
                } else {
                    OnePair
                }
            }
            Some(&1) => {
                HighCard
            }
            _ => unreachable!("LOl"),
        };

        match hand_type {
            FiveOfAKind => FiveOfAKind,
            FourOfAKind if joker_count >= 1 => FiveOfAKind,
            FullHouse if joker_count >= 2 => FiveOfAKind,
            ThreeOfAKind if joker_count > 0 => FourOfAKind,
            TwoPair if joker_count == 1 => FullHouse,
            TwoPair if joker_count == 2 => FourOfAKind,
            OnePair if joker_count > 0 => ThreeOfAKind,
            HighCard if joker_count > 0 => OnePair,
            _ => hand_type,
        }
    }
}

struct Hand {
    hand_type: HandType,
    cards: Vec<u8>,
    points: u32,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut splitted = value.split_whitespace();
        let hand = splitted.next().expect("Should be in right format.");
        let points: u32 = splitted
            .next()
            .expect("Should be in right format.")
            .parse()
            .expect("Should be a number");
        let hand_type = HandType::from(hand);
        let mut cards = Vec::new();
        for char in hand.chars() {
            cards.push(Self::parse_card(&char));
        }
        Self {
            hand_type,
            cards,
            points,
        }
    }
}

impl Hand {
    fn parse_card(card: &char) -> u8 {
        match card {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card.to_digit(10).expect("Should be a number.") as u8,
        }
    }
}


fn solve(input: &str) -> String {
    let mut hands = parse_hands(input);
    hands.sort_by(|a, b| compare_hands(a, b));
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, hand)| {
            let rank = idx + 1;
            rank * hand.points as usize
        })
        .sum::<usize>().to_string()
}

fn parse_hands(input: &str) -> Vec<Hand> {
    let mut result: Vec<_> = Vec::new();
    for line in input.lines() {
        result.push(Hand::from(line));
    }
    result
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    if a.hand_type > b.hand_type {
        return Ordering::Greater;
    } else if a.hand_type == b.hand_type {
        for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
            if card_a == card_b {
                continue;
            }

            if card_a > card_b {
                return Ordering::Less;
            } else if card_a < card_b {
                return Ordering::Greater;
            }
        }
        unreachable!("There should be no equal hands")
    }
    return Ordering::Less;
}

#[cfg(test)]
mod tests {
    use crate::{solve, HandType};

    #[test]
    fn example_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let output = solve(input);
        assert_eq!(output, "5905".to_string());
    }

    #[test]
    fn hand_type_test() {
        use HandType::*;
        let cases = vec![
            ("AAAAA", FiveOfAKind),
            ("AAAA2", FourOfAKind),
            ("AAA23", ThreeOfAKind),
            ("AAA22", FullHouse),
            ("AA223", TwoPair),
            ("AA423", OnePair),
            ("A7234", HighCard),
        ];
        for case in cases {
            assert_eq!(HandType::from(case.0), case.1);
        }
    }
}

