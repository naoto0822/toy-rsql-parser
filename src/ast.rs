#[derive(Debug, Clone)]
pub enum Statement {
    Select {
        columns: Vec<Column>,
        table: TableExpression,
    },
}

#[derive(Debug, Clone)]
pub enum ValueExpression {
    Identifier(String),
    Number(i64),
    Bool(bool),
    Prefix {
        op: PrefixOp,
        right: i64,
    }, // TODO: right to Box<Expr>
    Infix {
        op: InfixOp,
        left: Box<ValueExpression>,
        right: Box<ValueExpression>,
    }, // TODO: right, left to Box<Expr>
}

#[derive(Debug, Clone)]
pub struct Column {
    pub value: ValueExpression,
    pub alias: String,
}

#[derive(Debug, Clone)]
pub enum PrefixOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub enum InfixOp {
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
    pub group_by: Option<Vec<Column>>,
}
