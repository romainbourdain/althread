use crate::ast::token::Token;

use super::Expr;

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token<UnaryOp>,
    pub operand: Box<Expr>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum UnaryOp {
    Pos,
    Neg,
    Not,
}
