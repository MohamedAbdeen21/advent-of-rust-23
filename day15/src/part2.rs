use std::fs;

struct Bucket {
    mem: Vec<(String, u64)>,
}

impl Bucket {
    fn new() -> Bucket {
        Bucket { mem: Vec::new() }
    }

    fn push(&mut self, k: String, v: u64) {
        if let Some(idx) = self.mem.iter().position(|(key, _)| key == &k) {
            return self.mem[idx] = (k, v);
        }
        self.mem.push((k, v));
    }

    fn pop(&mut self, k: String) {
        if let Some(idx) = self.mem.iter().position(|(key, _)| key == &k) {
            self.mem.remove(idx);
        }
    }

    fn reflection_power(&self, idx: u64) -> u64 {
        self.mem
            .iter()
            .enumerate()
            .map(|(i, (_, v))| (idx + 1) * (i + 1) as u64 * v)
            .sum()
    }
}

pub fn run(filename: &str) -> u64 {
    let file = fs::read_to_string(filename).unwrap();
    let mut values = Vec::with_capacity(256);
    for _ in 0..256 {
        values.push(Bucket::new());
    }

    file.trim().split(",").for_each(|seq| {
        let key = seq
            .chars()
            .take_while(|&c| c != '=' && c != '-')
            .collect::<String>();

        let mut hash: u8 = 0;
        key.chars().for_each(|c| {
            hash = hash.wrapping_add(c as u8).wrapping_mul(17);
        });

        let operation: String = seq.chars().skip_while(|&c| c != '=' && c != '-').collect();
        match &operation[0..1] {
            "=" => values[hash as usize].push(key, operation[1..].parse().unwrap()),
            "-" => values[hash as usize].pop(key),
            _ => panic!("Shouldn't happen"),
        };
    });

    return values
        .iter()
        .enumerate()
        .map(|(i, bucket)| bucket.reflection_power(i as u64))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 145);
    }
}
