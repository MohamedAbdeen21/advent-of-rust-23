use std::{collections::HashSet, fs};

fn trace(
    grid: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    dir: (isize, isize),
    memo: &mut HashSet<(usize, usize, isize, isize)>,
) {
    if memo.contains(&(i, j, dir.0, dir.1)) {
        return;
    }
    let (mut i, mut j) = (i, j);
    loop {
        energized[i][j] = '#';
        if i == 0 && dir.0 == -1
            || j == 0 && dir.1 == -1
            || i == grid.len() - 1 && dir.0 == 1
            || j == grid[0].len() - 1 && dir.1 == 1
        {
            break;
        }
        memo.insert((i, j, dir.0, dir.1));

        (i, j) = (
            i.checked_add_signed(dir.0).unwrap(),
            j.checked_add_signed(dir.1).unwrap(),
        );

        match grid[i][j] {
            '\\' => break trace(grid, energized, i, j, (dir.1, dir.0), memo),
            '/' => break trace(grid, energized, i, j, (-dir.1, -dir.0), memo),
            '-' => {
                if dir.0 != 0 {
                    trace(grid, energized, i, j, (0, 1), memo);
                    trace(grid, energized, i, j, (0, -1), memo);
                    break;
                }
            }
            '|' => {
                if dir.1 != 0 {
                    trace(grid, energized, i, j, (1, 0), memo);
                    trace(grid, energized, i, j, (-1, 0), memo);
                    break;
                }
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

    // i HATE usize
    let dir = match grid[0][0] {
        '\\' | '|' => (1, 0),
        '/' => (-1, 0),
        _ => (0, 1),
    };

    trace(&grid, &mut energized, 0, 0, dir, &mut memo);

    // for row in energized.iter() {
    //     for c in row {
    //         print!("{c}")
    //     }
    //     println!()
    // }
    // println!();

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
