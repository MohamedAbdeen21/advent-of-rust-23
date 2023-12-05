use std::{cmp, fs};

fn parse(line: &str) -> [u32; 3] {
    let split = line.split_once(": ").unwrap();
    let mut colors = [0, 0, 0]; // R, G, B
    split
        .1
        .split("; ")
        .flat_map(|hand| hand.split(", "))
        .for_each(|hand| {
            let split = hand.split_once(" ").unwrap();
            let count = split.0.parse::<u32>().unwrap();
            let color = split.1;
            let index = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!("WTF is {} ??", color),
            };
            colors[index] = cmp::max(colors[index], count);
        });
    return colors;
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(parse)
        .map(|total| total[0] * total[1] * total[2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample.txt";
        assert_eq!(run(sample), 2286);
    }
}
