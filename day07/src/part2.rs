use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(PartialOrd, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    score: u8,
}

impl Hand {
    fn new(card_string: &str, scoring: &HashMap<char, u8>) -> Hand {
        let mut hand = [0; 13];
        let cards: [u8; 5] = card_string
            .chars()
            .map(|card| scoring[&card])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let mut jokers = 0;
        cards.iter().for_each(|&card| {
            if card != scoring[&'J'] {
                hand[card as usize] += 1
            } else {
                jokers += 1
            }
        });

        hand.sort_by(|a, b| b.cmp(a));
        hand[0] += jokers;
        let freq: [u8; 5] = hand[0..5].try_into().unwrap();

        let score = match freq {
            [1, 1, 1, 1, 1] => 0, // High Card
            [2, 1, 1, 1, 0] => 1, // One Pair
            [2, 2, 1, 0, 0] => 2, // Two Pair
            [3, 1, 1, 0, 0] => 3, // Three of a kind
            [3, 2, 0, 0, 0] => 4, // Full House
            [4, 1, 0, 0, 0] => 5, // Four of a kind
            [5, 0, 0, 0, 0] => 6, // Five of a kind
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
    let scoring: HashMap<char, u8> = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    let mut rounds: Vec<(Hand, u32)> = input
        .split_terminator("\n")
        .map(|round| round.split_once(" ").unwrap())
        .map(|(cards, bid)| (Hand::new(cards, &scoring), bid.parse().unwrap()))
        .collect();
    rounds.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(&hand2));
    rounds
        .iter()
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
        assert_eq!(run(input), 5905);
    }
}

