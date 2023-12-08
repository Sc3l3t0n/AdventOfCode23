use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    println!("Part 1");
    let input = include_str!("./input1.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind(u8),
    FourOfAKind(u8),
    FullHouse(u8, u8),
    ThreeOfAKind(u8),
    TwoPair(u8, u8),
    OnePair(u8),
    HighCard(u8),
}

impl HandType {
    fn power(&self) -> u8 {
        match self {
            Self::HighCard(_) => 1,
            Self::OnePair(_) => 2,
            Self::TwoPair(_, _) => 3,
            Self::ThreeOfAKind(_) => 4,
            Self::FullHouse(_, _) => 5,
            Self::FourOfAKind(_) => 6,
            Self::FiveOfAKind(_) => 7,
        }
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        use HandType::*;
        match (self, other) {
            (FiveOfAKind(rank1), FiveOfAKind(rank2)) => rank1 == rank2,
            (FourOfAKind(rank1), FourOfAKind(rank2)) => rank1 == rank2,
            (FullHouse(rank1a, rank1b), FullHouse(rank2a, rank2b)) => {
                rank1a == rank2a && rank1b == rank2b
            }
            (ThreeOfAKind(rank1), ThreeOfAKind(rank2)) => rank1 == rank2,
            (TwoPair(rank1a, rank1b), TwoPair(rank2a, rank2b)) => {
                rank1a == rank2a && rank1b == rank2b
            }
            (OnePair(rank1), OnePair(rank2)) => rank1 == rank2,
            (HighCard(rank1), HighCard(rank2)) => rank1 == rank2,
            _ => false,
        }
    }
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        let mut card_counts: HashMap<u8, u8> = HashMap::new();
        for char in value.chars() {
            let card_value = Hand::parse_card(&char);
            *card_counts.entry(card_value).or_insert(0) += 1;
        }

        match card_counts.values().max() {
            Some(&5) => Self::FiveOfAKind(*card_counts.iter().find(|&(_, &v)| v == 5).unwrap().0),
            Some(&4) => Self::FourOfAKind(*card_counts.iter().find(|&(_, &v)| v == 4).unwrap().0),
            Some(&3) => {
                if let Some(two) = card_counts.iter().find(|&(_, &v)| v == 2) {
                    Self::FullHouse(
                        *card_counts.iter().find(|&(_, &v)| v == 3).unwrap().0,
                        *two.0,
                    )
                } else {
                    Self::ThreeOfAKind(*card_counts.iter().find(|&(_, &v)| v == 3).unwrap().0)
                }
            }
            Some(&2) => {
                let mut pairs: Vec<(&u8, &u8)> =
                    card_counts.iter().filter(|&(_, &v)| v == 2).collect();
                if pairs.len() == 2 {
                    pairs.sort_by_key(|x| x.0);
                    Self::TwoPair(
                        *pairs.get(1).expect("Should be in right format").0,
                        *pairs.get(0).expect("Should be in right format").0,
                    )
                } else {
                    Self::OnePair(*card_counts.iter().find(|&(_, &v)| v == 2).unwrap().0)
                }
            }
            Some(&1) => {
                let mut cards: Vec<u8> = card_counts.iter().map(|x| *x.0).collect();
                cards.sort_unstable();
                Self::HighCard(*cards.get(4).expect("Should be in right format"))
            }
            _ => panic!("What the hell?"),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.power().cmp(&other.power())
    }
}

impl Eq for HandType {}

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
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card.to_digit(10).expect("Should be a number.") as u8,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (card_self, card_other) in self.cards.iter().zip(&other.cards) {
                    match card_self.cmp(card_other) {
                        Ordering::Equal => continue,
                        non_equal => return non_equal,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }
}

fn solve(input: &str) -> String {
    let mut hands = parse_hands(input);
    hands.sort_unstable();
    calculate_points(&hands).to_string()
}

fn parse_hands(input: &str) -> Vec<Hand> {
    let mut result: Vec<_> = Vec::new();
    for line in input.lines() {
        result.push(Hand::from(line));
    }
    result
}

fn calculate_points(hands: &[Hand]) -> u64 {
    let mut result: u64 = 0;
    for i in 0..hands.len() {
        result += hands.get(i).expect("Should be there").points as u64 * (i + 1) as u64;
    }
    result
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
        assert_eq!(output, "6440".to_string());
    }

    #[test]
    fn hand_type_test() {
        use HandType::*;
        let cases = vec![
            ("AAAAA", FiveOfAKind(14)),
            ("AAAA1", FourOfAKind(14)),
            ("AAA23", ThreeOfAKind(14)),
            ("AAA22", FullHouse(14, 2)),
            ("AA221", TwoPair(14, 2)),
            ("AA123", OnePair(14)),
            ("A1234", HighCard(14)),
        ];
        for case in cases {
            assert_eq!(HandType::from(case.0), case.1);
        }
    }
}

