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

    let (_stages, _) = parser::parse(tokens);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 167409079868000);
    }
}
