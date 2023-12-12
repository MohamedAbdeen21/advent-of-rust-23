use itertools::Itertools;
use std::iter::repeat;
use std::{collections::HashMap, fs};

// Can this be more concise? Definitely.
// Do I really care to find a "smarter" solution?
// Not really, I like how simple this is. But it can
// definitely be cleaner
fn solve(
    config: &[char],
    dmgs: &[usize],
    count: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let (n, m) = (config.len(), dmgs.len());

    if let Some(k) = memo.get(&(n, m)) {
        return *k;
    }

    // matched all damaged ones
    if dmgs.is_empty() || (dmgs.len() == 1 && count == dmgs[0]) {
        if config.contains(&'#') {
            // still have more #, must be an invalid configuration
            return 0;
        } else {
            return 1; // no more # ones (i.e. valid)
        }
    }

    // end of string, but haven't found all # (i.e. invalid)
    if config.is_empty() {
        return 0;
    }

    // found nums[j] number of #, current char must be either a . or become a . if ?
    // to conclude this set, notice that this is the only section allowed to progress dmgs slice
    if count == dmgs[0] {
        if ['.', '?'].contains(&config[0]) {
            return solve(&config[1..], &dmgs[1..], 0, memo);
        }
        return 0; // found another #, invalid
    }

    if config[0] == '?' {
        // if we've already collected some #, this also must be # (note the count
        // != nums[j] because we checked that above, so no way this is a . )
        if count != 0 {
            return solve(&config[1..], dmgs, count + 1, memo);
        }
        // can be either # or .
        let s = solve(&config[1..], dmgs, count + 1, memo) + solve(&config[1..], dmgs, 0, memo);
        memo.insert((n, m), s);
        return s;
    }

    // just add it to counter and keep going
    if config[0] == '#' {
        return solve(&config[1..], dmgs, count + 1, memo);
    }

    // char is a . , so continue as normal
    if count == 0 {
        return solve(&config[1..], dmgs, 0, memo);
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
                repeat(springs)
                    .take(5)
                    .join("?")
                    .chars()
                    .collect::<Vec<_>>(),
                repeat(
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
            solve(&springs, &nums, 0, &mut memo)
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
