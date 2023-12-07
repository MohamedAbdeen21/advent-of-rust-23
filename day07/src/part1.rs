use std::{cmp::Ordering, fs, ops::Deref};

use itertools::Itertools;

#[derive(PartialOrd, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    score: u8,
}

impl Hand {
    fn new(card_string: &str) -> Hand {
        let cards: [u8; 5] = card_string
            .chars()
            .map(|card| match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => card.to_digit(10).unwrap().try_into().unwrap(),
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let hand: String = cards
            .iter()
            .counts()
            .values()
            .sorted()
            .map(|ch| ch.to_string())
            .collect();

        let score = match hand.deref() {
            "11111" => 0, // High Card
            "1112" => 1,  // One Pair
            "122" => 2,   // Two Pair
            "113" => 3,   // Three of a kind
            "23" => 4,    // Full House
            "14" => 5,    // Four of a kind
            "5" => 6,     // Five of a kind
            _ => panic!("Invalid hand"),
        };

        Hand { cards, score }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(c1, c2)| c1.cmp(&c2))
                .find(|&ordering| ordering != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            other => other,
        }
    }
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .split_terminator("\n")
        .map(|round| round.split_once(" ").unwrap())
        .map(|(cards, bid)| (Hand::new(cards), bid.parse().unwrap()))
        .collect::<Vec<(Hand, u32)>>()
        .iter()
        .sorted_by(|(hand1, _), (hand2, _)| hand1.cmp(&hand2))
        .enumerate()
        .map(|(i, (_, score))| score * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 6440);
    }
}
