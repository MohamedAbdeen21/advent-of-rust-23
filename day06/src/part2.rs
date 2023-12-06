use std::fs;

fn parse(input: String) -> (u64, u64) {
    let split = input.split_once("\n").unwrap();
    (
        split
            .0
            .split_whitespace()
            .skip(1)
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .concat()
            .parse()
            .unwrap(),
        split
            .1
            .split_whitespace()
            .skip(1)
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .concat()
            .parse()
            .unwrap(),
    )
}

fn possible_distances(max_time: u64, to_beat: u64) -> u32 {
    let mut ans = 0;
    // peak of quadratic function is at max_time/2
    // start from there and walk both ways
    for x in (0..max_time / 2).rev() {
        let distance = x * (max_time - x);
        if distance <= to_beat {
            break;
        }
        ans += 1;
    }
    for x in max_time / 2..=max_time {
        let distance = x * (max_time - x);
        if distance <= to_beat {
            break;
        }
        ans += 1
    }
    return ans;
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let race = parse(input);
    possible_distances(race.0, race.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 71503);
    }
}
