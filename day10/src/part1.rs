use std::fs;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    None,
    North,
    East,
    West,
    South,
}

#[derive(PartialEq)]
struct Pipe {
    start: bool,
    ends: [Direction; 2],
    pos: (usize, usize),
    is_loop: bool,
}

impl Default for Pipe {
    fn default() -> Pipe {
        Pipe {
            start: false,
            ends: [Direction::None, Direction::None],
            pos: (0, 0),
            is_loop: false,
        }
    }
}

impl Pipe {
    fn new(symbol: char, i: usize, j: usize) -> Pipe {
        let pos = (i, j);
        match symbol {
            'S' => Pipe {
                start: true,
                pos,
                is_loop: true,
                ..Default::default()
            },
            'L' => Pipe {
                ends: [Direction::East, Direction::North],
                pos,
                ..Default::default()
            },
            'F' => Pipe {
                ends: [Direction::East, Direction::South],
                pos,
                ..Default::default()
            },
            '-' => Pipe {
                ends: [Direction::East, Direction::West],
                pos,
                ..Default::default()
            },
            '|' => Pipe {
                ends: [Direction::North, Direction::South],
                pos,
                ..Default::default()
            },
            'J' => Pipe {
                ends: [Direction::North, Direction::West],
                pos,
                ..Default::default()
            },
            '7' => Pipe {
                ends: [Direction::South, Direction::West],
                pos,
                ..Default::default()
            },
            '.' => Pipe {
                ..Default::default()
            },
            other => panic!("Got {other}. Shouldn't have happened"),
        }
    }
}

fn fill_pipe(matrix: &mut Vec<Vec<Pipe>>, i: usize, j: usize, from: Direction) -> bool {
    if i >= matrix.len() || j >= matrix[0].len() {
        return false;
    }
    let pipe = &matrix[i][j];
    if pipe.start {
        return true;
    }
    if !pipe.ends.contains(&from) {
        return false;
    }
    let next = *pipe.ends.iter().find(|&&d| d != from).unwrap();
    matrix[i][j].is_loop = match next {
        Direction::North => fill_pipe(matrix, i - 1, j, Direction::South),
        Direction::South => fill_pipe(matrix, i + 1, j, Direction::North),
        Direction::East => fill_pipe(matrix, i, j + 1, Direction::West),
        Direction::West => fill_pipe(matrix, i, j - 1, Direction::East),
        _ => false,
    };
    return matrix[i][j].is_loop;
}

pub fn run(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    let mut matrix: Vec<Vec<Pipe>> = input
        .split_terminator("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, &c)| Pipe::new(c, i, j))
                .collect()
        })
        .collect();

    let (i, j) = matrix.iter().flatten().find(|pipe| pipe.start).unwrap().pos;

    // I hate usize
    fill_pipe(&mut matrix, i.wrapping_sub(1), j, Direction::South);
    fill_pipe(&mut matrix, i, j.wrapping_sub(1), Direction::East);
    fill_pipe(&mut matrix, i, j + 1, Direction::West);
    fill_pipe(&mut matrix, i + 1, j, Direction::North);

    matrix.iter().flatten().filter(|pipe| pipe.is_loop).count() as u32 / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample1-1.txt";
        assert_eq!(run(input), 4);
        let input = "./src/sample1-2.txt";
        assert_eq!(run(input), 8);
    }
}
