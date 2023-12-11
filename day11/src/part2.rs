use itertools::Itertools;
use std::fs;

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

fn expand(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    matrix
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i) // can also unzip
        .collect()
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

    let expanded_rows = expand(&matrix);
    let expanded_cols = expand(&transpose(&matrix));

    let galaxies: Vec<(usize, usize, char)> = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .map(|(j, cell)| {
                    let a = expanded_rows
                        .iter()
                        .position(|&index| index > i)
                        .unwrap_or(expanded_rows.len())
                        * (1_000_000 - 1);
                    let b = expanded_cols
                        .iter()
                        .position(|&index| index > j)
                        .unwrap_or(expanded_cols.len())
                        * (1_000_000 - 1);
                    (a + i, b + j, cell)
                })
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
        assert_eq!(run(input), 82000210);
    }
}
