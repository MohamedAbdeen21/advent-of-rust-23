use std::fs;

use hashvec::HashVec;

fn calculate_load(matrix: &Vec<Vec<char>>) -> u64 {
    matrix
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (i + 1))
        .sum::<usize>() as u64
}

fn tilt_north(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();
    let m = matrix[0].len();
    for j in 0..m {
        let mut k = 0;
        for i in 0..n {
            if matrix[i][j] == '#' {
                k = i + 1
            } else if matrix[i][j] == 'O' {
                matrix[i][j] = '.';
                matrix[k][j] = 'O';
                k += 1;
            }
        }
    }
}

fn rotate_clockwise(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();
    let mut rotated = vec![vec!['.'; n]; n];
    for row in 0..n {
        for col in 0..n {
            rotated[col][n - 1 - row] = matrix[row][col];
        }
    }
    *matrix = rotated;
}

fn tilt(matrix: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        tilt_north(matrix);
        rotate_clockwise(matrix);
    }
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let mut matrix: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|row| row.chars().collect())
        .collect();

    let mut seen = HashVec::new();
    loop {
        tilt(&mut matrix);

        if let Some(start) = seen.index(&matrix) {
            let period = seen.len() - start;
            let ans_idx = start + (1_000_000_000 - start) % period - 1;
            break calculate_load(&seen[ans_idx].0);
        }
        seen.insert(matrix.clone(), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 64);
    }
}
