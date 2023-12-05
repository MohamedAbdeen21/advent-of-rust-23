use std::{collections::HashSet, fs};

// Keep track of the value and start coordinates to avoid replication
// Yes, I know. I can just use tuples, but I want to do a struct.
#[derive(Eq, PartialEq, Hash, Debug)]
struct PartNumber {
    value: u32,
    x: usize,
    y: usize,
}

fn parse_location(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Option<PartNumber> {
    // subtracting from usize type causes an overflow; don't need to check for < 0
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

    return Some(PartNumber {
        value: num.parse().unwrap(),
        x,
        y,
    });
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| matrix.push(Vec::from_iter(line.chars())));

    let mut parts: HashSet<Option<PartNumber>> = HashSet::new();
    for (i, line) in matrix.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if char.ne(&'.') && !char.is_alphanumeric() {
                for x in i - 1..=i + 1 {
                    for y in j - 1..=j + 1 {
                        parts.insert(parse_location(&matrix, y, x));
                    }
                }
            }
        }
    }

    parts
        .iter()
        .filter(|part| part.is_some())
        .map(|part| part.as_ref().unwrap().value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample.txt";
        assert_eq!(run(sample), 4361);
    }
}
