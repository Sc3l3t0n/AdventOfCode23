fn main() {
    println!("Part 2");
    let input = include_str!("./input2.txt");
    let result = solve(input);
    println!("Result = {}", result);
}

struct Card {
    winning: Vec<u16>,
    having: Vec<u16>,
    copies: u32,
}

impl Card {
    pub fn from(winning: Vec<u16>, having: Vec<u16>) -> Self {
        Self {
            winning,
            having,
            copies: 1,
        }
    }
    pub fn get_matched(&self) -> u32 {
        let mut matched = 0;
        for number in &self.having {
            if self.winning.contains(number) {
                matched += 1;
            }
        }
        matched
    }
    pub fn incr_copies(&mut self) {
        self.copies += 1;
    }
}

fn solve(input: &str) -> String {
    let mut cards = parse_cards(input);
    calculate_copies(&mut cards);
    sum_card_copies(&cards).to_string()
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<_> = Vec::new();
    for line in input.lines() {
        let card = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: Vec<u16> = card
            .0
            .trim()
            .split_ascii_whitespace()
            .filter(|x| x.chars().all(|c| c.is_ascii_digit()))
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        let having: Vec<u16> = card
            .1
            .trim()
            .split_ascii_whitespace()
            .filter(|x| x.chars().all(|c| c.is_ascii_digit()))
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        cards.append(&mut vec![Card::from(winning, having)]);
    }
    cards
}

fn calculate_copies(cards: &mut Vec<Card>) {
    for i in 0..cards.len() {
        for _ in 0..cards[i].copies {
            for j in 1..cards[i].get_matched() as usize + 1 {
                cards.get_mut(i + j).unwrap().incr_copies();
            }
        }
    }
}

fn sum_card_copies(cards: &Vec<Card>) -> u32 {
    let mut copies = 0;
    for card in cards {
        copies += card.copies;
    }
    copies
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let output = solve(input);
        assert_eq!(output, "30".to_string());
    }
}

