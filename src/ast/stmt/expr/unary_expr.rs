use crate::ast::node::Node;

use super::Expr;

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Node<UnaryOp>,
    pub operand: Box<Node<Expr>>,
}

#[derive(Debug)]
pub enum UnaryOp {
    Pos,
    Neg,
    Not,
}
