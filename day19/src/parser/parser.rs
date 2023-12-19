use core::panic;
use std::collections::HashMap;

use crate::tokenizer::tokens::{Token, TokenLiteral};

#[derive(Debug)]
pub struct Part {
    pub attributes: HashMap<TokenLiteral, u64>,
}

fn create_flow(
    attribute: TokenLiteral,
    operator: TokenLiteral,
    value: String,
    then: String,
) -> Box<dyn Fn(&Part) -> Option<String>> {
    match operator {
        TokenLiteral::LT => {
            return Box::new(move |part: &Part| {
                println!("{:?} lt {:?}?", part.attributes[&attribute], value);
                if part.attributes[&attribute] < value.parse().unwrap() {
                    Some(then.clone())
                } else {
                    None
                }
            })
        }
        TokenLiteral::GT => {
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

fn create_terminal(value: String) -> Box<dyn Fn(&Part) -> Option<String>> {
    return Box::new(move |_: &Part| Some(value.clone()));
}

fn parse_stage(tokens: Vec<Token>) -> (String, Vec<Box<dyn Fn(&Part) -> Option<String>>>) {
    if tokens[0].class != TokenLiteral::Stage {
        panic!("first token should be a Stage Literal");
    }

    if tokens[1].class != TokenLiteral::Lbracket {
        panic!("second token should be a Lbracket Literal");
    }

    if tokens[tokens.len() - 1].class != TokenLiteral::Rbracket {
        panic!("Last token should be a Rbracket Literal")
    }
    let key = tokens[0].clone().value.unwrap();
    let mut iterator = tokens.iter().skip(2);

    let mut fns = Vec::new();
    loop {
        let partition: Vec<Token> = iterator
            .by_ref()
            .take_while(|x| x.class != TokenLiteral::Comma)
            .cloned()
            .collect();
        if partition.len() == 0 {
            break;
        }
        match partition[1].class {
            TokenLiteral::LT => {
                fns.push(create_flow(
                    partition[0].class,
                    partition[1].class,
                    partition[2].clone().value.unwrap(),
                    partition[4].clone().value.unwrap(),
                ));
            }
            TokenLiteral::GT => fns.push(create_flow(
                partition[0].class,
                partition[1].class,
                partition[2].clone().value.unwrap(),
                partition[4].clone().value.unwrap(),
            )),
            TokenLiteral::Rbracket => {
                fns.push(create_terminal(partition[0].clone().value.unwrap()))
            }
            s => panic!("wtf is {:?}", s),
        }
    }

    (key, fns)
}

fn parse_part(tokens: Vec<Token>) -> Part {
    if tokens.len() != 17 {
        panic!("Expected part tokens len to be 17");
    }
    if tokens[0].class != TokenLiteral::Lbracket {
        panic!(
            "first token should be a Lbracket Literal, found {:?}",
            tokens[0].class
        );
    }

    if tokens[tokens.len() - 1].class != TokenLiteral::Rbracket {
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
            .take_while(|x| x.class != TokenLiteral::Comma)
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
pub fn parse(
    tokens: Vec<Token>,
) -> (
    HashMap<String, Vec<Box<dyn Fn(&Part) -> Option<String>>>>,
    Vec<Part>,
) {
    let mut stages = HashMap::new();
    let mut parts = Vec::new();
    let mut iterator = tokens.iter();
    loop {
        let partition: Vec<Token> = iterator
            .by_ref()
            .take_while(|x| x.class != TokenLiteral::EOL)
            .cloned()
            .collect();
        if partition.len() == 0 {
            break;
        }
        match partition[0].class {
            TokenLiteral::Stage => {
                let (k, v) = parse_stage(partition);
                stages.insert(k, v);
            }
            TokenLiteral::Lbracket => parts.push(parse_part(partition)),
            _ => panic!("Lines should only start with stage names or {{"),
        }
    }

    return (stages, parts);
}
