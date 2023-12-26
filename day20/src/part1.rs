use super::modules::types::*;
use std::{collections::HashMap, fs};

fn process<'a>(
    current: Vec<(&str, bool)>,
    graph: &HashMap<&str, Vec<&'a str>>,
    modules: &mut HashMap<&'a str, Module>,
    lows: &mut u64,
    highs: &mut u64,
) {
    if current.is_empty() {
        return;
    }
    let mut next = Vec::new();

    for (node, pulse) in current.iter() {
        for child in graph[node].iter() {
            if *pulse {
                *highs += 1;
            } else {
                *lows += 1;
            }

            if let Some(newpulse) = modules
                .get_mut(child)
                .unwrap_or(&mut Module::new(Class::Void))
                .send(node, *pulse)
            {
                next.push((*child, newpulse));
            }
        }
    }

    process(next, graph, modules, lows, highs)
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

    let mut lows = 0;
    let mut highs = 0;

    for _ in 0..1000 {
        lows += 1;
        process(
            Vec::from([("broadcaster", false)]),
            &graph,
            &mut modules,
            &mut lows,
            &mut highs,
        );
    }

    // println!("{lows}, {highs}");
    lows * highs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "./src/sample.txt";
        assert_eq!(run(input), 32_000_000);
        let input2 = "./src/sample2.txt";
        assert_eq!(run(input2), 11_687_500);
        let input3 = "./src/sample3.txt";
        assert_eq!(run(input3), 11_250_000);
    }
}
