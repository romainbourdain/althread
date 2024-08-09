use crate::ast::node::Node;

use super::Expr;

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Node<Expr>>,
    pub operator: Node<BinaryOp>,
    pub right: Box<Node<Expr>>,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}
