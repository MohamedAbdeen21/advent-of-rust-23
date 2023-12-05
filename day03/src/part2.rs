use std::{collections::HashSet, fs};

fn parse_location(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Option<u32> {
    // subtracting from usize type causes an overflow, don't need to check for < 0
    if y >= matrix.len() || x >= matrix[y].len() || !matrix[y][x].is_digit(10) {
        return None;
    }

    let mut x = x;
    // find the start of the integer
    x -= matrix[y][0..x]
        .iter()
        .rev()
        .take_while(|char| char.is_digit(10))
        .collect::<String>()
        .len();

    let num: String = matrix[y][x..]
        .iter()
        .take_while(|char| char.is_digit(10))
        .collect();

    return Some(num.parse().unwrap());
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| matrix.push(Vec::from_iter(line.chars())));

    let mut powers = Vec::new();
    for (i, line) in matrix.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == '*' {
                let mut gear_numbers = HashSet::new();
                for x in i - 1..=i + 1 {
                    for y in j - 1..=j + 1 {
                        gear_numbers.insert(parse_location(&matrix, y, x));
                    }
                }

                let gear_numbers: Vec<u32> = gear_numbers
                    .iter()
                    .filter(|s| !s.is_none())
                    .map(|s| s.unwrap())
                    .collect();
                if gear_numbers.len() == 2 {
                    powers.push(gear_numbers[0] * gear_numbers[1]);
                }
            }
        }
    }

    powers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample.txt";
        assert_eq!(run(sample), 467835);
    }
}
