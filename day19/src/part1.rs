use std::fs;

use crate::parser::parser;
use crate::tokenizer::tokenizer::tokenize;
use crate::tokenizer::tokens::{self};

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let tokens: Vec<tokens::Token> = input
        .replace("\n\n", "\n")
        .split("\n")
        .map(|line| tokenize(line))
        .flatten()
        .collect();

    let (stages, parts) = parser::parse(tokens);
    let mut ans: Vec<u64> = Vec::new();
    for part in parts {
        println!("{:?}", part);
        let mut idx = 0;
        let mut stage = String::from("in");

        loop {
            if let Some(next) = stages.get(&stage).unwrap()[idx](&part) {
                println!("{next}");
                if next == String::from("A") {
                    ans.push(
                        part.attributes
                            .into_iter()
                            .map(|t| t.1)
                            .fold(0, |a, b| a + b),
                    );
                    break;
                } else if next == String::from("R") {
                    break;
                }
                idx = 0;
                stage = next
            } else {
                idx += 1
            }
        }
    }
    return ans.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 19114);
    }
}
