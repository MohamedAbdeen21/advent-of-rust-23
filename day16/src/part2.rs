use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
    fs,
};

// should use struct to manage state (position and turns)
#[derive(Eq, PartialEq, Hash, Clone)]
struct Step {
    x: usize,
    y: usize,
    direction: (isize, isize),
}

impl Step {
    fn new(x: usize, y: usize, dx: isize, dy: isize) -> Step {
        Step {
            x,
            y,
            direction: (dx, dy),
        }
    }
}

fn traverse(grid: &Vec<Vec<char>>, s: Step) -> u64 {
    let mut queue: VecDeque<Step> = VecDeque::new();
    let mut seen: HashSet<Step> = HashSet::new();
    queue.push_back(s);

    while !queue.is_empty() {
        let head = queue.pop_front().unwrap();
        let (x, y) = (head.x, head.y);
        let (dx, dy) = head.direction;
        if x >= grid.len() || y >= grid[0].len() {
            continue;
        }
        if seen.contains(&head) {
            continue;
        }
        seen.insert(head.clone());

        match (grid[x][y], dx, dy) {
            ('/', _, _) => queue.push_back(Step::new(
                x.wrapping_add_signed(-dy),
                y.wrapping_add_signed(-dx),
                -dy,
                -dx,
            )),
            ('\\', _, _) => queue.push_back(Step::new(
                x.wrapping_add_signed(dy),
                y.wrapping_add_signed(dx),
                dy,
                dx,
            )),
            ('|', 0, _) => {
                queue.push_back(Step::new(x.wrapping_add_signed(-1), head.y, -1, 0));
                queue.push_back(Step::new(head.x + 1, head.y, 1, 0));
            }
            ('-', _, 0) => {
                queue.push_back(Step::new(head.x, head.y - 1, 0, -1));
                queue.push_back(Step::new(head.x, head.y + 1, 0, 1));
            }
            _ => queue.push_back(Step::new(
                x.wrapping_add_signed(dx),
                y.wrapping_add_signed(dy),
                dx,
                dy,
            )),
        }
    }

    seen.iter()
        .map(|step| (step.x, step.y))
        .collect::<HashSet<(usize, usize)>>()
        .len() as u64
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let rows_max = (0..grid.len())
        .map(|idx| {
            max(
                traverse(&grid, Step::new(idx, 0, 0, 1)),
                traverse(&grid, Step::new(idx, grid[0].len() - 1, 0, -1)),
            )
        })
        .max()
        .unwrap();

    let cols_max = (0..grid[0].len())
        .map(|idx| {
            max(
                traverse(&grid, Step::new(0, idx, 1, 0)),
                traverse(&grid, Step::new(grid.len() - 1, idx, -1, 0)),
            )
        })
        .max()
        .unwrap();

    return max(cols_max, rows_max);
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
