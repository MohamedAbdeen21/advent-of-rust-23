use std::fs;

fn parse(input: String) -> Vec<(u64, u64)> {
    let (time, distance) = input.split_once("\n").unwrap();
    let distances = distance
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .into_iter();

    time.split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .zip(distances)
        .collect()
}

fn possible_distances(max_time: u64, to_beat: u64) -> u64 {
    // maximum of this quadratic function is at max_time/2
    // start from there and walk both ways
    let ans = (0..max_time / 2)
        .rev()
        .take_while(|x| x * (max_time - x) > to_beat)
        .count() as u64;
    ans + (max_time / 2..=max_time)
        .take_while(|x| x * (max_time - x) > to_beat)
        .count() as u64
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    parse(input)
        .iter()
        .map(|&(time, distance)| possible_distances(time, distance))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 288);
    }
}
