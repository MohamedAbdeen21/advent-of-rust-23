use std::{collections::HashMap, fs, iter::Cycle, str::Bytes};

fn count_turns(
    mapping: &HashMap<&str, [&str; 2]>,
    mut directions: Cycle<Bytes>,
    start: &str,
) -> u64 {
    if start == "ZZZ" {
        return 0;
    }
    match directions.next().unwrap().into() {
        'L' => 1 + count_turns(mapping, directions, mapping[start][0]),
        'R' => 1 + count_turns(mapping, directions, mapping[start][1]),
        other => panic!("Expected direction L or R, got = {other}"),
    }
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let (directions, lines) = input.split_once("\n\n").unwrap();

    let mapping: HashMap<&str, [&str; 2]> = lines
        .split_terminator("\n")
        .map(|line| {
            let (k, v) = line.split_once(" = ").unwrap();
            (
                k,
                v[1..v.len() - 1]
                    .split(", ")
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect();

    mapping
        .keys()
        .filter(|&&k| k == "AAA")
        .map(|node| count_turns(&mapping, directions.bytes().cycle(), node))
        .nth(0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample1.txt";
        assert_eq!(run(input), 6);
    }
}
