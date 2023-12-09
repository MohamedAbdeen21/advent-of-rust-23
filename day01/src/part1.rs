use std::fs;

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .split_terminator("\n")
        .map(|line| {
            (
                line.chars()
                    .find(|char| char.is_digit(10))
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
                line.chars()
                    .rev()
                    .find(|char| char.is_digit(10))
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            )
        })
        .map(|(first, last)| first * 10 + last)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample1.txt";
        assert_eq!(run(sample), 142);
    }
}
