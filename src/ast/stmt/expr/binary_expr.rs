use std::fmt;

use crate::{
    ast::{display::AstDisplay, node::Node, token::binary_op::BinaryOp},
    write_indent,
};

use super::Expr;

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Node<Expr>>,
    pub operator: Node<BinaryOp>,
    pub right: Box<Node<Expr>>,
}

impl AstDisplay for BinaryExpr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        write_indent!(f, indent_level, "binary_expr:")?;
        write_indent!(f, indent_level + 1, "expr:")?;
        self.left.ast_fmt(f, indent_level + 1)?;
        write_indent!(f, indent_level + 1, "op: {}", self.operator)?;
        write_indent!(f, indent_level + 1, "expr:")?;
        self.right.ast_fmt(f, indent_level + 1)?;

        Ok(())
    }
}
