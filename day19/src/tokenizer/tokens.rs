#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
pub enum TokenLiteral {
    EOL,
    Int,

    LT,
    GT,
    EQ,

    Rbracket,
    Lbracket,
    Comma,
    Colon,

    Accepted,
    Rejected,

    Stage,

    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub class: TokenLiteral,
    pub value: Option<String>, // only int and stage
}

impl Token {
    pub fn new(class: TokenLiteral, symbols: Option<Vec<char>>) -> Token {
        Token {
            class,
            value: symbols.and_then(|sym| Some(sym.iter().collect::<String>())),
        }
    }
}
