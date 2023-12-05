use std::{collections::HashSet, fs};

fn parse_line(line: &str) -> u32 {
    let card = line.split_once("|").unwrap();
    let winning = card
        .0
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<HashSet<u32>>();
    let hand = card
        .1
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<HashSet<u32>>();

    winning.intersection(&hand).collect::<HashSet<_>>().len() as u32
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let base: u32 = 2;
    input
        .split_terminator("\n")
        .map(parse_line)
        .filter(|&num| num != 0)
        .map(|num: u32| base.pow(num - 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample.txt";
        assert_eq!(run(sample), 13);
    }
}
