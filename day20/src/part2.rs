use super::modules::types::*;
use num::integer::lcm;
use std::{collections::HashMap, fs};

fn process<'a>(
    current: Vec<(&'a str, bool)>,
    graph: &HashMap<&str, Vec<&'a str>>,
    modules: &mut HashMap<&'a str, Module>,
    to_track: &mut HashMap<(&'a str, bool), u64>,
    presses: u64,
) {
    if current.is_empty() {
        return;
    }
    let mut next = Vec::new();

    for (node, pulse) in current.iter() {
        if to_track.contains_key(&(node, *pulse)) {
            to_track.insert((node, *pulse), presses);
        }

        for child in graph[node].iter() {
            if let Some(newpulse) = modules
                .get_mut(child)
                .unwrap_or(&mut Module::new(Class::Void))
                .send(node, *pulse)
            {
                next.push((*child, newpulse));
            }
        }
    }

    process(next, graph, modules, to_track, presses)
}

pub fn run(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut modules: HashMap<&str, Module> = HashMap::new();

    input.trim().split('\n').for_each(|line| {
        let (module, children) = line.split_once(" -> ").unwrap();
        let (name, class) = match module.chars().next().unwrap() {
            '&' => (&module[1..], Class::Conjunction(HashMap::new())),
            '%' => (&module[1..], Class::FlipFlop(false)),
            'b' => (module, Class::Broadcaster),
            other => panic!("Got class {other} while parsing modules"),
        };
        graph.insert(&name, children.split(", ").collect());
        modules.insert(&name, Module::new(class));
    });

    // find parents of each conjunction, because these need to be initialized to false
    modules
        .iter_mut()
        .filter(|(_, module)| matches!(module.class, Class::Conjunction(_)))
        .for_each(|(name, module)| {
            for (k, v) in graph.iter() {
                if v.contains(name) {
                    module.add_parent(*k);
                }
            }
        });

    // Find the modules that feed "bn" (bn is the only module that feeds into rx)
    // all of which must be true for "bn" to feed false into rx
    // Keep track of when they send true and then find the LCM
    // of theses values.
    let mut to_track = HashMap::new();
    graph
        .iter()
        .filter(|(_, children)| children.contains(&"bn"))
        .map(|(node, _)| node)
        .for_each(|&node| {
            to_track.insert((node, true), 0);
        });

    let mut presses = 0;
    loop {
        presses += 1;
        process(
            Vec::from([("broadcaster", false)]),
            &graph,
            &mut modules,
            &mut to_track,
            presses,
        );

        if to_track.values().all(|&v| v != 0) {
            break;
        }
    }

    // .values() return references, we want values and we're fine with consuming the map
    to_track.into_values().reduce(|a, b| lcm(a, b)).unwrap()
}

// This part has no tests
