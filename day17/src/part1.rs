use crate::state::State;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::rc::Rc;

fn dijkstra(grid: Vec<Vec<u8>>) -> (u64, State) {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut seen: HashSet<State> = HashSet::new();
    queue.push(State::new(0, 0, 0, 0, 0, 0));

    loop {
        let node = queue.pop().unwrap();

        if (node.x, node.y) == (grid.len() - 1, grid[0].len() - 1) {
            break (node.heatloss, node);
        }

        if seen.contains(&node.hash()) {
            continue;
        }

        seen.insert(node.hash());

        let self_reference = Rc::new(node.clone());
        if node.steps < 3 && node.direction != (0, 0) {
            let next = node.step(Rc::clone(&self_reference));
            if next.x < grid.len() && next.y < grid[0].len() {
                queue.push(next.add_loss(grid[next.x][next.y]))
            }
        }

        for turn in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
            if node.direction == turn || node.direction == (-turn.0, -turn.1) {
                continue;
            }
            let next = node.turn(turn, Rc::clone(&self_reference));
            if next.x >= grid.len() || next.y >= grid[0].len() {
                continue;
            }
            queue.push(next.add_loss(grid[next.x][next.y]))
        }
    }
}

fn parse(input: String) -> Vec<Vec<u8>> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect()
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let grid = parse(input);
    let (score, _node) = dijkstra(grid);

    // let mut node = &_node;
    // while let Some(parent) = &node.parent {
    //     println!("{}, {}, {}", parent.x, parent.y, parent.steps);
    //     node = parent;
    // }

    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 102);
    }
}
