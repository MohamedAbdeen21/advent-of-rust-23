use std::fs;

pub fn run(filename: &str) -> u64 {
    let file = fs::read_to_string(filename).unwrap();
    let input: Vec<u8> = file
        .trim()
        .split(",")
        .map(|seq| {
            let mut result: u8 = 0;
            seq.chars().for_each(|c| {
                result = result.wrapping_add(c as u8).wrapping_mul(17);
            });
            result
        })
        .collect();
    return input.iter().map(|&hash| hash as u64).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 1320);
    }
}
