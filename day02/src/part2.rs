use std::{cmp, fs};

fn parse(line: &str) -> [u32; 3] {
    let (_, balls) = line.split_once(": ").unwrap();
    let mut colors = [0, 0, 0]; // R, G, B
    balls
        .split("; ")
        .flat_map(|hand| hand.split(", "))
        .for_each(|hand| {
            let (count, color) = hand.split_once(" ").unwrap();
            let index = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
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
        .filter(|line| !line.is_empty())
        .map(parse)
        .map(|total| total.iter().product::<u32>())
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
