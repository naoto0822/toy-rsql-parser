use crate::ast::{Column, InfixOp, PrefixOp, Statement, TableExpression, ValueExpression};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

static LOWEAT: i64 = 1;
static OR: i64 = 2; // OR
static AND: i64 = 3; // AND
static EQUALS: i64 = 4; // ==
static LESSGREATER: i64 = 5; // > or <
static SUM: i64 = 6; // +
static PRODUCT: i64 = 7; // *
static PREFIX: i64 = 8; // -X or +X

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

        let first_expr = self.parse_value_expression(LOWEAT);
        let first_column = Column {
            value: first_expr,
            alias: "".to_string(),
        };
        columns.push(first_column);

        while self.is_peek_token(TokenType::Comma) {
            self.next_token();
            self.next_token();

            let next_expr = self.parse_value_expression(LOWEAT);
            let next_column = Column {
                value: next_expr,
                alias: "".to_string(),
            };
            columns.push(next_column);
        }

        columns
    }

    fn parse_value_expression(&mut self, precedence: i64) -> ValueExpression {
        let mut left = self.parse_prefix_value_expression();

        while !self.is_peek_token(TokenType::SemiColon)
            && precedence < self.lookup_precedence(self.peek_token.value.clone())
        {
            let mut infix = self.parse_infix_value_expression(left.clone());
            match infix {
                Some(v) => {
                    left = v.clone();
                }
                None => {
                    return left.clone();
                }
            }
        }

        left
    }

    fn parse_prefix_value_expression(&mut self) -> ValueExpression {
        match self.current_token.value {
            TokenType::Ident(_) => self.parse_identifier(),
            TokenType::Number(_) => self.parse_number(),
            TokenType::Bool(_) => self.parse_bool(),
            _ => panic!("not expected tyep!"),
        }
    }

    // TODO
    fn parse_infix_value_expression(&mut self, left: ValueExpression) -> Option<ValueExpression> {
        match self.peek_token.value {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Slash
            | TokenType::Ast
            | TokenType::EqOp
            | TokenType::LessOp
            | TokenType::GreaterOp => {
                self.next_token();
                Some(self.parse_infix_expression(left))
            }
            _ => None,
        }
    }

    fn parse_infix_expression(&mut self, left: ValueExpression) -> ValueExpression {
        let op = self.lookup_infix(self.current_token.value.clone());

        let precedence = self.lookup_precedence(self.current_token.value.clone());
        self.next_token();

        let right = self.parse_value_expression(precedence);
        ValueExpression::Infix {
            op,
            left: Box::new(left),
            right: Box::new(right),
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

        let table_name = match &self.current_token.value {
            TokenType::Ident(table_name) => table_name.clone(),
            _ => panic!("not expected type!"),
        };

        let where_cond = if self.expect_peek(TokenType::Where) {
            self.next_token();
            Some(self.parse_value_expression(LOWEAT))
        } else {
            None
        };

        TableExpression {
            from: table_name.to_string(),
            where_cond: where_cond,
            group_by: None,
        }
    }

    fn lookup_precedence(&self, token_type: TokenType) -> i64 {
        // TODO: define global var
        let mut ps = HashMap::new();

        ps.insert(TokenType::EqOp, EQUALS);
        // TODO: NOTEQ
        ps.insert(TokenType::LessOp, LESSGREATER);
        ps.insert(TokenType::GreaterOp, LESSGREATER);
        ps.insert(TokenType::Plus, SUM);
        ps.insert(TokenType::Minus, SUM);
        // TODO: slash
        ps.insert(TokenType::Ast, PRODUCT);

        match ps.get(&token_type) {
            Some(v) => v.clone(),
            None => 0,
        }
    }

    fn lookup_prefix(&self, token_type: TokenType) -> PrefixOp {
        let mut p = HashMap::new();
        p.insert(TokenType::Plus, PrefixOp::Plus);
        p.insert(TokenType::Minus, PrefixOp::Minus);

        match p.get(&token_type) {
            Some(v) => v.clone(),
            None => panic!("not expected type!"),
        }
    }

    fn lookup_infix(&self, token_type: TokenType) -> InfixOp {
        let mut i = HashMap::new();
        i.insert(TokenType::EqOp, InfixOp::Eq);
        i.insert(TokenType::Plus, InfixOp::Plus);
        i.insert(TokenType::Minus, InfixOp::Minus);
        i.insert(TokenType::Ast, InfixOp::Ast);
        i.insert(TokenType::Slash, InfixOp::Slash);
        i.insert(TokenType::Bang, InfixOp::Bang);
        i.insert(TokenType::LessOp, InfixOp::Lt);
        i.insert(TokenType::GreaterOp, InfixOp::Gt);

        match i.get(&token_type) {
            Some(v) => v.clone(),
            None => panic!("not expected type!"),
        }
    }
}
