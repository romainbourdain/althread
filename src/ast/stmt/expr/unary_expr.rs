use std::fmt;

use crate::ast::{node::Node, token::unary_op::UnaryOp};

use super::Expr;

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Node<UnaryOp>,
    pub operand: Box<Node<Expr>>,
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.operator)?;
        write!(f, "{}", self.operand)?;

        Ok(())
    }
}
