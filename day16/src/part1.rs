use std::{collections::HashSet, fs};

fn trace(
    grid: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    dir: (isize, isize),
    seen: &mut HashSet<(usize, usize, isize, isize)>,
) {
    if seen.contains(&(i, j, dir.0, dir.1)) {
        return;
    }

    let (mut i, mut j) = (i, j);
    loop {
        energized[i][j] = '#';
        seen.insert((i, j, dir.0, dir.1));

        // We only expect it to underflow, not overflow
        (i, j) = (i.wrapping_add_signed(dir.0), j.wrapping_add_signed(dir.1));

        if i >= grid.len() || j >= grid[0].len() {
            break;
        }

        match (grid[i][j], dir.0, dir.1) {
            ('\\', _, _) => break trace(grid, energized, i, j, (dir.1, dir.0), seen),
            ('/', _, _) => break trace(grid, energized, i, j, (-dir.1, -dir.0), seen),
            ('-', _, 0) => {
                trace(grid, energized, i, j, (0, 1), seen);
                break trace(grid, energized, i, j, (0, -1), seen);
            }
            ('|', 0, _) => {
                trace(grid, energized, i, j, (1, 0), seen);
                break trace(grid, energized, i, j, (-1, 0), seen);
            }
            _ => (),
        }
    }
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();
    let mut memo = HashSet::new();
    let mut energized = grid.clone();

    // i HATE usize, now we have to process first cell separately
    let dir = match grid[0][0] {
        '\\' | '|' => (1, 0),
        '/' => (-1, 0),
        _ => (0, 1),
    };

    trace(&grid, &mut energized, 0, 0, dir, &mut memo);

    energized.iter().flatten().filter(|&&c| c == '#').count() as u64
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

