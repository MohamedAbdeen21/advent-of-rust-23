use itertools::Itertools;
use std::fs;

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

fn expand(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    matrix
        .iter()
        .flat_map(|row| {
            if row.iter().all(|&c| c == '.') {
                return Vec::from([row; 2]);
            }
            return Vec::from([row]);
        })
        .cloned()
        .collect::<Vec<Vec<char>>>()
}

fn calculate_distance(g1: (usize, usize, char), g2: (usize, usize, char)) -> u64 {
    let (x1, y1, _) = g1;
    let (x2, y2, _) = g2;
    (x1.abs_diff(x2) + y1.abs_diff(y2)) as u64
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let matrix: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let matrix = expand(transpose(expand(matrix)));

    let galaxies: Vec<(usize, usize, char)> = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .map(|(j, cell)| (i, j, cell))
                .collect::<Vec<(usize, usize, char)>>()
        })
        .flatten()
        .filter(|&(_, _, cell)| cell == '#')
        .collect();

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| calculate_distance(*pair[0], *pair[1]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 374);
    }
}
