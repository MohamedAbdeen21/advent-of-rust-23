use itertools::Itertools;
use std::{collections::VecDeque, fs};

fn parse(input: String) -> Vec<(char, isize)> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let i: Vec<&str> = line.split(" ").take(2).collect();
            (i[0].chars().nth(0).unwrap(), i[1].parse::<isize>().unwrap())
        })
        .collect::<Vec<(char, isize)>>()
}

fn shoelace_area(instr: Vec<(char, isize)>) -> (isize, isize) {
    let mut pos = (0, 0);
    let mut edges = 0;
    let mut vertices = instr
        .iter()
        .map(|&(direction, steps)| {
            let dir: (isize, isize) = match direction {
                'R' => (0, 1),
                'D' => (1, 0),
                'L' => (0, -1),
                'U' => (-1, 0),
                _ => panic!("Shouldn't happen"),
            };
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
        assert_eq!(run(input), 62);
    }
}
