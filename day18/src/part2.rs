use itertools::Itertools;
use std::i64;
use std::{collections::VecDeque, fs};

fn parse(input: String) -> Vec<(u32, i64)> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            // slice to remove parenthesis and '#' character
            let hexcode: &str = &line.split(" ").nth(2).unwrap()[2..8];
            (
                hexcode.chars().last().unwrap().to_digit(10).unwrap(),
                i64::from_str_radix(&hexcode[0..5], 16).unwrap(),
            )
        })
        .collect::<Vec<(u32, i64)>>()
}

fn shoelace_area(instr: Vec<(u32, i64)>) -> (isize, isize) {
    let mut pos = (0, 0);
    let mut edges = 0;
    let mut vertices = instr
        .iter()
        .map(|&(direction, steps)| {
            let dir: (isize, isize) = match direction {
                0 => (0, 1),
                1 => (1, 0),
                2 => (0, -1),
                3 => (-1, 0),
                _ => panic!("Shouldn't happen"),
            };
            let steps = steps as isize;
            edges += steps;
            pos = (pos.0 + dir.0 * steps, pos.1 + dir.1 * steps);
            pos
        })
        .collect::<VecDeque<(isize, isize)>>();

    // we need to first element to access the last and vice versa
    // and we can't index with -1 for example
    let last = vertices.back().unwrap().clone();
    let first = vertices.front().unwrap().clone();
    vertices.push_front(last);
    vertices.push_back(first);

    let area = vertices
        .into_iter()
        .tuple_windows() // Thank God for itertools
        .map(|(prev, curr, next)| curr.0 * (prev.1 - next.1))
        .sum::<isize>()
        .abs();

    (area, edges)
}

pub fn run(filename: &str) -> isize {
    let input = fs::read_to_string(filename).unwrap();
    let instr = parse(input);

    let (area, edges) = shoelace_area(instr);
    return area / 2 + edges / 2 + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 952408144115);
    }
}
