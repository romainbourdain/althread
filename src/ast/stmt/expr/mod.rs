pub mod binary_expr;
pub mod primary_expr;
pub mod unary_expr;

use binary_expr::BinaryExpr;
use pest::iterators::Pair;
use primary_expr::PrimaryExpr;
use unary_expr::UnaryExpr;

use crate::{
    ast::node::{Build, Node},
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub enum Expr {
    Binary(Node<BinaryExpr>),
    Unary(Node<UnaryExpr>),
    Primary(Node<PrimaryExpr>),
}

impl Build for Expr {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        Ok(Self::Primary(Node {
            value: PrimaryExpr::Null(),
            line: 0,
            column: 0,
        }))
    }
}
