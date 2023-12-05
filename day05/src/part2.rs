use std::fs;

struct Range {
    start_x: u64,
    end_x: u64, // exclusive
    start_y: u64,
    end_y: u64,
}

impl Range {
    fn new(start_x: u64, end_x: u64, start_y: u64) -> Range {
        Range {
            start_x,
            end_x,
            start_y,
            end_y: start_y + (end_x - start_x),
        }
    }

    fn intersect(&self, other: &Range) -> Option<(Range, Vec<Range>)> {
        let intersection: Range;
        // no overlap
        if other.end_x <= self.start_x || other.start_x >= self.end_x {
            return None;
        }

        let mut remaining = Vec::new();
        // self ; *----------------------*
        // other; *-----------------*
        if other.start_x >= self.start_x && other.end_x <= self.end_x {
            intersection = Range::new(
                (other.start_x - self.start_x) + self.start_y,
                (other.end_x - self.start_x) + self.start_y,
                0,
            );
        // self ;  *---------*
        // other;     *-----------------*
        } else if other.start_x >= self.start_x {
            intersection = Range::new((other.start_x - self.start_x) + self.start_y, self.end_y, 0);
            remaining.push(Range::new(self.end_x, other.end_x, 0));
        // self ;         *------*
        // other; *-----------*
        } else if other.end_x <= self.end_x {
            intersection = Range::new(self.start_y, (other.end_x - self.start_x) + self.start_y, 0);
            remaining.push(Range::new(other.start_x, self.start_x, 0));
        // self ;      *---------*
        // other;  *-----------------*
        } else {
            intersection = Range::new(self.start_y, self.end_y, 0);
            remaining.push(Range::new(other.start_x, self.start_x, 0));
            remaining.push(Range::new(self.end_x, other.end_x, 0));
        }
        return Some((intersection, remaining));
    }
}

fn parse_ranges(block: &str) -> Vec<Range> {
    let mut vec = Vec::new();
    block.split_terminator("\n").skip(1).for_each(|line| {
        let mut nums = line
            .split(" ")
            .take(3)
            .map(|num| num.parse().expect("Expected first 3 parts to be integers"));
        let i: u64 = nums.next().expect("Expected 3 integers");
        let j: u64 = nums.next().expect("Expected 3 integers");
        let k: u64 = nums.next().expect("Expected 3 integers");
        vec.push(Range::new(j, j + k, i));
    });
    return vec;
}

fn parse(input: String) -> (Vec<Range>, Vec<Vec<Range>>) {
    let seeds: Vec<Range> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|pair| Range::new(pair[0], pair[0] + pair[1], 0))
        .collect();

    let maps = input
        .split("\n\n")
        .skip(1)
        .map(|block| parse_ranges(block))
        .collect::<Vec<Vec<Range>>>();

    return (seeds, maps);
}

fn apply_mapping(inputs: &mut Vec<Range>, mapping: &Vec<Range>) -> Vec<Range> {
    if inputs.is_empty() {
        return Vec::new();
    }

    let input = inputs.pop().unwrap();
    match mapping.iter().find_map(|range| range.intersect(&input)) {
        Some((intersection, remaining)) => {
            inputs.extend(remaining);
            let mut res = apply_mapping(inputs, mapping);
            res.push(intersection);
            res
        }
        None => {
            let mut res = apply_mapping(inputs, mapping);
            res.push(input);
            res
        }
    }
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Can't find input file");

    let (seeds, maps) = parse(input);
    maps.iter()
        .fold(seeds, |mut input, mapping| {
            apply_mapping(&mut input, &mapping)
        })
        .iter()
        .map(|range| range.start_x)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 46);
    }
}
