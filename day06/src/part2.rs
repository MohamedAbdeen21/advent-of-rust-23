use itertools::{self, Itertools};
use std::fs;

fn parse(input: String) -> (u64, u64) {
    input
        .split_terminator("\n")
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .concat()
                .parse()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let (time, distance) = parse(input);

    (0..time)
        .filter_map(|x| {
            let my_distance = x * (time - x);
            (my_distance > distance).then_some(my_distance)
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 71503);
    }
}
