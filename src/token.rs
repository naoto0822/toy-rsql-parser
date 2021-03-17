use crate::annot::Annot;
use std::collections::HashMap;

pub const CHAR_ZERO_VALUE: char = 0 as char;
pub const CHAR_EOF: char = '0';

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    // System Type
    Start,   // Start
    Illegal(String), // Illegal
    EOF,     // EOF

    // Spacecial Char
    DoubleQuote, // "
    Percent,     // %
    Ampersand,   // &
    Quote,       // '
    Lparen,      // (
    Rparen,      // )
    Ast,         // *
    Comma,       // ,
    Plus,        // +
    Minus,       // -
    Period,      // .
    Colon,       // :
    SemiColon,   // ;
    LessOp,      // <
    EqOp,        // =
    GreaterOp,   // >
    LeftBra,     // [
    RightBra,    // ]
    UnderScore,  // _
    VerticalBar, // |
    LeftBrace,   // {
    RightBrace,  // }
    DollerSign,  // $

    // Reserved Word
    Select,    // SELECT
    FromTable, // FROM
    Where,     // WHERE
    Join,      // JOIN
    InnerJoin, // INNER
    LeftJoin,  // LEFT,
    RightJoin, // RIGHT
    Is, // IS
    Not, // NOT
    NULL, // NULL

    // Latter, Number,,,
    Ident(String), // a~z, A~Z, 0~9
    Number(i64),   // 0 ~9
    Bool(bool), // true or false
}

pub type Token = Annot<TokenType>;

impl Token {
    pub fn start_token() -> Self {
        Self::new(TokenType::Start)
    }

    pub fn lookup_ident(ident: String) -> TokenType {
        let mut keywords = HashMap::new();
        keywords.insert("SELECT", TokenType::Select);
        keywords.insert("FROM", TokenType::FromTable);
        keywords.insert("WHERE", TokenType::Where);
        keywords.insert("JOIN", TokenType::Join);
        keywords.insert("INNER", TokenType::InnerJoin);
        keywords.insert("LEFT", TokenType::LeftJoin);
        keywords.insert("RIGHT", TokenType::RightJoin);

        match keywords.get(ident.as_str()) {
            Some(v) => v.clone(),
            None => TokenType::Ident(ident),
        }
    }
}
