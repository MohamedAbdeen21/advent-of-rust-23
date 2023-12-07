use itertools::Itertools;
use std::{cmp::Ordering, fs, ops::Deref};

#[derive(PartialOrd, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    score: u8,
}

impl Hand {
    fn new(card_string: &str) -> Hand {
        let mut freq = Vec::from([0; 15]);
        let cards: [u8; 5] = card_string
            .chars()
            .map(|card| match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => card.to_digit(10).unwrap().try_into().unwrap(),
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        cards.iter().for_each(|&card| freq[card as usize] += 1);
        let jokers = freq.remove(1);

        freq.sort_by(|a, b| b.cmp(a));
        freq[0] += jokers;
        let hand: String = freq[0..5]
            .iter()
            .filter(|&&num| num != 0)
            .map(|num| num.to_string())
            .collect::<String>();

        let score = match hand.deref() {
            "11111" => 0, // High Card
            "2111" => 1,  // One Pair
            "221" => 2,   // Two Pair
            "311" => 3,   // Three of a kind
            "32" => 4,    // Full House
            "41" => 5,    // Four of a kind
            "5" => 6,     // Five of a kind
            _ => panic!("Invalid hand: {}, {}", hand, jokers),
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
        assert_eq!(run(input), 5905);
    }
}
