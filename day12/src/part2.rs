use std::fs;

// Unfortunately, I don't have time to do this one,
// I'll try to solve it over the weekend

fn dynamic(_springs: &Vec<char>, _nums: &Vec<usize>) -> u64 {
    0
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<_> = input
        .split_terminator("\n")
        .map(|line| line.split_once(" ").unwrap())
        .map(|(springs, nums)| {
            (
                springs.chars().collect(),
                nums.split(",")
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();

    lines
        .iter()
        .map(|(springs, nums)| dynamic(springs, nums))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 525152);
    }
}
