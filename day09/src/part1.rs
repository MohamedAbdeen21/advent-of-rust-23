use itertools::Itertools;
use std::fs;

fn extrapolate(pattern: Vec<i64>) -> i64 {
    if pattern.iter().all(|&x| x == 0) {
        return 0;
    }

    let inner_pattern = pattern.iter().tuple_windows().map(|(a, b)| b - a).collect();
    return pattern.last().unwrap() + extrapolate(inner_pattern);
}

pub fn run(filename: &str) -> i64 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .split_terminator("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|pattern| extrapolate(pattern))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 114);
    }
}
