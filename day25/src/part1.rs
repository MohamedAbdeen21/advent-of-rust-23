use rustworkx_core::{
    connectivity::stoer_wagner_min_cut,
    petgraph::{adj::NodeIndex, graph::UnGraph},
    Result,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut nodes: HashSet<&str> = HashSet::new();

    input.split_terminator("\n").for_each(|line| {
        let (left, right) = line.split_once(": ").unwrap();
        let right: Vec<&str> = right.split_whitespace().collect();
        edges.insert(left, right.clone());
        nodes.insert(left);
        nodes.extend(right);
    });

    let mut graph = UnGraph::new_undirected();

    let nodes_map: HashMap<&str, NodeIndex<_>> = nodes
        .iter()
        .map(|node| (*node, graph.add_node(())))
        .collect();

    edges.iter().for_each(|(node, neighbours)| {
        for n in neighbours {
            graph.add_edge(nodes_map[node], nodes_map[n], 0);
        }
    });

    let minimum_cut: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (min_cut, partition) = minimum_cut.unwrap().unwrap();

    println!("{:?}", min_cut);
    return (partition.len() * (nodes.len() - partition.len())) as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 54);
    }
}
