use std::fs;

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first(string: &str) -> u32 {
    let mut accum = String::new();
    for char in string.chars() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap();
        }

        accum.push(char);
        if let Some(digit) = NUMBERS
            .iter()
            .enumerate()
            .find(|(_, &k)| accum.ends_with(k))
        {
            return digit.0 as u32;
        }
    }
    return 0;
}

fn last(string: &str) -> u32 {
    let mut accum = String::new();
    for char in string.chars().rev() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap();
        }

        accum.insert(0, char);
        if let Some(digit) = NUMBERS
            .iter()
            .enumerate()
            .find(|(_, &k)| accum.starts_with(k))
        {
            return digit.0 as u32;
        }
    }
    return 0;
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();

    input
        .split("\n")
        .map(|line| (first(line), last(line)))
        .map(|(first, last)| first * 10 + last)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample2.txt";
        assert_eq!(run(sample), 281);
    }
}
