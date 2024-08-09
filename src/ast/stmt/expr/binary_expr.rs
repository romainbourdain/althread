use std::fmt;

use crate::ast::{
    display::{AstDisplay, Prefix},
    node::Node,
    token::binary_op::BinaryOp,
};

use super::Expr;

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Node<Expr>>,
    pub operator: Node<BinaryOp>,
    pub right: Box<Node<Expr>>,
}

impl AstDisplay for BinaryExpr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{}binary_expr", prefix)?;

        {
            let prefix = &prefix.add_branch();
            writeln!(f, "{}left", prefix)?;
            self.left.ast_fmt(f, &prefix.add_leaf())?;
        }

        let prefix = &prefix.switch();
        writeln!(f, "{}op: {}", prefix, self.operator)?;

        {
            let prefix = &prefix.add_branch();
            writeln!(f, "{}right", prefix)?;
            self.left.ast_fmt(f, &prefix.add_leaf())?;
        }

        Ok(())
    }
}
