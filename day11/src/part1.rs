use itertools::Itertools;
use std::fs;

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

fn expand(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    matrix
        .iter()
        .flat_map(|row| {
            if row.iter().all(|&c| c == '.') {
                return vec![row; 2];
            }
            return vec![row];
        })
        .cloned()
        .collect::<Vec<Vec<char>>>()
}

fn calculate_distance(g1: &(usize, usize, &char), g2: &(usize, usize, &char)) -> u64 {
    (g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)) as u64
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let matrix: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let matrix = expand(&matrix);
    let matrix = expand(&transpose(&matrix));

    let galaxies: Vec<(usize, usize, &char)> = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, cell)| (i, j, cell))
                .collect::<Vec<(usize, usize, &char)>>()
        })
        .flatten()
        .filter(|&(_, _, &cell)| cell == '#')
        .collect();

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| calculate_distance(pair[0], pair[1]))
        .sum()

    // for row in &matrix {
    //     for cell in row {
    //         print!("{}", cell)
    //     }
    //     println!()
    // }
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
