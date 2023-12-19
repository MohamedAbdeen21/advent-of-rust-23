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
                '=' => Token::new(TokenLiteral::EQ, None),
                '>' => Token::new(TokenLiteral::GT, None),
                '<' => Token::new(TokenLiteral::LT, None),
                '{' => Token::new(TokenLiteral::Lbracket, None),
                '}' => Token::new(TokenLiteral::Rbracket, None),
                ',' => Token::new(TokenLiteral::Comma, None),
                ':' => Token::new(TokenLiteral::Colon, None),
                'A' => Token::new(TokenLiteral::Accepted, Some(vec!['A'])),
                'R' => Token::new(TokenLiteral::Rejected, Some(vec!['R'])),
                d if d.is_digit(10) => {
                    let end = iterator[i..].iter().position(|c| !c.is_digit(10)).unwrap() + i;
                    let start = i.clone();
                    i = end - 1;
                    Token::new(TokenLiteral::Int, Some(iterator[start..end].into()))
                }
                'x' if ['>', '<', '='].contains(&iterator[i + 1]) => {
                    Token::new(TokenLiteral::ExtremelyCool, None)
                }
                'm' if ['>', '<', '='].contains(&iterator[i + 1]) => {
                    Token::new(TokenLiteral::Musical, None)
                }
                'a' if ['>', '<', '='].contains(&iterator[i + 1]) => {
                    Token::new(TokenLiteral::Aerodynamic, None)
                }
                's' if ['>', '<', '='].contains(&iterator[i + 1]) => {
                    Token::new(TokenLiteral::Shiny, None)
                }
                _ => {
                    let end = iterator[i..]
                        .iter()
                        .position(|c| !c.is_alphabetic())
                        .unwrap()
                        + i;
                    let start = i.clone();
                    i = end - 1;
                    Token::new(TokenLiteral::Stage, Some(iterator[start..end].into()))
                }
            }
        });
        i += 1;
    }
    tokens.push(Token::new(TokenLiteral::EOL, None));
    tokens
}
