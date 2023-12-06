use std::{collections::HashSet, fs};

fn parse_line(line: &str) -> usize {
    let (winning, hand) = line.split_once("|").unwrap();
    let winning = winning
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<HashSet<u32>>();
    let hand = hand
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<HashSet<u32>>();

    winning.intersection(&hand).collect::<HashSet<_>>().len()
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let lines = input.split_terminator("\n");
    let n = lines.clone().collect::<Vec<&str>>().len();

    let mut cards: Vec<u32> = vec![0; n];
    lines.map(parse_line).enumerate().for_each(|(i, l)| {
        cards[i] += 1;
        (i + 1..=i + l).for_each(|j| {
            cards[j] += cards[i];
        })
    });
    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample.txt";
        assert_eq!(run(sample), 30);
    }
}
