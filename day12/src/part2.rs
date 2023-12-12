use itertools::Itertools;
use std::{collections::HashMap, fs};

// Holy shit does this need refactoring
fn brute(
    springs: &Vec<char>,
    nums: &Vec<usize>,
    i: usize,
    j: usize,
    count: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(k) = memo.get(&(i, j)) {
        return *k;
    }

    // matched all damaged ones
    if j == nums.len() || (j == nums.len() - 1 && count == nums[j]) {
        if springs[i..].contains(&'#') {
            // still have more #, must be an invalid configuration
            return 0;
        } else {
            return 1; // no more # ones (i.e. valid)
        }
    }

    // end of string, but haven't found all # (i.e. invalid)
    if i == springs.len() {
        return 0;
    }

    // found nums[j] number of #, current char must be either a . or become a . if ?
    // to conclude this set, notice that this is the only section allowed to increment j
    if count == nums[j] {
        if ['.', '?'].contains(&springs[i]) {
            return brute(springs, nums, i + 1, j + 1, 0, memo);
        }
        return 0; // found another #, invalid
    }

    if springs[i] == '?' {
        // if we've already collected some #, this also must be # (note the count
        // != nums[j] because we checked that above, so no way this is a . )
        if count != 0 {
            return brute(springs, nums, i + 1, j, count + 1, memo);
        }
        // can be either # or .
        let s = brute(springs, nums, i + 1, j, count + 1, memo)
            + brute(springs, nums, i + 1, j, 0, memo);
        memo.insert((i, j), s);
        return s;
    }

    // just add it to counter and keep going
    if springs[i] == '#' {
        return brute(springs, nums, i + 1, j, count + 1, memo);
    }

    // continue as normal
    if springs[i] == '.' && count == 0 {
        return brute(springs, nums, i + 1, j, 0, memo);
    }

    // count wasn't 0 and wasn't == nums[j], but found a .
    return 0;
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let lines: Vec<_> = input
        .split_terminator("\n")
        .map(|line| line.split_once(" ").unwrap())
        .map(|(springs, nums)| {
            (
                std::iter::repeat(springs)
                    .take(5)
                    .join("?")
                    .chars()
                    .collect(),
                std::iter::repeat(
                    nums.split(",")
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<usize>>(),
                )
                .take(5)
                .flatten()
                .collect::<Vec<usize>>(),
            )
        })
        .collect();

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    lines
        .iter()
        .map(|(springs, nums)| {
            memo.clear();
            brute(springs, nums, 0, 0, 0, &mut memo)
        })
        .sum::<usize>() as u64
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
