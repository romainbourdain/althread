use std::fmt;

use crate::{
    ast::{display::AstDisplay, node::Node, token::unary_op::UnaryOp},
    write_indent,
};

use super::Expr;

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Node<UnaryOp>,
    pub operand: Box<Node<Expr>>,
}

impl AstDisplay for UnaryExpr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        write_indent!(f, indent_level, "unary_expr:")?;
        write_indent!(f, indent_level + 1, "expr:")?;
        self.operand.ast_fmt(f, indent_level + 1)?;
        write_indent!(f, indent_level + 1, "op: {}", self.operator)?;

        Ok(())
    }
}
