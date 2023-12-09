use std::{cmp, fs};

fn parse(line: &str) -> [u32; 4] {
    let (game, balls) = line.split_once(": ").unwrap();
    let mut colors = [0, 0, 0, 0]; // id, R, G, B
    colors[0] = game.split_once(" ").unwrap().1.parse().unwrap();
    balls
        .split("; ")
        .flat_map(|hand| hand.split(", "))
        .for_each(|hand| {
            let (count, color) = hand.split_once(" ").unwrap();
            let index = match color {
                "red" => 1,
                "green" => 2,
                "blue" => 3,
                _ => panic!("What is {}?", color),
            };
            colors[index] = cmp::max(colors[index], count.parse().unwrap());
        });
    return colors;
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .split("\n")
        .filter(|&line| line != "")
        .map(parse)
        .filter(|round| round[1] <= 12 && round[2] <= 13 && round[3] <= 14)
        .map(|round| round[0])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let sample = "./src/sample.txt";
        assert_eq!(run(sample), 8);
    }
}
