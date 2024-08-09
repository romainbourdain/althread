use std::fmt;

use crate::ast::{node::Node, token::binary_op::BinaryOp};

use super::Expr;

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Node<Expr>>,
    pub operator: Node<BinaryOp>,
    pub right: Box<Node<Expr>>,
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.left)?;
        write!(f, " {} ", self.operator)?;
        write!(f, "{}", self.right)?;

        Ok(())
    }
}
