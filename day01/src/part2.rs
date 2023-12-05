use std::fs;

fn first(string: &str, numbers: &[&str; 10]) -> u32 {
    let mut accum: String = String::new();
    for char in String::from(string).chars() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap();
        }

        accum.push(char);
        let digit = numbers
            .iter()
            .enumerate()
            .find(|(_, &k)| accum.ends_with(k));
        if digit.is_some() {
            return digit.unwrap().0 as u32;
        }
    }
    return 0;
}

fn last(string: &str, numbers: &[&str; 10]) -> u32 {
    let mut accum: String = String::new();
    for char in String::from(string).chars().rev() {
        if char.is_digit(10) {
            return char.to_digit(10).unwrap();
        }

        accum.insert(0, char);
        let digit = numbers
            .iter()
            .enumerate()
            .find(|(_, &k)| accum.starts_with(k));
        if digit.is_some() {
            return digit.unwrap().0 as u32;
        }
    }
    return 0;
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let numbers: [&str; 10] = [
        " ", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .split("\n")
        .map(|line| (first(line, &numbers), last(line, &numbers)))
        .map(|x| x.0 * 10 + x.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample.txt";
        assert_eq!(run(sample), 281);
    }
}
