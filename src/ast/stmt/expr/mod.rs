pub mod binary_expr;
pub mod unary_expr;

use std::fmt;

use binary_expr::BinaryExpr;
use pest::iterators::Pair;
use unary_expr::UnaryExpr;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
        token::literals::Literal,
    },
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub enum Expr {
    Binary(Node<BinaryExpr>),
    Unary(Node<UnaryExpr>),
    Primary(Node<Literal>),
}

impl Build for Expr {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        Ok(Self::Primary(Node {
            value: Literal::Null(),
            line: 0,
            column: 0,
        }))
    }
}

impl AstDisplay for Expr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Self::Binary(node) => node.ast_fmt(f, prefix),
            Self::Unary(node) => node.ast_fmt(f, prefix),
            Self::Primary(node) => writeln!(f, "{}literal: {}", prefix, node.value),
        }
    }
}
