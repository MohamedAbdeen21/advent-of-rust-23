use std::{collections::HashSet, fs};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
    fn advance(&self, direction: (isize, isize)) -> Self {
        Position {
            x: self.x.wrapping_add_signed(direction.0),
            y: self.y.wrapping_add_signed(direction.1),
        }
    }
}

fn bfs(grid: &Vec<Vec<char>>, positions: HashSet<Position>, depth: u8) -> HashSet<Position> {
    if depth == 0 {
        return positions;
    }

    let mut answer = HashSet::new();
    for position in positions {
        for direction in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let next = position.advance(direction);
            if next.x > grid.len() || next.y > grid[0].len() || grid[next.x][next.y] == '#' {
                continue;
            }
            answer.insert(next);
        }
    }

    return bfs(grid, answer, depth - 1);
}

pub fn run(filename: &str, steps: u8) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let grid = input
        .split_terminator("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (mut x, mut y) = (0, 0);

    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == &'S' {
                (x, y) = (i, j);
                break 'outer;
            }
        }
    }

    bfs(&grid, HashSet::from([Position::new(x, y)]), steps).len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input, 6), 16);
    }
}
