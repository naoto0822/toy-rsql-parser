use std::fmt;
use crate::token::{Token};

#[derive(Debug, Clone)]
pub enum Statement {
    Select{columns: Vec<Column>, table: TableExpression}
}

#[derive(Debug, Clone)]
pub enum ValueExpression {
    Identifier(String),
    Number(i64),
    Bool(bool),
    // Prefix,
    // Infix
}

#[derive(Debug, Clone)]
pub struct Column {
    pub value: ValueExpression,
    pub alias: String,
}

#[derive(Debug, Clone)]
pub enum Prefix {
    Plus,
    Minus
}

#[derive(Debug, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Ast,
    Slash,
    Bang,
    Eq,
    NotEq,
    Lt,
    Gt,
}

#[derive(Debug, Clone)]
pub struct TableExpression {
   pub from: String,
   pub where_cond: Option<ValueExpression>,
   pub group_by: Option<Vec<Column>>
}
