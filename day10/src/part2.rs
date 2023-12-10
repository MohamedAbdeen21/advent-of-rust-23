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
    is_edge: bool,
    symbol: char,
}

impl Default for Pipe {
    fn default() -> Pipe {
        Pipe {
            start: false,
            ends: [Direction::None, Direction::None],
            pos: (0, 0),
            is_edge: false,
            symbol: '.',
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
                is_edge: true,
                symbol,
                ..Default::default()
            },
            'L' => Pipe {
                ends: [Direction::East, Direction::North],
                pos,
                symbol,
                ..Default::default()
            },
            'F' => Pipe {
                ends: [Direction::East, Direction::South],
                pos,
                symbol,
                ..Default::default()
            },
            '-' => Pipe {
                ends: [Direction::East, Direction::West],
                pos,
                symbol,
                ..Default::default()
            },
            '|' => Pipe {
                ends: [Direction::North, Direction::South],
                pos,
                symbol,
                ..Default::default()
            },
            'J' => Pipe {
                ends: [Direction::North, Direction::West],
                pos,
                symbol,
                ..Default::default()
            },
            '7' => Pipe {
                ends: [Direction::South, Direction::West],
                pos,
                symbol,
                ..Default::default()
            },
            '.' => Pipe {
                pos,
                ..Default::default()
            },
            other => panic!("Got {other}. Shouldn't have happened"),
        }
    }
}

fn is_inside(matrix: &Vec<Vec<Pipe>>, pipe: &Pipe) -> bool {
    let (i, j) = pipe.pos;
    let mut edges = 0;
    for p in &matrix[i][j..] {
        if p.is_edge && ['L', 'J', '|'].contains(&p.symbol) {
            edges += 1;
        }
    }
    return edges % 2 == 1;
}

fn fill_pipe(matrix: &mut Vec<Vec<Pipe>>, i: usize, j: usize, from: Direction) -> bool {
    if i >= matrix.len() || j >= matrix[0].len() {
        return false;
    }
    let pipe = &matrix[i][j];
    if pipe.start {
        matrix[i][j].ends[1] = from;
        return true;
    }
    if !pipe.ends.contains(&from) {
        return false;
    }
    let next = *pipe.ends.iter().find(|&&d| d != from).unwrap();
    matrix[i][j].is_edge = match next {
        Direction::North => fill_pipe(matrix, i - 1, j, Direction::South),
        Direction::South => fill_pipe(matrix, i + 1, j, Direction::North),
        Direction::East => fill_pipe(matrix, i, j + 1, Direction::West),
        Direction::West => fill_pipe(matrix, i, j - 1, Direction::East),
        _ => false,
    };
    return matrix[i][j].is_edge;
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

    let (si, sj) = matrix.iter().flatten().find(|pipe| pipe.start).unwrap().pos;

    // I hate usize overflow, just let me check for it myself, please
    if matrix[si][sj + 1].ends.contains(&Direction::West) {
        matrix[si][sj].ends[0] = Direction::East;
        fill_pipe(&mut matrix, si, sj + 1, Direction::West);
    } else if matrix[si + 1][sj].ends.contains(&Direction::North) {
        matrix[si][sj].ends[0] = Direction::South;
        fill_pipe(&mut matrix, si + 1, sj, Direction::North);
    } else {
        // must be a J since other two directions are blocked
        fill_pipe(&mut matrix, si - 1, sj, Direction::South);
        // Last direction must be the other side of the loop, don't need to check
    }

    // infer type of S, note that we only care about cases
    // that start with East and South, as these are the first
    // 2 conditions of the if block above. Anything else
    // means that the two sides of the S pipe are Left and North
    matrix[si][sj].symbol = match matrix[si][sj].ends {
        [Direction::East, Direction::West] => '-',
        [Direction::East, Direction::North] => 'L',
        [Direction::East, Direction::South] => 'F',
        [Direction::South, Direction::North] => '|',
        [Direction::South, Direction::West] => '7',
        _ => 'J',
    };

    matrix
        .iter()
        .flatten()
        .filter(|pipe| !pipe.is_edge && is_inside(&matrix, &pipe))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample2-1.txt";
        assert_eq!(run(input), 4);
        let input = "./src/sample2-2.txt";
        assert_eq!(run(input), 8);
        let input = "./src/sample2-3.txt";
        assert_eq!(run(input), 10);
    }
}

