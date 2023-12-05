use std::fs;

struct Range {
    start_x: u64,
    end_x: u64, // exclusive
    start_y: u64,
}

impl Range {
    fn new(start_x: u64, end_x: u64, start_y: u64) -> Range {
        Range {
            start_x,
            end_x,
            start_y,
        }
    }
}

fn parse_ranges(block: &str) -> Vec<Range> {
    let mut vec = Vec::new();
    block.split_terminator("\n").skip(1).for_each(|line| {
        let mut nums = line
            .split(" ")
            .take(3)
            .map(|num| num.parse().expect("Expected first 3 parts to be integers"));
        let i: u64 = nums.next().expect("Expected 3 integers");
        let j: u64 = nums.next().expect("Expected 3 integers");
        let k: u64 = nums.next().expect("Expected 3 integers");
        vec.push(Range::new(j, j + k, i));
    });
    return vec;
}

fn apply_mapping(inputs: Vec<u64>, mapping: Vec<Range>) -> Vec<u64> {
    inputs
        .iter()
        .map(|&input| {
            mapping
                .iter()
                .find(|range| range.start_x <= input && input < range.end_x)
                .map_or(input, |range| input - range.start_x + range.start_y)
        })
        .collect()
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let sections = input.split("\n\n");
    let seeds: Vec<u64> = sections
        .clone()
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();

    *sections
        .skip(1)
        .map(|mapping| parse_ranges(mapping))
        .fold(seeds, |input, mapping| apply_mapping(input, mapping))
        .iter()
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 35);
    }
}
