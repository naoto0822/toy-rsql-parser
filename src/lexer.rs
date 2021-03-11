use crate::token::{Token, TokenType, lookup_ident};

pub struct Lexer {
    query: String,
    query_length: u8,
    current_position: u8,
    next_position: u8,
    current_char: char,
    is_plus_one: bool,
}

impl Lexer {
    pub fn new(query: String) -> Lexer {
        let query_length = query.chars().count() as u8;
        let default_cahr = 0 as char;

        let mut lexer = Lexer{
            query: query,
            query_length: query_length,
            current_position: 0,
            next_position: 0,
            current_char: default_cahr,
            is_plus_one: false,
        };

        // to first char
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.next_position >= self.query_length {
            // '0' is EOF
            self.current_char = '0';
        } else {
            let current_char = self.query.as_bytes()[self.next_position as usize];
            self.current_char = current_char as char;
        }

        self.current_position = self.next_position;
        self.next_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.current_position;
        while self.current_char.is_ascii_alphabetic() {
            self.read_char()
        }

        // FIXME: like index range access...  query[position:self.current_position]
        let cap = self.current_position - position;
        let mut collected: Vec<u8> = Vec::with_capacity(cap as usize);
        for (i, q) in self.query.as_bytes().iter().enumerate() {
            if position <= i as u8 && (i as u8) < self.current_position {
                collected.push(q.clone());
            }
        }

        String::from_utf8(collected).unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token =  match self.current_char {
            ';' => Token::new(TokenType::SemiColon, self.current_char.to_string()),
            '(' => Token::new(TokenType::Lparen, self.current_char.to_string()),
            ')' => Token::new(TokenType::Rparen, self.current_char.to_string()),
            '*' => Token::new(TokenType::Ast, self.current_char.to_string()),
            '0' => Token::new(TokenType::EOF, "".to_string()),
            _ => {
                if self.current_char.is_ascii_alphabetic() {
                    self.is_plus_one = true;
                    let ident = self.read_identifier();
                    let token_type = lookup_ident(ident.clone());
                    Token::new(token_type, ident.clone())
                } else {
                    Token::new(TokenType::Illegal, "".to_string())
                }
            }
        };

        // position is plus on when token is identifier, number...
        if self.is_plus_one {
            self.is_plus_one = false;
            return token;
        }

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.current_char == ' ' || self.current_char == '\n' {
                self.read_char()
            } else {
                break;
            }
        }
    }

    pub fn dump_raw_query(self) {
        println!("query: {}", self.query)
    }
}
