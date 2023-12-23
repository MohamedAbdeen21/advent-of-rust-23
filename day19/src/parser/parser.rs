use core::panic;
use std::collections::HashMap;

use crate::tokenizer::tokens::{Token, TokenClass};

#[derive(Debug)]
pub struct Part {
    pub attributes: HashMap<TokenClass, u64>,
}

type Rule = Box<dyn Fn(&Part) -> Option<String>>;

fn create_rule(attribute: TokenClass, operator: TokenClass, value: String, then: String) -> Rule {
    match operator {
        TokenClass::LT => {
            return Box::new(move |part: &Part| {
                println!("{:?} lt {:?}?", part.attributes[&attribute], value);
                if part.attributes[&attribute] < value.parse().unwrap() {
                    Some(then.clone())
                } else {
                    None
                }
            })
        }
        TokenClass::GT => {
            return Box::new(move |part: &Part| {
                if part.attributes[&attribute] > value.parse().unwrap() {
                    Some(then.clone())
                } else {
                    None
                }
            })
        }
        _ => panic!("WTF is this?"),
    }
}

fn create_terminal(value: String) -> Rule {
    return Box::new(move |_: &Part| Some(value.clone()));
}

fn parse_stage(tokens: Vec<Token>) -> (String, Vec<Rule>) {
    if tokens[0].class != TokenClass::Stage {
        panic!("first token should be a Stage Literal");
    }

    if tokens[1].class != TokenClass::Lbracket {
        panic!("second token should be a Lbracket Literal");
    }

    if tokens[tokens.len() - 1].class != TokenClass::Rbracket {
        panic!("Last token should be a Rbracket Literal")
    }
    let key = tokens[0].clone().value.unwrap();
    let mut iterator = tokens.iter().skip(2);

    let mut fns = Vec::new();
    loop {
        let partition: Vec<Token> = iterator
            .by_ref()
            .take_while(|x| x.class != TokenClass::Comma)
            .cloned()
            .collect();
        if partition.len() == 0 {
            break;
        }
        match partition[1].class {
            TokenClass::LT => {
                fns.push(create_rule(
                    partition[0].class,
                    partition[1].class,
                    partition[2].clone().value.unwrap(),
                    partition[4].clone().value.unwrap(),
                ));
            }
            TokenClass::GT => fns.push(create_rule(
                partition[0].class,
                partition[1].class,
                partition[2].clone().value.unwrap(),
                partition[4].clone().value.unwrap(),
            )),
            TokenClass::Rbracket => fns.push(create_terminal(partition[0].clone().value.unwrap())),
            s => panic!("wtf is {:?}", s),
        }
    }

    (key, fns)
}

fn parse_part(tokens: Vec<Token>) -> Part {
    if tokens.len() != 17 {
        panic!("Expected part tokens len to be 17");
    }
    if tokens[0].class != TokenClass::Lbracket {
        panic!(
            "first token should be a Lbracket Literal, found {:?}",
            tokens[0].class
        );
    }

    if tokens[tokens.len() - 1].class != TokenClass::Rbracket {
        panic!(
            "Last token should be a Rbracket Literal, found {:?}",
            tokens[tokens.len() - 1].class
        )
    }
    let mut map = HashMap::new();
    let mut iterator = tokens.iter().skip(1);
    loop {
        let partition: Vec<Token> = iterator
            .by_ref()
            .take_while(|x| x.class != TokenClass::Comma)
            .cloned()
            .collect();
        if partition.len() == 0 {
            break;
        }
        map.insert(
            partition[0].class,
            partition[2].value.as_ref().unwrap().parse().unwrap(),
        );
    }
    Part { attributes: map }
}

// no need for an AST, we can just use maps
pub fn parse(tokens: Vec<Token>) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let mut stages = HashMap::new();
    let mut parts = Vec::new();
    let mut iterator = tokens.iter();
    loop {
        let partition: Vec<Token> = iterator
            .by_ref()
            .take_while(|x| x.class != TokenClass::EOL)
            .cloned()
            .collect();
        if partition.len() == 0 {
            break;
        }
        match partition[0].class {
            TokenClass::Stage => {
                let (k, v) = parse_stage(partition);
                stages.insert(k, v);
            }
            TokenClass::Lbracket => parts.push(parse_part(partition)),
            _ => panic!("Lines should only start with stage names or {{"),
        }
    }

    return (stages, parts);
}
