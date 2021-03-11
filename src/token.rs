use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Start, // Start
    Illegal, // Illegal
    EOF, // EOF
    Ast, // *
    Comma, //
    Lparen, // (
    Rparen, // )
    SemiColon, // ;
    Equal, // =
    Join,
    InnerJoin, // INNER
    LeftJoin, // LEFT,
    RightJoin, // RIGHT
    Select, // SELECT
    FromTable, // FROM
    Where, // WHERE
    Ident, // Identifier
}

// TODO: move static method in Token
pub fn lookup_ident(ident: String) -> TokenType {
    let mut keywords = HashMap::new();
    keywords.insert("SELECT", TokenType::Select);
    keywords.insert("FROM", TokenType::FromTable);
    keywords.insert("WHERE", TokenType::Where);

    match keywords.get(ident.as_str()) {
        Some(v) => {
            v.clone()
        },
        None => {
            TokenType::Ident
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn start_token() -> Token {
        Token{
            token_type: TokenType::Start,
            literal: "".to_string(),
        }
    }

    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token{
            token_type,
            literal,
        }
    }
}
