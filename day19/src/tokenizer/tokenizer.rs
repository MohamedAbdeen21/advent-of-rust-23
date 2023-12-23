use super::tokens::*;

pub fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let iterator = line.chars().collect::<Vec<char>>();
    let mut i = 0;
    loop {
        tokens.push({
            let character = iterator.get(i);
            if character.is_none() {
                break;
            }
            match character.unwrap() {
                '=' => Token::new(TokenClass::EQ, None),
                '>' => Token::new(TokenClass::GT, None),
                '<' => Token::new(TokenClass::LT, None),
                '{' => Token::new(TokenClass::Lbracket, None),
                '}' => Token::new(TokenClass::Rbracket, None),
                ',' => Token::new(TokenClass::Comma, None),
                ':' => Token::new(TokenClass::Colon, None),
                'A' => Token::new(TokenClass::Accepted, Some(vec!['A'])),
                'R' => Token::new(TokenClass::Rejected, Some(vec!['R'])),
                d if d.is_digit(10) => {
                    let end = iterator[i..].iter().position(|c| !c.is_digit(10)).unwrap() + i;
                    let start = i.clone();
                    i = end - 1;
                    Token::new(TokenClass::Int, Some(iterator[start..end].into()))
                }
                'x' if ['>', '<', '='].contains(&iterator[i + 1]) => {
                    Token::new(TokenClass::ExtremelyCool, None)
                }
                'm' if ['>', '<', '='].contains(&iterator[i + 1]) => {
                    Token::new(TokenClass::Musical, None)
                }
                'a' if ['>', '<', '='].contains(&iterator[i + 1]) => {
                    Token::new(TokenClass::Aerodynamic, None)
                }
                's' if ['>', '<', '='].contains(&iterator[i + 1]) => {
                    Token::new(TokenClass::Shiny, None)
                }
                _ => {
                    let end = iterator[i..]
                        .iter()
                        .position(|c| !c.is_alphabetic())
                        .unwrap()
                        + i;
                    let start = i.clone();
                    i = end - 1;
                    Token::new(TokenClass::Stage, Some(iterator[start..end].into()))
                }
            }
        });
        i += 1;
    }
    tokens.push(Token::new(TokenClass::EOL, None));
    tokens
}
