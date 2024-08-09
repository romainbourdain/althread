pub mod binary_expr;
pub mod unary_expr;

use std::fmt;

use binary_expr::BinaryExpr;
use pest::iterators::Pair;
use unary_expr::UnaryExpr;

use crate::{
    ast::{
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Expr::Binary(binary_expr) => write!(f, "{}", binary_expr),
            Expr::Unary(unary_expr) => write!(f, "{}", unary_expr),
            Expr::Primary(primary_expr) => write!(f, "{}", primary_expr),
        }
    }
}
