use itertools::Itertools;
use std::fs;

// Terrible terrible performance, takes like 30 seconds to run on input

fn is_valid(springs: &Vec<char>, nums: &Vec<usize>) -> bool {
    let groups: Vec<_> = springs
        .iter()
        .group_by(|&&ch| ch == '#')
        .into_iter()
        .filter(|&(ok, _)| ok)
        .map(|(_, group)| group.collect_vec().len())
        .collect();
    // to avoid using stuff like zip_longest
    if groups.len() != nums.len() {
        return false;
    }
    groups
        .iter()
        .zip(nums)
        .map(|(a, b)| a == b)
        .all(|boolean| boolean)
}

fn generate_strings(chars: &Vec<char>, index: usize) -> Vec<Vec<char>> {
    if index == chars.len() {
        return vec![chars.clone()];
    }

    if chars[index] == '?' {
        let mut c1 = chars.clone();
        c1[index] = '#';
        let mut c2 = chars.clone();
        c2[index] = '.';
        return Vec::from([
            generate_strings(&c1, index + 1),
            generate_strings(&c2, index + 1),
        ])
        .iter()
        .flatten()
        .cloned()
        .collect();
    }

    Vec::from(generate_strings(&chars, index + 1))
}

fn brute(springs: &Vec<char>, nums: &Vec<usize>) -> u64 {
    generate_strings(springs, 0)
        .iter()
        .filter(|x| is_valid(x, nums))
        .count() as u64
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
        .map(|(springs, nums)| brute(springs, nums))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 21);
    }
}
