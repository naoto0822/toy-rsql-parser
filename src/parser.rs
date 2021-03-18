use crate::ast::{Column, Statement, TableExpression, ValueExpression};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

enum PrecedenceType {
    LOWEAT = 1,
    EQUALS = 2,      // ==
    LESSGREATER = 3, // > or <
    SUM = 4,         // +
    PRODUCT = 5,     // *
    PREFIX = 6,      // -X or +X
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let current_token = Token::start_token();
        let peek_token = Token::start_token();

        let mut p = Self {
            lexer,
            current_token,
            peek_token,
        };

        p.next_token();
        p.next_token();
        p
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut stmts: Vec<Statement> = vec![];

        while &self.current_token.value != &TokenType::EOF {
            // TODO: illegal error handling
            let stmt = self.parse_statement();
            stmts.push(stmt);
            self.next_token();
        }

        stmts
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();

        let next_token = self.lexer.next_token();
        match next_token {
            Some(tk) => {
                self.peek_token = tk;
            }
            None => panic!("not expected None"),
        }
    }

    fn is_current_token(&self, token_type: TokenType) -> bool {
        self.current_token.value == token_type
    }

    fn is_peek_token(&self, token_type: TokenType) -> bool {
        self.peek_token.value == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.is_peek_token(token_type) {
            self.next_token();
            return true;
        }

        // TODO: error handling
        false
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token.value {
            TokenType::Select => self.parse_select_statement(),
            _ => panic!("not supported statement!"),
        }
    }

    fn parse_select_statement(&mut self) -> Statement {
        self.next_token();

        let columns = self.parse_columns();

        if !self.expect_peek(TokenType::FromTable) {
            panic!("not expected type");
        }

        let table = self.parse_table_expression();

        if self.expect_peek(TokenType::SemiColon) {
            self.next_token();
        }

        Statement::Select { columns, table }
    }

    // TODO: alias
    fn parse_columns(&mut self) -> Vec<Column> {
        let mut columns = vec![];

        let first_expr = self.parse_value_expression();
        let first_column = Column {
            value: first_expr,
            alias: "".to_string(),
        };
        columns.push(first_column);

        while self.is_peek_token(TokenType::Comma) {
            self.next_token();
            self.next_token();

            let next_expr = self.parse_value_expression();
            let next_column = Column {
                value: next_expr,
                alias: "".to_string(),
            };
            columns.push(next_column);
        }

        columns
    }

    fn parse_value_expression(&mut self) -> ValueExpression {
        match self.current_token.value {
            TokenType::Ident(_) => self.parse_identifier(),
            TokenType::Number(_) => self.parse_number(),
            TokenType::Bool(_) => self.parse_bool(),
            _ => panic!("not expected tyep!"),
        }
    }

    fn parse_identifier(&mut self) -> ValueExpression {
        match &self.current_token.value {
            TokenType::Ident(ident) => ValueExpression::Identifier(ident.to_string()),
            _ => panic!("not expected type!"),
        }
    }

    fn parse_number(&mut self) -> ValueExpression {
        match self.current_token.value {
            TokenType::Number(num) => ValueExpression::Number(num),
            _ => panic!("not expected type!"),
        }
    }

    fn parse_bool(&mut self) -> ValueExpression {
        match self.current_token.value {
            TokenType::Bool(b) => ValueExpression::Bool(b),
            _ => panic!("not expected type!"),
        }
    }

    // TODO: WHERE, GROUP_BY
    fn parse_table_expression(&mut self) -> TableExpression {
        self.next_token();

        match &self.current_token.value {
            TokenType::Ident(table_name) => TableExpression {
                from: table_name.to_string(),
                where_cond: None,
                group_by: None,
            },
            _ => panic!("not expected type!"),
        }
    }

    fn lookup_precedence(&self, token_type: TokenType) -> i64 {
        // TODO: refactor
    }
}
