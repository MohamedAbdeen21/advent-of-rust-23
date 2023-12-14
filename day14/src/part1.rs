use std::fs;

fn sum_row(row: &Vec<char>) -> usize {
    let n = row.len();
    let mut result = vec![0; n];
    let mut k = 0;
    row.iter().enumerate().for_each(|(i, &ch)| {
        if ch == '#' {
            k = i + 1
        } else if ch == 'O' {
            result[k] = 1;
            k += 1
        }
    });
    result
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| v * (i + 1))
        .sum()
}

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let matrix: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|row| row.chars().collect())
        .collect();

    transpose(matrix)
        .iter()
        .map(|row| sum_row(row))
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 136);
    }
}
