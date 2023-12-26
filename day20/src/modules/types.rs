use std::collections::HashMap;

#[derive(Debug)]
pub enum Class {
    Void, // terminals (i.e. dummy nodes)
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

#[derive(Debug)]
pub struct Module {
    pub class: Class,
}

impl Module {
    pub fn new(class: Class) -> Self {
        Module { class }
    }

    pub fn add_parent(&mut self, parent: &str) {
        match &mut self.class {
            Class::Conjunction(memory) => memory.insert(parent.to_string(), false),
            other => panic!("Adding parent {parent} to {other:?}"),
        };
    }

    pub fn send(&mut self, from: &str, pulse: bool) -> Option<bool> {
        match &mut self.class {
            Class::Void => None,
            Class::Broadcaster => Some(false),
            Class::FlipFlop(state) => {
                if pulse {
                    return None;
                }
                *state = !*state;
                Some(*state)
            }
            Class::Conjunction(memory) => {
                memory.insert(from.to_string(), pulse);
                Some(!memory.values().all(|&pulse| pulse))
            }
        }
    }
}
