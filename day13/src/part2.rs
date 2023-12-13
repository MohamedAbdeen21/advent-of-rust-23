use std::cmp::min;
use std::fs;

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

fn find_symmetry(matrix: &Vec<Vec<char>>, max_smudges: usize) -> Option<usize> {
    (0..=matrix.len() - 1)
        .filter(|&split| {
            if split == 0 {
                return false;
            }
            let n = min(matrix.len() - split, split);
            matrix[split - n..split]
                .iter()
                .zip(matrix[split..split + n].iter().rev())
                .map(|(line1, line2)| total_differences(line1, line2))
                .sum::<usize>()
                == max_smudges
        })
        .next()
}

fn total_differences(slice1: &[char], slice2: &[char]) -> usize {
    slice1.iter().zip(slice2).filter(|(a, b)| a != b).count()
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let matrices: Vec<_> = input
        .split("\n\n")
        .map(|matrix| {
            matrix
                .split_terminator("\n")
                .map(|line| line.chars().collect())
                .collect::<Vec<_>>()
        })
        .collect();

    matrices
        .iter()
        .map(|matrix| {
            let row = find_symmetry(&matrix, 1).unwrap_or(0);
            let col = find_symmetry(&transpose(&matrix), 1).unwrap_or(0);
            row * 100 + col
        })
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 400);
    }
}
