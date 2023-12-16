use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
    fs,
};

// should use struct to manage state (position and turns)

fn traverse(grid: &Vec<Vec<char>>, s: (isize, isize, isize, isize)) -> u64 {
    let mut queue: VecDeque<(isize, isize, isize, isize)> = VecDeque::new();
    let mut seen: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    queue.push_back(s);

    while !queue.is_empty() {
        let head = queue.pop_front().unwrap();
        let x = head.0 as usize;
        let y = head.1 as usize;
        let (dx, dy) = (head.2, head.3);
        if head.0 < 0 || head.1 < 0 || x >= grid.len() || y >= grid[0].len() {
            continue;
        }
        if seen.contains(&head) {
            continue;
        }
        seen.insert(head);

        match (grid[x][y], dx, dy) {
            ('|', 0, _) => {
                queue.push_back((head.0 - 1, head.1, -1, 0));
                queue.push_back((head.0 + 1, head.1, 1, 0));
            }
            ('-', _, 0) => {
                queue.push_back((head.0, head.1 - 1, 0, -1));
                queue.push_back((head.0, head.1 + 1, 0, 1));
            }
            ('/', _, _) => queue.push_back((head.0 - dy, head.1 - dx, -dy, -dx)),
            ('\\', _, _) => queue.push_back((head.0 + dy, head.1 + dx, dy, dx)),
            _ => queue.push_back((head.0 + dx, head.1 + dy, dx, dy)),
        }
    }
    seen.iter()
        .map(|t| (t.0, t.1))
        .collect::<HashSet<(isize, isize)>>()
        .len() as u64
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    max(
        (0..grid.len())
            .map(|idx| {
                max(
                    traverse(&grid, (idx as isize, 0 as isize, 0, 1)),
                    traverse(&grid, (idx as isize, (grid[0].len() - 1) as isize, 0, -1)),
                )
            })
            .max()
            .unwrap(),
        (0..grid[0].len())
            .map(|idx| {
                max(
                    traverse(&grid, (0, idx as isize, 1, 0)),
                    traverse(&grid, ((grid.len() - 1) as isize, idx as isize, -1, 0)),
                )
            })
            .max()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 51);
    }
}
